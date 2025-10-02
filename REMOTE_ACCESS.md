# Remote Access Guide - Ectus-R Demo

## Quick Start: Access from Client Terminal

### Server Setup (Run Once)

```bash
# Navigate to project directory
cd C:\Users\Propietario\Ectus-R

# Start the demo server
start-demo.bat
```

The server will start on port **3000** and be accessible from the network.

### Find Your Server IP

**Option 1 - PowerShell:**
```powershell
(Get-NetIPAddress -AddressFamily IPv4 | Where-Object {$_.InterfaceAlias -notlike "*Loopback*"}).IPAddress
```

**Option 2 - Command Prompt:**
```cmd
ipconfig | findstr /i "IPv4"
```

**Current Network IPs** (detected at startup):
- `192.168.1.181:3000`
- `169.254.7.215:3000`

### Client Access

From any terminal/browser on the same network:

```bash
# Using curl
curl http://192.168.1.181:3000

# Using wget
wget http://192.168.1.181:3000

# Using browser
open http://192.168.1.181:3000
```

## Architecture

### Current Configuration

```
Client Terminal          Network          Demo Server
    │                      │                   │
    │── HTTP Request ──────┼────────────►      │
    │   (port 3000)        │              Vite Dev Server
    │                      │              (0.0.0.0:3000)
    │                      │                   │
    │◄─── HTML/JS ─────────┼───────────────────│
    │   (React App)        │                   │
    │                      │                   │
    │   API calls          │              API Client
    │   (mock mode)        │              (fallback data)
```

### Features Available

- ✅ **Dashboard**: Real-time metrics (mock data)
- ✅ **Project Management**: Full CRUD operations (mock)
- ✅ **Marketplace**: Browse plugins and templates (mock)
- ✅ **Code Editor**: Monaco editor integration
- ✅ **Analytics**: Charts and visualizations

### Fallback Mode

The frontend operates in **graceful degradation mode**:
- All UI components fully functional
- Mock data simulates real backend responses
- No backend required for demo purposes
- Production API can be connected later

## Firewall Configuration

### Windows Firewall (if blocked)

```powershell
# Allow inbound connections on port 3000
New-NetFirewallRule -DisplayName "Ectus-R Demo" -Direction Inbound -LocalPort 3000 -Protocol TCP -Action Allow
```

### Linux Firewall (ufw)

```bash
sudo ufw allow 3000/tcp
sudo ufw reload
```

## Production Deployment

For production with real backend:

### 1. Backend Setup

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build backend (Linux/macOS)
cd crates/aion-web-api
cargo build --release

# Start backend
./target/release/aion-web-api
```

Backend will run on **port 8080** by default.

### 2. Frontend Setup

The frontend is already configured to proxy API calls to `localhost:8080`.

```bash
cd web-dashboard
npm run build       # Production build
npm run preview     # Serve production build
```

### 3. Docker Deployment (Recommended)

```bash
# Start all services (Postgres, Redis, Backend, Frontend)
docker compose -f docker-compose.production.yml up -d
```

Services:
- **Frontend**: Port 3000
- **Backend API**: Port 8080
- **PostgreSQL**: Port 5432
- **Redis**: Port 6379
- **Prometheus**: Port 9090
- **Grafana**: Port 3000 (conflicts, use 3001)

## Troubleshooting

### Cannot connect from client

1. **Check server is running:**
   ```bash
   curl http://localhost:3000
   ```

2. **Verify firewall allows port 3000:**
   ```powershell
   Get-NetFirewallRule -DisplayName "*Ectus*"
   ```

3. **Check server IP is correct:**
   ```bash
   ipconfig
   ```

4. **Test from server first:**
   ```bash
   curl http://192.168.1.181:3000
   ```

### Port 3000 already in use

```bash
# Find process using port 3000
netstat -ano | findstr :3000

# Kill process (replace PID)
taskkill /PID <PID> /F
```

### Slow connection

The Vite dev server includes HMR (Hot Module Replacement) which can be slower over network. Use production build for better performance:

```bash
npm run build
npm run preview
```

## API Endpoints (When Backend Running)

```
GET    /api/health              - Health check
GET    /api/projects            - List projects
POST   /api/projects            - Create project
GET    /api/projects/:id        - Get project
PUT    /api/projects/:id        - Update project
DELETE /api/projects/:id        - Delete project
POST   /api/projects/:id/deploy - Deploy project
GET    /api/projects/:id/logs   - Get logs
POST   /api/ai/generate         - Generate code
POST   /api/qa/run              - Run QA tests
POST   /api/refactor/apply      - Apply refactoring
GET    /api/analytics/:id       - Get analytics
WS     /ws                      - WebSocket connection
```

## Security Notes

### Development Mode (Current)
- ⚠️ No authentication required
- ⚠️ All requests allowed (CORS: *)
- ⚠️ Debug logging enabled
- ⚠️ Suitable for trusted networks only

### Production Mode
- ✅ JWT authentication required
- ✅ Rate limiting enabled
- ✅ CORS restricted to specific origins
- ✅ HTTPS/TLS encryption
- ✅ SQL injection protection
- ✅ XSS/CSRF protection

**Never expose development server to public internet.**

## Support

For issues or questions:
- GitHub: https://github.com/Yatrogenesis/Ectus-R
- Documentation: `PROJECT_STATUS_REPORT.md`
- Security: `SECURITY_AUDIT_REPORT_FINAL.md`

---

**Status**: ✅ Ready for network access
**Mode**: Development (Mock Data)
**Last Updated**: 2025-10-01
