// Rate limiting middleware for API protection
export function rateLimit(options = {}) {
  const defaults = {
    windowMs: 60 * 1000, // 1 minute
    max: 100, // 100 requests per minute
    keyGenerator: (request) => {
      return request.headers.get('CF-Connecting-IP') ||
             request.headers.get('X-Forwarded-For') ||
             'unknown';
    },
    skipSuccessfulRequests: false,
    skipFailedRequests: false,
    message: 'Too many requests, please try again later.'
  };

  const config = { ...defaults, ...options };

  return async (request, env, ctx) => {
    try {
      const key = config.keyGenerator(request);
      const rateLimitKey = `rate_limit:${key}`;

      // Get current count
      const current = await env.CACHE.get(rateLimitKey);
      const count = current ? parseInt(current) : 0;

      // Check if limit exceeded
      if (count >= config.max) {
        return new Response(JSON.stringify({
          error: config.message,
          code: 'RATE_LIMITED',
          limit: config.max,
          window_ms: config.windowMs,
          retry_after: Math.ceil(config.windowMs / 1000)
        }), {
          status: 429,
          headers: {
            'Content-Type': 'application/json',
            'X-RateLimit-Limit': config.max.toString(),
            'X-RateLimit-Remaining': '0',
            'X-RateLimit-Reset': (Date.now() + config.windowMs).toString(),
            'Retry-After': Math.ceil(config.windowMs / 1000).toString()
          }
        });
      }

      // Increment counter
      const newCount = count + 1;
      const ttl = Math.ceil(config.windowMs / 1000);

      await env.CACHE.put(rateLimitKey, newCount.toString(), {
        expirationTtl: ttl
      });

      // Add rate limit headers to the request for downstream middleware
      request.rateLimitInfo = {
        limit: config.max,
        remaining: Math.max(0, config.max - newCount),
        reset: Date.now() + config.windowMs,
        used: newCount
      };

      return null; // Continue to next middleware
    } catch (error) {
      console.error('Rate limiting error:', error);
      return null; // Continue on error
    }
  };
}

// Specialized rate limiters for different endpoints
export const authRateLimit = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 5, // 5 attempts per 15 minutes
  message: 'Too many authentication attempts'
});

export const deploymentRateLimit = rateLimit({
  windowMs: 5 * 60 * 1000, // 5 minutes
  max: 3, // 3 deployments per 5 minutes
  message: 'Too many deployment requests',
  keyGenerator: (request) => {
    // Rate limit by user ID if authenticated
    const userId = request.user?.id;
    const ip = request.headers.get('CF-Connecting-IP') || 'unknown';
    return userId ? `user:${userId}` : `ip:${ip}`;
  }
});

export const apiKeyRateLimit = rateLimit({
  windowMs: 60 * 1000, // 1 minute
  max: 1000, // 1000 requests per minute for API keys
  keyGenerator: (request) => {
    const apiKey = request.headers.get('X-API-Key');
    const ip = request.headers.get('CF-Connecting-IP') || 'unknown';
    return apiKey ? `apikey:${apiKey.substring(0, 10)}` : `ip:${ip}`;
  }
});

// Burst protection for expensive operations
export const burstProtection = rateLimit({
  windowMs: 1000, // 1 second
  max: 10, // 10 requests per second
  message: 'Request rate too high, please slow down'
});

// Rate limiting for different user plans
export function planBasedRateLimit(request, env) {
  const user = request.user;
  if (!user) {
    return rateLimit({ max: 10, windowMs: 60 * 1000 }); // Free tier for unauthenticated
  }

  const limits = {
    'free': { max: 50, windowMs: 60 * 1000 }, // 50/minute
    'pro': { max: 500, windowMs: 60 * 1000 }, // 500/minute
    'enterprise': { max: 5000, windowMs: 60 * 1000 } // 5000/minute
  };

  const limit = limits[user.plan] || limits['free'];

  return rateLimit({
    ...limit,
    keyGenerator: () => `user:${user.id}`,
    message: `Rate limit exceeded for ${user.plan} plan`
  });
}

// Add rate limit headers to response
export function addRateLimitHeaders(response, request) {
  if (request.rateLimitInfo) {
    const headers = new Headers(response.headers);
    const info = request.rateLimitInfo;

    headers.set('X-RateLimit-Limit', info.limit.toString());
    headers.set('X-RateLimit-Remaining', info.remaining.toString());
    headers.set('X-RateLimit-Reset', info.reset.toString());
    headers.set('X-RateLimit-Used', info.used.toString());

    return new Response(response.body, {
      status: response.status,
      statusText: response.statusText,
      headers
    });
  }

  return response;
}

// Distributed rate limiting for enterprise customers
export async function distributedRateLimit(request, env, options = {}) {
  try {
    const userId = request.user?.id;
    if (!userId || request.user?.plan !== 'enterprise') {
      return null; // Skip for non-enterprise users
    }

    // Use Durable Objects for distributed counting
    const durableObjectId = env.DEPLOYMENT_TRACKER.idFromName(`rate_limit:${userId}`);
    const stub = env.DEPLOYMENT_TRACKER.get(durableObjectId);

    const response = await stub.fetch(request.clone(), {
      method: 'POST',
      body: JSON.stringify({
        action: 'check_rate_limit',
        userId,
        limit: options.max || 1000,
        window: options.windowMs || 60000
      })
    });

    const result = await response.json();

    if (result.limited) {
      return new Response(JSON.stringify({
        error: 'Enterprise rate limit exceeded',
        code: 'ENTERPRISE_RATE_LIMITED',
        limit: result.limit,
        used: result.used,
        reset: result.reset
      }), {
        status: 429,
        headers: {
          'Content-Type': 'application/json',
          'X-RateLimit-Limit': result.limit.toString(),
          'X-RateLimit-Remaining': result.remaining.toString(),
          'X-RateLimit-Reset': result.reset.toString()
        }
      });
    }

    return null; // Continue
  } catch (error) {
    console.error('Distributed rate limiting error:', error);
    return null; // Continue on error
  }
}