#  Acceso al Demo Ectus-R - Guía Rápida

##  URL Principal del Demo

```
https://ectus-r-creator.pages.dev/demo
```

---

##  Credenciales de Acceso

### Opción 1: Usuario y Contraseña
```
Usuario:    demo_user
Contraseña: SecureDemo2025!
```

### Opción 2: Certificado SAT
```
Archivo: D:\00001000000702080308.cer
RFC:     MOBF8108153Q5
Titular: Francisco Molina Burgos
```

---

##  Cómo Acceder (3 Pasos)

### 1. Abre el Demo
Navega a: **https://ectus-r-creator.pages.dev/demo**

### 2. Autentícate
- **Tab "Credenciales"**: Ingresa `demo_user` / `SecureDemo2025!`
- **Tab "Certificado SAT"**: Sube el archivo `.cer` desde `D:\`

### 3. Genera Código
1. Selecciona lenguaje (Rust, TypeScript, Python, Go)
2. Selecciona framework (Axum, Actix, Rocket)
3. Describe tu proyecto en el campo de texto
4. Click "Generar Código"
5. ¡Listo! Código y tests aparecen en 2-5 segundos

---

##  Ejemplos de Prompts

### REST API Blog
```
Create a REST API for a blog platform with user authentication,
posts, comments, and tags using PostgreSQL
```

### Microservicio de Pagos
```
Create a payment processing microservice with Stripe integration,
webhook handling, and transaction logging
```

### Sistema de Autenticación
```
Create a secure authentication system with JWT tokens,
refresh tokens, password reset, and email verification
```

### API de E-commerce
```
Create an e-commerce API with products, shopping cart,
orders, and inventory management
```

---

##  Verificar Conexión

Si tienes problemas para acceder, usa el test automático:

```
https://ectus-r-creator.pages.dev/test-demo-connection.html
```

Este test verifica:
-  Conexión con backend API
-  Headers CORS
-  Autenticación funcional
-  Generación de código con IA

---

##  Métricas que Verás

Después de generar código, el sistema muestra:

| Métrica | Descripción |
|---------|-------------|
| **Líneas** | Cantidad de líneas de código generadas |
| **Coverage** | Cobertura de tests (objetivo: 95%) |
| **Tiempo** | Tiempo de generación (típicamente 2-5s) |
| **Seguridad** | Score de seguridad OWASP (objetivo: 100) |

---

##  Troubleshooting Rápido

### "No puedo acceder"
→ Usa: https://ectus-r-creator.pages.dev/test-demo-connection.html

### "Credenciales inválidas"
→ Verifica: `demo_user` (minúsculas) y `SecureDemo2025!` (exacto)

### "Certificado no autorizado"
→ Solo funciona con: `D:\00001000000702080308.cer` (RFC: MOBF8108153Q5)

### "Error de conexión"
→ Verifica que el backend esté activo:
```bash
curl https://ectus-r-demo.pako-molina.workers.dev/api/demo/status
```

---

##  URLs Completas del Sistema

| Recurso | URL |
|---------|-----|
| **Landing Comercial** | https://ectus-r-creator.pages.dev/landing.html |
| **Demo Privado** | https://ectus-r-creator.pages.dev/demo |
| **Test Conexión** | https://ectus-r-creator.pages.dev/test-demo-connection.html |
| **API Backend** | https://ectus-r-demo.pako-molina.workers.dev/api |
| **Health Check** | https://ectus-r-demo.pako-molina.workers.dev/api/demo/status |

---

##  Para Compartir el Demo

### Mensaje Corto
```
 Ectus-R - Generación de Código con IA

Demo: https://ectus-r-creator.pages.dev/demo
Usuario: demo_user
Password: SecureDemo2025!

Genera código production-ready en segundos.
```

### Mensaje Completo
```
Te invito a probar Ectus-R, nuestra plataforma de ingeniería
de software autónoma con IA.

 Demo en vivo: https://ectus-r-creator.pages.dev/demo

Credenciales:
- Usuario: demo_user
- Contraseña: SecureDemo2025!

Características:
 Código production-ready generado por IA
 Tests automáticos incluidos (95% coverage)
 Soporte multi-lenguaje (Rust, TS, Python, Go)
 100% seguridad OWASP
 Generación en 2-5 segundos

Pruébalo con: "Create a REST API for user authentication"
```

---

##  Quick Start (30 Segundos)

1. **Abre**: https://ectus-r-creator.pages.dev/demo
2. **Login**: `demo_user` / `SecureDemo2025!`
3. **Prompt**: "Create a REST API health check endpoint"
4. **Selecciona**: Rust + Axum
5. **Click**: "Generar Código"
6. **Resultado**: Código + tests en <5 segundos

---

##  Soporte

Si encuentras algún problema:

1. **Test automático**: https://ectus-r-creator.pages.dev/test-demo-connection.html
2. **Guía completa**: Ver `DIAGNOSTICO_CONEXION_DEMO.md`
3. **Verificar backend**: `curl https://ectus-r-demo.pako-molina.workers.dev/api/demo/status`

---

**Status**: 🟢 Sistema Operacional
**Última Actualización**: 2025-09-30 22:15 UTC
**Deployment**: Production (Cloudflare Pages + Workers)
