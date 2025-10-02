// Ectus-R Multi-AI Provider System
// Soporta: Cloudflare AI, OpenAI (HuggingFace), DeepSeek, Ollama

export class MultiAIProvider {
  constructor(env) {
    this.env = env;
    this.providers = {
      cloudflare: {
        name: 'Cloudflare AI Workers',
        models: [
          '@cf/meta/llama-3.1-8b-instruct',
          '@cf/meta/llama-3.1-70b-instruct',
          '@cf/mistral/mistral-7b-instruct-v0.1',
          '@cf/qwen/qwen1.5-14b-chat-awq',
          '@cf/deepseek-ai/deepseek-math-7b-instruct'
        ],
        priority: 1,
        free: true
      },
      huggingface: {
        name: 'HuggingFace Inference API',
        models: [
          'meta-llama/Meta-Llama-3-8B-Instruct',
          'mistralai/Mistral-7B-Instruct-v0.2',
          'microsoft/phi-2',
          'tiiuae/falcon-7b-instruct',
          'bigcode/starcoder'
        ],
        priority: 2,
        free: true
      },
      deepseek: {
        name: 'DeepSeek API',
        models: [
          'deepseek-chat',
          'deepseek-coder'
        ],
        priority: 3,
        free: false
      },
      ollama: {
        name: 'Ollama Local',
        models: [
          'llama3.1:8b',
          'codellama:7b',
          'mistral:7b',
          'deepseek-coder:6.7b',
          'phi3:mini'
        ],
        priority: 4,
        free: true
      },
      openai: {
        name: 'OpenAI API',
        models: [
          'gpt-3.5-turbo',
          'gpt-4',
          'gpt-4-turbo-preview'
        ],
        priority: 5,
        free: false
      }
    };
  }

  // Cloudflare AI Worker - GRATIS
  async generateWithCloudflare(prompt, model = '@cf/meta/llama-3.1-8b-instruct') {
    try {
      console.log(`🟢 [Cloudflare AI] Usando modelo: ${model}`);

      const response = await this.env.AI.run(model, {
        messages: [
          {
            role: 'system',
            content: 'You are an expert web developer. Generate only clean HTML with inline CSS and JavaScript. Never use markdown or explanations.'
          },
          { role: 'user', content: prompt }
        ],
        max_tokens: 3000,
        temperature: 0.3,
        top_p: 0.9
      });

      const code = this.cleanAIResponse(response.response || '');
      if (this.validateHTML(code)) {
        return { success: true, code, provider: 'cloudflare', model };
      }
      throw new Error('Invalid HTML output');
    } catch (error) {
      console.error(`❌ [Cloudflare AI] Error: ${error.message}`);
      return { success: false, error: error.message };
    }
  }

  // HuggingFace Inference API - GRATIS (rate limited)
  async generateWithHuggingFace(prompt, model = 'meta-llama/Meta-Llama-3-8B-Instruct') {
    try {
      console.log(`🟡 [HuggingFace] Usando modelo: ${model}`);

      const HF_API_KEY = this.env.HUGGINGFACE_API_KEY;
      if (!HF_API_KEY) {
        throw new Error('HuggingFace API key not configured');
      }

      const response = await fetch(
        `https://api-inference.huggingface.co/models/${model}`,
        {
          method: 'POST',
          headers: {
            'Authorization': `Bearer ${HF_API_KEY}`,
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({
            inputs: prompt,
            parameters: {
              max_new_tokens: 3000,
              temperature: 0.3,
              top_p: 0.9,
              return_full_text: false
            }
          })
        }
      );

      if (!response.ok) {
        throw new Error(`HF API returned ${response.status}`);
      }

      const data = await response.json();
      const code = this.cleanAIResponse(data[0]?.generated_text || '');

      if (this.validateHTML(code)) {
        return { success: true, code, provider: 'huggingface', model };
      }
      throw new Error('Invalid HTML output');
    } catch (error) {
      console.error(`❌ [HuggingFace] Error: ${error.message}`);
      return { success: false, error: error.message };
    }
  }

  // DeepSeek API - Requiere API Key
  async generateWithDeepSeek(prompt, model = 'deepseek-chat') {
    try {
      console.log(`🔵 [DeepSeek] Usando modelo: ${model}`);

      const DEEPSEEK_API_KEY = this.env.DEEPSEEK_API_KEY;
      if (!DEEPSEEK_API_KEY) {
        throw new Error('DeepSeek API key not configured');
      }

      const response = await fetch('https://api.deepseek.com/v1/chat/completions', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${DEEPSEEK_API_KEY}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          model: model,
          messages: [
            {
              role: 'system',
              content: 'You are an expert web developer. Generate only clean HTML with inline CSS and JavaScript.'
            },
            { role: 'user', content: prompt }
          ],
          max_tokens: 3000,
          temperature: 0.3
        })
      });

      if (!response.ok) {
        throw new Error(`DeepSeek API returned ${response.status}`);
      }

      const data = await response.json();
      const code = this.cleanAIResponse(data.choices[0]?.message?.content || '');

      if (this.validateHTML(code)) {
        return { success: true, code, provider: 'deepseek', model };
      }
      throw new Error('Invalid HTML output');
    } catch (error) {
      console.error(`❌ [DeepSeek] Error: ${error.message}`);
      return { success: false, error: error.message };
    }
  }

  // Ollama Local - GRATIS (requiere servidor local)
  async generateWithOllama(prompt, model = 'llama3.1:8b') {
    try {
      console.log(`🟣 [Ollama] Usando modelo: ${model}`);

      const OLLAMA_URL = this.env.OLLAMA_URL || 'http://localhost:11434';

      const response = await fetch(`${OLLAMA_URL}/api/generate`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          model: model,
          prompt: prompt,
          stream: false,
          options: {
            temperature: 0.3,
            num_predict: 3000
          }
        })
      });

      if (!response.ok) {
        throw new Error(`Ollama API returned ${response.status}`);
      }

      const data = await response.json();
      const code = this.cleanAIResponse(data.response || '');

      if (this.validateHTML(code)) {
        return { success: true, code, provider: 'ollama', model };
      }
      throw new Error('Invalid HTML output');
    } catch (error) {
      console.error(`❌ [Ollama] Error: ${error.message}`);
      return { success: false, error: error.message };
    }
  }

  // OpenAI API - Requiere API Key y tiene costos
  async generateWithOpenAI(prompt, model = 'gpt-3.5-turbo') {
    try {
      console.log(`🟠 [OpenAI] Usando modelo: ${model}`);

      const OPENAI_API_KEY = this.env.OPENAI_API_KEY;
      if (!OPENAI_API_KEY) {
        throw new Error('OpenAI API key not configured');
      }

      const response = await fetch('https://api.openai.com/v1/chat/completions', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${OPENAI_API_KEY}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          model: model,
          messages: [
            {
              role: 'system',
              content: 'You are an expert web developer. Generate only clean HTML with inline CSS and JavaScript.'
            },
            { role: 'user', content: prompt }
          ],
          max_tokens: 3000,
          temperature: 0.3
        })
      });

      if (!response.ok) {
        throw new Error(`OpenAI API returned ${response.status}`);
      }

      const data = await response.json();
      const code = this.cleanAIResponse(data.choices[0]?.message?.content || '');

      if (this.validateHTML(code)) {
        return { success: true, code, provider: 'openai', model };
      }
      throw new Error('Invalid HTML output');
    } catch (error) {
      console.error(`❌ [OpenAI] Error: ${error.message}`);
      return { success: false, error: error.message };
    }
  }

  // Sistema de Fallback Inteligente
  async generateWithFallback(prompt, preferredProvider = 'cloudflare') {
    console.log(`🚀 [Multi-AI] Iniciando generación con fallback`);
    console.log(`📋 Prompt: ${prompt.substring(0, 100)}...`);

    // Orden de fallback basado en prioridad y disponibilidad
    const fallbackOrder = [
      // 1. Cloudflare AI (siempre gratis, siempre disponible)
      { fn: () => this.generateWithCloudflare(prompt), name: 'cloudflare' },

      // 2. HuggingFace (gratis pero con rate limits)
      { fn: () => this.generateWithHuggingFace(prompt), name: 'huggingface' },

      // 3. Ollama (gratis si está configurado localmente)
      { fn: () => this.generateWithOllama(prompt), name: 'ollama' },

      // 4. DeepSeek (requiere API key)
      { fn: () => this.generateWithDeepSeek(prompt), name: 'deepseek' },

      // 5. OpenAI (último recurso, tiene costos)
      { fn: () => this.generateWithOpenAI(prompt), name: 'openai' }
    ];

    // Intentar con el proveedor preferido primero
    const preferredIndex = fallbackOrder.findIndex(p => p.name === preferredProvider);
    if (preferredIndex > 0) {
      const preferred = fallbackOrder.splice(preferredIndex, 1)[0];
      fallbackOrder.unshift(preferred);
    }

    // Intentar cada proveedor hasta que uno funcione
    for (const provider of fallbackOrder) {
      console.log(`🔄 Intentando con ${provider.name}...`);
      const result = await provider.fn();

      if (result.success) {
        console.log(`✅ Generación exitosa con ${result.provider}`);
        return result;
      }
    }

    // Si todos fallan, retornar error
    console.error(`❌ Todos los proveedores fallaron`);
    return {
      success: false,
      error: 'All AI providers failed',
      fallbackOrder: fallbackOrder.map(p => p.name)
    };
  }

  // Utilidades
  cleanAIResponse(text) {
    // Remover markdown code blocks
    text = text.replace(/```html/gi, '').replace(/```/g, '').trim();

    // Extraer HTML si está presente
    const htmlMatch = text.match(/<!DOCTYPE html[\s\S]*?<\/html>/i);
    if (htmlMatch) {
      return htmlMatch[0];
    }

    return text;
  }

  validateHTML(code) {
    return code &&
           code.length > 500 &&
           code.includes('<!DOCTYPE html') &&
           code.includes('</html>') &&
           code.includes('<body>') &&
           code.includes('</body>');
  }

  // Obtener información de proveedores disponibles
  getAvailableProviders() {
    return {
      cloudflare: {
        ...this.providers.cloudflare,
        available: !!this.env.AI,
        configured: true
      },
      huggingface: {
        ...this.providers.huggingface,
        available: !!this.env.HUGGINGFACE_API_KEY,
        configured: !!this.env.HUGGINGFACE_API_KEY
      },
      deepseek: {
        ...this.providers.deepseek,
        available: !!this.env.DEEPSEEK_API_KEY,
        configured: !!this.env.DEEPSEEK_API_KEY
      },
      ollama: {
        ...this.providers.ollama,
        available: !!this.env.OLLAMA_URL,
        configured: !!this.env.OLLAMA_URL
      },
      openai: {
        ...this.providers.openai,
        available: !!this.env.OPENAI_API_KEY,
        configured: !!this.env.OPENAI_API_KEY
      }
    };
  }
}