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

// Magic Loop endpoint with Cloudflare AI integration
router.post('/api/v1/deployments/magic-loop', async (request, env) => {
  try {
    const body = await request.json();
    const { prompt } = body;

    if (!prompt) {
      return new Response(JSON.stringify({
        error: 'Missing prompt',
        message: 'Prompt is required for Magic Loop deployment'
      }), {
        headers: { 'Content-Type': 'application/json' },
        status: 400
      });
    }

    const deploymentId = `deploy_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;

    // Generate code using Cloudflare AI
    const codeGenPrompt = `Generate a complete web application based on this requirement: "${prompt}".
    Return only valid HTML with inline CSS and JavaScript. Make it professional and functional.`;

    try {
      const aiResponse = await env.AI.run('@cf/meta/llama-2-7b-chat-int8', {
        messages: [
          { role: 'system', content: 'You are an expert web developer. Generate ONLY pure HTML code with inline CSS and JavaScript. Do not use markdown formatting or code blocks.' },
          { role: 'user', content: codeGenPrompt }
        ]
      });

      let generatedCode = aiResponse.response || `
        <!DOCTYPE html>
        <html><head><title>Generated App</title>
        <style>body{font-family:Arial;padding:40px;background:#f0f0f0;}</style></head>
        <body><h1>Generated Application</h1><p>Prompt: ${prompt}</p>
        <p>Deployment ID: ${deploymentId}</p></body></html>`;

      // Extract HTML from markdown code blocks if present
      if (generatedCode.includes('```')) {
        const htmlMatch = generatedCode.match(/```(?:html)?\s*([\s\S]*?)```/);
        if (htmlMatch && htmlMatch[1]) {
          generatedCode = htmlMatch[1].trim();
        }
      }

      // Store in KV for retrieval
      await env.CACHE.put(`deployment:${deploymentId}`, JSON.stringify({
        id: deploymentId,
        prompt: prompt,
        code: generatedCode,
        status: 'completed',
        created_at: new Date().toISOString(),
        url: `https://ectus-r-saas.pako-molina.workers.dev/api/v1/deployments/${deploymentId}/preview`
      }));

      return new Response(JSON.stringify({
        success: true,
        deployment_id: deploymentId,
        prompt: prompt,
        status: 'completed',
        message: 'Magic Loop deployment completed successfully',
        preview_url: `https://ectus-r-saas.pako-molina.workers.dev/api/v1/deployments/${deploymentId}/preview`,
        code_preview: generatedCode.substring(0, 500) + '...'
      }), {
        headers: { 'Content-Type': 'application/json' },
        status: 200
      });

    } catch (aiError) {
      // Fallback to template-based generation
      const fallbackCode = `
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Generated by Ectus-R</title>
            <style>
                body { font-family: -apple-system, BlinkMacSystemFont, sans-serif; margin: 0; padding: 40px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; }
                .container { max-width: 800px; margin: 0 auto; text-align: center; }
                .card { background: rgba(255,255,255,0.1); padding: 30px; border-radius: 15px; backdrop-filter: blur(10px); }
                .btn { background: #ff6b6b; color: white; padding: 15px 30px; border: none; border-radius: 8px; cursor: pointer; font-size: 16px; }
                .btn:hover { background: #ff5252; }
            </style>
        </head>
        <body>
            <div class="container">
                <div class="card">
                    <h1>🚀 Generated by Ectus-R AI</h1>
                    <p><strong>Your Prompt:</strong> ${prompt}</p>
                    <p><strong>Deployment ID:</strong> ${deploymentId}</p>
                    <p>This application was generated using AI-powered Magic Loop deployment.</p>
                    <button class="btn" onclick="alert('Application is live!')">Test Application</button>
                </div>
            </div>
            <script>
                console.log('Ectus-R Magic Loop Application Loaded');
                console.log('Deployment ID: ${deploymentId}');
            </script>
        </body>
        </html>`;

      await env.CACHE.put(`deployment:${deploymentId}`, JSON.stringify({
        id: deploymentId,
        prompt: prompt,
        code: fallbackCode,
        status: 'completed',
        created_at: new Date().toISOString(),
        url: `https://ectus-r-saas.pako-molina.workers.dev/api/v1/deployments/${deploymentId}/preview`
      }));

      return new Response(JSON.stringify({
        success: true,
        deployment_id: deploymentId,
        prompt: prompt,
        status: 'completed',
        message: 'Magic Loop deployment completed (fallback)',
        preview_url: `https://ectus-r-saas.pako-molina.workers.dev/api/v1/deployments/${deploymentId}/preview`
      }), {
        headers: { 'Content-Type': 'application/json' },
        status: 200
      });
    }

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

// Deployment preview endpoint
router.get('/api/v1/deployments/:id/preview', async (request, env) => {
  try {
    const { id } = request.params;
    const deployment = await env.CACHE.get(`deployment:${id}`);

    if (!deployment) {
      return new Response('Deployment not found', { status: 404 });
    }

    const deploymentData = JSON.parse(deployment);

    return new Response(deploymentData.code, {
      headers: {
        'Content-Type': 'text/html; charset=utf-8',
        'Cache-Control': 'public, max-age=3600',
        'X-Generated-By': 'Ectus-R-AI'
      },
      status: 200
    });

  } catch (error) {
    return new Response('Error loading deployment', { status: 500 });
  }
});

// Get deployment info
router.get('/api/v1/deployments/:id', async (request, env) => {
  try {
    const { id } = request.params;
    const deployment = await env.CACHE.get(`deployment:${id}`);

    if (!deployment) {
      return new Response(JSON.stringify({
        error: 'Not found',
        message: 'Deployment not found'
      }), {
        headers: { 'Content-Type': 'application/json' },
        status: 404
      });
    }

    const deploymentData = JSON.parse(deployment);
    delete deploymentData.code; // Don't return full code in API response

    return new Response(JSON.stringify(deploymentData), {
      headers: { 'Content-Type': 'application/json' },
      status: 200
    });

  } catch (error) {
    return new Response(JSON.stringify({
      error: 'Server error',
      message: error.message
    }), {
      headers: { 'Content-Type': 'application/json' },
      status: 500
    });
  }
});

// List deployments
router.get('/api/v1/deployments', async (request, env) => {
  try {
    const deployments = [];
    const list = await env.CACHE.list({ prefix: 'deployment:' });

    for (const key of list.keys) {
      const deployment = await env.CACHE.get(key.name);
      if (deployment) {
        const data = JSON.parse(deployment);
        delete data.code;
        deployments.push(data);
      }
    }

    return new Response(JSON.stringify({
      deployments: deployments.slice(0, 20), // Limit to 20 recent
      total: deployments.length
    }), {
      headers: { 'Content-Type': 'application/json' },
      status: 200
    });

  } catch (error) {
    return new Response(JSON.stringify({
      error: 'Server error',
      message: error.message
    }), {
      headers: { 'Content-Type': 'application/json' },
      status: 500
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

// Add CORS headers to all responses
function addCorsHeaders(response) {
  const newResponse = new Response(response.body, response);
  newResponse.headers.set('Access-Control-Allow-Origin', '*');
  newResponse.headers.set('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS');
  newResponse.headers.set('Access-Control-Allow-Headers', 'Content-Type, Authorization');
  return newResponse;
}

// Handle OPTIONS preflight requests
router.options('*', () => {
  return new Response(null, {
    headers: {
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS',
      'Access-Control-Allow-Headers': 'Content-Type, Authorization',
    },
    status: 200
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
      const response = await router.handle(request, env, ctx);
      return addCorsHeaders(response);
    } catch (error) {
      const errorResponse = new Response(JSON.stringify({
        error: 'Internal Server Error',
        message: error.message
      }), {
        headers: { 'Content-Type': 'application/json' },
        status: 500
      });
      return addCorsHeaders(errorResponse);
    }
  }
};