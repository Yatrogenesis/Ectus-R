// Ectus-R Dashboard API Worker
// Mock API implementation compatible with web-dashboard APIClient

import { Router } from 'itty-router';
import { v4 as uuidv4 } from 'uuid';

const router = Router();

// CORS headers for frontend compatibility
const corsHeaders = {
  'Access-Control-Allow-Origin': '*',
  'Access-Control-Allow-Methods': 'GET, POST, PUT, PATCH, DELETE, OPTIONS',
  'Access-Control-Allow-Headers': 'Content-Type, Authorization',
  'Content-Type': 'application/json',
};

// Mock data generator
function generateMockProject(overrides = {}) {
  const id = overrides.id || uuidv4();
  const now = new Date().toISOString();

  return {
    id,
    name: overrides.name || `Project ${id.substring(0, 8)}`,
    description: overrides.description || 'AI-generated full-stack application',
    language: overrides.language || 'TypeScript',
    framework: overrides.framework || 'React',
    status: overrides.status || 'active',
    created_at: overrides.created_at || now,
    updated_at: overrides.updated_at || now,
    repository_url: overrides.repository_url || `https://github.com/ectus-r/${id}`,
    deployment_count: overrides.deployment_count || Math.floor(Math.random() * 50),
    lastDeployment: overrides.lastDeployment || now,
    createdAt: overrides.createdAt || now,
    repository: overrides.repository || `ectus-r/${id.substring(0, 8)}`,
    environment: overrides.environment || 'production',
    team: overrides.team || ['AI Agent', 'Claude'],
    deploymentUrl: overrides.deploymentUrl || `https://${id.substring(0, 8)}.ectus.ai`,
    visibility: overrides.visibility || 'private',
    tags: overrides.tags || ['ai-generated', 'production'],
    ...overrides,
  };
}

// In-memory storage (reset on each worker restart)
let projectsCache = [
  generateMockProject({
    name: 'E-Commerce Platform',
    description: 'Full-featured e-commerce with AI recommendations',
    language: 'TypeScript',
    framework: 'Next.js',
    status: 'active',
    tags: ['e-commerce', 'ai', 'production'],
  }),
  generateMockProject({
    name: 'Analytics Dashboard',
    description: 'Real-time analytics with ML-powered insights',
    language: 'Python',
    framework: 'FastAPI',
    status: 'deploying',
    tags: ['analytics', 'ml', 'dashboard'],
  }),
  generateMockProject({
    name: 'Chat Application',
    description: 'WebSocket-based real-time chat with AI moderation',
    language: 'TypeScript',
    framework: 'Nest.js',
    status: 'testing',
    tags: ['chat', 'websocket', 'ai'],
  }),
];

// ===== ROUTES =====

// Health check
router.get('/health', () => {
  return new Response(JSON.stringify({ status: 'ok', timestamp: new Date().toISOString() }), {
    headers: corsHeaders,
  });
});

// List all projects
router.get('/api/v1/projects', (request) => {
  const url = new URL(request.url);
  const search = url.searchParams.get('search');
  const status = url.searchParams.get('status');
  const language = url.searchParams.get('language');
  const offset = parseInt(url.searchParams.get('offset') || '0');
  const limit = parseInt(url.searchParams.get('limit') || '20');

  let filtered = [...projectsCache];

  // Apply filters
  if (search) {
    const searchLower = search.toLowerCase();
    filtered = filtered.filter(p =>
      p.name.toLowerCase().includes(searchLower) ||
      p.description.toLowerCase().includes(searchLower)
    );
  }

  if (status && status !== 'all') {
    filtered = filtered.filter(p => p.status === status);
  }

  if (language && language !== 'all') {
    filtered = filtered.filter(p => p.language === language);
  }

  // Pagination
  const paginatedProjects = filtered.slice(offset, offset + limit);

  return new Response(JSON.stringify({
    projects: paginatedProjects,
    total: filtered.length,
    offset,
    limit,
  }), {
    headers: corsHeaders,
  });
});

// Get single project
router.get('/api/v1/projects/:id', (request) => {
  const { id } = request.params;
  const project = projectsCache.find(p => p.id === id);

  if (!project) {
    return new Response(JSON.stringify({ error: 'Project not found' }), {
      status: 404,
      headers: corsHeaders,
    });
  }

  return new Response(JSON.stringify(project), {
    headers: corsHeaders,
  });
});

// Create project
router.post('/api/v1/projects', async (request) => {
  const body = await request.json();
  const newProject = generateMockProject(body);

  projectsCache.push(newProject);

  return new Response(JSON.stringify(newProject), {
    status: 201,
    headers: corsHeaders,
  });
});

// Update project
router.patch('/api/v1/projects/:id', async (request) => {
  const { id } = request.params;
  const updates = await request.json();

  const index = projectsCache.findIndex(p => p.id === id);

  if (index === -1) {
    return new Response(JSON.stringify({ error: 'Project not found' }), {
      status: 404,
      headers: corsHeaders,
    });
  }

  projectsCache[index] = {
    ...projectsCache[index],
    ...updates,
    updated_at: new Date().toISOString(),
  };

  return new Response(JSON.stringify(projectsCache[index]), {
    headers: corsHeaders,
  });
});

// Delete project
router.delete('/api/v1/projects/:id', (request) => {
  const { id } = request.params;
  const index = projectsCache.findIndex(p => p.id === id);

  if (index === -1) {
    return new Response(JSON.stringify({ error: 'Project not found' }), {
      status: 404,
      headers: corsHeaders,
    });
  }

  projectsCache.splice(index, 1);

  return new Response(JSON.stringify({ success: true }), {
    headers: corsHeaders,
  });
});

// Deploy project
router.post('/api/v1/projects/:id/deploy', async (request) => {
  const { id } = request.params;
  const { environment } = await request.json();

  const project = projectsCache.find(p => p.id === id);

  if (!project) {
    return new Response(JSON.stringify({ error: 'Project not found' }), {
      status: 404,
      headers: corsHeaders,
    });
  }

  // Update project status
  const index = projectsCache.findIndex(p => p.id === id);
  projectsCache[index].status = 'deploying';
  projectsCache[index].environment = environment || 'production';

  // Simulate deployment
  const deploymentResult = {
    deploymentUrl: `https://${id.substring(0, 8)}.${environment || 'production'}.ectus.ai`,
    deploymentId: uuidv4(),
    status: 'deploying',
  };

  return new Response(JSON.stringify(deploymentResult), {
    headers: corsHeaders,
  });
});

// Get project logs
router.get('/api/v1/projects/:id/logs', (request) => {
  const url = new URL(request.url);
  const limit = parseInt(url.searchParams.get('limit') || '100');

  const logs = Array.from({ length: Math.min(limit, 50) }, (_, i) => {
    const timestamp = new Date(Date.now() - i * 60000).toISOString();
    const messages = [
      `[${timestamp}] Starting build process...`,
      `[${timestamp}] Installing dependencies...`,
      `[${timestamp}] Running tests... PASSED`,
      `[${timestamp}] Building production bundle...`,
      `[${timestamp}] Deployment successful`,
    ];
    return messages[Math.floor(Math.random() * messages.length)];
  });

  return new Response(JSON.stringify({ logs }), {
    headers: corsHeaders,
  });
});

// AI Code Generation
router.post('/api/ai/generate', async (request) => {
  const { requirements, language, framework } = await request.json();

  // Simulate AI generation
  const generated = {
    language,
    framework: framework || 'None',
    code: `// AI-generated code for: ${requirements}\nfunction main() {\n  console.log('Generated by Ectus-R AI');\n}`,
    files: [
      {
        path: 'src/main.js',
        content: `// Main entry point\nconsole.log('${requirements}');`,
      },
    ],
    tests: [
      {
        path: 'tests/main.test.js',
        content: `// Auto-generated tests\ntest('${requirements}', () => {\n  expect(true).toBe(true);\n});`,
      },
    ],
  };

  return new Response(JSON.stringify(generated), {
    headers: corsHeaders,
  });
});

// Run QA
router.post('/api/v1/projects/:id/qa', () => {
  const result = {
    success: true,
    testsRun: 42,
    testsPassed: 40,
    testsFailed: 2,
    failures: [
      {
        testName: 'Login flow validation',
        failureMessage: 'Expected status 200, got 401',
        filePath: 'tests/auth.test.ts',
        lineNumber: 23,
      },
    ],
    autocorrectionAttempts: 1,
  };

  return new Response(JSON.stringify(result), {
    headers: corsHeaders,
  });
});

// Analyze project
router.post('/api/v1/projects/:id/analyze', () => {
  const analysis = {
    technicalDebtScore: 72,
    codeQualityScore: 85,
    securityScore: 90,
    performanceScore: 78,
    recommendations: [
      'Reduce cyclomatic complexity in auth module',
      'Update dependencies to latest versions',
      'Add error boundaries to React components',
      'Implement API response caching',
    ],
  };

  return new Response(JSON.stringify(analysis), {
    headers: corsHeaders,
  });
});

// Apply refactoring
router.post('/api/v1/projects/:id/refactor', async (request) => {
  const operation = await request.json();

  const result = {
    success: true,
    changesApplied: [
      `Applied ${operation.operationType} to ${operation.targetFile}`,
      'Generated unit tests for refactored code',
    ],
    testsGenerated: 5,
    testsPassed: true,
  };

  return new Response(JSON.stringify(result), {
    headers: corsHeaders,
  });
});

// Analytics
router.get('/api/analytics/:projectId', (request) => {
  const url = new URL(request.url);
  const range = url.searchParams.get('range') || '7d';

  const analytics = {
    deployments: 42,
    apiCalls: 15847,
    avgResponseTime: 142,
    errorRate: 0.3,
    uptime: 99.9,
    timeRange: range,
  };

  return new Response(JSON.stringify(analytics), {
    headers: corsHeaders,
  });
});

// OPTIONS for CORS preflight
router.options('*', () => {
  return new Response(null, {
    headers: corsHeaders,
  });
});

// 404 handler
router.all('*', () => {
  return new Response(JSON.stringify({ error: 'Not Found' }), {
    status: 404,
    headers: corsHeaders,
  });
});

// Worker entry point
export default {
  async fetch(request, env, ctx) {
    return router.handle(request, env, ctx);
  },
};
