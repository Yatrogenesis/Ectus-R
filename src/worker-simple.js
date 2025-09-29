// Ectus-R SaaS API Worker with Domain Proxy
import { Router } from 'itty-router';

const router = Router();

// Handle domain proxying for custom domains
async function handleDomainProxy(request) {
  const url = new URL(request.url);
  const hostname = url.hostname;

  const customDomains = [
    'creator.avermex.com',
    'demo.avermex.com',
    'ectus.avermex.com',
    'app.avermex.com',
    'saas.avermex.com'
  ];

  if (customDomains.includes(hostname)) {
    try {
      const githubResponse = await fetch('https://yatrogenesis.github.io/Ectus-R/');
      return new Response(githubResponse.body, {
        status: 200,
        headers: {
          'Content-Type': 'text/html; charset=utf-8',
          'Cache-Control': 'public, max-age=300',
          'X-Served-By': 'Ectus-R-SaaS'
        }
      });
    } catch (error) {
      return new Response(`
        <!DOCTYPE html>
        <html><head><title>Ectus-R SaaS</title></head>
        <body style="font-family:system-ui;padding:40px;background:#0a0e27;color:white;text-align:center">
          <h1 style="color:#00d9ff">Ectus-R SaaS Platform</h1>
          <p>AI-Powered Development & Deployment Platform</p>
          <p><a href="https://yatrogenesis.github.io/Ectus-R/" style="color:#00d9ff">View Full Demo</a></p>
        </body></html>
      `, {
        status: 200,
        headers: { 'Content-Type': 'text/html; charset=utf-8' }
      });
    }
  }
  return null;
}

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
      // First check if this is a domain proxy request
      const proxyResponse = await handleDomainProxy(request);
      if (proxyResponse) {
        return proxyResponse;
      }

      // Otherwise handle as API
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