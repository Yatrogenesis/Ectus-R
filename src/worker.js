// Ectus-R SaaS API Worker - Main entry point for all SaaS operations
// Handles authentication, routing, and core API functionality

import { Router } from 'itty-router';
import { cors } from './middleware/cors.js';
import { authenticate } from './middleware/auth.js';
import { rateLimit } from './middleware/rateLimit.js';
import { logger } from './middleware/logger.js';
import { errorHandler } from './middleware/errorHandler.js';

// Route handlers
import { authRoutes } from './routes/auth.js';
import { healthRoutes } from './routes/health.js';
// Other routes to be implemented:
// import { projectRoutes } from './routes/projects.js';
// import { deploymentRoutes } from './routes/deployments.js';
// import { templateRoutes } from './routes/templates.js';
// import { marketplaceRoutes } from './routes/marketplace.js';
// import { analyticsRoutes } from './routes/analytics.js';
// import { webhookRoutes } from './routes/webhooks.js';
// import { billingRoutes } from './routes/billing.js';
// import { complianceRoutes } from './routes/compliance.js';

// Durable Objects (to be implemented)
// export { DeploymentTracker } from './durable-objects/DeploymentTracker.js';
// export { WebSocketManager } from './durable-objects/WebSocketManager.js';

// Create router
const router = Router();

// Simple route initialization
healthRoutes(router);
authRoutes(router);

// TODO: Initialize other routes when implemented
// projectRoutes(router);
// deploymentRoutes(router);
// templateRoutes(router);
// marketplaceRoutes(router);
// analyticsRoutes(router);
// webhookRoutes(router);
// billingRoutes(router);
// complianceRoutes(router);

// WebSocket upgrade for real-time features (disabled until Durable Objects are implemented)
// router.get('/ws', async (request, env) => {
//   const upgradeHeader = request.headers.get('Upgrade');
//   if (upgradeHeader !== 'websocket') {
//     return new Response('Expected Upgrade: websocket', { status: 426 });
//   }
//
//   const webSocketPair = new WebSocketPair();
//   const [client, server] = Object.values(webSocketPair);
//
//   // Get WebSocket manager instance
//   const id = env.WEBSOCKET_MANAGER.idFromName('global');
//   const wsManager = env.WEBSOCKET_MANAGER.get(id);
//
//   await wsManager.fetch(request, {
//     websocket: server,
//     headers: request.headers,
//   });
//
//   return new Response(null, {
//     status: 101,
//     webSocket: client,
//   });
// });

// Catch-all 404
router.all('*', () => new Response('Not Found', { status: 404 }));

// Main worker handler
export default {
  async fetch(request, env, ctx) {
    try {
      // Add environment to request for access in handlers
      request.env = env;
      request.ctx = ctx;

      return await router.handle(request, env, ctx);
    } catch (error) {
      return errorHandler()(error, request, env, ctx);
    }
  },

  // Scheduled handler for background tasks
  async scheduled(event, env, ctx) {
    console.log('Running scheduled tasks...');

    // Cleanup expired sessions
    await cleanupExpiredSessions(env);

    // Process deployment metrics
    await processDeploymentMetrics(env);

    // Generate analytics reports
    await generateAnalyticsReports(env);

    // Cleanup old logs
    await cleanupOldLogs(env);

    // Update marketplace stats
    await updateMarketplaceStats(env);
  },

  // Queue handler for background processing
  async queue(batch, env) {
    for (const message of batch.messages) {
      try {
        await processQueueMessage(message, env);
      } catch (error) {
        console.error('Queue processing error:', error);
        message.retry();
      }
    }
  },

  // Trace handler for observability
  async trace(traces) {
    for (const trace of traces) {
      console.log('Trace:', JSON.stringify(trace));
    }
  }
};

// Background task functions
async function cleanupExpiredSessions(env) {
  const now = Date.now();
  const expiredSessions = await env.SESSIONS.list({ prefix: 'session:' });

  for (const key of expiredSessions.keys) {
    const session = await env.SESSIONS.get(key.name);
    if (session) {
      const sessionData = JSON.parse(session);
      if (sessionData.expiresAt < now) {
        await env.SESSIONS.delete(key.name);
      }
    }
  }
}

async function processDeploymentMetrics(env) {
  // Get all active deployments
  const deployments = await env.DB.prepare(`
    SELECT id, status, created_at, updated_at
    FROM deployments
    WHERE status IN ('running', 'pending')
  `).all();

  for (const deployment of deployments.results) {
    // Update metrics
    await env.ANALYTICS.writeDataPoint({
      blobs: [deployment.id, deployment.status],
      doubles: [Date.now()],
      indexes: ['deployment_metrics']
    });
  }
}

async function generateAnalyticsReports(env) {
  const today = new Date().toISOString().split('T')[0];

  // Daily deployment stats
  const deploymentStats = await env.DB.prepare(`
    SELECT COUNT(*) as total, status
    FROM deployments
    WHERE DATE(created_at) = ?
    GROUP BY status
  `).bind(today).all();

  // Store in KV for quick access
  await env.CACHE.put(`daily_stats:${today}`, JSON.stringify(deploymentStats.results));
}

async function cleanupOldLogs(env) {
  const cutoffDate = new Date();
  cutoffDate.setDate(cutoffDate.getDate() - 30); // Keep 30 days

  await env.DB.prepare(`
    DELETE FROM logs
    WHERE created_at < ?
  `).bind(cutoffDate.toISOString()).run();
}

async function updateMarketplaceStats(env) {
  // Update download counts, ratings, etc.
  const stats = await env.DB.prepare(`
    SELECT template_id, COUNT(*) as downloads
    FROM template_downloads
    WHERE DATE(created_at) = CURRENT_DATE
    GROUP BY template_id
  `).all();

  for (const stat of stats.results) {
    await env.CACHE.put(`template_stats:${stat.template_id}`, JSON.stringify(stat));
  }
}

async function processQueueMessage(message, env) {
  const { type, payload } = message.body;

  switch (type) {
    case 'deployment_started':
      await handleDeploymentStarted(payload, env);
      break;
    case 'deployment_completed':
      await handleDeploymentCompleted(payload, env);
      break;
    case 'user_registered':
      await handleUserRegistered(payload, env);
      break;
    case 'billing_event':
      await handleBillingEvent(payload, env);
      break;
    case 'compliance_check':
      await handleComplianceCheck(payload, env);
      break;
    default:
      console.warn('Unknown message type:', type);
  }
}

async function handleDeploymentStarted(payload, env) {
  const { deploymentId, userId, projectId } = payload;

  // Update deployment status
  await env.DB.prepare(`
    UPDATE deployments
    SET status = 'running', started_at = CURRENT_TIMESTAMP
    WHERE id = ?
  `).bind(deploymentId).run();

  // Send real-time notification
  const wsId = env.WEBSOCKET_MANAGER.idFromName('global');
  const wsManager = env.WEBSOCKET_MANAGER.get(wsId);

  await wsManager.fetch(new Request('https://fake.com/broadcast', {
    method: 'POST',
    body: JSON.stringify({
      type: 'deployment_update',
      userId,
      data: { deploymentId, status: 'running' }
    })
  }));
}

async function handleDeploymentCompleted(payload, env) {
  const { deploymentId, status, duration, resources } = payload;

  await env.DB.prepare(`
    UPDATE deployments
    SET status = ?, completed_at = CURRENT_TIMESTAMP, duration = ?
    WHERE id = ?
  `).bind(status, duration, deploymentId).run();

  // Record analytics
  await env.ANALYTICS.writeDataPoint({
    blobs: [deploymentId, status],
    doubles: [duration, resources.length],
    indexes: ['deployment_completed']
  });
}

async function handleUserRegistered(payload, env) {
  const { userId, email, plan } = payload;

  // Set up default resources
  await env.DB.prepare(`
    INSERT INTO user_quotas (user_id, plan, deployments_limit, storage_limit)
    VALUES (?, ?, ?, ?)
  `).bind(userId, plan, getPlanLimits(plan).deployments, getPlanLimits(plan).storage).run();

  // Send welcome email (would integrate with email service)
  console.log(`Welcome email queued for: ${email}`);
}

async function handleBillingEvent(payload, env) {
  const { userId, event, amount } = payload;

  await env.DB.prepare(`
    INSERT INTO billing_events (user_id, event_type, amount, created_at)
    VALUES (?, ?, ?, CURRENT_TIMESTAMP)
  `).bind(userId, event, amount).run();
}

async function handleComplianceCheck(payload, env) {
  const { projectId, framework } = payload;

  // Run compliance validation
  const result = await runComplianceCheck(projectId, framework, env);

  await env.DB.prepare(`
    INSERT INTO compliance_reports (project_id, framework, status, score, created_at)
    VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP)
  `).bind(projectId, framework, result.status, result.score).run();
}

function getPlanLimits(plan) {
  const limits = {
    free: { deployments: 5, storage: 1024 }, // 1GB
    pro: { deployments: 50, storage: 10240 }, // 10GB
    enterprise: { deployments: -1, storage: -1 } // Unlimited
  };
  return limits[plan] || limits.free;
}

async function runComplianceCheck(projectId, framework, env) {
  // This would integrate with the Rust compliance engine
  // For now, return mock data
  return {
    status: 'compliant',
    score: 95.5
  };
}