# 使用 launchd 的 macOS 部署

使用 launchd 进行服务管理，在 macOS 上部署 R Commerce。

## 使用场景

- macOS 上的开发环境
- Mac 服务器上的小规模生产
- 云部署前的本地测试
- 基于 macOS 的 CI/CD 运行器

## 先决条件

```bash
# 如果尚未安装 Homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 安装依赖
brew install postgresql@15 redis

# 启动服务
brew services start postgresql@15
brew services start redis
```

## 安装

### 1. 创建目录

```bash
sudo mkdir -p /usr/local/rcommerce
sudo mkdir -p /usr/local/etc/rcommerce
sudo mkdir -p /usr/local/var/log/rcommerce
sudo mkdir -p /usr/local/var/rcommerce/uploads
```

### 2. 下载二进制文件

```bash
# 适用于 Apple Silicon (M1/M2/M3)
curl -L -o /usr/local/rcommerce/rcommerce \
  https://github.com/captainjez/gocart/releases/latest/download/rcommerce-darwin-arm64

# 适用于 Intel Macs
curl -L -o /usr/local/rcommerce/rcommerce \
  https://github.com/captainjez/gocart/releases/latest/download/rcommerce-darwin-amd64

chmod +x /usr/local/rcommerce/rcommerce
```

或从源代码构建：

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 克隆并构建
git clone https://github.com/creativebastard/rcommerce.git
cd gocart
cargo build --release
sudo cp target/release/rcommerce /usr/local/rcommerce/
```

### 3. 配置

```bash
sudo tee /usr/local/etc/rcommerce/config.toml << 'EOF'
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
pool_size = 10

[cache]
cache_type = "Redis"
redis_url = "redis://localhost:6379"

[security.jwt]
secret = "${JWT_SECRET}"
expiry_hours = 24

[media]
storage_type = "Local"
local_path = "/usr/local/var/rcommerce/uploads"

[logging]
level = "info"
format = "json"
EOF
```

### 4. 环境文件

```bash
sudo tee /usr/local/etc/rcommerce/environment << 'EOF'
DB_PASSWORD=your_secure_password
JWT_SECRET=your_jwt_secret_min_32_chars
RCOMMERCE_CONFIG=/usr/local/etc/rcommerce/config.toml
RUST_LOG=info
EOF

sudo chmod 600 /usr/local/etc/rcommerce/environment
```

### 5. 创建 launchd Plist

创建 `/Library/LaunchDaemons/com.rcommerce.rcommerce.plist`：

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.rcommerce.rcommerce</string>
    
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/rcommerce/rcommerce</string>
        <string>server</string>
    </array>
    
    <key>EnvironmentVariables</key>
    <dict>
        <key>RCOMMERCE_CONFIG</key>
        <string>/usr/local/etc/rcommerce/config.toml</string>
        <key>RUST_LOG</key>
        <string>info</string>
    </dict>
    
    <key>WorkingDirectory</key>
    <string>/usr/local/rcommerce</string>
    
    <key>StandardOutPath</key>
    <string>/usr/local/var/log/rcommerce/output.log</string>
    
    <key>StandardErrorPath</key>
    <string>/usr/local/var/log/rcommerce/error.log</string>
    
    <key>KeepAlive</key>
    <true/>
    
    <key>RunAtLoad</key>
    <true/>
    
    <key>ThrottleInterval</key>
    <integer>5</integer>
    
    <key>ProcessType</key>
    <string>Background</string>
    
    <key>UserName</key>
    <string>_rcommerce</string>
    
    <key>GroupName</key>
    <string>_rcommerce</string>
    
    <key>SoftResourceLimits</key>
    <dict>
        <key>NumberOfFiles</key>
        <integer>65536</integer>
    </dict>
</dict>
</plist>
```

### 6. 创建用户

```bash
# 创建系统用户
sudo dscl . -create /Users/_rcommerce
sudo dscl . -create /Users/_rcommerce UserShell /usr/bin/false
sudo dscl . -create /Users/_rcommerce NFSHomeDirectory /usr/local/rcommerce
sudo dscl . -create /Users/_rcommerce PrimaryGroupID 80

# 设置所有权
sudo chown -R _rcommerce:_rcommerce /usr/local/rcommerce
sudo chown -R _rcommerce:_rcommerce /usr/local/var/rcommerce
```

### 7. 数据库设置

```bash
# 创建数据库
createuser -s rcommerce
createdb -O rcommerce rcommerce

# 设置密码
psql -c "ALTER USER rcommerce WITH PASSWORD 'your_secure_password';"

# 运行迁移
sudo -u _rcommerce /usr/local/rcommerce/rcommerce migrate
```

### 8. 启动服务

```bash
# 加载并启动
sudo launchctl load /Library/LaunchDaemons/com.rcommerce.rcommerce.plist
sudo launchctl start com.rcommerce.rcommerce

# 检查状态
sudo launchctl list | grep rcommerce

# 查看日志
tail -f /usr/local/var/log/rcommerce/output.log
tail -f /usr/local/var/log/rcommerce/error.log
```

## 反向代理 (nginx)

通过 Homebrew 安装 nginx：

```bash
brew install nginx

# 创建配置
cat > /opt/homebrew/etc/nginx/servers/rcommerce.conf << 'EOF'
upstream rcommerce {
    server 127.0.0.1:8080;
    keepalive 32;
}

server {
    listen 80;
    server_name localhost;
    
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
    }
}
EOF

brew services start nginx
```

## 服务管理

```bash
# 启动
sudo launchctl start com.rcommerce.rcommerce

# 停止
sudo launchctl stop com.rcommerce.rcommerce

# 重启
sudo launchctl stop com.rcommerce.rcommerce
sudo launchctl start com.rcommerce.rcommerce

# 卸载
sudo launchctl unload /Library/LaunchDaemons/com.rcommerce.rcommerce.plist

# 更改后重新加载
sudo launchctl unload /Library/LaunchDaemons/com.rcommerce.rcommerce.plist
sudo launchctl load /Library/LaunchDaemons/com.rcommerce.rcommerce.plist
```

## 故障排除

| 问题 | 解决方案 |
|-------|----------|
| 服务无法启动 | 检查 Console.app 获取错误 |
| 权限被拒绝 | 使用 `_rcommerce` 用户验证文件所有权 |
| 端口已被占用 | 更改端口或停止冲突的服务 |
| 数据库连接失败 | 检查 PostgreSQL 是否正在运行：`brew services list` |

## 开发设置

对于本地开发，直接使用而不使用 launchd：

```bash
# 在项目目录中
cargo run -- server

# 或使用热重载
cargo watch -x "run -- server"
```
