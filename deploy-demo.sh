#!/bin/bash
# Deploy AION-R Demo to creator.avermex.com
# This script builds and deploys both the frontend and Cloudflare Worker

set -e  # Exit on error

echo "========================================="
echo "  AION-R Demo Deployment Script"
echo "========================================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Step 1: Build Frontend
echo -e "${YELLOW}[1/4]${NC} Building React frontend..."
cd web-dashboard

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "Installing npm dependencies..."
    npm install
fi

# Build for production
echo "Running production build..."
REACT_APP_API_URL=https://ectus-r-saas.pako-molina.workers.dev npm run build

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓${NC} Frontend build successful"
else
    echo -e "${RED}✗${NC} Frontend build failed"
    exit 1
fi

cd ..

# Step 2: Deploy Cloudflare Worker (Backend API)
echo ""
echo -e "${YELLOW}[2/4]${NC} Deploying Cloudflare Worker..."

# Check if wrangler is installed
if ! command -v wrangler &> /dev/null; then
    echo -e "${RED}✗${NC} Wrangler CLI not found. Installing..."
    npm install -g wrangler
fi

# Deploy worker
echo "Deploying to Cloudflare Workers..."
wrangler deploy

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓${NC} Worker deployed successfully"
    echo "   URL: https://ectus-r-saas.pako-molina.workers.dev"
else
    echo -e "${RED}✗${NC} Worker deployment failed"
    exit 1
fi

# Step 3: Deploy Frontend to GitHub Pages
echo ""
echo -e "${YELLOW}[3/4]${NC} Deploying frontend to GitHub Pages..."

# Create docs directory if it doesn't exist
mkdir -p docs

# Copy build files to docs
echo "Copying build files..."
cp -r web-dashboard/dist/* docs/

# Create CNAME file for custom domain
echo "creator.avermex.com" > docs/CNAME

# Add .nojekyll to prevent Jekyll processing
touch docs/.nojekyll

# Git operations
echo "Committing and pushing to GitHub..."
git add docs/
git add web-dashboard/

git commit -m "DEPLOY: Update demo to GitHub Pages and Cloudflare Worker

Deployment Details:
- Frontend: creator.avermex.com (via GitHub Pages)
- Backend API: ectus-r-saas.pako-molina.workers.dev
- Build Time: $(date)
- Auto-deployed via deploy-demo.sh

Changes:
- Updated production build with API URL
- Deployed latest Cloudflare Worker
- Synced with custom domain

🤖 Automated Deployment" || echo "No changes to commit"

git push origin main

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓${NC} Pushed to GitHub successfully"
else
    echo -e "${RED}✗${NC} Git push failed"
    exit 1
fi

# Step 4: Verify Deployment
echo ""
echo -e "${YELLOW}[4/4]${NC} Verifying deployment..."

echo "Testing frontend..."
sleep 2
STATUS_CODE=$(curl -s -o /dev/null -w "%{http_code}" https://creator.avermex.com/ || echo "000")

if [ "$STATUS_CODE" = "200" ] || [ "$STATUS_CODE" = "301" ] || [ "$STATUS_CODE" = "302" ]; then
    echo -e "${GREEN}✓${NC} Frontend is accessible (HTTP $STATUS_CODE)"
else
    echo -e "${YELLOW}⚠${NC} Frontend returned HTTP $STATUS_CODE (may take a few minutes to propagate)"
fi

echo "Testing backend API..."
API_STATUS=$(curl -s -o /dev/null -w "%{http_code}" https://ectus-r-saas.pako-molina.workers.dev/health || echo "000")

if [ "$API_STATUS" = "200" ]; then
    echo -e "${GREEN}✓${NC} Backend API is healthy"
else
    echo -e "${YELLOW}⚠${NC} Backend API returned HTTP $API_STATUS"
fi

# Summary
echo ""
echo "========================================="
echo -e "${GREEN}  Deployment Complete!${NC}"
echo "========================================="
echo ""
echo "Access your demo at:"
echo "  🌐 Frontend:  https://creator.avermex.com/"
echo "  ⚙️  Backend:   https://ectus-r-saas.pako-molina.workers.dev"
echo ""
echo "Demo Features:"
echo "  • AI-powered code generation"
echo "  • Autonomous QA testing"
echo "  • Real-time refactoring"
echo "  • Multi-LLM support (Groq, OpenAI, etc.)"
echo ""
echo "Next Steps:"
echo "  1. Wait 2-5 minutes for GitHub Pages to update"
echo "  2. Visit https://creator.avermex.com/"
echo "  3. Test API connection in browser console"
echo "  4. Share demo link with clients!"
echo ""
echo "Troubleshooting:"
echo "  • Check deployment: gh browse"
echo "  • View worker logs: wrangler tail"
echo "  • Test API: curl https://ectus-r-saas.pako-molina.workers.dev/health"
echo ""
