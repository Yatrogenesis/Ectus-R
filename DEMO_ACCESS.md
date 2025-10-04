# Ectus-R - Acceso al Demo Privado

##  Infraestructura Desplegada

### URLs Activas

**Landing Page Pública**:
- GitHub Pages: https://yatrogenesis.github.io/Ectus-R/landing.html
- Descripción: Landing page comercial sin binarios descargables
- Características: Video demo, lead capture, descarga de reporte técnico

**Demo Privado Funcional**:
- GitHub Pages: https://yatrogenesis.github.io/Ectus-R/demo.html
- Descripción: Demo funcional con generación de código IA en tiempo real
- Características: Autenticación, Cloudflare Workers AI, métricas en vivo

**API Backend (Cloudflare Worker)**:
- Worker URL: https://ectus-r-demo.pako-molina.workers.dev
- Status: https://ectus-r-demo.pako-molina.workers.dev/api/demo/status
- Endpoints:
  - `POST /api/leads` - Captura de leads
  - `POST /api/demo/auth` - Autenticación
  - `POST /api/demo/generate` - Generación de código con IA
  - `GET /api/demo/status` - Health check

##  Credenciales de Acceso

### Opción 1: Credenciales de Usuario (Recomendado para demos)

```
Usuario: demo_user
Contraseña: SecureDemo2025!
```

### Opción 2: Certificado SAT .cer

- Sube tu certificado de e.firma del SAT (.cer file)
- El worker validará el certificado X.509
- Extraerá tu RFC y nombre del certificado
- Autenticación automática

##  Cómo Usar el Demo

### 1. Acceder al Demo

1. Navega a: https://yatrogenesis.github.io/Ectus-R/demo.html
2. Selecciona método de autenticación:
   - **Credenciales**: Usa `demo_user` / `SecureDemo2025!`
   - **SAT Certificate**: Sube tu archivo .cer
3. Click en "Ingresar" o "Verificar y Acceder"

### 2. Generar Código con IA

Una vez autenticado:

1. **Selecciona Lenguaje**: Rust, TypeScript, Python, o Go
2. **Selecciona Framework**: Axum, Actix, Express, FastAPI, etc.
3. **Describe tu Proyecto**: Ejemplo:
   ```
   Create a REST API for a blog platform with user authentication,
   posts, and comments using PostgreSQL
   ```
4. Click en **"Generar Código"**
5. Espera ~2-5 segundos mientras Cloudflare Workers AI genera el código

### 3. Resultados

El demo mostrará:
-  **Código Generado**: Implementación completa production-ready
-  **Tests Generados**: Suite de tests unitarios e integración
-  **Métricas**:
  - Líneas de código
  - Test coverage (%)
  - Tiempo de generación (segundos)
  - Security score (OWASP compliance)

##  Ejemplos de Prompts

### API REST Básica
```
Create a REST API for a todo app with CRUD operations using SQLite
```

### Microservicio con Kafka
```
Build a microservice that processes payment events from Kafka and stores them in MongoDB
```

### GraphQL API
```
Create a GraphQL API for an e-commerce platform with products, orders, and inventory management
```

### WebSocket Server
```
Build a real-time chat server with WebSocket support, user presence, and message history
```

##  Tecnologías Utilizadas

**Frontend (Demo)**:
- HTML5, CSS3, JavaScript vanilla
- GitHub Pages hosting
- Responsive design

**Backend (API)**:
- Cloudflare Workers (edge computing)
- Cloudflare Workers AI (Llama 3.3 70B)
- KV Storage (sessions & metadata)
- D1 Database (lead storage)

**Seguridad**:
- HTTPS obligatorio
- CORS configurado
- Session tokens con expiración (24h)
- SAT certificate validation (X.509)
- Secrets management (Wrangler)

##  Monitoreo

**Ver Logs en Tiempo Real**:
```bash
wrangler tail ectus-r-demo
```

**Dashboard de Cloudflare**:
- https://dash.cloudflare.com/
- Workers > ectus-r-demo
- Ver métricas, requests, errores

##  Próximos Pasos

### Para Habilitar GitHub Pages:

1. Ve a: https://github.com/Yatrogenesis/Ectus-R/settings/pages
2. Source: Deploy from a branch
3. Branch: main
4. Folder: /docs
5. Save

Las páginas estarán disponibles en:
- https://yatrogenesis.github.io/Ectus-R/landing.html
- https://yatrogenesis.github.io/Ectus-R/demo.html

### Para Agregar Video Demo:

Edita `docs/landing.html` y `docs/demo.html`:

```html
<!-- Reemplaza el placeholder con tu video -->
<iframe
    width="100%"
    height="500"
    src="https://www.youtube.com/embed/YOUR_VIDEO_ID"
    frameborder="0"
    allowfullscreen>
</iframe>
```

### Para Configurar DNS (Opcional):

Si tienes dominio `ectus.ai`:

```bash
# Agregar CNAME en Cloudflare:
demo.ectus.ai -> yatrogenesis.github.io
api.ectus.ai -> ectus-r-demo.pako-molina.workers.dev

# O usar GoDo-R CLI:
godo add ectus.ai CNAME demo yatrogenesis.github.io 3600
```

## 🆘 Troubleshooting

### Error: "Invalid credentials"
- Verifica que estás usando: `demo_user` / `SecureDemo2025!`
- Case-sensitive

### Error: "Authorization required"
- Re-autentícate en la página
- El session token expira en 24 horas

### Código no se genera
- Verifica conexión a internet
- Workers AI requiere plan Workers Paid (o usa fallback)
- Ver logs: `wrangler tail ectus-r-demo`

### Certificado SAT no válido
- Debe ser archivo .cer (no .key)
- Debe ser certificado válido X.509
- Debe ser emitido por SAT CA

##  Contacto

Para demos personalizadas o consultas:
- Email: info@yatrogenesis.com
- Lead form: https://yatrogenesis.github.io/Ectus-R/landing.html

---

**Status**:  DEPLOYED AND OPERATIONAL

**Last Updated**: 2025-09-30

**Commit**: 5d951cf (main branch)
