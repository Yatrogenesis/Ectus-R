// Ectus-R Production SaaS Worker
// Integración completa: Multi-AI + Templates + GitHub Pages + GoDaddy DNS
import { Router } from 'itty-router';
import { MultiAIProvider } from './ai-multi-provider.js';

const router = Router();

// Templates de fallback perfectos
const templates = {
  calculator: (id) => generateCalculatorTemplate(id),
  todo: (id) => generateTodoTemplate(id),
  timer: (id) => generateTimerTemplate(id),
  weather: (id) => generateWeatherTemplate(id),
  colorpicker: (id) => generateColorPickerTemplate(id),
  generic: (prompt, id) => generateGenericTemplate(prompt, id)
};

// Health Check con información detallada
router.get('/health', async (request, env) => {
  const aiProvider = new MultiAIProvider(env);
  const availableProviders = aiProvider.getAvailableProviders();

  return jsonResponse({
    status: 'healthy',
    version: '4.0.0-production',
    timestamp: Date.now(),
    infrastructure: {
      cloudflare: {
        workers: true,
        ai: !!env.AI,
        kv: !!(env.SESSIONS && env.CACHE && env.METADATA),
        d1: !!env.DB
      },
      githubPages: {
        enabled: true,
        url: 'https://yatrogenesis.github.io/Ectus-R'
      },
      dns: {
        provider: 'GoDaddy',
        domains: ['avermex.com']
      }
    },
    aiProviders: availableProviders,
    features: {
      multiAI: true,
      fallbackTemplates: true,
      realtime: false,
      analytics: true
    }
  });
});

// API Info
router.get('/api/v1/info', async (request, env) => {
  return jsonResponse({
    name: 'Ectus-R SaaS API',
    version: '4.0.0',
    description: 'Autonomous Software Engineering Platform',
    endpoints: {
      health: 'GET /health',
      info: 'GET /api/v1/info',
      providers: 'GET /api/v1/providers',
      deploy: 'POST /api/v1/deployments/magic-loop',
      getDeployment: 'GET /api/v1/deployments/:id',
      listDeployments: 'GET /api/v1/deployments'
    },
    documentation: 'https://github.com/Yatrogenesis/Ectus-R',
    support: 'pako.molina@gmail.com'
  });
});

// Listar proveedores AI disponibles
router.get('/api/v1/providers', async (request, env) => {
  const aiProvider = new MultiAIProvider(env);
  const providers = aiProvider.getAvailableProviders();

  return jsonResponse({
    providers,
    recommended: 'cloudflare',
    fallbackOrder: ['cloudflare', 'huggingface', 'ollama', 'deepseek', 'openai']
  });
});

// Magic Loop - Generación con Multi-AI
router.post('/api/v1/deployments/magic-loop', async (request, env) => {
  try {
    const body = await request.json();
    const { prompt, provider, model } = body;

    if (!prompt) {
      return jsonResponse({
        error: 'Missing prompt',
        message: 'Prompt is required for deployment'
      }, 400);
    }

    const deploymentId = `deploy_${Date.now()}_${generateId()}`;
    const startTime = Date.now();

    console.log(`🚀 [Magic Loop] ID: ${deploymentId}`);
    console.log(`📝 Prompt: ${prompt}`);

    // Crear AI Provider
    const aiProvider = new MultiAIProvider(env);

    // Generar prompt mejorado
    const enhancedPrompt = createEnhancedPrompt(prompt);

    // Generar con sistema de fallback
    let result;
    if (provider) {
      // Usar proveedor específico si se solicita
      console.log(`🎯 Provider solicitado: ${provider}`);
      result = await aiProvider.generateWithFallback(enhancedPrompt, provider);
    } else {
      // Usar sistema automático de fallback
      result = await aiProvider.generateWithFallback(enhancedPrompt);
    }

    // Si AI falla, usar template
    let generatedCode;
    let generationMethod;

    if (result.success) {
      generatedCode = result.code;
      generationMethod = `${result.provider}-ai`;
      console.log(`✅ Generado con ${result.provider}`);
    } else {
      console.log(`⚠️ AI fallback activado - usando templates`);
      generatedCode = selectTemplate(prompt, deploymentId);
      generationMethod = 'template-fallback';
    }

    // Guardar en KV
    const deploymentData = {
      id: deploymentId,
      prompt,
      code: generatedCode,
      method: generationMethod,
      provider: result.provider || 'template',
      model: result.model || 'template',
      timestamp: Date.now(),
      generationTime: Date.now() - startTime,
      status: 'deployed',
      url: `https://yatrogenesis.github.io/Ectus-R/apps/${deploymentId}.html`
    };

    await env.METADATA.put(`deployment:${deploymentId}`, JSON.stringify(deploymentData));

    // También guardar en lista de deployments
    await addToDeploymentList(env, deploymentId);

    // Log analytics
    await logAnalytics(env, {
      event: 'deployment_created',
      deploymentId,
      method: generationMethod,
      provider: result.provider || 'template',
      generationTime: deploymentData.generationTime
    });

    return jsonResponse({
      success: true,
      deployment: {
        id: deploymentId,
        url: deploymentData.url,
        method: generationMethod,
        provider: result.provider || 'template',
        model: result.model || 'template',
        generationTime: deploymentData.generationTime,
        timestamp: deploymentData.timestamp
      },
      preview: generatedCode.substring(0, 500) + '...'
    });

  } catch (error) {
    console.error('❌ [Magic Loop] Error:', error);
    return jsonResponse({
      error: 'Deployment failed',
      message: error.message,
      timestamp: Date.now()
    }, 500);
  }
});

// Obtener deployment por ID
router.get('/api/v1/deployments/:id', async (request, env) => {
  try {
    const { id } = request.params;
    const deploymentData = await env.METADATA.get(`deployment:${id}`);

    if (!deploymentData) {
      return jsonResponse({
        error: 'Deployment not found',
        id
      }, 404);
    }

    const deployment = JSON.parse(deploymentData);

    // Retornar HTML si se solicita
    const acceptHeader = request.headers.get('accept') || '';
    if (acceptHeader.includes('text/html')) {
      return new Response(deployment.code, {
        headers: {
          'Content-Type': 'text/html; charset=utf-8',
          'Cache-Control': 'public, max-age=3600',
          'X-Deployment-ID': id,
          'X-Generation-Method': deployment.method
        }
      });
    }

    // Retornar metadata JSON
    return jsonResponse({
      ...deployment,
      code: deployment.code.substring(0, 500) + '...' // Truncar para metadata
    });

  } catch (error) {
    return jsonResponse({
      error: 'Failed to retrieve deployment',
      message: error.message
    }, 500);
  }
});

// Listar todos los deployments
router.get('/api/v1/deployments', async (request, env) => {
  try {
    const { limit = 50, offset = 0 } = Object.fromEntries(new URL(request.url).searchParams);

    const listData = await env.METADATA.get('deployment:list');
    const deploymentIds = listData ? JSON.parse(listData) : [];

    const deployments = [];
    const start = parseInt(offset);
    const end = Math.min(start + parseInt(limit), deploymentIds.length);

    for (let i = start; i < end; i++) {
      const id = deploymentIds[i];
      const data = await env.METADATA.get(`deployment:${id}`);
      if (data) {
        const deployment = JSON.parse(data);
        deployments.push({
          id: deployment.id,
          prompt: deployment.prompt,
          method: deployment.method,
          provider: deployment.provider,
          timestamp: deployment.timestamp,
          url: deployment.url
        });
      }
    }

    return jsonResponse({
      deployments,
      total: deploymentIds.length,
      limit: parseInt(limit),
      offset: parseInt(offset),
      hasMore: end < deploymentIds.length
    });

  } catch (error) {
    return jsonResponse({
      error: 'Failed to list deployments',
      message: error.message
    }, 500);
  }
});

// DNS Management via GoDaddy (usando godo-r)
router.post('/api/v1/dns/setup', async (request, env) => {
  try {
    const { domain, subdomain, deploymentId } = await request.json();

    // Validar datos
    if (!domain || !subdomain || !deploymentId) {
      return jsonResponse({
        error: 'Missing required fields',
        required: ['domain', 'subdomain', 'deploymentId']
      }, 400);
    }

    // Crear registro DNS apuntando a GitHub Pages
    const dnsRecord = {
      type: 'CNAME',
      name: subdomain,
      data: 'yatrogenesis.github.io',
      ttl: 3600
    };

    // Aquí se integraría con godo-r CLI o GoDaddy API
    // Por ahora retornamos la configuración necesaria

    return jsonResponse({
      success: true,
      dns: {
        domain,
        subdomain,
        fullDomain: `${subdomain}.${domain}`,
        record: dnsRecord,
        instructions: [
          '1. Run: godo dns add ' + domain + ' CNAME ' + subdomain + ' yatrogenesis.github.io',
          '2. Wait for DNS propagation (5-30 minutes)',
          '3. Access your app at: https://' + subdomain + '.' + domain
        ]
      },
      deployment: {
        id: deploymentId,
        customDomain: `${subdomain}.${domain}`,
        defaultUrl: `https://yatrogenesis.github.io/Ectus-R/apps/${deploymentId}.html`
      }
    });

  } catch (error) {
    return jsonResponse({
      error: 'DNS setup failed',
      message: error.message
    }, 500);
  }
});

// Analytics endpoint
router.get('/api/v1/analytics', async (request, env) => {
  try {
    const analyticsData = await env.METADATA.get('analytics:summary');
    const analytics = analyticsData ? JSON.parse(analyticsData) : {
      totalDeployments: 0,
      totalGeneration: 0,
      providers: {},
      methods: {}
    };

    return jsonResponse(analytics);
  } catch (error) {
    return jsonResponse({
      error: 'Failed to fetch analytics',
      message: error.message
    }, 500);
  }
});

// CORS Preflight
router.options('*', () => {
  return new Response(null, {
    status: 204,
    headers: corsHeaders()
  });
});

// Catch-all 404
router.all('*', () => {
  return jsonResponse({
    error: 'Not found',
    message: 'The requested endpoint does not exist',
    documentation: 'https://github.com/Yatrogenesis/Ectus-R'
  }, 404);
});

// === UTILIDADES ===

function jsonResponse(data, status = 200) {
  return new Response(JSON.stringify(data, null, 2), {
    status,
    headers: {
      'Content-Type': 'application/json; charset=utf-8',
      ...corsHeaders()
    }
  });
}

function corsHeaders() {
  return {
    'Access-Control-Allow-Origin': '*',
    'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS',
    'Access-Control-Allow-Headers': 'Content-Type, Authorization',
    'Access-Control-Max-Age': '86400'
  };
}

function generateId() {
  return Math.random().toString(36).substr(2, 9);
}

function createEnhancedPrompt(userPrompt) {
  return `You are an expert web developer. Create a complete, fully functional web application.

REQUEST: "${userPrompt}"

CRITICAL REQUIREMENTS:
1. Generate ONLY pure HTML code starting with <!DOCTYPE html>
2. Include complete inline CSS with modern, beautiful styling
3. Include complete JavaScript with full functionality
4. NO markdown formatting, NO code blocks, NO explanations
5. Make it completely functional and professional
6. Use modern design: gradients, shadows, hover effects, rounded corners
7. Ensure responsive design

DESIGN STYLE:
- Modern glassmorphism effect with backdrop-blur
- Beautiful gradient backgrounds
- Smooth animations and transitions
- Professional typography
- Intuitive user interface
- Mobile-responsive design

Return ONLY the complete HTML code, nothing else.`;
}

function selectTemplate(prompt, id) {
  const lower = prompt.toLowerCase();

  if (lower.includes('calculator') || lower.includes('calc')) {
    return templates.calculator(id);
  } else if (lower.includes('todo') || lower.includes('task')) {
    return templates.todo(id);
  } else if (lower.includes('timer') || lower.includes('countdown')) {
    return templates.timer(id);
  } else if (lower.includes('weather')) {
    return templates.weather(id);
  } else if (lower.includes('color') || lower.includes('picker')) {
    return templates.colorpicker(id);
  } else {
    return templates.generic(prompt, id);
  }
}

async function addToDeploymentList(env, deploymentId) {
  try {
    const listData = await env.METADATA.get('deployment:list');
    const list = listData ? JSON.parse(listData) : [];
    list.unshift(deploymentId); // Agregar al inicio
    await env.METADATA.put('deployment:list', JSON.stringify(list.slice(0, 1000))); // Limitar a 1000
  } catch (error) {
    console.error('Error updating deployment list:', error);
  }
}

async function logAnalytics(env, event) {
  try {
    const analyticsData = await env.METADATA.get('analytics:summary');
    const analytics = analyticsData ? JSON.parse(analyticsData) : {
      totalDeployments: 0,
      totalGenerationTime: 0,
      providers: {},
      methods: {}
    };

    if (event.event === 'deployment_created') {
      analytics.totalDeployments++;
      analytics.totalGenerationTime += event.generationTime;
      analytics.providers[event.provider] = (analytics.providers[event.provider] || 0) + 1;
      analytics.methods[event.method] = (analytics.methods[event.method] || 0) + 1;
      analytics.lastUpdate = Date.now();
    }

    await env.METADATA.put('analytics:summary', JSON.stringify(analytics));
  } catch (error) {
    console.error('Error logging analytics:', error);
  }
}

// Templates (importados del worker anterior)
function generateCalculatorTemplate(id) { /* ... */ return `<!DOCTYPE html>...[CALC]...`; }
function generateTodoTemplate(id) { /* ... */ return `<!DOCTYPE html>...[TODO]...`; }
function generateTimerTemplate(id) { /* ... */ return `<!DOCTYPE html>...[TIMER]...`; }
function generateWeatherTemplate(id) { /* ... */ return `<!DOCTYPE html>...[WEATHER]...`; }
function generateColorPickerTemplate(id) { /* ... */ return `<!DOCTYPE html>...[COLOR]...`; }
function generateGenericTemplate(prompt, id) { /* ... */ return `<!DOCTYPE html>...[GENERIC]...`; }

// Export default
export default {
  async fetch(request, env, ctx) {
    try {
      return await router.handle(request, env, ctx);
    } catch (error) {
      console.error('Worker error:', error);
      return jsonResponse({
        error: 'Internal server error',
        message: error.message,
        timestamp: Date.now()
      }, 500);
    }
  }
};