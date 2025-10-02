@echo off
REM Start Ectus-R Demo (Frontend Only - Backend Mock Mode)
REM Accessible from external clients

echo ========================================
echo   ECTUS-R DEMO - Starting Frontend
echo ========================================
echo.

echo [1/2] Checking npm dependencies...
cd web-dashboard
if not exist "node_modules\" (
    echo Installing dependencies...
    call npm install
)

echo.
echo [2/2] Starting Vite dev server...
echo.
echo ========================================
echo   DEMO WILL BE ACCESSIBLE AT:
echo   - Local:   http://localhost:3000
echo   - Network: http://YOUR_IP:3000
echo ========================================
echo.
echo Press Ctrl+C to stop the server
echo.

call npm run dev
