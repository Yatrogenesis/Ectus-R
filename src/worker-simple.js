// Ectus-R SaaS API Worker with Domain Proxy
import { Router } from 'itty-router';

const router = Router();

// Generate intelligent fallback templates based on prompt
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

// Complete calculator template
function generateCalculatorTemplate(deploymentId) {
  return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Calculator App</title>
    <style>
        body { font-family: 'Segoe UI', sans-serif; margin: 0; padding: 20px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; }
        .calculator { max-width: 300px; margin: 50px auto; background: rgba(255,255,255,0.1); padding: 20px; border-radius: 20px; backdrop-filter: blur(15px); box-shadow: 0 8px 32px rgba(0,0,0,0.3); }
        .display { width: 100%; height: 60px; font-size: 24px; text-align: right; padding: 0 15px; border: none; border-radius: 10px; margin-bottom: 15px; background: rgba(255,255,255,0.9); }
        .buttons { display: grid; grid-template-columns: repeat(4, 1fr); gap: 10px; }
        button { height: 60px; border: none; border-radius: 10px; font-size: 18px; font-weight: bold; cursor: pointer; transition: all 0.2s; }
        .number, .operator { background: rgba(255,255,255,0.8); color: #333; }
        .number:hover, .operator:hover { background: rgba(255,255,255,0.9); transform: scale(1.05); }
        .equals { background: #ff6b6b; color: white; }
        .equals:hover { background: #ff5252; transform: scale(1.05); }
        .clear { background: #ffa726; color: white; }
        .clear:hover { background: #ff9800; transform: scale(1.05); }
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
            <button class="equals" onclick="calculate()" rowspan="2">=</button>

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
                display.value = eval(display.value);
            } catch (e) {
                display.value = 'Error';
                setTimeout(clearDisplay, 1500);
            }
        }

        console.log('Calculator loaded - Deployment ID: ${deploymentId}');
    </script>
</body>
</html>`;
}

// Todo app template
function generateTodoTemplate(deploymentId) {
  return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Todo List Manager</title>
    <style>
        body { font-family: 'Segoe UI', sans-serif; margin: 0; padding: 20px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; }
        .container { max-width: 500px; margin: 50px auto; background: rgba(255,255,255,0.1); padding: 30px; border-radius: 20px; backdrop-filter: blur(15px); box-shadow: 0 8px 32px rgba(0,0,0,0.3); }
        h1 { text-align: center; color: white; margin-bottom: 30px; }
        .input-container { display: flex; gap: 10px; margin-bottom: 20px; }
        input[type="text"] { flex: 1; padding: 12px; border: none; border-radius: 10px; font-size: 16px; }
        button { padding: 12px 20px; border: none; border-radius: 10px; cursor: pointer; font-weight: bold; transition: all 0.2s; }
        .add-btn { background: #4CAF50; color: white; }
        .add-btn:hover { background: #45a049; transform: scale(1.05); }
        .todo-list { list-style: none; padding: 0; }
        .todo-item { background: rgba(255,255,255,0.9); margin: 10px 0; padding: 15px; border-radius: 10px; display: flex; justify-content: space-between; align-items: center; }
        .todo-item.completed { opacity: 0.6; text-decoration: line-through; }
        .todo-actions button { margin-left: 5px; padding: 5px 10px; font-size: 12px; }
        .complete-btn { background: #2196F3; color: white; }
        .delete-btn { background: #f44336; color: white; }
    </style>
</head>
<body>
    <div class="container">
        <h1>📝 Todo List Manager</h1>
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

        console.log('Todo app loaded - Deployment ID: ${deploymentId}');
    </script>
</body>
</html>`;
}

// Timer template
function generateTimerTemplate(deploymentId) {
  return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Timer App</title>
    <style>
        body { font-family: 'Segoe UI', sans-serif; margin: 0; padding: 20px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; color: white; }
        .container { max-width: 400px; margin: 50px auto; background: rgba(255,255,255,0.1); padding: 40px; border-radius: 20px; backdrop-filter: blur(15px); box-shadow: 0 8px 32px rgba(0,0,0,0.3); text-align: center; }
        .timer-display { font-size: 4rem; font-weight: bold; margin: 30px 0; color: #fff; text-shadow: 0 0 20px rgba(255,255,255,0.5); }
        .controls { display: flex; gap: 15px; justify-content: center; margin: 30px 0; }
        button { padding: 15px 25px; border: none; border-radius: 10px; font-size: 16px; font-weight: bold; cursor: pointer; transition: all 0.2s; }
        .start { background: #4CAF50; color: white; }
        .start:hover { background: #45a049; transform: scale(1.05); }
        .stop { background: #f44336; color: white; }
        .stop:hover { background: #da190b; transform: scale(1.05); }
        .reset { background: #ffa726; color: white; }
        .reset:hover { background: #ff9800; transform: scale(1.05); }
        .input-section { margin: 20px 0; }
        input[type="number"] { padding: 10px; border: none; border-radius: 5px; width: 60px; text-align: center; margin: 0 5px; }
    </style>
</head>
<body>
    <div class="container">
        <h1>⏱️ Timer App</h1>
        <div class="timer-display" id="display">00:00:00</div>

        <div class="input-section">
            <label>Set Timer: </label>
            <input type="number" id="hours" placeholder="HH" min="0" max="23" value="0">
            <input type="number" id="minutes" placeholder="MM" min="0" max="59" value="5">
            <input type="number" id="seconds" placeholder="SS" min="0" max="59" value="0">
        </div>

        <div class="controls">
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
            let hours = Math.floor(timeLeft / 3600);
            let minutes = Math.floor((timeLeft % 3600) / 60);
            let seconds = timeLeft % 60;

            display.textContent =
                String(hours).padStart(2, '0') + ':' +
                String(minutes).padStart(2, '0') + ':' +
                String(seconds).padStart(2, '0');
        }

        function startTimer() {
            if (timeLeft === 0) {
                let hours = parseInt(document.getElementById('hours').value) || 0;
                let minutes = parseInt(document.getElementById('minutes').value) || 0;
                let seconds = parseInt(document.getElementById('seconds').value) || 0;
                timeLeft = hours * 3600 + minutes * 60 + seconds;
            }

            if (timeLeft > 0 && !timer) {
                timer = setInterval(() => {
                    timeLeft--;
                    updateDisplay();

                    if (timeLeft === 0) {
                        clearInterval(timer);
                        timer = null;
                        alert('⏰ Time\'s up!');
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
        console.log('Timer app loaded - Deployment ID: ${deploymentId}');
    </script>
</body>
</html>`;
}

// Color picker template
function generateColorPickerTemplate(deploymentId) {
  return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Color Picker Tool</title>
    <style>
        body { font-family: 'Segoe UI', sans-serif; margin: 0; padding: 20px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; color: white; }
        .container { max-width: 500px; margin: 50px auto; background: rgba(255,255,255,0.1); padding: 30px; border-radius: 20px; backdrop-filter: blur(15px); box-shadow: 0 8px 32px rgba(0,0,0,0.3); }
        h1 { text-align: center; margin-bottom: 30px; }
        .color-display { width: 100%; height: 200px; border-radius: 10px; margin: 20px 0; border: 3px solid rgba(255,255,255,0.3); }
        .color-inputs { display: grid; grid-template-columns: 1fr 1fr; gap: 20px; margin: 20px 0; }
        .input-group { background: rgba(255,255,255,0.1); padding: 15px; border-radius: 10px; }
        label { display: block; margin-bottom: 5px; font-weight: bold; }
        input[type="range"] { width: 100%; margin: 10px 0; }
        input[type="color"] { width: 100%; height: 50px; border: none; border-radius: 5px; cursor: pointer; }
        .value-display { background: rgba(255,255,255,0.9); color: #333; padding: 10px; border-radius: 5px; margin: 10px 0; font-family: monospace; }
        .copy-btn { background: #4CAF50; color: white; border: none; padding: 8px 15px; border-radius: 5px; cursor: pointer; font-size: 12px; }
        .copy-btn:hover { background: #45a049; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🎨 Color Picker Tool</h1>

        <div class="color-display" id="colorDisplay"></div>

        <div class="color-inputs">
            <div class="input-group">
                <label>Color Picker:</label>
                <input type="color" id="colorPicker" onchange="updateFromPicker()">
            </div>

            <div class="input-group">
                <label>RGB Values:</label>
                <label>Red: <span id="redValue">128</span></label>
                <input type="range" id="redSlider" min="0" max="255" value="128" oninput="updateFromRGB()">

                <label>Green: <span id="greenValue">128</span></label>
                <input type="range" id="greenSlider" min="0" max="255" value="128" oninput="updateFromRGB()">

                <label>Blue: <span id="blueValue">128</span></label>
                <input type="range" id="blueSlider" min="0" max="255" value="128" oninput="updateFromRGB()">
            </div>
        </div>

        <div class="input-group">
            <label>RGB Value:</label>
            <div class="value-display" id="rgbDisplay">rgb(128, 128, 128)</div>
            <button class="copy-btn" onclick="copyToClipboard('rgb')">Copy RGB</button>
        </div>

        <div class="input-group">
            <label>HEX Value:</label>
            <div class="value-display" id="hexDisplay">#808080</div>
            <button class="copy-btn" onclick="copyToClipboard('hex')">Copy HEX</button>
        </div>
    </div>
    <script>
        let r = 128, g = 128, b = 128;

        function updateDisplay() {
            const rgbValue = \`rgb(\${r}, \${g}, \${b})\`;
            const hexValue = '#' + [r, g, b].map(x => x.toString(16).padStart(2, '0')).join('');

            document.getElementById('colorDisplay').style.backgroundColor = rgbValue;
            document.getElementById('rgbDisplay').textContent = rgbValue;
            document.getElementById('hexDisplay').textContent = hexValue.toUpperCase();

            document.getElementById('redValue').textContent = r;
            document.getElementById('greenValue').textContent = g;
            document.getElementById('blueValue').textContent = b;

            document.getElementById('colorPicker').value = hexValue;
        }

        function updateFromRGB() {
            r = parseInt(document.getElementById('redSlider').value);
            g = parseInt(document.getElementById('greenSlider').value);
            b = parseInt(document.getElementById('blueSlider').value);
            updateDisplay();
        }

        function updateFromPicker() {
            const hex = document.getElementById('colorPicker').value;
            r = parseInt(hex.substr(1, 2), 16);
            g = parseInt(hex.substr(3, 2), 16);
            b = parseInt(hex.substr(5, 2), 16);

            document.getElementById('redSlider').value = r;
            document.getElementById('greenSlider').value = g;
            document.getElementById('blueSlider').value = b;

            updateDisplay();
        }

        function copyToClipboard(type) {
            const text = type === 'rgb'
                ? document.getElementById('rgbDisplay').textContent
                : document.getElementById('hexDisplay').textContent;

            navigator.clipboard.writeText(text).then(() => {
                alert(\`\${type.toUpperCase()} value copied to clipboard!\`);
            });
        }

        updateDisplay();
        console.log('Color picker loaded - Deployment ID: ${deploymentId}');
    </script>
</body>
</html>`;
}

// Weather template
function generateWeatherTemplate(deploymentId) {
  return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Weather Dashboard</title>
    <style>
        body { font-family: 'Segoe UI', sans-serif; margin: 0; padding: 20px; background: linear-gradient(135deg, #74b9ff 0%, #0984e3 100%); min-height: 100vh; color: white; }
        .container { max-width: 600px; margin: 50px auto; }
        .weather-card { background: rgba(255,255,255,0.1); padding: 30px; border-radius: 20px; backdrop-filter: blur(15px); box-shadow: 0 8px 32px rgba(0,0,0,0.3); margin: 20px 0; }
        h1 { text-align: center; margin-bottom: 30px; }
        .search-box { display: flex; gap: 10px; margin-bottom: 30px; }
        input[type="text"] { flex: 1; padding: 15px; border: none; border-radius: 10px; font-size: 16px; }
        button { padding: 15px 25px; border: none; border-radius: 10px; background: #00b894; color: white; cursor: pointer; font-weight: bold; }
        button:hover { background: #00a085; }
        .current-weather { text-align: center; }
        .temperature { font-size: 4rem; font-weight: bold; margin: 20px 0; }
        .weather-info { display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 20px; margin: 30px 0; }
        .info-card { background: rgba(255,255,255,0.1); padding: 20px; border-radius: 10px; text-align: center; }
        .forecast { display: grid; grid-template-columns: repeat(auto-fit, minmax(120px, 1fr)); gap: 15px; }
        .forecast-item { background: rgba(255,255,255,0.1); padding: 15px; border-radius: 10px; text-align: center; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🌤️ Weather Dashboard</h1>

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

            <div class="weather-info">
                <div class="info-card">
                    <h3>💨 Wind</h3>
                    <p id="windSpeed">15 km/h</p>
                </div>
                <div class="info-card">
                    <h3>💧 Humidity</h3>
                    <p id="humidity">65%</p>
                </div>
                <div class="info-card">
                    <h3>👁️ Visibility</h3>
                    <p id="visibility">10 km</p>
                </div>
                <div class="info-card">
                    <h3>🌡️ Feels Like</h3>
                    <p id="feelsLike">25°C</p>
                </div>
            </div>
        </div>

        <div class="weather-card">
            <h3>5-Day Forecast</h3>
            <div class="forecast" id="forecast">
                <div class="forecast-item">
                    <p><strong>Mon</strong></p>
                    <p>☀️</p>
                    <p>24°C</p>
                </div>
                <div class="forecast-item">
                    <p><strong>Tue</strong></p>
                    <p>⛅</p>
                    <p>22°C</p>
                </div>
                <div class="forecast-item">
                    <p><strong>Wed</strong></p>
                    <p>🌧️</p>
                    <p>18°C</p>
                </div>
                <div class="forecast-item">
                    <p><strong>Thu</strong></p>
                    <p>⛈️</p>
                    <p>16°C</p>
                </div>
                <div class="forecast-item">
                    <p><strong>Fri</strong></p>
                    <p>☀️</p>
                    <p>26°C</p>
                </div>
            </div>
        </div>
    </div>
    <script>
        function getWeather() {
            const city = document.getElementById('cityInput').value;

            // Simulate weather data (in a real app, you'd call a weather API)
            const weatherData = {
                'new york': { temp: '22°C', desc: 'Partly Cloudy', wind: '15 km/h', humidity: '65%' },
                'london': { temp: '18°C', desc: 'Rainy', wind: '12 km/h', humidity: '80%' },
                'tokyo': { temp: '28°C', desc: 'Sunny', wind: '8 km/h', humidity: '45%' },
                'paris': { temp: '20°C', desc: 'Overcast', wind: '10 km/h', humidity: '70%' }
            };

            const data = weatherData[city.toLowerCase()] || weatherData['new york'];

            document.getElementById('cityName').textContent = city;
            document.getElementById('temperature').textContent = data.temp;
            document.getElementById('description').textContent = data.desc;
            document.getElementById('windSpeed').textContent = data.wind;
            document.getElementById('humidity').textContent = data.humidity;
            document.getElementById('visibility').textContent = '10 km';
            document.getElementById('feelsLike').textContent = (parseInt(data.temp) + 3) + '°C';
        }

        document.getElementById('cityInput').addEventListener('keypress', function(e) {
            if (e.key === 'Enter') getWeather();
        });

        console.log('Weather dashboard loaded - Deployment ID: ${deploymentId}');
    </script>
</body>
</html>`;
}

// Generic template for other apps
function generateGenericTemplate(prompt, deploymentId) {
  return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Generated by Ectus-R AI</title>
    <style>
        body { font-family: 'Segoe UI', sans-serif; margin: 0; padding: 40px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; color: white; }
        .container { max-width: 800px; margin: 0 auto; text-align: center; }
        .card { background: rgba(255,255,255,0.1); padding: 40px; border-radius: 20px; backdrop-filter: blur(15px); box-shadow: 0 8px 32px rgba(0,0,0,0.3); }
        h1 { margin-bottom: 20px; }
        .prompt { background: rgba(255,255,255,0.1); padding: 20px; border-radius: 10px; margin: 20px 0; }
        .btn { background: #ff6b6b; color: white; padding: 15px 30px; border: none; border-radius: 10px; cursor: pointer; font-size: 16px; font-weight: bold; margin: 10px; transition: all 0.2s; }
        .btn:hover { background: #ff5252; transform: scale(1.05); }
        .features { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin-top: 30px; }
        .feature { background: rgba(255,255,255,0.1); padding: 20px; border-radius: 10px; }
    </style>
</head>
<body>
    <div class="container">
        <div class="card">
            <h1>🚀 Generated by Ectus-R AI</h1>
            <div class="prompt">
                <strong>Your Request:</strong><br>
                "${prompt}"
            </div>
            <p>This application was generated using AI-powered Magic Loop deployment.</p>
            <button class="btn" onclick="showAlert()">Test Interaction</button>
            <button class="btn" onclick="changeTheme()">Change Theme</button>

            <div class="features">
                <div class="feature">
                    <h3>⚡ Fast</h3>
                    <p>Built with modern web standards</p>
                </div>
                <div class="feature">
                    <h3>🎨 Beautiful</h3>
                    <p>Professional design and animations</p>
                </div>
                <div class="feature">
                    <h3>📱 Responsive</h3>
                    <p>Works on all devices</p>
                </div>
            </div>
        </div>
    </div>
    <script>
        function showAlert() {
            alert('🎉 Application is working! This is a functional template.');
        }

        function changeTheme() {
            document.body.style.background = document.body.style.background.includes('667eea')
                ? 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)'
                : 'linear-gradient(135deg, #f093fb 0%, #f5576c 100%)';
        }

        console.log('Ectus-R app loaded - Deployment ID: ${deploymentId}');
        console.log('Generated for prompt: "${prompt}"');
    </script>
</body>
</html>`;
}

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

    // Generate code using Cloudflare AI with improved prompt
    const codeGenPrompt = `Create a fully functional web application for: "${prompt}"

REQUIREMENTS:
- Complete HTML document with DOCTYPE, head, and body
- Inline CSS with modern, responsive design
- Full JavaScript functionality for all features
- Professional styling with gradients, shadows, and animations
- All buttons and inputs must be functional
- Include proper event handlers
- Add visual feedback for user interactions
- Make it production-ready and visually appealing

For calculators: Include ALL basic operations (+, -, *, /) and number buttons (0-9)
For todo apps: Include add, delete, edit, and mark complete functionality
For timers: Include start, stop, reset, and display functionality
For dashboards: Include multiple sections and interactive elements

Return ONLY the HTML code without markdown or explanations.`;

    try {
      // Try with a better model first
      const aiResponse = await env.AI.run('@cf/meta/llama-3.1-8b-instruct', {
        messages: [
          { role: 'system', content: 'You are a senior full-stack developer specializing in creating complete, functional web applications. Generate only clean HTML with inline CSS and JavaScript. No markdown, no explanations, just pure working code.' },
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
      console.log('Llama 3.1 failed, trying Llama 2.7b fallback:', aiError.message);

      try {
        // Fallback to previous model
        const fallbackResponse = await env.AI.run('@cf/meta/llama-2-7b-chat-int8', {
          messages: [
            { role: 'system', content: 'You are a senior full-stack developer. Generate only clean HTML with inline CSS and JavaScript. No markdown, no explanations, just pure working code.' },
            { role: 'user', content: codeGenPrompt }
          ]
        });

        let generatedCode = fallbackResponse.response || generateFallbackTemplate(prompt, deploymentId);

        // Extract HTML from markdown if present
        if (generatedCode.includes('```')) {
          const htmlMatch = generatedCode.match(/```(?:html)?\s*([\s\S]*?)```/);
          if (htmlMatch && htmlMatch[1]) {
            generatedCode = htmlMatch[1].trim();
          }
        }

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
          message: 'Magic Loop deployment completed (fallback model)',
          preview_url: `https://ectus-r-saas.pako-molina.workers.dev/api/v1/deployments/${deploymentId}/preview`
        }), {
          headers: { 'Content-Type': 'application/json' },
          status: 200
        });

      } catch (fallbackError) {
        console.log('Both AI models failed, using template:', fallbackError.message);

        // Generate smart template based on prompt
        const fallbackCode = generateFallbackTemplate(prompt, deploymentId);

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

// Handle OPTIONS preflight requests (MUST be before catch-all)
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