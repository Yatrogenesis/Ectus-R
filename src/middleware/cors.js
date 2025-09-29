// CORS middleware for cross-origin requests
export function cors(options = {}) {
  const defaults = {
    origin: '*',
    methods: ['GET', 'POST', 'PUT', 'DELETE', 'PATCH', 'OPTIONS'],
    headers: ['Content-Type', 'Authorization', 'X-API-Key', 'X-Requested-With'],
    credentials: false,
    maxAge: 86400 // 24 hours
  };

  const config = { ...defaults, ...options };

  return (request, env) => {
    // Handle preflight OPTIONS requests
    if (request.method === 'OPTIONS') {
      return new Response(null, {
        status: 204,
        headers: {
          'Access-Control-Allow-Origin': getAllowedOrigin(request, config.origin, env),
          'Access-Control-Allow-Methods': config.methods.join(', '),
          'Access-Control-Allow-Headers': config.headers.join(', '),
          'Access-Control-Max-Age': config.maxAge.toString(),
          'Access-Control-Allow-Credentials': config.credentials.toString()
        }
      });
    }

    // Add CORS headers to the response
    return null; // Continue to next middleware
  };
}

export function addCorsHeaders(response, request, env) {
  const corsOrigins = env.CORS_ORIGINS || '*';

  const allowedOrigin = getAllowedOrigin(request, corsOrigins, env);

  const headers = new Headers(response.headers);
  headers.set('Access-Control-Allow-Origin', allowedOrigin);
  headers.set('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, PATCH, OPTIONS');
  headers.set('Access-Control-Allow-Headers', 'Content-Type, Authorization, X-API-Key, X-Requested-With');
  headers.set('Access-Control-Allow-Credentials', 'true');

  // Security headers
  headers.set('X-Content-Type-Options', 'nosniff');
  headers.set('X-Frame-Options', 'DENY');
  headers.set('X-XSS-Protection', '1; mode=block');
  headers.set('Referrer-Policy', 'strict-origin-when-cross-origin');
  headers.set('Content-Security-Policy', "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'");

  return new Response(response.body, {
    status: response.status,
    statusText: response.statusText,
    headers
  });
}

function getAllowedOrigin(request, corsOrigins, env) {
  const origin = request.headers.get('Origin');

  // If CORS_ORIGINS is '*', allow any origin
  if (corsOrigins === '*') {
    return origin || '*';
  }

  // Check if origin is in allowed list
  const allowedOrigins = corsOrigins.split(',').map(o => o.trim());

  if (origin && allowedOrigins.includes(origin)) {
    return origin;
  }

  // Check for wildcard patterns
  if (origin) {
    for (const allowed of allowedOrigins) {
      if (allowed.includes('*')) {
        const pattern = allowed.replace(/\*/g, '.*');
        const regex = new RegExp(`^${pattern}$`);
        if (regex.test(origin)) {
          return origin;
        }
      }
    }
  }

  // For development, allow localhost and common dev origins
  if (env.ENVIRONMENT === 'development') {
    const devOrigins = [
      'http://localhost:3000',
      'http://localhost:8080',
      'http://127.0.0.1:3000',
      'http://127.0.0.1:8080'
    ];

    if (origin && (devOrigins.includes(origin) || origin.includes('localhost'))) {
      return origin;
    }
  }

  // Default to first allowed origin or '*'
  return allowedOrigins[0] || '*';
}