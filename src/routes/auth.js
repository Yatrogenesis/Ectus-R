// Authentication routes for user management
import { signJWT, generateApiKey, hashPassword, verifyPassword, generateUserId, isValidEmail, validatePasswordStrength } from '../utils/auth.js';
import { ValidationError, AuthenticationError } from '../middleware/errorHandler.js';

export function authRoutes(router) {
  // User registration
  router.post('/auth/register', async (request, env) => {
    try {
      const { email, password, name } = await request.json();

      // Validate input
      if (!email || !password) {
        throw new ValidationError('Email and password are required');
      }

      if (!isValidEmail(email)) {
        throw new ValidationError('Invalid email format', 'email');
      }

      const passwordValidation = validatePasswordStrength(password);
      if (!passwordValidation.valid) {
        throw new ValidationError('Password requirements not met', 'password', passwordValidation.errors);
      }

      // Check if user already exists
      const existingUser = await env.DB.prepare(
        'SELECT id FROM users WHERE email = ?'
      ).bind(email).first();

      if (existingUser) {
        throw new ValidationError('User already exists', 'email');
      }

      // Create user
      const userId = generateUserId();
      const hashedPassword = await hashPassword(password);
      const now = Date.now();

      await env.DB.prepare(`
        INSERT INTO users (id, email, name, password_hash, created_at, updated_at, plan)
        VALUES (?, ?, ?, ?, ?, ?, ?)
      `).bind(userId, email, name || '', hashedPassword, now, now, 'free').run();

      // Generate API key
      const apiKey = await generateApiKey();
      const hashedApiKey = await hashApiKey(apiKey);

      await env.DB.prepare(`
        INSERT INTO api_keys (id, user_id, name, key_hash, created_at)
        VALUES (?, ?, ?, ?, ?)
      `).bind(
        `key_${Date.now()}`,
        userId,
        'Default API Key',
        hashedApiKey,
        now
      ).run();

      // Generate JWT token
      const token = await signJWT({ sub: userId, email }, env.JWT_SECRET, { expiresIn: 3600 });

      return new Response(JSON.stringify({
        success: true,
        user: {
          id: userId,
          email,
          name: name || '',
          plan: 'free'
        },
        token,
        api_key: apiKey
      }), {
        status: 201,
        headers: { 'Content-Type': 'application/json' }
      });

    } catch (error) {
      if (error instanceof ValidationError) {
        throw error;
      }
      throw new Error('Registration failed: ' + error.message);
    }
  });

  // User login
  router.post('/auth/login', async (request, env) => {
    try {
      const { email, password } = await request.json();

      if (!email || !password) {
        throw new ValidationError('Email and password are required');
      }

      // Find user
      const user = await env.DB.prepare(
        'SELECT id, email, name, password_hash, plan, is_active FROM users WHERE email = ?'
      ).bind(email).first();

      if (!user || !user.is_active) {
        throw new AuthenticationError('Invalid credentials');
      }

      // Verify password
      const isValidPassword = await verifyPassword(password, user.password_hash);
      if (!isValidPassword) {
        throw new AuthenticationError('Invalid credentials');
      }

      // Update last login
      await env.DB.prepare(
        'UPDATE users SET last_login = ? WHERE id = ?'
      ).bind(Date.now(), user.id).run();

      // Generate JWT token
      const token = await signJWT({
        sub: user.id,
        email: user.email
      }, env.JWT_SECRET, { expiresIn: 3600 });

      return new Response(JSON.stringify({
        success: true,
        user: {
          id: user.id,
          email: user.email,
          name: user.name,
          plan: user.plan
        },
        token
      }), {
        headers: { 'Content-Type': 'application/json' }
      });

    } catch (error) {
      if (error instanceof AuthenticationError || error instanceof ValidationError) {
        throw error;
      }
      throw new AuthenticationError('Login failed');
    }
  });

  // Get current user info
  router.get('/auth/me', async (request, env) => {
    const user = request.user;
    if (!user) {
      throw new AuthenticationError();
    }

    // Get detailed user info
    const userDetails = await env.DB.prepare(`
      SELECT id, email, name, plan, created_at, last_login, usage_quota
      FROM users WHERE id = ?
    `).bind(user.id).first();

    // Get API keys
    const apiKeys = await env.DB.prepare(`
      SELECT id, name, created_at, last_used, is_active
      FROM api_keys WHERE user_id = ? ORDER BY created_at DESC
    `).bind(user.id).all();

    return new Response(JSON.stringify({
      user: userDetails,
      api_keys: apiKeys.results || []
    }), {
      headers: { 'Content-Type': 'application/json' }
    });
  });

  // Generate new API key
  router.post('/auth/api-keys', async (request, env) => {
    const user = request.user;
    if (!user) {
      throw new AuthenticationError();
    }

    const { name, permissions } = await request.json();

    if (!name) {
      throw new ValidationError('API key name is required', 'name');
    }

    // Generate new API key
    const apiKey = await generateApiKey();
    const hashedKey = await hashApiKey(apiKey);
    const keyId = `key_${Date.now()}_${Math.random().toString(36).substr(2, 6)}`;

    await env.DB.prepare(`
      INSERT INTO api_keys (id, user_id, name, key_hash, permissions, created_at)
      VALUES (?, ?, ?, ?, ?, ?)
    `).bind(
      keyId,
      user.id,
      name,
      hashedKey,
      JSON.stringify(permissions || []),
      Date.now()
    ).run();

    return new Response(JSON.stringify({
      success: true,
      api_key: {
        id: keyId,
        name,
        key: apiKey, // Only shown once
        permissions: permissions || []
      }
    }), {
      status: 201,
      headers: { 'Content-Type': 'application/json' }
    });
  });

  // Revoke API key
  router.delete('/auth/api-keys/:keyId', async (request, env) => {
    const user = request.user;
    if (!user) {
      throw new AuthenticationError();
    }

    const keyId = request.params.keyId;

    const result = await env.DB.prepare(
      'UPDATE api_keys SET is_active = 0 WHERE id = ? AND user_id = ?'
    ).bind(keyId, user.id).run();

    if (result.changes === 0) {
      throw new NotFoundError('API key');
    }

    return new Response(JSON.stringify({
      success: true,
      message: 'API key revoked'
    }), {
      headers: { 'Content-Type': 'application/json' }
    });
  });

  // Change password
  router.post('/auth/change-password', async (request, env) => {
    const user = request.user;
    if (!user) {
      throw new AuthenticationError();
    }

    const { current_password, new_password } = await request.json();

    if (!current_password || !new_password) {
      throw new ValidationError('Current and new password are required');
    }

    // Validate new password
    const passwordValidation = validatePasswordStrength(new_password);
    if (!passwordValidation.valid) {
      throw new ValidationError('New password requirements not met', 'new_password', passwordValidation.errors);
    }

    // Get current password hash
    const userRecord = await env.DB.prepare(
      'SELECT password_hash FROM users WHERE id = ?'
    ).bind(user.id).first();

    // Verify current password
    const isValidCurrent = await verifyPassword(current_password, userRecord.password_hash);
    if (!isValidCurrent) {
      throw new AuthenticationError('Current password is incorrect');
    }

    // Update password
    const newPasswordHash = await hashPassword(new_password);
    await env.DB.prepare(
      'UPDATE users SET password_hash = ?, updated_at = ? WHERE id = ?'
    ).bind(newPasswordHash, Date.now(), user.id).run();

    return new Response(JSON.stringify({
      success: true,
      message: 'Password updated successfully'
    }), {
      headers: { 'Content-Type': 'application/json' }
    });
  });

  // Refresh JWT token
  router.post('/auth/refresh', async (request, env) => {
    const user = request.user;
    if (!user) {
      throw new AuthenticationError();
    }

    // Generate new token
    const token = await signJWT({
      sub: user.id,
      email: user.email
    }, env.JWT_SECRET, { expiresIn: 3600 });

    return new Response(JSON.stringify({
      success: true,
      token
    }), {
      headers: { 'Content-Type': 'application/json' }
    });
  });

  // Logout (invalidate session)
  router.post('/auth/logout', async (request, env) => {
    // In a stateless JWT setup, logout is mainly client-side
    // But we can log the event for security
    if (request.user) {
      console.log(`User ${request.user.id} logged out`);
    }

    return new Response(JSON.stringify({
      success: true,
      message: 'Logged out successfully'
    }), {
      headers: { 'Content-Type': 'application/json' }
    });
  });
}

async function hashApiKey(apiKey) {
  const encoder = new TextEncoder();
  const data = encoder.encode(apiKey);
  const hashBuffer = await crypto.subtle.digest('SHA-256', data);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
}