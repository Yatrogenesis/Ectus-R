# Ectus-R Deployment Instructions

## Infraestructura de Comercialización Completa

### 1. Landing Page Pública (GitHub Pages)

**Archivo**: `docs/landing.html`

**Características**:
- Landing page profesional sin descargas directas
- Captura de leads con formulario
- Sección de video embebido (placeholder listo)
- Diseño responsive y optimizado

**Despliegue**:
```bash
# GitHub Pages está configurado para servir desde /docs
# URL: https://yatrogenesis.github.io/Ectus-R/landing.html

# Commit y push
git add docs/landing.html
git commit -m "feat: add commercial landing page"
git push origin main

# Configurar GitHub Pages:
# Settings > Pages > Source: main branch, /docs folder
```

### 2. Demo Privado Funcional

**Archivo**: `docs/demo.html`

**Características**:
- Autenticación dual: Credenciales O Certificado SAT .cer
- Generación de código en tiempo real con Cloudflare Workers AI
- Soporte para Rust, TypeScript, Python, Go
- Métricas en vivo (LOC, coverage, tiempo, seguridad)
- Tests auto-generados

**Acceso**:
```
URL: https://yatrogenesis.github.io/Ectus-R/demo.html
```

### 3. Cloudflare Worker (API Backend)

**Archivo**: `src/worker-demo.js`
**Config**: `wrangler-demo.toml`

**Capacidades**:
- `/api/leads` - Captura y almacenamiento de leads
- `/api/demo/auth` - Autenticación (credentials + SAT .cer)
- `/api/demo/generate` - Generación de código con AI
- `/api/demo/status` - Health check

**Deployment**:
```bash
cd Ectus-R

# 1. Configurar secrets
wrangler secret put DEMO_USERNAME --config wrangler-demo.toml
# Ingresa: admin (o tu usuario preferido)

wrangler secret put DEMO_PASSWORD --config wrangler-demo.toml
# Ingresa: tu_password_seguro

wrangler secret put SENDGRID_API_KEY --config wrangler-demo.toml
# Ingresa: tu_sendgrid_api_key (opcional)

wrangler secret put NOTIFICATION_EMAIL --config wrangler-demo.toml
# Ingresa: info@yatrogenesis.com

# 2. Desplegar worker
wrangler deploy --config wrangler-demo.toml

# 3. Desplegar a producción
wrangler deploy --config wrangler-demo.toml --env production
```

### 4. Configuración DNS

**GoDaddy + Cloudflare**:

```bash
# Opción 1: Usar GoDo-R CLI
godo auth YOUR_API_KEY YOUR_API_SECRET
godo list ectus.ai

# Agregar registros DNS:
godo add ectus.ai CNAME demo yatrogenesis.github.io 3600
godo add ectus.ai CNAME api ectus-r-demo.yatrogenesis.workers.dev 3600

# Opción 2: Panel de Cloudflare
# 1. Agregar dominio ectus.ai a Cloudflare
# 2. Configurar Workers Routes:
#    - demo.ectus.ai/* -> ectus-r-demo-prod
#    - api.ectus.ai/demo/* -> ectus-r-demo-prod
# 3. Configurar Page Rules para GitHub Pages
```

### 5. GitHub Pages Configuration

```bash
# En el repositorio Ectus-R:
# Settings > Pages
# Source: Deploy from a branch
# Branch: main
# Folder: /docs

# Custom domain (opcional):
# demo.ectus.ai
```

### 6. Video Embebido

Para agregar el video de demostración, edita `docs/landing.html` y `docs/demo.html`:

```html
<!-- Reemplazar el placeholder con: -->
<iframe
    width="100%"
    height="500"
    src="https://www.youtube.com/embed/YOUR_VIDEO_ID"
    frameborder="0"
    allowfullscreen>
</iframe>

<!-- O para video propio: -->
<video controls width="100%">
    <source src="https://your-cdn.com/demo-video.mp4" type="video/mp4">
</video>
```

### 7. Reporte Técnico y Ficha de Producto

Crea los PDFs y súbelos a Cloudflare R2 o GitHub:

```bash
# Opción 1: GitHub Release
# 1. Crear release en GitHub
# 2. Adjuntar PDFs
# 3. Obtener URLs de descarga

# Opción 2: Cloudflare R2
wrangler r2 bucket create ectus-public-assets
wrangler r2 object put ectus-public-assets/Ectus-R-Technical-Report.pdf --file ./assets/report.pdf
wrangler r2 object put ectus-public-assets/Ectus-R-Product-Sheet.pdf --file ./assets/sheet.pdf

# URLs públicas:
# https://pub-YOUR-ID.r2.dev/Ectus-R-Technical-Report.pdf
```

### 8. Flujo de Usuario Completo

**Promoción → Landing → Lead Capture → Demo Privado**

1. **Usuario llega** a `https://yatrogenesis.github.io/Ectus-R/landing.html`
2. **Ve video** de demostración
3. **Completa formulario** para descargar reporte técnico
4. **Recibe email** con:
   - Link al reporte PDF
   - Credenciales temporales de demo
   - Agenda para demo en vivo
5. **Accede a demo** en `demo.html` con credenciales
6. **Prueba generación** de código en tiempo real
7. **Agenda demo** personalizada contigo

### 9. Autenticación para Demo Live

**Opción A: Credenciales Simples**
```javascript
// Configurar en Wrangler secrets:
DEMO_USERNAME=admin
DEMO_PASSWORD=your_secure_password

// Usuario ingresa en demo.html
```

**Opción B: Certificado SAT .cer**
```javascript
// Usuario sube su .cer del SAT
// Worker valida:
// 1. Formato X.509 válido
// 2. Emisor = SAT
// 3. No expirado
// 4. Extrae RFC del Subject
```

**Opción C: Para tu uso exclusivo**
- Crea contraseña fuerte única
- O usa tu certificado .cer del SAT
- Comparte credenciales solo en demos en vivo

### 10. Monitoreo y Analytics

**Cloudflare Analytics**:
```bash
# Ver métricas del worker
wrangler tail ectus-r-demo-prod

# Dashboard: https://dash.cloudflare.com/
# Workers > ectus-r-demo-prod > Metrics
```

**Leads Capturados**:
```bash
# Los leads se guardan en KV: METADATA
# Consultar via Worker o dashboard

# Agregar endpoint de admin:
GET /api/admin/leads?auth=YOUR_ADMIN_TOKEN
```

### 11. Testing Local

```bash
# Worker local
cd Ectus-R
wrangler dev --config wrangler-demo.toml

# Test API:
curl http://localhost:8787/api/demo/status

# Demo HTML local
python -m http.server 8000 --directory docs
# Open: http://localhost:8000/demo.html
```

### 12. URLs Finales

Después de configuración completa:

- **Landing Page**: https://demo.ectus.ai (o GitHub Pages)
- **Demo Privado**: https://demo.ectus.ai/demo.html
- **API Backend**: https://api.ectus.ai/demo/
- **Worker Directo**: https://ectus-r-demo.yatrogenesis.workers.dev

### 13. Seguridad

-  HTTPS en todas las URLs
-  CORS configurado
-  Rate limiting en Workers
-  Secrets en Wrangler (no en código)
-  Validación de inputs
-  Session tokens con expiración
-  SAT certificate validation

### 14. Próximos Pasos

1.  Configurar secrets de Wrangler
2.  Desplegar worker a Cloudflare
3. ⏳ Configurar DNS (GoDaddy/Cloudflare)
4. ⏳ Habilitar GitHub Pages
5. ⏳ Crear video de demostración
6. ⏳ Generar PDFs de reporte técnico
7. ⏳ Configurar email automation (SendGrid)
8. ⏳ Testing end-to-end

### 15. Comandos Rápidos

```bash
# Deploy todo
cd Ectus-R
git add .
git commit -m "feat: complete commercial infrastructure"
git push origin main
wrangler deploy --config wrangler-demo.toml --env production

# Ver logs en tiempo real
wrangler tail ectus-r-demo-prod

# Actualizar secrets
wrangler secret put DEMO_PASSWORD --config wrangler-demo.toml

# Test local
wrangler dev --config wrangler-demo.toml
```

---

**NOTA IMPORTANTE**:
- El worker requiere Cloudflare Workers AI (plan Workers Paid)
- Los LLMs usan `@cf/meta/llama-3.3-70b-instruct-fp8-fast`
- Alternativa gratuita: Usar OpenAI API en lugar de Workers AI
- Para acceso privado tuyo: Usa credenciales únicas o tu .cer SAT

**Contacto para Demo**:
- Email: info@yatrogenesis.com
- Formulario: Captura automática en landing page
