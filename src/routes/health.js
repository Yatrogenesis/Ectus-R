// Health check routes for monitoring and status
export function healthRoutes(router) {
  // Basic health check
  router.get('/health', async (request, env) => {
    const timestamp = Date.now();

    try {
      // Test database connectivity
      const dbHealth = await testDbConnection(env.DB);

      // Test KV storage
      const kvHealth = await testKvConnection(env.SESSIONS);

      // Check AI service
      const aiHealth = await testAiConnection(env.AI);

      return new Response(JSON.stringify({
        status: 'healthy',
        timestamp,
        version: '1.0.0',
        environment: env.ENVIRONMENT || 'development',
        services: {
          database: dbHealth,
          kv_storage: kvHealth,
          ai_service: aiHealth
        },
        uptime: getUptime(),
        memory: getMemoryUsage()
      }), {
        headers: { 'Content-Type': 'application/json' },
        status: 200
      });
    } catch (error) {
      return new Response(JSON.stringify({
        status: 'unhealthy',
        timestamp,
        error: error.message,
        services: {
          database: 'error',
          kv_storage: 'error',
          ai_service: 'error'
        }
      }), {
        headers: { 'Content-Type': 'application/json' },
        status: 503
      });
    }
  });

  // Detailed system status
  router.get('/status', async (request, env) => {
    const metrics = await getSystemMetrics(env);

    return new Response(JSON.stringify({
      timestamp: Date.now(),
      version: '1.0.0',
      environment: env.ENVIRONMENT,
      metrics,
      deployment: {
        region: request.cf?.colo || 'unknown',
        country: request.cf?.country || 'unknown',
        datacenter: request.cf?.asOrganization || 'unknown'
      }
    }), {
      headers: { 'Content-Type': 'application/json' }
    });
  });

  // Readiness probe for Kubernetes-style health checks
  router.get('/ready', async (request, env) => {
    try {
      await env.DB.prepare('SELECT 1').first();
      await env.SESSIONS.get('health-check');

      return new Response('OK', { status: 200 });
    } catch (error) {
      return new Response('Not Ready', { status: 503 });
    }
  });

  // Liveness probe
  router.get('/alive', async (request, env) => {
    return new Response('OK', { status: 200 });
  });
}

async function testDbConnection(db) {
  try {
    const result = await db.prepare('SELECT 1 as test').first();
    return result ? 'healthy' : 'error';
  } catch (error) {
    return 'error';
  }
}

async function testKvConnection(kv) {
  try {
    await kv.put('health-check', Date.now().toString(), { expirationTtl: 60 });
    const value = await kv.get('health-check');
    return value ? 'healthy' : 'error';
  } catch (error) {
    return 'error';
  }
}

async function testAiConnection(ai) {
  try {
    if (!ai) return 'not_configured';

    const response = await ai.run('@cf/meta/llama-2-7b-chat-int8', {
      messages: [{ role: 'user', content: 'health check' }],
      max_tokens: 5
    });

    return response ? 'healthy' : 'error';
  } catch (error) {
    return 'error';
  }
}

function getUptime() {
  // In a real worker, this would track startup time
  return {
    uptime_seconds: Math.floor(Date.now() / 1000) % 86400,
    start_time: new Date().toISOString()
  };
}

function getMemoryUsage() {
  // Cloudflare Workers don't expose memory usage directly
  return {
    used_mb: 'not_available',
    limit_mb: 128,
    percentage: 'not_available'
  };
}

async function getSystemMetrics(env) {
  const metrics = {
    requests_per_minute: await getRequestRate(env),
    active_deployments: await getActiveDeployments(env),
    error_rate: await getErrorRate(env),
    response_time_avg: await getAverageResponseTime(env)
  };

  return metrics;
}

async function getRequestRate(env) {
  try {
    // This would integrate with Analytics Engine in production
    return Math.floor(Math.random() * 1000);
  } catch (error) {
    return 0;
  }
}

async function getActiveDeployments(env) {
  try {
    const result = await env.DB.prepare(
      'SELECT COUNT(*) as count FROM deployments WHERE status IN (?, ?)'
    ).bind('generating', 'deploying').first();

    return result?.count || 0;
  } catch (error) {
    return 0;
  }
}

async function getErrorRate(env) {
  try {
    // This would calculate from logs/analytics in production
    return Math.random() * 5; // 0-5% error rate
  } catch (error) {
    return 0;
  }
}

async function getAverageResponseTime(env) {
  try {
    // This would calculate from performance metrics in production
    return Math.floor(Math.random() * 100) + 50; // 50-150ms
  } catch (error) {
    return 0;
  }
}