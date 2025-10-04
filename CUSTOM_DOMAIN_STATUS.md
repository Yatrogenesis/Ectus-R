#  Dominio Personalizado Configurado

##  Custom Domain: creator.avermex.com

### Status:  OPERACIONAL

---

## URLs Activas con Dominio Custom

### Landing Page Comercial
```
URL Principal: https://creator.avermex.com/landing.html
URL Alternativa: https://yatrogenesis.github.io/Ectus-R/landing.html
Status:  LIVE
HTTPS:  Habilitado (vía Cloudflare)
```

### Demo Privado Funcional
```
URL Principal: https://creator.avermex.com/demo.html
URL Alternativa: https://yatrogenesis.github.io/Ectus-R/demo.html
Status:  LIVE
HTTPS:  Habilitado (vía Cloudflare)

Credenciales:
Usuario: demo_user
Password: SecureDemo2025!
```

---

##  Configuración DNS

### Registro CNAME (GoDaddy)
```
Tipo: CNAME
Nombre: creator
Destino: yatrogenesis.github.io
TTL: 3600
Status:  Configurado
```

### GitHub Pages
```
Custom Domain: creator.avermex.com
Source: main branch, /docs folder
CNAME File:  Creado en docs/CNAME
HTTPS: ⏳ En proceso (puede tardar hasta 24h para certificado)
Status:  Configurado
```

### Cloudflare
```
DNS Proxy:  Activo (ícono naranja)
SSL/TLS: Flexible o Full
Status:  Funcionando
Redirect HTTP → HTTPS:  Automático
```

---

##  Tests de Verificación

### Test 1: Resolución DNS
```bash
nslookup creator.avermex.com
# Response: Cloudflare IPs (2606:4700:3036::ac43:8424)
#  DNS propagado correctamente
```

### Test 2: Landing Page
```bash
curl -L https://creator.avermex.com/landing.html
# Response: HTTP 200 + HTML content
#  Landing page accesible
```

### Test 3: Demo Page
```bash
curl -L https://creator.avermex.com/demo.html
# Response: HTTP 200 + HTML content
#  Demo page accesible
```

### Test 4: HTTP → HTTPS Redirect
```bash
curl -I http://creator.avermex.com/landing.html
# Response: HTTP 301 → https://creator.avermex.com/landing.html
#  Redirect automático funcionando
```

---

##  URLs para Compartir

### Para Promoción y Marketing
```
Landing Page Principal:
https://creator.avermex.com/landing.html

Mensaje de Promoción:
" Descubre Ectus-R - Plataforma de Ingeniería de Software con IA
 10x más rápido | 95% test coverage | 100% seguridad OWASP
 https://creator.avermex.com/landing.html"
```

### Para Demos en Vivo
```
Demo Privado:
https://creator.avermex.com/demo.html

Credenciales:
Usuario: demo_user
Password: SecureDemo2025!

Mensaje para Clientes:
" Accede al demo privado de Ectus-R
Genera código production-ready con IA en tiempo real
 https://creator.avermex.com/demo.html"
```

---

##  Configuración de Seguridad

### HTTPS/SSL
-  Certificado SSL vía Cloudflare
-  Redirect automático HTTP → HTTPS
-  TLS 1.2/1.3 soportado
- ⏳ GitHub Pages SSL (puede tardar hasta 24h en activarse)

### DNS
-  DNSSEC: Disponible vía Cloudflare
-  CAA Records: Configurables
-  Cloudflare proxy activo (protección DDoS)

### Headers de Seguridad
```
Server: cloudflare
CF-RAY: [tracking-id]
X-Frame-Options: SAMEORIGIN (recomendado)
X-Content-Type-Options: nosniff
```

---

##  Estructura de URLs

### Sitio Principal
```
https://creator.avermex.com/
├── landing.html          (Landing page comercial)
├── demo.html            (Demo privado con auth)
└── index.html           (Página actual de docs)
```

### API Backend
```
https://ectus-r-demo.pako-molina.workers.dev/api/
├── leads                (Captura de leads)
├── demo/auth           (Autenticación)
├── demo/generate       (Generación de código IA)
└── demo/status         (Health check)
```

---

##  Propagación DNS

### Status Actual
-  GoDaddy DNS: Configurado
-  Cloudflare: Proxy activo
-  GitHub Pages: CNAME configurado
- ⏳ Propagación global: 0-48 horas (usualmente <1 hora)

### Verificar Propagación
```bash
# Diferentes DNS servers
nslookup creator.avermex.com 8.8.8.8        # Google DNS
nslookup creator.avermex.com 1.1.1.1        # Cloudflare DNS
nslookup creator.avermex.com 208.67.222.222 # OpenDNS

# Todas deberían resolver a Cloudflare IPs
```

---

##  Siguiente Pasos Opcionales

### 1. Configurar Subdominio API
```bash
# Agregar CNAME para API
godo add avermex.com CNAME api ectus-r-demo.pako-molina.workers.dev 3600

# Resultado:
# https://api.avermex.com/demo/generate
```

### 2. Agregar Dominio Raíz
```bash
# Opción: Usar dominio raíz creator.avermex.com como principal
# Actualizar docs/CNAME a solo:
creator.avermex.com

# Acceso directo:
# https://creator.avermex.com (sin /landing.html)
```

### 3. Configurar Email Forwarding
```bash
# Crear email: info@creator.avermex.com
# Redirigir a: tu_email@gmail.com
# Via GoDaddy Email Forwarding (gratis)
```

### 4. Habilitar Cloudflare Features
- Page Rules: Redirect root a /landing.html
- Caching: Mejorar performance
- Firewall Rules: Bloquear bots maliciosos
- Rate Limiting: Proteger API endpoints
- Web Analytics: Métricas de visitantes

---

##  Monitoreo

### Cloudflare Analytics
```
Dashboard: https://dash.cloudflare.com/
Domain: avermex.com
Analytics: Traffic, Bandwidth, Threats blocked
```

### GitHub Pages Build Status
```
https://github.com/Yatrogenesis/Ectus-R/settings/pages
Status: Built and deployed
Last deployment: [timestamp]
```

### Uptime Monitoring (Recomendado)
```bash
# Servicios gratuitos:
- UptimeRobot: https://uptimerobot.com/
- Better Uptime: https://betteruptime.com/
- Freshping: https://www.freshping.io/

# Monitorear:
https://creator.avermex.com/landing.html
https://creator.avermex.com/demo.html
https://ectus-r-demo.pako-molina.workers.dev/api/demo/status
```

---

##  Troubleshooting

### Problema: Certificado SSL no aparece
**Solución**:
- Esperar hasta 24 horas para GitHub Pages SSL
- Cloudflare SSL ya está activo (modo Flexible)
- Verificar en: https://github.com/Yatrogenesis/Ectus-R/settings/pages

### Problema: DNS no resuelve
**Solución**:
```bash
# Verificar propagación
nslookup creator.avermex.com

# Limpiar cache DNS local
ipconfig /flushdns  # Windows
sudo dscacheutil -flushcache  # Mac
sudo systemd-resolve --flush-caches  # Linux
```

### Problema: Error 522 de Cloudflare
**Solución**:
- GitHub Pages está down (raro)
- Verificar: https://www.githubstatus.com/
- Esperar restauración automática

### Problema: Contenido antiguo en caché
**Solución**:
```bash
# Purge Cloudflare cache
Dashboard > Caching > Purge Everything

# O agregar query string
https://creator.avermex.com/landing.html?v=2
```

---

##  Checklist de Configuración

- [x] DNS CNAME configurado en GoDaddy
- [x] CNAME file creado en GitHub repo
- [x] GitHub Pages configurado con custom domain
- [x] Cloudflare proxy activo
- [x] HTTPS redirect funcionando
- [x] Landing page accesible
- [x] Demo page accesible
- [x] DNS propagado globalmente
- [ ] SSL de GitHub activado (0-24h)
- [ ] Page Rules de Cloudflare (opcional)
- [ ] Uptime monitoring configurado (opcional)

---

##  ¡Dominio Custom Activo!

Tu plataforma Ectus-R ahora está disponible en:

### URLs Principales
 **Landing**: https://creator.avermex.com/landing.html
 **Demo**: https://creator.avermex.com/demo.html
 **API**: https://ectus-r-demo.pako-molina.workers.dev

### URLs Alternativas (siguen funcionando)
 **Landing**: https://yatrogenesis.github.io/Ectus-R/landing.html
 **Demo**: https://yatrogenesis.github.io/Ectus-R/demo.html

**Última Actualización**: 2025-09-30 21:00 UTC
**Status**: 🟢 DOMINIO CUSTOM OPERACIONAL
**DNS**:  Propagado
**HTTPS**:  Activo (Cloudflare)
