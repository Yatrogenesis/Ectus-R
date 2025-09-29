// Ectus-R SaaS API Worker - HYBRID: HuggingFace AI + Perfect Templates
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

// Todo Template
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

// Timer Template
function generateTimerTemplate(deploymentId) {
  return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Perfect Timer</title>
    <style>
        body { font-family: 'Segoe UI', sans-serif; margin: 0; padding: 20px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; color: white; display: flex; justify-content: center; align-items: center; }
        .container { background: rgba(255,255,255,0.1); padding: 40px; border-radius: 20px; backdrop-filter: blur(15px); box-shadow: 0 8px 32px rgba(0,0,0,0.3); text-align: center; }
        .timer-display { font-size: 4rem; font-weight: bold; margin: 30px 0; color: #fff; text-shadow: 0 0 20px rgba(255,255,255,0.5); }
        button { padding: 15px 25px; border: none; border-radius: 10px; font-size: 16px; font-weight: bold; cursor: pointer; margin: 5px; transition: all 0.2s; }
        .start { background: #4CAF50; color: white; }
        .stop { background: #f44336; color: white; }
        .reset { background: #ffa726; color: white; }
        input { padding: 10px; border: none; border-radius: 5px; width: 60px; text-align: center; margin: 5px; }
    </style>
</head>
<body>
    <div class="container">
        <h1>⏱️ Perfect Timer</h1>
        <div class="timer-display" id="display">00:00:00</div>
        <div>
            <input type="number" id="minutes" placeholder="MM" min="0" max="59" value="5">
            <input type="number" id="seconds" placeholder="SS" min="0" max="59" value="0">
        </div>
        <div>
            <button class="start" onclick="startTimer()">Start</button>
            <button class="stop" onclick="stopTimer()">Stop</button>
            <button class="reset" onclick="resetTimer()">Reset</button>
        </div>
    </div>
    <script>
        let timer = null;
        let timeLeft = 0;
        let display = document.getElementById('display');
        function updateDisplay() {
            let minutes = Math.floor(timeLeft / 60);
            let seconds = timeLeft % 60;
            display.textContent = String(minutes).padStart(2, '0') + ':' + String(seconds).padStart(2, '0') + ':00';
        }
        function startTimer() {
            if (timeLeft === 0) {
                let minutes = parseInt(document.getElementById('minutes').value) || 0;
                let seconds = parseInt(document.getElementById('seconds').value) || 0;
                timeLeft = minutes * 60 + seconds;
            }
            if (timeLeft > 0 && !timer) {
                timer = setInterval(() => {
                    timeLeft--;
                    updateDisplay();
                    if (timeLeft === 0) {
                        clearInterval(timer);
                        timer = null;
                        alert('⏰ Time\\'s up!');
                    }
                }, 1000);
            }
        }
        function stopTimer() {
            if (timer) {
                clearInterval(timer);
                timer = null;
            }
        }
        function resetTimer() {
            stopTimer();
            timeLeft = 0;
            updateDisplay();
        }
        updateDisplay();
        console.log('Perfect Timer - ID: ${deploymentId}');
    </script>
</body>
</html>`;
}

// Color Picker Template
function generateColorPickerTemplate(deploymentId) {
  return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Perfect Color Picker</title>
    <style>
        body { font-family: 'Segoe UI', sans-serif; margin: 0; padding: 20px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; color: white; }
        .container { max-width: 500px; margin: 50px auto; background: rgba(255,255,255,0.1); padding: 30px; border-radius: 20px; backdrop-filter: blur(15px); box-shadow: 0 8px 32px rgba(0,0,0,0.3); }
        .color-display { width: 100%; height: 200px; border-radius: 10px; margin: 20px 0; border: 3px solid rgba(255,255,255,0.3); }
        .color-inputs { display: grid; grid-template-columns: 1fr 1fr; gap: 20px; margin: 20px 0; }
        .input-group { background: rgba(255,255,255,0.1); padding: 15px; border-radius: 10px; }
        input[type="color"] { width: 100%; height: 50px; border: none; border-radius: 5px; cursor: pointer; }
        .value-display { background: rgba(255,255,255,0.9); color: #333; padding: 10px; border-radius: 5px; margin: 10px 0; font-family: monospace; }
        button { background: #4CAF50; color: white; border: none; padding: 8px 15px; border-radius: 5px; cursor: pointer; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🎨 Perfect Color Picker</h1>
        <div class="color-display" id="colorDisplay"></div>
        <div class="color-inputs">
            <div class="input-group">
                <label>Color Picker:</label>
                <input type="color" id="colorPicker" onchange="updateFromPicker()">
            </div>
            <div class="input-group">
                <label>RGB Value:</label>
                <div class="value-display" id="rgbDisplay">rgb(128, 128, 128)</div>
                <button onclick="copyToClipboard('rgb')">Copy RGB</button>
                <label>HEX Value:</label>
                <div class="value-display" id="hexDisplay">#808080</div>
                <button onclick="copyToClipboard('hex')">Copy HEX</button>
            </div>
        </div>
    </div>
    <script>
        function updateFromPicker() {
            const hex = document.getElementById('colorPicker').value;
            const r = parseInt(hex.substr(1, 2), 16);
            const g = parseInt(hex.substr(3, 2), 16);
            const b = parseInt(hex.substr(5, 2), 16);
            const rgbValue = \`rgb(\${r}, \${g}, \${b})\`;
            document.getElementById('colorDisplay').style.backgroundColor = rgbValue;
            document.getElementById('rgbDisplay').textContent = rgbValue;
            document.getElementById('hexDisplay').textContent = hex.toUpperCase();
        }
        function copyToClipboard(type) {
            const text = type === 'rgb'
                ? document.getElementById('rgbDisplay').textContent
                : document.getElementById('hexDisplay').textContent;
            navigator.clipboard.writeText(text).then(() => {
                alert(\`\${type.toUpperCase()} value copied to clipboard!\`);
            });
        }
        updateFromPicker();
        console.log('Perfect Color Picker - ID: ${deploymentId}');
    </script>
</body>
</html>`;
}

// Weather Template
function generateWeatherTemplate(deploymentId) {
  return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Perfect Weather Dashboard</title>
    <style>
        body { font-family: 'Segoe UI', sans-serif; margin: 0; padding: 20px; background: linear-gradient(135deg, #74b9ff 0%, #0984e3 100%); min-height: 100vh; color: white; }
        .container { max-width: 600px; margin: 50px auto; }
        .weather-card { background: rgba(255,255,255,0.1); padding: 30px; border-radius: 20px; backdrop-filter: blur(15px); box-shadow: 0 8px 32px rgba(0,0,0,0.3); margin: 20px 0; }
        .search-box { display: flex; gap: 10px; margin-bottom: 30px; }
        input[type="text"] { flex: 1; padding: 15px; border: none; border-radius: 10px; font-size: 16px; }
        button { padding: 15px 25px; border: none; border-radius: 10px; background: #00b894; color: white; cursor: pointer; font-weight: bold; }
        .current-weather { text-align: center; }
        .temperature { font-size: 4rem; font-weight: bold; margin: 20px 0; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🌤️ Perfect Weather Dashboard</h1>
        <div class="weather-card">
            <div class="search-box">
                <input type="text" id="cityInput" placeholder="Enter city name..." value="New York">
                <button onclick="getWeather()">Get Weather</button>
            </div>
            <div class="current-weather">
                <h2 id="cityName">New York</h2>
                <div class="temperature" id="temperature">22°C</div>
                <p id="description">Partly Cloudy</p>
            </div>
        </div>
    </div>
    <script>
        function getWeather() {
            const city = document.getElementById('cityInput').value;
            const weatherData = {
                'new york': { temp: '22°C', desc: 'Partly Cloudy' },
                'london': { temp: '18°C', desc: 'Rainy' },
                'tokyo': { temp: '28°C', desc: 'Sunny' },
                'paris': { temp: '20°C', desc: 'Overcast' }
            };
            const data = weatherData[city.toLowerCase()] || weatherData['new york'];
            document.getElementById('cityName').textContent = city;
            document.getElementById('temperature').textContent = data.temp;
            document.getElementById('description').textContent = data.desc;
        }
        document.getElementById('cityInput').addEventListener('keypress', function(e) {
            if (e.key === 'Enter') getWeather();
        });
        console.log('Perfect Weather Dashboard - ID: ${deploymentId}');
    </script>
</body>
</html>`;
}

// Health check endpoint
router.get('/health', async (request, env) => {
  return new Response(JSON.stringify({
    status: 'healthy',
    timestamp: Date.now(),
    version: '3.0.0-huggingface-hybrid',
    ai_model: 'openai/gpt-oss-120b',
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

    const deploymentId = `deploy_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;

    // Step 1: Try HuggingFace AI first
    let generatedCode = null;
    let generationMethod = 'template';

    try {
      console.log('Attempting AI generation with HuggingFace GPT-OSS-120B...');

      const optimizedPrompt = `Create a complete, functional web application for: "${prompt}"

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

Return ONLY the HTML code, nothing else.`;

      const hfResponse = await fetch('https://api-inference.huggingface.co/models/openai/gpt-oss-120b', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          inputs: `System: You are an expert web developer. Generate ONLY clean HTML with inline CSS and JavaScript. No markdown, no explanations, just pure HTML starting with <!DOCTYPE html>.

User: ${optimizedPrompt}`,
          parameters: {
            max_new_tokens: 2000,
            temperature: 0.7,
            top_p: 0.9,
            repetition_penalty: 1.1
          }
        })
      });

      if (hfResponse.ok) {
        const hfData = await hfResponse.json();
        let aiCode = '';

        if (Array.isArray(hfData) && hfData[0] && hfData[0].generated_text) {
          aiCode = hfData[0].generated_text;
        } else if (hfData.generated_text) {
          aiCode = hfData.generated_text;
        } else {
          throw new Error('Invalid HuggingFace response format');
        }

        // Clean the AI response
        aiCode = aiCode.replace(/```html/g, '').replace(/```/g, '').trim();

        // Extract HTML if it starts with system prompt echo
        const htmlMatch = aiCode.match(/<!DOCTYPE html[\s\S]*?<\/html>/i);
        if (htmlMatch) {
          aiCode = htmlMatch[0];
        }

        // Validate that we have proper HTML
        if (aiCode.includes('<!DOCTYPE html') && aiCode.includes('</html>')) {
          generatedCode = aiCode;
          generationMethod = 'huggingface-ai';
          console.log('✅ HuggingFace AI generation successful');
        } else {
          throw new Error('Generated code is not valid HTML');
        }
      } else {
        throw new Error(`HuggingFace API error: ${hfResponse.status}`);
      }

    } catch (aiError) {
      console.log('❌ HuggingFace AI generation failed:', aiError.message);
      console.log('🔄 Falling back to perfect template...');
    }

    // Step 2: Fallback to perfect template if AI fails
    if (!generatedCode) {
      generatedCode = generateFallbackTemplate(prompt, deploymentId);
      generationMethod = 'template-fallback';
      console.log('✅ Template fallback used');
    }

    // Store deployment with metadata
    const deploymentData = {
      id: deploymentId,
      prompt: prompt,
      code: generatedCode,
      timestamp: Date.now(),
      method: generationMethod,
      status: 'deployed'
    };

    // Store in KV
    await env.METADATA.put(`deployment:${deploymentId}`, JSON.stringify(deploymentData));

    // Return deployment info
    return new Response(JSON.stringify({
      success: true,
      deploymentId: deploymentId,
      url: `https://yatrogenesis.github.io/Ectus-R/apps/${deploymentId}.html`,
      method: generationMethod,
      prompt: prompt,
      timestamp: Date.now()
    }), {
      headers: { 'Content-Type': 'application/json' },
      status: 200
    });

  } catch (error) {
    console.error('Magic Loop error:', error);
    return new Response(JSON.stringify({
      error: 'Generation failed',
      message: error.message,
      fallback_available: true
    }), {
      headers: { 'Content-Type': 'application/json' },
      status: 500
    });
  }
});

// Get deployment endpoint
router.get('/api/v1/deployments/:id', async (request, env) => {
  try {
    const { id } = request.params;
    const deploymentData = await env.METADATA.get(`deployment:${id}`);

    if (!deploymentData) {
      return new Response(JSON.stringify({
        error: 'Deployment not found',
        id: id
      }), {
        headers: { 'Content-Type': 'application/json' },
        status: 404
      });
    }

    const deployment = JSON.parse(deploymentData);
    return new Response(deployment.code, {
      headers: {
        'Content-Type': 'text/html',
        'Cache-Control': 'public, max-age=3600'
      },
      status: 200
    });

  } catch (error) {
    return new Response(JSON.stringify({
      error: 'Failed to retrieve deployment',
      message: error.message
    }), {
      headers: { 'Content-Type': 'application/json' },
      status: 500
    });
  }
});

// CORS handler
function addCorsHeaders(response) {
  const corsHeaders = {
    'Access-Control-Allow-Origin': '*',
    'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS',
    'Access-Control-Allow-Headers': 'Content-Type, Authorization',
    'Access-Control-Max-Age': '86400'
  };

  // Create new response with CORS headers
  const newResponse = new Response(response.body, {
    status: response.status,
    statusText: response.statusText,
    headers: {
      ...Object.fromEntries(response.headers),
      ...corsHeaders
    }
  });

  return newResponse;
}

// OPTIONS handler for CORS preflight
router.options('*', () => {
  return new Response(null, {
    status: 204,
    headers: {
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS',
      'Access-Control-Allow-Headers': 'Content-Type, Authorization',
      'Access-Control-Max-Age': '86400'
    }
  });
});

// Main export
export default {
  async fetch(request, env, ctx) {
    try {
      const response = await router.handle(request, env, ctx);
      return addCorsHeaders(response);
    } catch (error) {
      console.error('Worker error:', error);
      const errorResponse = new Response(JSON.stringify({
        error: 'Internal server error',
        message: error.message,
        timestamp: Date.now()
      }), {
        headers: { 'Content-Type': 'application/json' },
        status: 500
      });
      return addCorsHeaders(errorResponse);
    }
  }
};