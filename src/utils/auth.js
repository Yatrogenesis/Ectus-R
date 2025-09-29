// Authentication utilities for JWT and API key handling

export function extractApiKey(request) {
  // Check X-API-Key header
  const apiKeyHeader = request.headers.get('X-API-Key');
  if (apiKeyHeader) {
    return apiKeyHeader;
  }

  // Check Authorization header with API key format
  const authorization = request.headers.get('Authorization');
  if (authorization && authorization.startsWith('ApiKey ')) {
    return authorization.substring(7);
  }

  // Check query parameter
  const url = new URL(request.url);
  const apiKeyParam = url.searchParams.get('api_key');
  if (apiKeyParam) {
    return apiKeyParam;
  }

  return null;
}

export async function verifyJWT(token, secret) {
  try {
    if (!secret) {
      throw new Error('JWT secret not configured');
    }

    // Simple JWT verification for Cloudflare Workers
    const parts = token.split('.');
    if (parts.length !== 3) {
      throw new Error('Invalid JWT format');
    }

    const [headerB64, payloadB64, signatureB64] = parts;

    // Decode header and payload
    const header = JSON.parse(atob(headerB64));
    const payload = JSON.parse(atob(payloadB64));

    // Check expiration
    if (payload.exp && payload.exp < Math.floor(Date.now() / 1000)) {
      throw new Error('Token expired');
    }

    // Verify signature
    const isValid = await verifySignature(
      `${headerB64}.${payloadB64}`,
      signatureB64,
      secret
    );

    if (!isValid) {
      throw new Error('Invalid signature');
    }

    return payload;
  } catch (error) {
    console.error('JWT verification error:', error);
    return null;
  }
}

export async function signJWT(payload, secret, options = {}) {
  try {
    if (!secret) {
      throw new Error('JWT secret not configured');
    }

    const header = {
      alg: 'HS256',
      typ: 'JWT'
    };

    // Add standard claims
    const now = Math.floor(Date.now() / 1000);
    const claims = {
      iat: now,
      exp: now + (options.expiresIn || 3600), // Default 1 hour
      ...payload
    };

    // Encode header and payload
    const headerB64 = btoa(JSON.stringify(header));
    const payloadB64 = btoa(JSON.stringify(claims));

    // Create signature
    const signature = await createSignature(`${headerB64}.${payloadB64}`, secret);

    return `${headerB64}.${payloadB64}.${signature}`;
  } catch (error) {
    console.error('JWT signing error:', error);
    throw error;
  }
}

async function verifySignature(data, signature, secret) {
  try {
    const expectedSignature = await createSignature(data, secret);
    return signature === expectedSignature;
  } catch (error) {
    return false;
  }
}

async function createSignature(data, secret) {
  const encoder = new TextEncoder();
  const secretKey = await crypto.subtle.importKey(
    'raw',
    encoder.encode(secret),
    { name: 'HMAC', hash: 'SHA-256' },
    false,
    ['sign']
  );

  const signature = await crypto.subtle.sign(
    'HMAC',
    secretKey,
    encoder.encode(data)
  );

  // Convert to base64url
  const base64 = btoa(String.fromCharCode(...new Uint8Array(signature)));
  return base64.replace(/\+/g, '-').replace(/\//g, '_').replace(/=/g, '');
}

export async function generateApiKey() {
  // Generate a cryptographically secure random API key
  const array = new Uint8Array(32);
  crypto.getRandomValues(array);

  // Convert to hex string
  const hex = Array.from(array, byte => byte.toString(16).padStart(2, '0')).join('');

  // Add prefix and format
  return `ectus_${hex}`;
}

export async function hashPassword(password) {
  const encoder = new TextEncoder();
  const data = encoder.encode(password);
  const hashBuffer = await crypto.subtle.digest('SHA-256', data);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
}

export async function verifyPassword(password, hash) {
  const hashedInput = await hashPassword(password);
  return hashedInput === hash;
}

// Generate secure session ID
export function generateSessionId() {
  const array = new Uint8Array(16);
  crypto.getRandomValues(array);
  return Array.from(array, byte => byte.toString(16).padStart(2, '0')).join('');
}

// Validate email format
export function isValidEmail(email) {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}

// Generate user ID
export function generateUserId() {
  return `user_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

// Generate deployment ID
export function generateDeploymentId() {
  return `deploy_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

// Password strength validation
export function validatePasswordStrength(password) {
  const errors = [];

  if (password.length < 8) {
    errors.push('Password must be at least 8 characters long');
  }

  if (!/[A-Z]/.test(password)) {
    errors.push('Password must contain at least one uppercase letter');
  }

  if (!/[a-z]/.test(password)) {
    errors.push('Password must contain at least one lowercase letter');
  }

  if (!/[0-9]/.test(password)) {
    errors.push('Password must contain at least one number');
  }

  if (!/[^A-Za-z0-9]/.test(password)) {
    errors.push('Password must contain at least one special character');
  }

  return {
    valid: errors.length === 0,
    errors
  };
}