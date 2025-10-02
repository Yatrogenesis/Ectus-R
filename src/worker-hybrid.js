// Ectus-R SaaS API Worker - HYBRID: Smart AI + Perfect Templates
import { Router } from 'itty-router';

const router = Router();

// Perfect Templates (for fallback)
function generateFallbackTemplate(prompt, deploymentId) {
  const lowerPrompt = prompt.toLowerCase();

  if (lowerPrompt.includes('calculator')) {
    return generateCalculatorTemplate(deploymentId);
  } else if (lowerPrompt.includes('todo') || lowerPrompt.includes('task')) {
    return generateTodoTemplate(deploymentId);
  } else if (lowerPrompt.includes('timer')) {
    return generateTimerTemplate(deploymentId);
  } else if (lowerPrompt.includes('weather')) {
    return generateWeatherTemplate(deploymentId);
  } else if (lowerPrompt.includes('color')) {
    return generateColorPickerTemplate(deploymentId);
  } else {
    return generateGenericTemplate(prompt, deploymentId);
  }
}

// Perfect Calculator Template
function generateCalculatorTemplate(deploymentId) {
  return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Perfect Calculator</title>
    <style>
        body {
            font-family: 'Segoe UI', sans-serif;
            margin: 0;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
        }
        .calculator {
            background: rgba(255,255,255,0.1);
            padding: 25px;
            border-radius: 20px;
            backdrop-filter: blur(15px);
            box-shadow: 0 8px 32px rgba(0,0,0,0.3);
        }
        .display {
            width: 100%;
            height: 80px;
            font-size: 28px;
            text-align: right;
            padding: 0 20px;
            border: none;
            border-radius: 15px;
            margin-bottom: 20px;
            background: rgba(255,255,255,0.9);
            color: #333;
            box-sizing: border-box;
        }
        .buttons {
            display: grid;
            grid-template-columns: repeat(4, 1fr);
            gap: 15px;
            width: 300px;
        }
        button {
            height: 70px;
            border: none;
            border-radius: 15px;
            font-size: 20px;
            font-weight: bold;
            cursor: pointer;
            transition: all 0.2s ease;
        }
        .number {
            background: rgba(255,255,255,0.8);
            color: #333;
        }
        .number:hover {
            background: rgba(255,255,255,0.9);
            transform: scale(1.05);
        }
        .operator {
            background: #ff6b6b;
            color: white;
        }
        .operator:hover {
            background: #ff5252;
            transform: scale(1.05);
        }
        .equals {
            background: #4CAF50;
            color: white;
        }
        .equals:hover {
            background: #45a049;
            transform: scale(1.05);
        }
        .clear {
            background: #ffa726;
            color: white;
        }
        .clear:hover {
            background: #ff9800;
            transform: scale(1.05);
        }
    </style>
</head>
<body>
    <div class="calculator">
        <input type="text" class="display" id="display" readonly>
        <div class="buttons">
            <button class="clear" onclick="clearDisplay()">C</button>
            <button class="operator" onclick="appendToDisplay('/')">/</button>
            <button class="operator" onclick="appendToDisplay('*')">*</button>
            <button class="operator" onclick="deleteLast()">⌫</button>

            <button class="number" onclick="appendToDisplay('7')">7</button>
            <button class="number" onclick="appendToDisplay('8')">8</button>
            <button class="number" onclick="appendToDisplay('9')">9</button>
            <button class="operator" onclick="appendToDisplay('-')">-</button>

            <button class="number" onclick="appendToDisplay('4')">4</button>
            <button class="number" onclick="appendToDisplay('5')">5</button>
            <button class="number" onclick="appendToDisplay('6')">6</button>
            <button class="operator" onclick="appendToDisplay('+')">+</button>

            <button class="number" onclick="appendToDisplay('1')">1</button>
            <button class="number" onclick="appendToDisplay('2')">2</button>
            <button class="number" onclick="appendToDisplay('3')">3</button>
            <button class="equals" onclick="calculate()" style="grid-row: span 2;">=</button>

            <button class="number" onclick="appendToDisplay('0')" style="grid-column: span 2;">0</button>
            <button class="number" onclick="appendToDisplay('.')">.</button>
        </div>
    </div>

    <script>
        let display = document.getElementById('display');

        function appendToDisplay(value) {
            display.value += value;
        }

        function clearDisplay() {
            display.value = '';
        }

        function deleteLast() {
            display.value = display.value.slice(0, -1);
        }

        function calculate() {
            try {
                const result = eval(display.value);
                display.value = result;
            } catch (e) {
                display.value = 'Error';
                setTimeout(clearDisplay, 1500);
            }
        }

        console.log('Perfect Calculator - ID: ${deploymentId}');
    </script>
</body>
</html>`;
}

// Todo Template (abbreviated for space)
function generateTodoTemplate(deploymentId) {
  return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Perfect Todo List</title>
    <style>
        body { font-family: 'Segoe UI', sans-serif; margin: 0; padding: 20px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; color: white; }
        .container { max-width: 500px; margin: 50px auto; background: rgba(255,255,255,0.1); padding: 30px; border-radius: 20px; backdrop-filter: blur(15px); box-shadow: 0 8px 32px rgba(0,0,0,0.3); }
        .input-container { display: flex; gap: 10px; margin-bottom: 20px; }
        input[type="text"] { flex: 1; padding: 15px; border: none; border-radius: 10px; font-size: 16px; }
        button { padding: 15px 25px; border: none; border-radius: 10px; background: #4CAF50; color: white; cursor: pointer; font-weight: bold; }
        .todo-item { background: rgba(255,255,255,0.9); color: #333; margin: 10px 0; padding: 15px; border-radius: 10px; display: flex; justify-content: space-between; align-items: center; }
    </style>
</head>
<body>
    <div class="container">
        <h1>📝 Perfect Todo List</h1>
        <div class="input-container">
            <input type="text" id="todoInput" placeholder="Add a new task...">
            <button onclick="addTodo()">Add</button>
        </div>
        <ul id="todoList" style="list-style: none; padding: 0;"></ul>
    </div>
    <script>
        let todos = [];
        function addTodo() {
            const input = document.getElementById('todoInput');
            if (input.value.trim()) {
                todos.push({ id: Date.now(), text: input.value.trim(), completed: false });
                input.value = '';
                renderTodos();
            }
        }
        function toggleTodo(id) {
            todos = todos.map(todo => todo.id === id ? { ...todo, completed: !todo.completed } : todo);
            renderTodos();
        }
        function deleteTodo(id) {
            todos = todos.filter(todo => todo.id !== id);
            renderTodos();
        }
        function renderTodos() {
            document.getElementById('todoList').innerHTML = todos.map(todo => \`
                <li class="todo-item \${todo.completed ? 'completed' : ''}">
                    <span>\${todo.text}</span>
                    <div>
                        <button onclick="toggleTodo(\${todo.id})">\${todo.completed ? 'Undo' : 'Done'}</button>
                        <button onclick="deleteTodo(\${todo.id})" style="background: #f44336;">Delete</button>
                    </div>
                </li>
            \`).join('');
        }
        document.getElementById('todoInput').addEventListener('keypress', e => { if (e.key === 'Enter') addTodo(); });
        console.log('Perfect Todo - ID: ${deploymentId}');
    </script>
</body>
</html>`;
}

// Generic Template
function generateGenericTemplate(prompt, deploymentId) {
  return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Custom App</title>
    <style>
        body { font-family: 'Segoe UI', sans-serif; margin: 0; padding: 40px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; color: white; }
        .container { max-width: 800px; margin: 0 auto; text-align: center; }
        .card { background: rgba(255,255,255,0.1); padding: 40px; border-radius: 20px; backdrop-filter: blur(15px); box-shadow: 0 8px 32px rgba(0,0,0,0.3); }
        .btn { background: #ff6b6b; color: white; padding: 15px 30px; border: none; border-radius: 10px; cursor: pointer; font-size: 16px; font-weight: bold; margin: 10px; transition: all 0.2s; }
        .btn:hover { background: #ff5252; transform: scale(1.05); }
    </style>
</head>
<body>
    <div class="container">
        <div class="card">
            <h1>🚀 Custom Generated App</h1>
            <p><strong>Your Request:</strong> "${prompt}"</p>
            <p>This application was generated based on your specific requirements.</p>
            <button class="btn" onclick="alert('Application is working!')">Test App</button>
            <button class="btn" onclick="document.body.style.background = document.body.style.background.includes('667eea') ? 'linear-gradient(135deg, #f093fb 0%, #f5576c 100%)' : 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)'">Change Theme</button>
        </div>
    </div>
    <script>console.log('Custom App - ID: ${deploymentId}');</script>
</body>
</html>`;
}

// Health check endpoint
router.get('/health', async (request, env) => {
  return new Response(JSON.stringify({
    status: 'healthy',
    timestamp: Date.now(),
    version: '3.0.0-hybrid',
    ai_enabled: true,
    fallback_enabled: true
  }), {
    headers: { 'Content-Type': 'application/json' },
    status: 200
  });
});

// Magic Loop endpoint - HYBRID APPROACH
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

    const deploymentId = \`deploy_\${Date.now()}_\${Math.random().toString(36).substr(2, 9)}\`;

    // Step 1: Try AI first with optimized prompt
    let generatedCode = null;
    let generationMethod = 'fallback';

    try {
      console.log('Attempting AI generation with Groq...');

      const optimizedPrompt = \`Create a complete, functional web application for: "\${prompt}"

CRITICAL REQUIREMENTS:
- Generate ONLY pure HTML code that starts with <!DOCTYPE html>
- Include complete inline CSS with modern styling
- Include complete JavaScript functionality
- NO markdown, NO code blocks, NO explanations
- Make it fully functional and professional
- Use modern design with gradients and animations

For calculators: Include ALL numbers 0-9 and operations +, -, *, /, =, clear
For todo apps: Include add, delete, edit, mark complete functionality
For timers: Include start, stop, reset with time input
For other apps: Make them interactive and functional

Return ONLY the HTML code, nothing else.\`;

      const hfResponse = await fetch('https://api-inference.huggingface.co/models/openai/gpt-oss-120b', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          inputs: \`System: You are an expert web developer. Generate ONLY clean HTML with inline CSS and JavaScript. No markdown, no explanations, just pure HTML starting with <!DOCTYPE html>.

User: \${optimizedPrompt}
          if (htmlMatch && htmlMatch[1]) {
            aiCode = htmlMatch[1].trim();
          }
        }

        // Validate AI response
        if (aiCode.includes('<!DOCTYPE html>') && aiCode.includes('</html>')) {
          generatedCode = aiCode;
          generationMethod = 'ai';
          console.log('AI generation successful!');
        } else {
          console.log('AI response invalid, falling back to template');
        }
      } else {
        console.log('Groq API failed, falling back to template');
      }
    } catch (aiError) {
      console.log('AI error:', aiError.message, '- falling back to template');
    }

    // Step 2: Use perfect template as fallback
    if (!generatedCode) {
      console.log('Using perfect template fallback');
      generatedCode = generateFallbackTemplate(prompt, deploymentId);
      generationMethod = 'template';
    }

    // Store in KV
    await env.CACHE.put(\`deployment:\${deploymentId}\`, JSON.stringify({
      id: deploymentId,
      prompt: prompt,
      code: generatedCode,
      status: 'completed',
      method: generationMethod,
      created_at: new Date().toISOString(),
      url: \`https://ectus-r-saas.pako-molina.workers.dev/api/v1/deployments/\${deploymentId}/preview\`
    }));

    return new Response(JSON.stringify({
      success: true,
      deployment_id: deploymentId,
      prompt: prompt,
      status: 'completed',
      message: \`App generated via \${generationMethod === 'ai' ? 'AI' : 'template'}\`,
      preview_url: \`https://ectus-r-saas.pako-molina.workers.dev/api/v1/deployments/\${deploymentId}/preview\`,
      generation_method: generationMethod
    }), {
      headers: { 'Content-Type': 'application/json' },
      status: 200
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

// Deployment preview endpoint
router.get('/api/v1/deployments/:id/preview', async (request, env) => {
  try {
    const { id } = request.params;
    const deployment = await env.CACHE.get(\`deployment:\${id}\`);

    if (!deployment) {
      return new Response('Deployment not found', { status: 404 });
    }

    const deploymentData = JSON.parse(deployment);

    return new Response(deploymentData.code, {
      headers: {
        'Content-Type': 'text/html; charset=utf-8',
        'Cache-Control': 'public, max-age=3600',
        'X-Generated-By': \`Ectus-R-\${deploymentData.method || 'hybrid'}\`
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
    const deployment = await env.CACHE.get(\`deployment:\${id}\`);

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
    delete deploymentData.code;

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
      deployments: deployments.slice(0, 20),
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
  const newResponse = new Response(response.body, {
    status: response.status,
    statusText: response.statusText,
    headers: response.headers
  });
  newResponse.headers.set('Access-Control-Allow-Origin', '*');
  newResponse.headers.set('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS');
  newResponse.headers.set('Access-Control-Allow-Headers', 'Content-Type, Authorization');
  return newResponse;
}

// Main worker handler
export default {
  async fetch(request, env, ctx) {
    try {
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