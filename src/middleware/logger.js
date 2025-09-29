// Logging middleware for request/response tracking and analytics
export function logger(options = {}) {
  const defaults = {
    logRequests: true,
    logResponses: true,
    logHeaders: false,
    logBody: false,
    includeUserAgent: true,
    includeTimestamp: true
  };

  const config = { ...defaults, ...options };

  return async (request, env, ctx) => {
    const startTime = Date.now();
    const requestId = generateRequestId();

    // Add request ID to request for tracing
    request.requestId = requestId;

    // Log incoming request
    if (config.logRequests) {
      await logRequest(request, env, config, requestId, startTime);
    }

    // Continue processing
    return null;
  };
}

export async function logResponse(response, request, env) {
  try {
    const endTime = Date.now();
    const duration = endTime - (request.startTime || endTime);

    const logEntry = {
      timestamp: new Date().toISOString(),
      request_id: request.requestId,
      method: request.method,
      url: request.url,
      status: response.status,
      duration_ms: duration,
      user_id: request.user?.id || null,
      ip: request.headers.get('CF-Connecting-IP'),
      user_agent: request.headers.get('User-Agent'),
      referer: request.headers.get('Referer'),
      cf_ray: request.headers.get('CF-Ray'),
      cf_colo: request.cf?.colo,
      cf_country: request.cf?.country,
      content_length: response.headers.get('Content-Length'),
      content_type: response.headers.get('Content-Type')
    };

    // Store in KV for analytics
    const logKey = `log:${request.requestId}`;
    await env.METADATA.put(logKey, JSON.stringify(logEntry), {
      expirationTtl: 7 * 24 * 60 * 60 // 7 days
    });

    // Log to console for real-time monitoring
    console.log(`${request.method} ${request.url} - ${response.status} - ${duration}ms`);

    // Record metrics for analytics
    await recordMetrics(logEntry, env);

  } catch (error) {
    console.error('Logging error:', error);
  }
}

async function logRequest(request, env, config, requestId, startTime) {
  try {
    const logEntry = {
      timestamp: new Date().toISOString(),
      request_id: requestId,
      type: 'request',
      method: request.method,
      url: request.url,
      ip: request.headers.get('CF-Connecting-IP'),
      user_agent: config.includeUserAgent ? request.headers.get('User-Agent') : null,
      referer: request.headers.get('Referer'),
      cf_ray: request.headers.get('CF-Ray'),
      cf_colo: request.cf?.colo,
      cf_country: request.cf?.country,
      content_type: request.headers.get('Content-Type'),
      content_length: request.headers.get('Content-Length'),
      headers: config.logHeaders ? Object.fromEntries(request.headers.entries()) : null
    };

    // Store request start time
    request.startTime = startTime;

    // Log for debugging in development
    if (env.ENVIRONMENT === 'development') {
      console.log('Request:', JSON.stringify(logEntry, null, 2));
    }

  } catch (error) {
    console.error('Request logging error:', error);
  }
}

async function recordMetrics(logEntry, env) {
  try {
    // Record basic metrics
    const metricsKey = `metrics:${new Date().toISOString().substring(0, 13)}`; // Hour-based key
    const existingMetrics = await env.CACHE.get(metricsKey);

    let metrics = existingMetrics ? JSON.parse(existingMetrics) : {
      requests: 0,
      errors: 0,
      total_duration: 0,
      status_codes: {},
      countries: {},
      user_agents: {}
    };

    // Update metrics
    metrics.requests += 1;
    metrics.total_duration += logEntry.duration_ms;

    if (logEntry.status >= 400) {
      metrics.errors += 1;
    }

    // Track status codes
    const statusGroup = `${Math.floor(logEntry.status / 100)}xx`;
    metrics.status_codes[statusGroup] = (metrics.status_codes[statusGroup] || 0) + 1;

    // Track geographic distribution
    if (logEntry.cf_country) {
      metrics.countries[logEntry.cf_country] = (metrics.countries[logEntry.cf_country] || 0) + 1;
    }

    // Track popular user agents (simplified)
    if (logEntry.user_agent) {
      const ua = simplifyUserAgent(logEntry.user_agent);
      metrics.user_agents[ua] = (metrics.user_agents[ua] || 0) + 1;
    }

    // Store updated metrics
    await env.CACHE.put(metricsKey, JSON.stringify(metrics), {
      expirationTtl: 2 * 60 * 60 // 2 hours
    });

    // For production, integrate with Analytics Engine
    if (env.ANALYTICS && env.ENVIRONMENT === 'production') {
      await env.ANALYTICS.writeDataPoint({
        blobs: [
          logEntry.method,
          logEntry.url,
          logEntry.user_agent,
          logEntry.cf_country || 'unknown'
        ],
        doubles: [logEntry.duration_ms],
        indexes: [logEntry.status]
      });
    }

  } catch (error) {
    console.error('Metrics recording error:', error);
  }
}

function generateRequestId() {
  return `req_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

function simplifyUserAgent(userAgent) {
  if (!userAgent) return 'unknown';

  if (userAgent.includes('Chrome')) return 'Chrome';
  if (userAgent.includes('Firefox')) return 'Firefox';
  if (userAgent.includes('Safari') && !userAgent.includes('Chrome')) return 'Safari';
  if (userAgent.includes('Edge')) return 'Edge';
  if (userAgent.includes('curl')) return 'curl';
  if (userAgent.includes('wget')) return 'wget';
  if (userAgent.includes('Postman')) return 'Postman';
  if (userAgent.includes('bot') || userAgent.includes('Bot')) return 'bot';

  return 'other';
}

// Security event logging
export async function logSecurityEvent(event, request, env, details = {}) {
  try {
    const securityLog = {
      timestamp: new Date().toISOString(),
      event_type: event,
      request_id: request.requestId,
      ip: request.headers.get('CF-Connecting-IP'),
      user_agent: request.headers.get('User-Agent'),
      url: request.url,
      method: request.method,
      user_id: request.user?.id || null,
      cf_ray: request.headers.get('CF-Ray'),
      cf_country: request.cf?.country,
      details
    };

    // Store security events separately
    const securityKey = `security:${Date.now()}_${Math.random().toString(36).substr(2, 6)}`;
    await env.METADATA.put(securityKey, JSON.stringify(securityLog), {
      expirationTtl: 30 * 24 * 60 * 60 // 30 days retention for security events
    });

    // Log to console for immediate attention
    console.warn(`SECURITY EVENT: ${event}`, securityLog);

    // For critical security events, consider triggering alerts
    if (['bruteforce_attempt', 'sql_injection', 'unauthorized_access'].includes(event)) {
      await triggerSecurityAlert(securityLog, env);
    }

  } catch (error) {
    console.error('Security logging error:', error);
  }
}

async function triggerSecurityAlert(securityLog, env) {
  // In production, this could send alerts via webhook, email, or SMS
  try {
    const alertKey = `alert:${securityLog.event_type}:${securityLog.ip}`;
    const existing = await env.CACHE.get(alertKey);

    if (!existing) {
      // First occurrence, send alert
      console.error('SECURITY ALERT:', securityLog);

      // Rate limit alerts per IP per event type
      await env.CACHE.put(alertKey, '1', { expirationTtl: 3600 }); // 1 hour
    }
  } catch (error) {
    console.error('Security alert error:', error);
  }
}

// Performance monitoring
export async function logPerformanceMetric(metric, value, request, env) {
  try {
    const perfLog = {
      timestamp: new Date().toISOString(),
      metric,
      value,
      request_id: request.requestId,
      url: request.url,
      user_id: request.user?.id,
      cf_colo: request.cf?.colo
    };

    const perfKey = `perf:${metric}:${Date.now()}`;
    await env.METADATA.put(perfKey, JSON.stringify(perfLog), {
      expirationTtl: 24 * 60 * 60 // 24 hours
    });

  } catch (error) {
    console.error('Performance logging error:', error);
  }
}