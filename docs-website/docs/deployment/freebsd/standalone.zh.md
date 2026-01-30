# FreeBSD 独立部署

直接在 FreeBSD 上部署 R Commerce，无需使用 jails。这是单服务器设置最简单的部署方法。

## 支持的 FreeBSD 版本

- **FreeBSD 14.2** - 最新生产版本（推荐）
- **FreeBSD 15.0** - 当前稳定分支

## 何时使用独立部署

- **单应用服务器** - 不需要隔离
- **开发环境** - 快速设置进行测试
- **资源限制** - 避免 jail 开销
- **简单基础设施** - 更易于管理

## 先决条件

```bash
# FreeBSD 14.0 或更高版本
freebsd-version

# Root 或 sudo 访问
# 互联网连接用于软件包安装
```

## 安装

### 1. 系统准备

```bash
# 更新系统软件包
pkg update
pkg upgrade -y

# 安装所需软件包
pkg install -y \
  postgresql15-server \
  postgresql15-client \
  redis \
  nginx \
  ca_root_nss \
  curl \
  sudo

# 启用开机启动服务
tee -a /etc/rc.conf << 'EOF'
postgresql_enable="YES"
redis_enable="YES"
nginx_enable="YES"
EOF
```

### 2. 数据库设置

```bash
# 初始化 PostgreSQL
service postgresql initdb

# 启动 PostgreSQL
service postgresql start

# 创建数据库和用户
su - postgres << 'EOF'
createdb rcommerce
createuser -P rcommerce
# 按提示输入密码
psql -c "GRANT ALL PRIVILEGES ON DATABASE rcommerce TO rcommerce;"
EOF
```

### 3. Redis 设置

```bash
# 启动 Redis
service redis start

# 验证 Redis 正在运行
redis-cli ping
# 应返回: PONG
```

### 4. 创建 R Commerce 用户

```bash
# 创建专用用户
pw useradd -n rcommerce -s /bin/sh -d /usr/local/rcommerce -m

# 添加到 sudoers 进行维护（可选）
echo "rcommerce ALL=(ALL) NOPASSWD: /usr/sbin/service rcommerce *" >> /usr/local/etc/sudoers.d/rcommerce
```

### 5. 安装 R Commerce

```bash
# 下载最新版本
curl -L -o /usr/local/bin/rcommerce \
  "https://github.com/creativebastard/rcommerce/releases/latest/download/rcommerce-freebsd-amd64"

# 使其可执行
chmod +x /usr/local/bin/rcommerce

# 验证安装
rcommerce --version
```

### 6. 配置

```bash
# 创建配置目录
mkdir -p /usr/local/etc/rcommerce

# 创建配置文件
cat > /usr/local/etc/rcommerce/config.toml << 'EOF'
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
password = "your_secure_password"
pool_size = 20

[cache]
cache_type = "Redis"
redis_url = "redis://127.0.0.1:6379"

[logging]
level = "info"
format = "Json"

[media]
storage_type = "Local"
local_path = "/usr/local/rcommerce/uploads"
EOF

# 设置权限
chown -R rcommerce:rcommerce /usr/local/etc/rcommerce
chmod 600 /usr/local/etc/rcommerce/config.toml

# 创建上传目录
mkdir -p /usr/local/rcommerce/uploads
chown -R rcommerce:rcommerce /usr/local/rcommerce
```

### 7. rc.d 服务脚本

创建 `/usr/local/etc/rc.d/rcommerce`：

```sh
#!/bin/sh
# PROVIDE: rcommerce
# REQUIRE: postgresql redis
# KEYWORD: shutdown

. /etc/rc.subr

name="rcommerce"
rcvar="rcommerce_enable"

load_rc_config $name

: ${rcommerce_enable:="NO"}
: ${rcommerce_config:="/usr/local/etc/rcommerce/config.toml"}
: ${rcommerce_user:="rcommerce"}
: ${rcommerce_group:="rcommerce"}
: ${rcommerce_dir:="/usr/local/rcommerce"}

command="/usr/local/bin/rcommerce"
procname="/usr/local/bin/rcommerce"
pidfile="/var/run/${name}.pid"

start_cmd="rcommerce_start"
stop_cmd="rcommerce_stop"
status_cmd="rcommerce_status"
reload_cmd="rcommerce_reload"

rcommerce_start() {
    echo "Starting ${name}."
    export RCOMMERCE_CONFIG=${rcommerce_config}
    cd ${rcommerce_dir}
    /usr/sbin/daemon -f -p ${pidfile} -u ${rcommerce_user} \
        ${command} server
}

rcommerce_stop() {
    echo "Stopping ${name}."
    if [ -f ${pidfile} ]; then
        kill $(cat ${pidfile})
        rm -f ${pidfile}
    fi
}

rcommerce_status() {
    if [ -f ${pidfile} ] && kill -0 $(cat ${pidfile}) 2>/dev/null; then
        echo "${name} is running as pid $(cat ${pidfile})."
    else
        echo "${name} is not running."
        return 1
    fi
}

rcommerce_reload() {
    echo "Reloading ${name} configuration..."
    if [ -f ${pidfile} ]; then
        kill -HUP $(cat ${pidfile})
    else
        echo "${name} is not running."
        return 1
    fi
}

run_rc_command "$1"
```

启用服务：

```bash
chmod +x /usr/local/etc/rc.d/rcommerce
sysrc rcommerce_enable=YES
```

### 8. Nginx 反向代理

```bash
cat > /usr/local/etc/nginx/nginx.conf << 'EOF'
user www;
worker_processes auto;
error_log /var/log/nginx/error.log;
pid /var/run/nginx.pid;

events {
    worker_connections 1024;
    use kqueue;
}

http {
    include mime.types;
    default_type application/octet-stream;

    log_format main '$remote_addr - $remote_user [$time_local] "$request" '
                    '$status $body_bytes_sent "$http_referer" '
                    '"$http_user_agent" "$http_x_forwarded_for"';

    access_log /var/log/nginx/access.log main;

    sendfile on;
    tcp_nopush on;
    tcp_nodelay on;
    keepalive_timeout 65;

    # Gzip 压缩
    gzip on;
    gzip_vary on;
    gzip_proxied any;
    gzip_comp_level 6;
    gzip_types text/plain text/css text/xml application/json application/javascript application/rss+xml application/atom+xml image/svg+xml;

    # 速率限制
    limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;
    limit_req_zone $binary_remote_addr zone=login:10m rate=1r/s;

    upstream rcommerce {
        server 127.0.0.1:8080;
        keepalive 32;
    }

    server {
        listen 80;
        server_name _;
        
        # 安全头
        add_header X-Frame-Options "SAMEORIGIN" always;
        add_header X-Content-Type-Options "nosniff" always;
        add_header X-XSS-Protection "1; mode=block" always;
        add_header Referrer-Policy "strict-origin-when-cross-origin" always;

        # 健康检查端点（无速率限制）
        location /health {
            proxy_pass http://rcommerce;
            proxy_http_version 1.1;
            proxy_set_header Connection "";
        }

        # 带速率限制的 API 端点
        location /api/ {
            limit_req zone=api burst=20 nodelay;
            
            proxy_pass http://rcommerce;
            proxy_http_version 1.1;
            proxy_set_header Connection "";
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
            
            proxy_connect_timeout 30s;
            proxy_send_timeout 30s;
            proxy_read_timeout 30s;
        }

        # 静态文件
        location /uploads/ {
            alias /usr/local/rcommerce/uploads/;
            expires 1y;
            add_header Cache-Control "public, immutable";
        }

        # 默认位置
        location / {
            proxy_pass http://rcommerce;
            proxy_http_version 1.1;
            proxy_set_header Connection "";
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }
    }
}
EOF

# 测试 nginx 配置
nginx -t

# 启动 nginx
service nginx start
```

### 9. 启动 R Commerce

```bash
# 启动服务
service rcommerce start

# 检查状态
service rcommerce status

# 查看日志
tail -f /var/log/rcommerce.log
```

## 使用 Let's Encrypt 的 SSL/TLS

```bash
# 安装 certbot
pkg install py39-certbot

# 获取证书
certbot certonly --standalone -d yourdomain.com

# 更新 nginx 配置以支持 SSL
# （请参阅反向代理文档获取完整 SSL 配置）
```

## 监控

```bash
# 安装监控工具
pkg install monit

# 创建 monit 配置
cat > /usr/local/etc/monit.d/rcommerce << 'EOF'
check process rcommerce matching "/usr/local/bin/rcommerce"
    start program = "/usr/sbin/service rcommerce start"
    stop program = "/usr/sbin/service rcommerce stop"
    if failed host 127.0.0.1 port 8080 protocol http
        request /health
        with timeout 10 seconds
        then restart
    if 5 restarts within 5 cycles then timeout
EOF

sysrc monit_enable=YES
service monit start
```

## 备份策略

```bash
# 数据库备份
pg_dump -U rcommerce rcommerce > /backup/rcommerce-$(date +%Y%m%d).sql

# 文件备份
tar -czf /backup/rcommerce-files-$(date +%Y%m%d).tar.gz /usr/local/rcommerce/uploads

# 自动备份脚本
cat > /usr/local/bin/backup-rcommerce.sh << 'EOF'
#!/bin/sh
BACKUP_DIR="/backup"
DATE=$(date +%Y%m%d)

# 数据库
pg_dump -U rcommerce rcommerce | gzip > $BACKUP_DIR/rcommerce-db-$DATE.sql.gz

# 文件
tar -czf $BACKUP_DIR/rcommerce-files-$DATE.tar.gz /usr/local/rcommerce/uploads

# 只保留最近 7 天
find $BACKUP_DIR -name "rcommerce-*" -mtime +7 -delete
EOF

chmod +x /usr/local/bin/backup-rcommerce.sh

# 添加到 cron
0 2 * * * /usr/local/bin/backup-rcommerce.sh
```

## 故障排除

| 问题 | 解决方案 |
|-------|----------|
| 服务无法启动 | 检查 `service rcommerce status` 和日志 |
| 数据库连接失败 | 验证 PostgreSQL 是否正在运行和凭据 |
| 端口已被占用 | 使用 `sockstat -4 -l` 检查 |
| 权限被拒绝 | 使用 `ls -la` 检查文件所有权 |
| 内存不足 | 使用 `top` 或 `vmstat` 检查 |

## 另请参阅

- [Jail 部署](jails.md) - 使用 iocage jails 部署
- [rc.d 服务](rc.d.md) - 服务管理详情
- [Nginx 反向代理](../../operations/reverse-proxies/nginx.md)
- [运维概览](../../operations/index.md)
