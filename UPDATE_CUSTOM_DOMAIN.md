# ✅ Actualización de Dominio Custom

## Status Actual

### Cloudflare Pages Deployment
- ✅ **Proyecto**: ectus-r-creator
- ✅ **Deployment**: https://81cfda5d.ectus-r-creator.pages.dev
- ✅ **Production**: https://ectus-r-creator.pages.dev
- ✅ **Archivos**: 11 archivos desplegados
- ✅ **Landing**: Funcionando
- ✅ **Demo**: Funcionando

### URLs Funcionando

**Cloudflare Pages URLs**:
```
https://ectus-r-creator.pages.dev/landing.html ✅ LIVE
https://ectus-r-creator.pages.dev/demo.html ✅ LIVE
```

**Custom Domain** (requiere configuración manual):
```
https://creator.avermex.com/landing.html ⏳ REDIRECT (via Worker)
https://creator.avermex.com/demo.html ⏳ REDIRECT (via Worker)
```

---

## 🔧 Configurar Custom Domain en Cloudflare Pages

### Opción A: Via Dashboard (Recomendado)

1. **Ve a Cloudflare Dashboard**:
   ```
   https://dash.cloudflare.com/
   → Workers & Pages
   → ectus-r-creator
   → Custom domains
   ```

2. **Agregar Custom Domain**:
   - Click "Set up a custom domain"
   - Ingresar: `creator.avermex.com`
   - Click "Continue"
   - Cloudflare configurará automáticamente el DNS

3. **Verificar**:
   - El dominio aparecerá en la lista
   - Status: Active (puede tardar 1-5 minutos)

### Opción B: Via Wrangler CLI

```bash
# Agregar custom domain al proyecto Pages
wrangler pages domain add creator.avermex.com --project-name=ectus-r-creator

# Verificar custom domains
wrangler pages project get ectus-r-creator
```

---

## 🚨 Problema Actual: Worker Redirect

El dominio `creator.avermex.com` actualmente tiene un Worker que redirige a:
```
https://ectus-r-saas.pako-molina.workers.dev
```

### Solución: Eliminar Worker Route

1. **Dashboard Cloudflare**:
   ```
   https://dash.cloudflare.com/
   → Workers & Pages
   → Routes (en el menú)
   ```

2. **Buscar Route**:
   - Buscar: `creator.avermex.com/*`
   - O: `creator.avermex.com/landing.html`
   - O: `creator.avermex.com/demo.html`

3. **Eliminar Route**:
   - Click en los 3 puntos (...)
   - "Delete"
   - Confirmar

4. **Verificar DNS**:
   - Zona: avermex.com
   - DNS Records
   - Asegurar que `creator` apunta a:
     - CNAME: `ectus-r-creator.pages.dev` (recomendado)
     - O mantener: `yatrogenesis.github.io`

---

## 📋 Instrucciones Paso a Paso

### Paso 1: Eliminar Worker Route (si existe)

```bash
# Listar Workers Routes
wrangler routes list

# O via dashboard:
# https://dash.cloudflare.com/[account-id]/workers/routes
```

### Paso 2: Configurar DNS Correcto

**Opción A: Apuntar a Pages directamente**
```bash
# Actualizar CNAME a Pages
godo add avermex.com CNAME creator ectus-r-creator.pages.dev 3600

# O vía Cloudflare Dashboard:
# DNS > Records > Edit creator
# Target: ectus-r-creator.pages.dev
```

**Opción B: Mantener GitHub Pages** (configuración actual)
```bash
# Ya está configurado:
creator.avermex.com -> yatrogenesis.github.io

# Agregar custom domain en Pages Dashboard para conectar
```

### Paso 3: Configurar Custom Domain en Pages

Via Dashboard:
1. https://dash.cloudflare.com/
2. Workers & Pages > ectus-r-creator
3. Custom domains > "Set up a custom domain"
4. Ingresar: `creator.avermex.com`
5. Continue

### Paso 4: Verificar

```bash
# Esperar 1-5 minutos para propagación
# Luego probar:
curl -sL https://creator.avermex.com/landing.html | grep -o "<title>.*</title>"

# Esperado:
# <title>Ectus-R - Plataforma de Ingeniería de Software Autónoma</title>
```

---

## 🎯 URLs Finales Esperadas

Después de la configuración:

```
Landing Page:
https://creator.avermex.com/landing.html ✅
https://ectus-r-creator.pages.dev/landing.html ✅

Demo Privado:
https://creator.avermex.com/demo.html ✅
https://ectus-r-creator.pages.dev/demo.html ✅

Credenciales Demo:
Usuario: demo_user
Password: SecureDemo2025!
```

---

## 🔍 Troubleshooting

### Si sigue redirigiendo al Worker:

1. **Verificar Routes**:
   ```
   Dashboard > Workers & Pages > Routes
   Buscar: creator.avermex.com/*
   Eliminar si existe
   ```

2. **Limpiar Cache**:
   ```
   Dashboard > Caching > Configuration
   Purge Everything
   ```

3. **Verificar Page Rules**:
   ```
   Dashboard > Rules > Page Rules
   Buscar reglas que afecten creator.avermex.com
   Eliminar o modificar
   ```

### Si DNS no resuelve:

```bash
# Verificar DNS actual
nslookup creator.avermex.com

# Limpiar cache local
ipconfig /flushdns  # Windows

# Probar con DNS público
nslookup creator.avermex.com 8.8.8.8
```

---

## ✅ Estado Actual del Deployment

### Cloudflare Pages: ✅ DESPLEGADO
- Landing page: ✅ Funcional
- Demo page: ✅ Funcional
- Worker API: ✅ Operacional
- Custom domain: ⏳ Requiere configuración manual

### Próximo Paso:
**Configurar custom domain vía Dashboard de Cloudflare**
1. Ir a: https://dash.cloudflare.com/
2. Workers & Pages > ectus-r-creator > Custom domains
3. Agregar: creator.avermex.com

**Última Actualización**: 2025-09-30 21:15 UTC
**Deployment ID**: 81cfda5d
**Status**: ✅ PAGES DEPLOYED - ⏳ CUSTOM DOMAIN CONFIG NEEDED
