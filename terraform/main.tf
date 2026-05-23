terraform {
  required_providers {
    keycloak = {
      source  = "mrparkers/keycloak"
      version = ">= 4.0.0"
    }
    local = {
      source = "hashicorp/local"
      version = ">= 2.0.0"
    }
  }
}

provider "keycloak" {
  client_id = "admin-cli"
  url       = "http://localhost:8080"
  username  = "admin"
  password  = "admin"
}

resource "keycloak_realm" "open_donate" {
  realm                       = "open-donate"
  enabled                     = true
  display_name                = "Open Donate"
  ssl_required                = "none"
  
  # Registration settings
  registration_allowed        = true
  registration_email_as_username = true
  verify_email                = false
  remember_me                 = true
  reset_password_allowed      = true
  
  # Optimization for dev/test
  sso_session_idle_timeout    = "24h"
  sso_session_max_lifespan    = "168h"

  # Enable declarative user profile
  attributes = {
    userProfileEnabled = "true"
  }
}

resource "keycloak_realm_user_profile" "open_donate_profile" {
  realm_id = keycloak_realm.open_donate.id

  attribute {
    name = "username"
    display_name = "$${ro.username}"
    permissions {
      view = ["admin", "user"]
      edit = ["admin", "user"]
    }
  }
  
  attribute {
    name = "email"
    display_name = "$${email}"
    required_for_roles = ["user"]
    permissions {
      view = ["admin", "user"]
      edit = ["admin", "user"]
    }
  }

  # First name and Last name are NOT required for user role, 
  # and we remove them from the registration screen by removing permissions or making them not required
}

resource "keycloak_openid_client" "open_donate_web" {
  realm_id                     = keycloak_realm.open_donate.id
  client_id                    = "open-donate-web"
  name                         = "Open Donate Web App"
  enabled                      = true
  
  access_type                  = "CONFIDENTIAL"
  standard_flow_enabled        = true
  implicit_flow_enabled        = false
  direct_access_grants_enabled = false
  
  valid_redirect_uris = [
    "http://localhost:3000/api/auth/callback"
  ]
  
  web_origins = [
    "+"
  ]
}

resource "local_file" "env_file" {
  filename = "../.env"
  content  = <<EOF
OIDC_ISSUER=http://localhost:8080/realms/open-donate
OIDC_CLIENT_ID=${keycloak_openid_client.open_donate_web.client_id}
OIDC_CLIENT_SECRET=${keycloak_openid_client.open_donate_web.client_secret}
OIDC_REDIRECT_URI=http://localhost:3000/api/auth/callback
DATABASE_URL=postgres://postgres:postgres@localhost:5432/open_donate
EOF
}
