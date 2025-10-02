#!/bin/bash
# Script to add custom domains to Cloudflare Pages via API

ACCOUNT_ID="b11ab3fe6c1a3625b65cb22d170793b6"
PROJECT_NAME="ectus-r-creator"

# Get API token from wrangler config
CF_API_TOKEN=$(cat ~/.wrangler/config/default.toml 2>/dev/null | grep oauth_token | cut -d'"' -f2)

if [ -z "$CF_API_TOKEN" ]; then
    echo "Error: No OAuth token found"
    exit 1
fi

# Add ectus.avermex.com
echo "Adding ectus.avermex.com to $PROJECT_NAME..."
curl -X POST "https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/pages/projects/$PROJECT_NAME/domains" \
  -H "Authorization: Bearer $CF_API_TOKEN" \
  -H "Content-Type: application/json" \
  --data '{"name":"ectus.avermex.com"}'

echo ""
echo "Adding creator.avermex.com to $PROJECT_NAME..."
curl -X POST "https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/pages/projects/$PROJECT_NAME/domains" \
  -H "Authorization: Bearer $CF_API_TOKEN" \
  -H "Content-Type: application/json" \
  --data '{"name":"creator.avermex.com"}'

echo ""
echo "Done! Custom domains added."
