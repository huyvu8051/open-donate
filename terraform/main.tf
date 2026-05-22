terraform {
  required_providers {
    zitadel = {
      source  = "zitadel/zitadel"
      version = "~> 2.0"
    }
  }
}

variable "zitadel_token" {
  type      = string
  sensitive = true
}

provider "zitadel" {
  domain       = "zitadel"
  port         = "8080"
  insecure     = true
  access_token = var.zitadel_token

  transport_headers = {
    "Host"                  = "localhost:8080"
    "x-zitadel-public-host" = "localhost:8080"
  }
}

resource "zitadel_project" "open_donate" {
  name = "open-donate"
}

resource "zitadel_application_oidc" "open_donate_web" {
  project_id                  = zitadel_project.open_donate.id
  name                        = "open-donate-web"
  redirect_uris               = ["http://localhost:3000/api/auth/callback"]
  response_types              = ["OIDC_RESPONSE_TYPE_CODE"]
  grant_types                 = ["OIDC_GRANT_TYPE_AUTHORIZATION_CODE"]
  app_type                    = "OIDC_APP_TYPE_WEB"
  auth_method_type            = "OIDC_AUTH_METHOD_TYPE_POST"
  post_logout_redirect_uris   = ["http://localhost:3000"]
}

resource "local_file" "env_file" {
  filename = "/app/.env"
  content  = <<EOF
ZITADEL_ISSUER=http://localhost:8080
ZITADEL_CLIENT_ID=${zitadel_application_oidc.open_donate_web.client_id}
ZITADEL_CLIENT_SECRET=${zitadel_application_oidc.open_donate_web.client_secret}
ZITADEL_REDIRECT_URI=http://localhost:3000/api/auth/callback
DATABASE_URL=postgres://postgres:postgres@localhost:5432/open_donate
EOF
}
