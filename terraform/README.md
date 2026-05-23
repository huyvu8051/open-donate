# Setup Keycloak via Terraform

Mỗi khi bạn reset Docker (xoá volume `keycloak_db_data`), Keycloak sẽ trở về trạng thái trắng.
Để thiết lập lại Realm `open-donate` và Client `open-donate-web`, bạn chỉ cần chạy Terraform theo các bước sau:

## Điều kiện
- Docker compose đang chạy (`docker-compose up -d`) và Keycloak đã khởi động hoàn toàn ở `http://localhost:8080`.

## Các bước chạy
1. Vào thư mục `terraform`:
   ```bash
   cd terraform
   ```
2. Khởi tạo Terraform (nếu chưa chạy bao giờ):
   ```bash
   terraform init
   ```
3. Apply cấu hình (tự động tạo Realm, Client và ghi đè ra file `.env` ở thư mục gốc):
   ```bash
   terraform apply -auto-approve
   ```

*Lưu ý: Nếu bạn gặp lỗi Terraform báo Resource đã tồn tại (khi chạy lại mà quên xoá Terraform state cũ), bạn có thể chạy `rm -rf terraform.tfstate*` trước khi apply lại từ đầu.*
