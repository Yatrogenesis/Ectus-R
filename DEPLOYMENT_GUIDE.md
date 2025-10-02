# Ectus-R SaaS - Guía de Despliegue Completa

## 🚀 Infraestructura

```
Usuario → Cloudflare SSL → creator.avermex.com → Cloudflare Workers → GitHub Pages
                                                  ↓
                                           Multi-AI System:
                                           • Cloudflare AI (FREE)
                                           • HuggingFace (FREE)
                                           • DeepSeek (API)
                                           • Ollama (Local)
                                           • OpenAI (API)
```

## ✨ Características

### Multi-AI Provider System
- **5 proveedores AI** con fallback automático
- **Sistema inteligente** que elige el mejor proveedor disponible
- **Templates perfectos** como último fallback
- **Alta disponibilidad** - si uno falla, usa el siguiente

### Integración Completa
- ✅ Cloudflare Workers (Edge Computing)
- ✅ Cloudflare AI (Modelos gratis)
- ✅ KV Storage (Sessions, Cache, Metadata)
- ✅ D1 Database (SQL persistente)
- ✅ GitHub Pages (Static hosting)
- ✅ GoDaddy DNS (vía godo-r CLI)
- ✅ SSL/TLS automático (Cloudflare)

## 📋 Requisitos Previos

### 1. Cuenta Cloudflare
- Account ID: `b11ab3fe6c1a3625b65cb22d170793b6` ✓
- Workers habilitados ✓
- AI Workers habilitados ✓

### 2. Wrangler CLI
```bash
npm install -g wrangler
wrangler login
wrangler whoami  # Verificar autenticación
```

### 3. GitHub Pages
- Repositorio: `yatrogenesis/Ectus-R`
- GitHub Pages habilitado en `/docs` o branch `gh-pages`

### 4. GoDaddy DNS (Opcional)
- CLI instalado: `godo-r` ✓
- API credentials configuradas

## 🔧 Configuración de Secrets

Los secrets se configuran una sola vez en Cloudflare:

```bash
cd Ectus-R

# AI Providers (opcionales - fallback disponible)
wrangler secret put HUGGINGFACE_API_KEY --env production
wrangler secret put DEEPSEEK_API_KEY --env production
wrangler secret put OLLAMA_URL --env production
wrangler secret put OPENAI_API_KEY --env production

# GoDaddy DNS (opcional)
wrangler secret put GODADDY_API_KEY --env production
wrangler secret put GODADDY_API_SECRET --env production

# GitHub (opcional)
wrangler secret put GITHUB_TOKEN --env production

# Security
wrangler secret put JWT_SECRET --env production
```

### Obtener API Keys

#### HuggingFace (GRATIS)
1. Ir a https://huggingface.co/settings/tokens
2. Crear token con scope "Read"
3. Copiar y configurar: `wrangler secret put HUGGINGFACE_API_KEY`

#### DeepSeek (DeepSeek-V3 disponible hoy!)
1. Ir a https://platform.deepseek.com
2. Registrarse y obtener API key
3. Configurar: `wrangler secret put DEEPSEEK_API_KEY`

#### Ollama (GRATIS - Local)
1. Instalar: https://ollama.ai
2. Ejecutar: `ollama serve`
3. Configurar URL: `wrangler secret put OLLAMA_URL` (default: http://localhost:11434)

#### OpenAI (Opcional - Tiene costos)
1. Ir a https://platform.openai.com/api-keys
2. Crear API key
3. Configurar: `wrangler secret put OPENAI_API_KEY`

## 🚀 Despliegue

### Paso 1: Instalar Dependencias
```bash
cd Ectus-R
npm install
```

### Paso 2: Desarrollo Local
```bash
# Usar configuración de producción
wrangler dev --config wrangler-production.toml --local

# Abrir: http://localhost:8787
# Probar: http://localhost:8787/health
```

### Paso 3: Desplegar a Staging
```bash
# Desplegar a staging primero
wrangler deploy --config wrangler-production.toml --env staging

# Verificar: https://staging.ectus.avermex.com/health
```

### Paso 4: Desplegar a Production
```bash
# Desplegar a producción
wrangler deploy --config wrangler-production.toml --env production

# Verificar:
# https://creator.avermex.com/health
# https://ectus.avermex.com/health
```

## 🌐 Configuración DNS con GoDaddy

### Opción 1: Usar godo-r CLI (Recomendado)

```bash
# Compilar godo-r
cd ../godo-r
cargo build --release

# Configurar credentials
./target/release/godo auth YOUR_API_KEY YOUR_API_SECRET

# Crear registros DNS
./target/release/godo dns add avermex.com CNAME creator yatrogenesis.github.io
./target/release/godo dns add avermex.com CNAME ectus yatrogenesis.github.io
./target/release/godo dns add avermex.com CNAME api yatrogenesis.github.io

# Verificar
./target/release/godo dns list avermex.com
```

### Opción 2: Manual en Cloudflare Dashboard

1. Ir a Cloudflare Dashboard
2. Seleccionar dominio `avermex.com`
3. DNS → Add Record:
   - Type: `CNAME`
   - Name: `creator`
   - Target: `yatrogenesis.github.io`
   - Proxy: ✓ (Orange cloud)

## 📊 Endpoints API

### Health Check
```bash
curl https://creator.avermex.com/health
```

Respuesta:
```json
{
  "status": "healthy",
  "version": "4.0.0-production",
  "infrastructure": {
    "cloudflare": { "workers": true, "ai": true, "kv": true },
    "githubPages": { "enabled": true }
  },
  "aiProviders": {
    "cloudflare": { "available": true, "free": true },
    "huggingface": { "available": true, "free": true },
    "deepseek": { "available": true, "free": false },
    "ollama": { "available": false, "free": true },
    "openai": { "available": false, "free": false }
  }
}
```

### Crear Deployment (Magic Loop)
```bash
curl -X POST https://creator.avermex.com/api/v1/deployments/magic-loop \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "Create a beautiful calculator app",
    "provider": "cloudflare"
  }'
```

Respuesta:
```json
{
  "success": true,
  "deployment": {
    "id": "deploy_1234567890_abcdef123",
    "url": "https://yatrogenesis.github.io/Ectus-R/apps/deploy_1234567890_abcdef123.html",
    "method": "cloudflare-ai",
    "provider": "cloudflare",
    "model": "@cf/meta/llama-3.1-8b-instruct",
    "generationTime": 2341
  }
}
```

### Listar Proveedores AI
```bash
curl https://creator.avermex.com/api/v1/providers
```

### Obtener Deployment
```bash
# Metadata
curl https://creator.avermex.com/api/v1/deployments/DEPLOYMENT_ID

# HTML directo
curl -H "Accept: text/html" https://creator.avermex.com/api/v1/deployments/DEPLOYMENT_ID
```

### Listar Deployments
```bash
curl "https://creator.avermex.com/api/v1/deployments?limit=20&offset=0"
```

### Configurar DNS
```bash
curl -X POST https://creator.avermex.com/api/v1/dns/setup \
  -H "Content-Type: application/json" \
  -d '{
    "domain": "avermex.com",
    "subdomain": "myapp",
    "deploymentId": "deploy_1234567890_abcdef123"
  }'
```

## 🧪 Testing

### Test Health
```bash
curl https://creator.avermex.com/health | jq
```

### Test AI Generation
```bash
# Con Cloudflare AI (gratis)
curl -X POST https://creator.avermex.com/api/v1/deployments/magic-loop \
  -H "Content-Type: application/json" \
  -d '{"prompt": "calculator", "provider": "cloudflare"}' | jq

# Con HuggingFace (gratis)
curl -X POST https://creator.avermex.com/api/v1/deployments/magic-loop \
  -H "Content-Type: application/json" \
  -d '{"prompt": "todo app", "provider": "huggingface"}' | jq

# Fallback automático (prueba todos)
curl -X POST https://creator.avermex.com/api/v1/deployments/magic-loop \
  -H "Content-Type: application/json" \
  -d '{"prompt": "timer app"}' | jq
```

## 📈 Monitoreo

### Cloudflare Dashboard
- Workers Analytics: https://dash.cloudflare.com/
- AI Usage: Workers & Pages → AI
- KV Storage: Workers & Pages → KV
- Logs: Real-time logs en wrangler tail

### Ver Logs en Tiempo Real
```bash
# Production
wrangler tail --config wrangler-production.toml --env production

# Staging
wrangler tail --config wrangler-production.toml --env staging
```

### Analytics
```bash
curl https://creator.avermex.com/api/v1/analytics | jq
```

## 🔒 Seguridad

### SSL/TLS
- ✅ SSL automático vía Cloudflare
- ✅ HTTPS enforced
- ✅ TLS 1.3

### CORS
- Configurado para dominios específicos
- Headers de seguridad incluidos

### Rate Limiting
- Cloudflare automático
- Workers CPU limit: 50ms por request
- Memory limit: 128MB por request

## 🐛 Troubleshooting

### Error: "AI generation failed"
**Solución**: El sistema usará fallback automático. Si todos fallan, usará templates.

### Error: "Deployment not found"
**Solución**: Verificar que el deployment ID existe en KV.

### Error: "DNS setup failed"
**Solución**: Verificar credentials de GoDaddy y que godo-r esté configurado.

### Logs no aparecen
```bash
wrangler tail --format pretty --config wrangler-production.toml --env production
```

### Worker no responde
```bash
# Verificar estado
wrangler deployments list --config wrangler-production.toml

# Re-deploy
wrangler deploy --config wrangler-production.toml --env production
```

## 📚 Recursos

### Documentación
- Cloudflare Workers: https://developers.cloudflare.com/workers/
- Cloudflare AI: https://developers.cloudflare.com/workers-ai/
- Wrangler CLI: https://developers.cloudflare.com/workers/wrangler/
- GitHub Pages: https://pages.github.com/

### Repositorios
- Ectus-R: https://github.com/Yatrogenesis/Ectus-R
- godo-r: https://github.com/Yatrogenesis/godo-r

### Soporte
- Email: pako.molina@gmail.com
- Issues: https://github.com/Yatrogenesis/Ectus-R/issues

## 🎯 Próximos Pasos

1. ✅ **Configurar AI Providers** - Obtener API keys
2. ✅ **Desplegar a Staging** - Probar en staging.ectus.avermex.com
3. ✅ **Configurar DNS** - Usar godo-r para crear registros
4. ✅ **Desplegar a Production** - creator.avermex.com
5. ⏳ **Setup GitHub Actions** - CI/CD automático
6. ⏳ **Monitoreo** - Configurar alertas

## 🏆 Features Completadas

- ✅ Multi-AI Provider System (5 proveedores)
- ✅ Fallback automático inteligente
- ✅ Templates perfectos de respaldo
- ✅ Cloudflare Workers + AI
- ✅ KV Storage
- ✅ D1 Database
- ✅ GitHub Pages integration
- ✅ GoDaddy DNS management
- ✅ SSL/TLS automático
- ✅ CORS configurado
- ✅ Analytics básico
- ✅ Production-ready

---

**Ectus-R Production SaaS v4.0.0**
*Powered by Cloudflare Workers + Multi-AI System*