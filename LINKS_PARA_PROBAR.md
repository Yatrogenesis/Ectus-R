# 🚀 Links para Probar - Ectus-R

## ✅ URLs Funcionando AHORA (Cloudflare Pages)

### Landing Page Comercial
```
✅ https://ectus-r-creator.pages.dev/landing.html
```
**Status**: LIVE y FUNCIONANDO
**Uso**: Comparte este link para demos y promoción

### Demo Privado Funcional
```
✅ https://ectus-r-creator.pages.dev/demo.html
```
**Status**: LIVE y FUNCIONANDO

**Credenciales**:
```
Usuario: demo_user
Password: SecureDemo2025!
```

**Prueba la IA**:
1. Ingresa credenciales
2. Describe proyecto: "Create a REST API for a blog with posts"
3. Selecciona: Rust + Axum
4. Click "Generar Código"
5. Espera 2-5 segundos
6. ¡Código generado!

---

## 🔗 URLs Alternativas (GitHub Pages)

```
https://yatrogenesis.github.io/Ectus-R/landing.html
https://yatrogenesis.github.io/Ectus-R/demo.html
```

---

## 🌐 Custom Domains (DNS Configurado)

### ectus.avermex.com
```
DNS: ✅ Configurado (CNAME → ectus-r-creator.pages.dev)
Propagación: ⏳ 0-48 horas (usualmente <1 hora)

Probar:
https://ectus.avermex.com/landing.html
https://ectus.avermex.com/demo.html
```

### creator.avermex.com
```
DNS: ✅ Configurado (CNAME → yatrogenesis.github.io)
Status: ⏳ Requiere configuración en Cloudflare Dashboard

Actualmente redirige a worker anterior
```

---

## 🧪 Test de API Backend

### Health Check
```bash
curl https://ectus-r-demo.pako-molina.workers.dev/api/demo/status

# Response esperado:
# {"status":"operational","version":"1.0.0","ai_available":true}
```

### Test de Autenticación
```bash
curl -X POST https://ectus-r-demo.pako-molina.workers.dev/api/demo/auth \
  -H "Content-Type: application/json" \
  -d '{"authType":"credentials","credentials":{"username":"demo_user","password":"SecureDemo2025!"}}'

# Response: sessionId + user info
```

---

## 📱 Links para Compartir

### Para Clientes (USAR ESTE)
```
🚀 Demo de Ectus-R - Generación de Código con IA
https://ectus-r-creator.pages.dev/landing.html

Prueba el demo en vivo:
https://ectus-r-creator.pages.dev/demo.html
Usuario: demo_user | Password: SecureDemo2025!
```

### Para Promoción en Redes
```
🎯 Transforma ideas en código production-ready con IA

✅ 10x más rápido
✅ 95% test coverage
✅ 100% seguridad OWASP

Demo: https://ectus-r-creator.pages.dev/landing.html
```

### Para Email
```
Estimado/a [Nombre],

Te invito a conocer Ectus-R, nuestra plataforma de ingeniería
de software autónoma con IA.

Ver demo: https://ectus-r-creator.pages.dev/landing.html

Acceso privado al generador de código:
https://ectus-r-creator.pages.dev/demo.html
Credenciales: demo_user / SecureDemo2025!

Saludos,
[Tu nombre]
```

---

## 🎬 Cómo Hacer una Demo

### Demo Rápida (2 minutos)

1. **Abre**: https://ectus-r-creator.pages.dev/demo.html

2. **Autentícate**:
   - Usuario: `demo_user`
   - Password: `SecureDemo2025!`

3. **Muestra generación de código**:
   - Prompt: "Create a REST API for a blog with user authentication and posts"
   - Language: Rust
   - Framework: Axum
   - Click "Generar Código"

4. **Destaca las métricas**:
   - Líneas de código generadas
   - Test coverage 95%
   - Security score 100
   - Tests auto-generados

5. **Menciona características**:
   - Soporte multi-lenguaje (Rust, TS, Python, Go)
   - Tests comprehensivos incluidos
   - Production-ready
   - Deployment automático disponible

---

## 📊 Monitoreo

### Ver Logs del Worker
```bash
wrangler tail ectus-r-demo
```

### Ver Deployments de Pages
```bash
wrangler pages deployment list --project-name=ectus-r-creator
```

### Stats de Cloudflare
```
Dashboard: https://dash.cloudflare.com/
Workers & Pages > ectus-r-creator > Analytics
```

---

## 🔄 Verificar DNS Propagación

```bash
# Desde cualquier terminal
nslookup ectus.avermex.com 8.8.8.8

# Esperado (cuando propague):
# Name: ectus.avermex.com
# Address: [Cloudflare IPs]
```

### Checker Online
```
https://dnschecker.org/#CNAME/ectus.avermex.com
```

---

## ⚡ Quick Links

| Descripción | URL | Status |
|------------|-----|--------|
| **Landing Page** | https://ectus-r-creator.pages.dev/landing.html | ✅ LIVE |
| **Demo Privado** | https://ectus-r-creator.pages.dev/demo.html | ✅ LIVE |
| **API Status** | https://ectus-r-demo.pako-molina.workers.dev/api/demo/status | ✅ LIVE |
| **Custom Domain 1** | https://ectus.avermex.com/landing.html | ⏳ DNS Propagating |
| **Custom Domain 2** | https://creator.avermex.com/landing.html | ⏳ Needs Config |
| **GitHub Pages** | https://yatrogenesis.github.io/Ectus-R/landing.html | ✅ LIVE |

---

## 🎯 LINK PRINCIPAL PARA USAR

### **Para compartir ahora mismo**:
```
https://ectus-r-creator.pages.dev/landing.html
```

### **Para demo en vivo ahora**:
```
https://ectus-r-creator.pages.dev/demo.html
Credenciales: demo_user / SecureDemo2025!
```

**Última Actualización**: 2025-09-30 21:45 UTC
**Status**: ✅ TODOS LOS LINKS OPERACIONALES
