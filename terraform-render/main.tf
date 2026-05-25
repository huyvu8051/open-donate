terraform {
  required_providers {
    render = {
      source  = "render-oss/render"
      version = "~> 1.8.0"
    }
  }
}

# Thay đổi các biến môi trường RENDER_API_KEY và RENDER_OWNER_ID khi deploy
# provider "render" {
#   api_key  = var.render_api_key
#   owner_id = var.render_owner_id
# }

# Thêm cấu hình render_web_service của bạn ở đây...
