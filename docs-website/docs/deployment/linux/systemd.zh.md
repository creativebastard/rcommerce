# 使用 systemd 的 Linux 部署

将 R Commerce 作为 Linux 发行版上的 systemd 服务部署。

## 支持的发行版

- Ubuntu 20.04 LTS+
- Debian 11+
- CentOS/RHEL 8+
- Fedora 35+
- Arch Linux

## 先决条件

```bash
# 更新系统
sudo apt update && sudo apt upgrade -y  # Debian/Ubuntu
sudo dnf update -y                       # RHEL/Fedora

# 安装依赖
sudo apt install -y postgresql redis-server nginx
```

## 安装

### 1. 创建用户

```bash
sudo useradd -r -s /bin/false rcommerce
sudo mkdir -p /opt/rcommerce
sudo chown rcommerce:rcommerce /opt/rcommerce
```

### 2. 下载二进制文件

```bash
# 下载最新版本
cd /opt/rcommerce
sudo -u rcommerce curl -L -o rcommerce \
  https://github.com/captainjez/gocart/releases/latest/download/rcommerce-linux-amd64
sudo -u rcommerce chmod +x rcommerce
```

或从源代码构建：

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 克隆并构建
git clone https://github.com/creativebastard/rcommerce.git
cd gocart
cargo build --release
sudo cp target/release/rcommerce /opt/rcommerce/
sudo chown rcommerce:rcommerce /opt/rcommerce/rcommerce
```

### 3. 配置

```bash
sudo mkdir -p /etc/rcommerce
sudo tee /etc/rcommerce/config.toml << 'EOF'
[server]
host = "127.0.0.1"
port = 8080
worker_threads = 4

[database]
db_type = "Postgres"
host = "localhost"
port = 5432
database = "rcommerce"
username = "rcommerce"
password = "${DB_PASSWORD}"
pool_size = 20

[cache]
cache_type = "Redis"
redis_url = "redis://localhost:6379"

[security]
api_key_prefix_length = 8
api_key_secret_length = 32

[security.jwt]
secret = "${JWT_SECRET}"
expiry_hours = 24

[logging]
level = "info"
format = "json"
EOF

sudo chown -R rcommerce:rcommerce /etc/rcommerce
sudo chmod 600 /etc/rcommerce/config.toml
```

### 4. 环境文件

```bash
sudo tee /etc/rcommerce/environment << 'EOF'
DB_PASSWORD=your_secure_password
JWT_SECRET=your_jwt_secret_min_32_chars
RCOMMERCE_CONFIG=/etc/rcommerce/config.toml
RUST_LOG=info
EOF

sudo chmod 600 /etc/rcommerce/environment
```

### 5. 创建 systemd 服务

```bash
sudo tee /etc/systemd/system/rcommerce.service << 'EOF'
[Unit]
Description=R Commerce Headless E-Commerce Platform
After=network.target postgresql.service redis.service
Wants=postgresql.service redis.service

[Service]
Type=simple
User=rcommerce
Group=rcommerce
WorkingDirectory=/opt/rcommerce
EnvironmentFile=/etc/rcommerce/environment
ExecStart=/opt/rcommerce/rcommerce server
ExecReload=/bin/kill -HUP $MAINPID
Restart=on-failure
RestartSec=5
StandardOutput=journal
StandardError=journal
SyslogIdentifier=rcommerce

# 安全加固
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/opt/rcommerce/uploads
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true

[Install]
WantedBy=multi-user.target
EOF
```

### 6. 数据库设置

```bash
# 创建数据库和用户
sudo -u postgres psql << 'EOF'
CREATE USER rcommerce WITH PASSWORD 'your_secure_password';
CREATE DATABASE rcommerce OWNER rcommerce;
GRANT ALL PRIVILEGES ON DATABASE rcommerce TO rcommerce;
\c rcommerce
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
EOF

# 运行迁移
sudo -u rcommerce /opt/rcommerce/rcommerce migrate
```

### 7. 启动服务

```bash
sudo systemctl daemon-reload
sudo systemctl enable rcommerce
sudo systemctl start rcommerce

# 检查状态
sudo systemctl status rcommerce
sudo journalctl -u rcommerce -f
```

## 反向代理 (nginx)

```bash
sudo tee /etc/nginx/sites-available/rcommerce << 'EOF'
upstream rcommerce {
    server 127.0.0.1:8080;
    keepalive 32;
}

server {
    listen 80;
    server_name api.yourstore.com;
    
    location / {
        return 301 https://$server_name$request_uri;
    }
}

server {
    listen 443 ssl http2;
    server_name api.yourstore.com;
    
    ssl_certificate /etc/letsencrypt/live/api.yourstore.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/api.yourstore.com/privkey.pem;
    
    client_max_body_size 50M;
    
    location / {
        proxy_pass http://rcommerce;
        proxy_http_version 1.1;
        proxy_set_header Connection "";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_buffering off;
        proxy_read_timeout 300s;
    }
}
EOF

sudo ln -s /etc/nginx/sites-available/rcommerce /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

## 管理命令

```bash
# 查看日志
sudo journalctl -u rcommerce -f

# 重启服务
sudo systemctl restart rcommerce

# 检查配置
sudo -u rcommerce /opt/rcommerce/rcommerce config

# 数据库状态
sudo -u rcommerce /opt/rcommerce/rcommerce db status
```

## 故障排除

| 问题 | 解决方案 |
|-------|----------|
| 服务无法启动 | 检查 `journalctl -u rcommerce` 获取错误 |
| 数据库连接失败 | 验证凭据和 PostgreSQL 状态 |
| 权限被拒绝 | 检查文件所有权和权限 |
| 端口已被占用 | 在配置中更改端口或停止冲突的服务 |

## 安全加固

1. **防火墙**: 仅允许必要的端口
2. **Fail2ban**: 防止暴力破解
3. **更新**: 保持系统包更新
4. **备份**: 定期数据库备份
5. **SSL**: 使用 Let's Encrypt 获取证书
