// Error handling middleware for comprehensive error management
export function errorHandler(options = {}) {
  const defaults = {
    logErrors: true,
    includeStack: false,
    genericMessage: 'Internal server error occurred'
  };

  const config = { ...defaults, ...options };

  return async (error, request, env, ctx) => {
    try {
      // Log the error
      if (config.logErrors) {
        await logError(error, request, env);
      }

      // Determine error type and response
      const errorResponse = createErrorResponse(error, request, env, config);

      return errorResponse;
    } catch (handlerError) {
      console.error('Error handler failed:', handlerError);

      // Fallback response
      return new Response(JSON.stringify({
        error: 'Internal server error',
        code: 'HANDLER_ERROR'
      }), {
        status: 500,
        headers: { 'Content-Type': 'application/json' }
      });
    }
  };
}

function createErrorResponse(error, request, env, config) {
  const isDevelopment = env.ENVIRONMENT === 'development';

  // Known error types
  if (error.name === 'ValidationError') {
    return new Response(JSON.stringify({
      error: 'Validation failed',
      code: 'VALIDATION_ERROR',
      details: error.details || error.message,
      field: error.field
    }), {
      status: 400,
      headers: { 'Content-Type': 'application/json' }
    });
  }

  if (error.name === 'AuthenticationError') {
    return new Response(JSON.stringify({
      error: 'Authentication required',
      code: 'AUTH_ERROR',
      message: error.message
    }), {
      status: 401,
      headers: { 'Content-Type': 'application/json' }
    });
  }

  if (error.name === 'AuthorizationError') {
    return new Response(JSON.stringify({
      error: 'Insufficient permissions',
      code: 'AUTH_INSUFFICIENT',
      message: error.message
    }), {
      status: 403,
      headers: { 'Content-Type': 'application/json' }
    });
  }

  if (error.name === 'NotFoundError') {
    return new Response(JSON.stringify({
      error: 'Resource not found',
      code: 'NOT_FOUND',
      resource: error.resource || 'unknown'
    }), {
      status: 404,
      headers: { 'Content-Type': 'application/json' }
    });
  }

  if (error.name === 'RateLimitError') {
    return new Response(JSON.stringify({
      error: 'Rate limit exceeded',
      code: 'RATE_LIMIT',
      retry_after: error.retryAfter || 60
    }), {
      status: 429,
      headers: {
        'Content-Type': 'application/json',
        'Retry-After': (error.retryAfter || 60).toString()
      }
    });
  }

  if (error.name === 'DatabaseError') {
    return new Response(JSON.stringify({
      error: 'Database operation failed',
      code: 'DATABASE_ERROR',
      message: isDevelopment ? error.message : 'Database temporarily unavailable'
    }), {
      status: 503,
      headers: { 'Content-Type': 'application/json' }
    });
  }

  if (error.name === 'ExternalServiceError') {
    return new Response(JSON.stringify({
      error: 'External service unavailable',
      code: 'SERVICE_UNAVAILABLE',
      service: error.service,
      message: isDevelopment ? error.message : 'Service temporarily unavailable'
    }), {
      status: 503,
      headers: { 'Content-Type': 'application/json' }
    });
  }

  // HTTP status code errors
  if (error.status) {
    return new Response(JSON.stringify({
      error: error.message || 'HTTP error',
      code: error.code || 'HTTP_ERROR',
      status: error.status
    }), {
      status: error.status,
      headers: { 'Content-Type': 'application/json' }
    });
  }

  // Generic server error
  return new Response(JSON.stringify({
    error: isDevelopment ? error.message : config.genericMessage,
    code: 'INTERNAL_ERROR',
    stack: isDevelopment && config.includeStack ? error.stack : undefined,
    request_id: request.requestId
  }), {
    status: 500,
    headers: { 'Content-Type': 'application/json' }
  });
}

async function logError(error, request, env) {
  try {
    const errorLog = {
      timestamp: new Date().toISOString(),
      request_id: request.requestId,
      error_name: error.name,
      error_message: error.message,
      error_stack: error.stack,
      url: request.url,
      method: request.method,
      user_id: request.user?.id,
      ip: request.headers.get('CF-Connecting-IP'),
      user_agent: request.headers.get('User-Agent'),
      cf_ray: request.headers.get('CF-Ray'),
      cf_colo: request.cf?.colo,
      environment: env.ENVIRONMENT
    };

    // Store error log
    const errorKey = `error:${Date.now()}_${Math.random().toString(36).substr(2, 6)}`;
    await env.METADATA.put(errorKey, JSON.stringify(errorLog), {
      expirationTtl: 7 * 24 * 60 * 60 // 7 days
    });

    // Console logging
    console.error(`Error in ${request.method} ${request.url}:`, {
      message: error.message,
      stack: error.stack,
      user: request.user?.id
    });

    // Alert on critical errors
    if (isCriticalError(error)) {
      await triggerErrorAlert(errorLog, env);
    }

  } catch (logError) {
    console.error('Failed to log error:', logError);
  }
}

function isCriticalError(error) {
  // Define what constitutes a critical error
  const criticalPatterns = [
    'Database connection lost',
    'Out of memory',
    'Service unavailable',
    'Authentication bypass',
    'Security violation'
  ];

  return criticalPatterns.some(pattern =>
    error.message?.includes(pattern) || error.stack?.includes(pattern)
  );
}

async function triggerErrorAlert(errorLog, env) {
  try {
    // Rate limit alerts to prevent spam
    const alertKey = `error_alert:${errorLog.error_name}`;
    const existing = await env.CACHE.get(alertKey);

    if (!existing) {
      console.error('CRITICAL ERROR ALERT:', errorLog);

      // In production, this could trigger webhooks, emails, or Slack notifications
      await env.CACHE.put(alertKey, '1', { expirationTtl: 300 }); // 5 minute cooldown
    }
  } catch (alertError) {
    console.error('Failed to trigger error alert:', alertError);
  }
}

// Custom error classes
export class ValidationError extends Error {
  constructor(message, field = null, details = null) {
    super(message);
    this.name = 'ValidationError';
    this.field = field;
    this.details = details;
  }
}

export class AuthenticationError extends Error {
  constructor(message = 'Authentication required') {
    super(message);
    this.name = 'AuthenticationError';
  }
}

export class AuthorizationError extends Error {
  constructor(message = 'Insufficient permissions') {
    super(message);
    this.name = 'AuthorizationError';
  }
}

export class NotFoundError extends Error {
  constructor(resource = 'Resource') {
    super(`${resource} not found`);
    this.name = 'NotFoundError';
    this.resource = resource;
  }
}

export class RateLimitError extends Error {
  constructor(message = 'Rate limit exceeded', retryAfter = 60) {
    super(message);
    this.name = 'RateLimitError';
    this.retryAfter = retryAfter;
  }
}

export class DatabaseError extends Error {
  constructor(message = 'Database operation failed') {
    super(message);
    this.name = 'DatabaseError';
  }
}

export class ExternalServiceError extends Error {
  constructor(service, message = 'External service error') {
    super(message);
    this.name = 'ExternalServiceError';
    this.service = service;
  }
}

// Error boundaries for different operations
export async function withErrorBoundary(operation, fallback = null) {
  try {
    return await operation();
  } catch (error) {
    console.error('Operation failed:', error);
    return fallback;
  }
}

export function asyncErrorHandler(fn) {
  return async (request, env, ctx) => {
    try {
      return await fn(request, env, ctx);
    } catch (error) {
      return errorHandler()(error, request, env, ctx);
    }
  };
}