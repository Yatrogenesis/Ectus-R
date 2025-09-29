// Authentication middleware for API security
import { verifyJWT, extractApiKey } from '../utils/auth.js';

export function authenticate(options = {}) {
  const { required = true, roles = [], apiKeyOnly = false } = options;

  return async (request, env, ctx) => {
    try {
      // Extract authentication from request
      const authResult = await extractAuth(request, env);

      // If authentication is required but not provided
      if (required && !authResult.user) {
        return new Response(JSON.stringify({
          error: 'Authentication required',
          code: 'AUTH_REQUIRED'
        }), {
          status: 401,
          headers: { 'Content-Type': 'application/json' }
        });
      }

      // Check role permissions if roles are specified
      if (authResult.user && roles.length > 0) {
        const userRoles = authResult.user.roles || [];
        const hasPermission = roles.some(role => userRoles.includes(role));

        if (!hasPermission) {
          return new Response(JSON.stringify({
            error: 'Insufficient permissions',
            code: 'INSUFFICIENT_PERMISSIONS',
            required_roles: roles
          }), {
            status: 403,
            headers: { 'Content-Type': 'application/json' }
          });
        }
      }

      // Add user to request context
      request.user = authResult.user;
      request.authMethod = authResult.method;

      return null; // Continue to next middleware
    } catch (error) {
      console.error('Authentication error:', error);

      return new Response(JSON.stringify({
        error: 'Authentication failed',
        code: 'AUTH_FAILED'
      }), {
        status: 401,
        headers: { 'Content-Type': 'application/json' }
      });
    }
  };
}

async function extractAuth(request, env) {
  // Try JWT token first
  const jwtResult = await tryJWTAuth(request, env);
  if (jwtResult.user) {
    return { user: jwtResult.user, method: 'jwt' };
  }

  // Try API key
  const apiKeyResult = await tryApiKeyAuth(request, env);
  if (apiKeyResult.user) {
    return { user: apiKeyResult.user, method: 'api_key' };
  }

  return { user: null, method: null };
}

async function tryJWTAuth(request, env) {
  try {
    const authorization = request.headers.get('Authorization');
    if (!authorization || !authorization.startsWith('Bearer ')) {
      return { user: null };
    }

    const token = authorization.substring(7);
    const payload = await verifyJWT(token, env.JWT_SECRET);

    if (!payload || !payload.sub) {
      return { user: null };
    }

    // Get user from database
    const user = await env.DB.prepare(
      'SELECT id, email, name, plan, is_active FROM users WHERE id = ? AND is_active = 1'
    ).bind(payload.sub).first();

    if (!user) {
      return { user: null };
    }

    // Update last login
    await env.DB.prepare(
      'UPDATE users SET last_login = ? WHERE id = ?'
    ).bind(Date.now(), user.id).run();

    return {
      user: {
        id: user.id,
        email: user.email,
        name: user.name,
        plan: user.plan,
        roles: getUserRoles(user.plan)
      }
    };
  } catch (error) {
    console.error('JWT auth error:', error);
    return { user: null };
  }
}

async function tryApiKeyAuth(request, env) {
  try {
    const apiKey = extractApiKey(request);
    if (!apiKey) {
      return { user: null };
    }

    // Hash the API key for comparison
    const hashedKey = await hashApiKey(apiKey);

    // Look up API key in database
    const keyRecord = await env.DB.prepare(`
      SELECT ak.id, ak.user_id, ak.name, ak.permissions, ak.last_used,
             u.email, u.name as user_name, u.plan, u.is_active
      FROM api_keys ak
      JOIN users u ON ak.user_id = u.id
      WHERE ak.key_hash = ? AND ak.is_active = 1 AND u.is_active = 1
    `).bind(hashedKey).first();

    if (!keyRecord) {
      return { user: null };
    }

    // Update last used timestamp
    await env.DB.prepare(
      'UPDATE api_keys SET last_used = ? WHERE id = ?'
    ).bind(Date.now(), keyRecord.id).run();

    const permissions = keyRecord.permissions ? JSON.parse(keyRecord.permissions) : [];

    return {
      user: {
        id: keyRecord.user_id,
        email: keyRecord.email,
        name: keyRecord.user_name,
        plan: keyRecord.plan,
        roles: getUserRoles(keyRecord.plan),
        api_key_permissions: permissions,
        api_key_name: keyRecord.name
      }
    };
  } catch (error) {
    console.error('API key auth error:', error);
    return { user: null };
  }
}

function getUserRoles(plan) {
  const roleMap = {
    'free': ['user'],
    'pro': ['user', 'pro'],
    'enterprise': ['user', 'pro', 'admin']
  };

  return roleMap[plan] || ['user'];
}

async function hashApiKey(apiKey) {
  const encoder = new TextEncoder();
  const data = encoder.encode(apiKey);
  const hashBuffer = await crypto.subtle.digest('SHA-256', data);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
}

// Rate limiting for failed authentication attempts
export async function rateLimitAuth(request, env) {
  const ip = request.headers.get('CF-Connecting-IP') || 'unknown';
  const key = `auth_attempts:${ip}`;

  try {
    const attempts = await env.CACHE.get(key);
    const count = attempts ? parseInt(attempts) : 0;

    if (count >= 10) { // Max 10 failed attempts per hour
      return new Response(JSON.stringify({
        error: 'Too many authentication attempts',
        code: 'RATE_LIMITED',
        retry_after: 3600
      }), {
        status: 429,
        headers: {
          'Content-Type': 'application/json',
          'Retry-After': '3600'
        }
      });
    }

    return null; // Continue
  } catch (error) {
    console.error('Rate limit check error:', error);
    return null; // Continue on error
  }
}

export async function recordFailedAuth(request, env) {
  const ip = request.headers.get('CF-Connecting-IP') || 'unknown';
  const key = `auth_attempts:${ip}`;

  try {
    const attempts = await env.CACHE.get(key);
    const count = attempts ? parseInt(attempts) + 1 : 1;

    await env.CACHE.put(key, count.toString(), { expirationTtl: 3600 });
  } catch (error) {
    console.error('Failed to record auth attempt:', error);
  }
}