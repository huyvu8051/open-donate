#!/bin/sh
set -e

echo "Waiting for ZITADEL to become healthy..."
until curl -s -f http://zitadel:8080/debug/healthz; do
  sleep 2
done
echo "ZITADEL is healthy."

# Wait for PAT file to exist
echo "Waiting for PAT file..."
until [ -f /shared/setup-bot-pat.txt ]; do
  sleep 1
done
PAT=$(cat /shared/setup-bot-pat.txt)
echo "PAT token read successfully."

# Try to create Project
echo "Creating project 'open-donate'..."
PROJECT_RESP=$(curl -s -X POST \
  -H "Authorization: Bearer $PAT" \
  -H "Content-Type: application/json" \
  -H "Connect-Protocol-Version: 1" \
  -H "Host: localhost:8080" \
  -d '{"name": "open-donate"}' \
  http://zitadel:8080/zitadel.project.v2.ProjectService/CreateProject || true)

echo "Project response: $PROJECT_RESP"
PROJECT_ID=$(echo "$PROJECT_RESP" | jq -r '.projectId // empty')

if [ -z "$PROJECT_ID" ] || [ "$PROJECT_ID" = "null" ]; then
  echo "Project might already exist. Listing projects..."
  LIST_RESP=$(curl -s -X POST \
    -H "Authorization: Bearer $PAT" \
    -H "Content-Type: application/json" \
    -H "Connect-Protocol-Version: 1" \
    -H "Host: localhost:8080" \
    -d '{}' \
    http://zitadel:8080/zitadel.project.v2.ProjectService/ListProjects)
  
  PROJECT_ID=$(echo "$LIST_RESP" | jq -r '.result[] | select(.name == "open-donate") | .id')
fi

if [ -z "$PROJECT_ID" ] || [ "$PROJECT_ID" = "null" ]; then
  echo "Error: Could not find or create project 'open-donate'."
  exit 1
fi
echo "Project ID is: $PROJECT_ID"

# Check if application already exists
echo "Checking if app 'open-donate-web' already exists..."
APP_LIST_RESP=$(curl -s -X POST \
  -H "Authorization: Bearer $PAT" \
  -H "Content-Type: application/json" \
  -H "Connect-Protocol-Version: 1" \
  -H "Host: localhost:8080" \
  -d '{"projectId": "'"$PROJECT_ID"'"}' \
  http://zitadel:8080/zitadel.application.v2.ApplicationService/ListApps || true)

CLIENT_ID=$(echo "$APP_LIST_RESP" | jq -r '.result[] | select(.name == "open-donate-web") | .id // empty')

if [ -n "$CLIENT_ID" ] && [ "$CLIENT_ID" != "null" ]; then
  echo "App 'open-donate-web' already exists. Client ID: $CLIENT_ID"
  # Note: Client Secret cannot be retrieved again, so if we are re-running on an existing DB,
  # we might have to recreate the app or recreate its client secret if we don't have it.
  # To keep it simple and idempotent for development: we can delete the existing app and recreate it,
  # so we get a fresh Client Secret!
  echo "Deleting existing app to rotate client secret..."
  curl -s -X POST \
    -H "Authorization: Bearer $PAT" \
    -H "Content-Type: application/json" \
    -H "Connect-Protocol-Version: 1" \
    -H "Host: localhost:8080" \
    -d '{"projectId": "'"$PROJECT_ID"'", "appId": "'"$CLIENT_ID"'"}' \
    http://zitadel:8080/zitadel.application.v2.ApplicationService/RemoveApp
fi

# Create OIDC Application
echo "Creating OIDC application 'open-donate-web'..."
APP_RESP=$(curl -s -X POST \
  -H "Authorization: Bearer $PAT" \
  -H "Content-Type: application/json" \
  -H "Connect-Protocol-Version: 1" \
  -H "Host: localhost:8080" \
  -d '{
    "projectId": "'"$PROJECT_ID"'",
    "name": "open-donate-web",
    "redirectUris": ["http://localhost:3000/api/auth/callback"],
    "responseTypes": ["OIDC_RESPONSE_TYPE_CODE"],
    "grantTypes": ["OIDC_GRANT_TYPE_AUTHORIZATION_CODE"],
    "appType": "OIDC_APP_TYPE_WEB",
    "authMethodType": "OIDC_AUTH_METHOD_TYPE_BASIC"
  }' \
  http://zitadel:8080/zitadel.application.v2.ApplicationService/AddOIDCApplication)

echo "App response: $APP_RESP"
CLIENT_ID=$(echo "$APP_RESP" | jq -r '.clientId')
CLIENT_SECRET=$(echo "$APP_RESP" | jq -r '.clientSecret')

if [ -z "$CLIENT_ID" ] || [ "$CLIENT_ID" = "null" ]; then
  echo "Error: Failed to create OIDC application."
  exit 1
fi

echo "Successfully created OIDC application."
echo "Client ID: $CLIENT_ID"

# Write to host .env file
cat <<EOF > /app/.env
ZITADEL_ISSUER=http://localhost:8080
ZITADEL_CLIENT_ID=$CLIENT_ID
ZITADEL_CLIENT_SECRET=$CLIENT_SECRET
ZITADEL_REDIRECT_URI=http://localhost:3000/api/auth/callback
EOF

echo ".env file generated successfully."
