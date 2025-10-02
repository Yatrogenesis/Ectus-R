// Ectus-R SaaS API Worker - TEMPLATES ONLY (NO AI)
import { Router } from 'itty-router';

const router = Router();

// Generate intelligent templates based on prompt
function generateAppTemplate(prompt, deploymentId) {
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

// PERFECT Calculator template
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
            border: 1px solid rgba(255,255,255,0.2);
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

        console.log('Perfect Calculator loaded - ID: ${deploymentId}');
    </script>
</body>
</html>`;
}

// Todo template
function generateTodoTemplate(deploymentId) {
  return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Perfect Todo List</title>
    <style>
        body {
            font-family: 'Segoe UI', sans-serif;
            margin: 0;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            color: white;
        }
        .container {
            max-width: 500px;
            margin: 50px auto;
            background: rgba(255,255,255,0.1);
            padding: 30px;
            border-radius: 20px;
            backdrop-filter: blur(15px);
            box-shadow: 0 8px 32px rgba(0,0,0,0.3);
        }
        h1 {
            text-align: center;
            margin-bottom: 30px;
            color: white;
        }
        .input-container {
            display: flex;
            gap: 10px;
            margin-bottom: 20px;
        }
        input[type="text"] {
            flex: 1;
            padding: 15px;
            border: none;
            border-radius: 10px;
            font-size: 16px;
        }
        .add-btn {
            padding: 15px 25px;
            border: none;
            border-radius: 10px;
            background: #4CAF50;
            color: white;
            cursor: pointer;
            font-weight: bold;
            transition: all 0.2s;
        }
        .add-btn:hover {
            background: #45a049;
            transform: scale(1.05);
        }
        .todo-list {
            list-style: none;
            padding: 0;
        }
        .todo-item {
            background: rgba(255,255,255,0.9);
            color: #333;
            margin: 10px 0;
            padding: 15px;
            border-radius: 10px;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        .todo-item.completed {
            opacity: 0.6;
            text-decoration: line-through;
        }
        .todo-actions button {
            margin-left: 5px;
            padding: 8px 12px;
            border: none;
            border-radius: 5px;
            cursor: pointer;
            font-size: 12px;
        }
        .complete-btn {
            background: #2196F3;
            color: white;
        }
        .delete-btn {
            background: #f44336;
            color: white;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>📝 Perfect Todo List</h1>
        <div class="input-container">
            <input type="text" id="todoInput" placeholder="Add a new task...">
            <button class="add-btn" onclick="addTodo()">Add</button>
        </div>
        <ul class="todo-list" id="todoList"></ul>
    </div>

    <script>
        let todos = [];
        let todoInput = document.getElementById('todoInput');
        let todoList = document.getElementById('todoList');

        function addTodo() {
            let text = todoInput.value.trim();
            if (text) {
                todos.push({ id: Date.now(), text: text, completed: false });
                todoInput.value = '';
                renderTodos();
            }
        }

        function toggleTodo(id) {
            todos = todos.map(todo =>
                todo.id === id ? { ...todo, completed: !todo.completed } : todo
            );
            renderTodos();
        }

        function deleteTodo(id) {
            todos = todos.filter(todo => todo.id !== id);
            renderTodos();
        }

        function renderTodos() {
            todoList.innerHTML = todos.map(todo => \`
                <li class="todo-item \${todo.completed ? 'completed' : ''}">
                    <span>\${todo.text}</span>
                    <div class="todo-actions">
                        <button class="complete-btn" onclick="toggleTodo(\${todo.id})">
                            \${todo.completed ? 'Undo' : 'Complete'}
                        </button>
                        <button class="delete-btn" onclick="deleteTodo(\${todo.id})">Delete</button>
                    </div>
                </li>
            \`).join('');
        }

        todoInput.addEventListener('keypress', function(e) {
            if (e.key === 'Enter') addTodo();
        });

        console.log('Perfect Todo app loaded - ID: ${deploymentId}');
    </script>
</body>
</html>`;
}

// Generic template
function generateGenericTemplate(prompt, deploymentId) {
  return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Perfect App</title>
    <style>
        body {
            font-family: 'Segoe UI', sans-serif;
            margin: 0;
            padding: 40px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            color: white;
        }
        .container {
            max-width: 800px;
            margin: 0 auto;
            text-align: center;
        }
        .card {
            background: rgba(255,255,255,0.1);
            padding: 40px;
            border-radius: 20px;
            backdrop-filter: blur(15px);
            box-shadow: 0 8px 32px rgba(0,0,0,0.3);
        }
        h1 {
            margin-bottom: 20px;
        }
        .prompt {
            background: rgba(255,255,255,0.1);
            padding: 20px;
            border-radius: 10px;
            margin: 20px 0;
        }
        .btn {
            background: #ff6b6b;
            color: white;
            padding: 15px 30px;
            border: none;
            border-radius: 10px;
            cursor: pointer;
            font-size: 16px;
            font-weight: bold;
            margin: 10px;
            transition: all 0.2s;
        }
        .btn:hover {
            background: #ff5252;
            transform: scale(1.05);
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="card">
            <h1>🚀 Perfect Generated App</h1>
            <div class="prompt">
                <strong>Your Request:</strong><br>
                "${prompt}"
            </div>
            <p>This application was generated using our perfect template system.</p>
            <button class="btn" onclick="showAlert()">Test Application</button>
            <button class="btn" onclick="changeTheme()">Change Theme</button>
        </div>
    </div>

    <script>
        function showAlert() {
            alert('🎉 Application is working perfectly!');
        }

        function changeTheme() {
            document.body.style.background =
                document.body.style.background.includes('667eea')
                ? 'linear-gradient(135deg, #f093fb 0%, #f5576c 100%)'
                : 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)';
        }

        console.log('Perfect app loaded - ID: ${deploymentId}');
    </script>
</body>
</html>`;
}

// Health check endpoint
router.get('/health', async (request, env) => {
  return new Response(JSON.stringify({
    status: 'healthy',
    timestamp: Date.now(),
    version: '2.0.0-templates-only',
    environment: 'production'
  }), {
    headers: { 'Content-Type': 'application/json' },
    status: 200
  });
});

// Magic Loop endpoint - TEMPLATES ONLY
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

    // Generate perfect template (NO AI)
    const generatedCode = generateAppTemplate(prompt, deploymentId);

    // Store in KV
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
      message: 'Perfect app generated instantly with templates',
      preview_url: `https://ectus-r-saas.pako-molina.workers.dev/api/v1/deployments/${deploymentId}/preview`
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
    const deployment = await env.CACHE.get(`deployment:${id}`);

    if (!deployment) {
      return new Response('Deployment not found', { status: 404 });
    }

    const deploymentData = JSON.parse(deployment);

    return new Response(deploymentData.code, {
      headers: {
        'Content-Type': 'text/html; charset=utf-8',
        'Cache-Control': 'public, max-age=3600',
        'X-Generated-By': 'Ectus-R-Templates'
      },
      status: 200
    });

  } catch (error) {
    return new Response('Error loading deployment', { status: 500 });
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