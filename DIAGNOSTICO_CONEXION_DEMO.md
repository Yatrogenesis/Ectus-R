# 🔧 Diagnóstico de Conexión Demo - Ectus-R

## ✅ Estado del Sistema

**Fecha**: 2025-09-30 22:00 UTC
**Status**: 🟢 OPERACIONAL

---

## 📊 Verificación Completa

### 1. Backend Worker ✅
```bash
URL: https://ectus-r-demo.pako-molina.workers.dev
Status: OPERACIONAL
```

**Health Check**:
```bash
curl https://ectus-r-demo.pako-molina.workers.dev/api/demo/status
# Response: {"status":"operational","version":"1.0.0","ai_available":true}
```

**Autenticación**:
```bash
curl -X POST https://ectus-r-demo.pako-molina.workers.dev/api/demo/auth \
  -H "Content-Type: application/json" \
  -d '{"authType":"credentials","credentials":{"username":"demo_user","password":"SecureDemo2025!"}}'

# Response: {"success":true,"sessionId":"...","user":{...}}
```

### 2. Frontend Pages ✅
```
URL: https://ectus-r-creator.pages.dev
Deployment: Production (25 minutos ago)
Commit: 2883006
```

**Páginas Activas**:
- ✅ `/landing.html` - Landing comercial
- ✅ `/demo.html` - Demo con autenticación
- ✅ `/demo` - Alias sin extensión (Cloudflare Pages auto-redirect)
- ✅ `/test-demo-connection.html` - Diagnóstico automático

### 3. CORS Headers ✅
```
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: GET, POST, OPTIONS
Access-Control-Allow-Headers: Content-Type, Authorization
```

### 4. API Endpoints ✅

| Endpoint | Método | Status | Descripción |
|----------|--------|--------|-------------|
| `/api/demo/status` | GET | ✅ | Health check |
| `/api/demo/auth` | POST | ✅ | Autenticación |
| `/api/demo/generate` | POST | ✅ | Generación código IA |
| `/api/leads` | POST | ✅ | Captura de leads |

---

## 🧪 Cómo Probar la Conexión

### Opción 1: Test Automático (RECOMENDADO)

1. **Abre el test de diagnóstico**:
   ```
   https://ectus-r-creator.pages.dev/test-demo-connection.html
   ```

2. **Verifica que todos los tests pasen**:
   - ✅ Health Check
   - ✅ CORS Headers
   - ✅ Authentication (Credentials)
   - ✅ Code Generation (with session)

3. **Si algún test falla**: Ver sección de troubleshooting

### Opción 2: Acceso Manual al Demo

1. **Abre el demo**:
   ```
   https://ectus-r-creator.pages.dev/demo
   ```

2. **Ingresa credenciales**:
   - Usuario: `demo_user`
   - Password: `SecureDemo2025!`

3. **Genera código**:
   - Prompt: "Create a REST API for user authentication"
   - Language: Rust
   - Framework: Axum
   - Click "Generar Código"

4. **Verifica resultados**:
   - Código generado debe aparecer en ~2-5 segundos
   - Métricas: Líneas, Coverage, Tiempo, Seguridad
   - Tests auto-generados deben aparecer

### Opción 3: Test con cURL (Técnico)

```bash
# 1. Health check
curl https://ectus-r-demo.pako-molina.workers.dev/api/demo/status

# 2. Autenticación
SESSION=$(curl -s -X POST https://ectus-r-demo.pako-molina.workers.dev/api/demo/auth \
  -H "Content-Type: application/json" \
  -d '{"authType":"credentials","credentials":{"username":"demo_user","password":"SecureDemo2025!"}}' \
  | grep -o '"sessionId":"[^"]*"' | cut -d'"' -f4)

echo "Session: $SESSION"

# 3. Generación de código
curl -X POST https://ectus-r-demo.pako-molina.workers.dev/api/demo/generate \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $SESSION" \
  -d '{"prompt":"Create a health check endpoint","language":"rust","framework":"axum"}'
```

---

## 🔍 Troubleshooting

### Problema: "No se puede conectar al servidor"

**Síntomas**:
- Error en consola del navegador: "Failed to fetch"
- Demo no carga después de login

**Solución**:
1. Verifica que el worker esté activo:
   ```bash
   curl https://ectus-r-demo.pako-molina.workers.dev/api/demo/status
   ```

2. Si no responde, redeploy del worker:
   ```bash
   cd C:\Users\Propietario\Ectus-R
   wrangler deploy --config wrangler-demo.toml
   ```

### Problema: "Credenciales inválidas"

**Síntomas**:
- Mensaje "Credenciales inválidas" después de login
- Error 401 en la respuesta

**Solución**:
1. Verifica las credenciales exactas:
   - Usuario: `demo_user` (case-sensitive)
   - Password: `SecureDemo2025!` (incluye mayúsculas y símbolos)

2. Verifica secrets del worker:
   ```bash
   wrangler secret list --config wrangler-demo.toml
   ```

3. Si faltan secrets, créalos:
   ```bash
   echo "demo_user" | wrangler secret put DEMO_USERNAME --config wrangler-demo.toml
   echo "SecureDemo2025!" | wrangler secret put DEMO_PASSWORD --config wrangler-demo.toml
   ```

### Problema: "CORS error"

**Síntomas**:
- Error en consola: "CORS policy blocked"
- Preflight request fails

**Solución**:
1. Verifica CORS headers en worker-demo.js:103-105
2. Asegúrate que OPTIONS requests están manejados:117-119
3. Redeploy si es necesario

### Problema: "Session expired"

**Síntomas**:
- Error 401 al generar código
- Mensaje "Invalid session"

**Solución**:
1. Las sesiones expiran después de 24 horas
2. Cierra sesión y vuelve a autenticarte
3. La sesión se guarda en KV namespace `SESSIONS`

### Problema: Certificado SAT no valida

**Síntomas**:
- Error "Certificado inválido o no autorizado"
- Validación falla con RFC correcto

**Solución**:
1. Verifica que el certificado es: `D:\00001000000702080308.cer`
2. RFC autorizado: `MOBF8108153Q5`
3. CURP autorizado: `MOBF810815HYNLRR00`
4. Ver logs del worker:
   ```bash
   wrangler tail ectus-r-demo
   ```

---

## 📞 Tests de Conectividad

### Test 1: Worker Responde
```bash
curl -I https://ectus-r-demo.pako-molina.workers.dev/api/demo/status
# Esperado: HTTP/2 200
```

### Test 2: CORS Funciona
```bash
curl -X OPTIONS https://ectus-r-demo.pako-molina.workers.dev/api/demo/auth \
  -H "Origin: https://ectus-r-creator.pages.dev" \
  -H "Access-Control-Request-Method: POST" -v
# Esperado: Access-Control-Allow-Origin: *
```

### Test 3: Autenticación Funciona
```bash
curl -X POST https://ectus-r-demo.pako-molina.workers.dev/api/demo/auth \
  -H "Content-Type: application/json" \
  -d '{"authType":"credentials","credentials":{"username":"demo_user","password":"SecureDemo2025!"}}' | jq
# Esperado: {"success":true,"sessionId":"..."}
```

### Test 4: Demo Page Carga
```bash
curl -I https://ectus-r-creator.pages.dev/demo
# Esperado: HTTP/2 200
```

---

## 🎯 URLs Importantes

| Recurso | URL |
|---------|-----|
| **Demo Principal** | https://ectus-r-creator.pages.dev/demo |
| **Test Diagnóstico** | https://ectus-r-creator.pages.dev/test-demo-connection.html |
| **API Backend** | https://ectus-r-demo.pako-molina.workers.dev/api |
| **Landing Page** | https://ectus-r-creator.pages.dev/landing.html |

---

## ✅ Confirmación Final

**Todo está operacional cuando**:
1. ✅ Health check devuelve `{"status":"operational"}`
2. ✅ Autenticación con credenciales devuelve `sessionId`
3. ✅ Demo page carga sin errores en consola
4. ✅ Generación de código funciona y devuelve código + tests
5. ✅ Test automático pasa todos los checks

**Si todo lo anterior está OK**: El sistema está completamente funcional.

**Si algo falla**: Usar la sección de Troubleshooting o ejecutar:
```bash
wrangler tail ectus-r-demo
```
para ver logs en tiempo real.

---

**Última Verificación**: 2025-09-30 22:00 UTC
**Status**: 🟢 TODOS LOS SISTEMAS OPERACIONALES
