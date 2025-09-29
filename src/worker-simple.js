// Simple Ectus-R SaaS API Worker for testing
import { Router } from 'itty-router';

const router = Router();

// Health check endpoint
router.get('/health', async (request, env) => {
  return new Response(JSON.stringify({
    status: 'healthy',
    timestamp: Date.now(),
    version: '1.0.0',
    environment: env?.ENVIRONMENT || 'development'
  }), {
    headers: { 'Content-Type': 'application/json' },
    status: 200
  });
});

// User registration endpoint
router.post('/auth/register', async (request, env) => {
  try {
    const body = await request.json();

    return new Response(JSON.stringify({
      success: true,
      message: 'User registration endpoint working',
      data: {
        email: body.email,
        timestamp: Date.now()
      }
    }), {
      headers: { 'Content-Type': 'application/json' },
      status: 200
    });
  } catch (error) {
    return new Response(JSON.stringify({
      error: 'Invalid JSON',
      message: error.message
    }), {
      headers: { 'Content-Type': 'application/json' },
      status: 400
    });
  }
});

// Magic Loop endpoint
router.post('/api/v1/deployments/magic-loop', async (request, env) => {
  try {
    const body = await request.json();

    return new Response(JSON.stringify({
      success: true,
      deployment_id: `deploy_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      prompt: body.prompt,
      status: 'initiated',
      message: 'Magic Loop deployment started',
      estimated_completion: '2-5 minutes'
    }), {
      headers: { 'Content-Type': 'application/json' },
      status: 202
    });
  } catch (error) {
    return new Response(JSON.stringify({
      error: 'Invalid request',
      message: error.message
    }), {
      headers: { 'Content-Type': 'application/json' },
      status: 400
    });
  }
});

// Catch all other routes
router.all('*', () => {
  return new Response(JSON.stringify({
    error: 'Not Found',
    message: 'Endpoint not available'
  }), {
    headers: { 'Content-Type': 'application/json' },
    status: 404
  });
});

// Main worker handler
export default {
  async fetch(request, env, ctx) {
    try {
      return await router.handle(request, env, ctx);
    } catch (error) {
      return new Response(JSON.stringify({
        error: 'Internal Server Error',
        message: error.message
      }), {
        headers: { 'Content-Type': 'application/json' },
        status: 500
      });
    }
  }
};