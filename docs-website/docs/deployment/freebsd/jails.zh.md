# 使用 iocage 的 FreeBSD Jail 部署

使用 iocage（现代 jail 管理框架）在 FreeBSD jail 中部署 R Commerce，以增强安全性和隔离性。

## 支持的 FreeBSD 版本

- **FreeBSD 14.2** - 最新生产版本（推荐）
- **FreeBSD 15.0** - 当前稳定分支

## 为什么选择 Jails？

- **安全**: 进程隔离防止逃逸
- **资源控制**: 每个 jail 的 CPU/内存限制
- **易于管理**: 快速创建/销毁/克隆 jail
- **网络隔离**: 独立的 IP 地址和防火墙规则
- **ZFS 集成**: 内置快照和克隆

## 先决条件

```bash
# 安装 iocage
pkg install iocage

# 启用 iocage 服务
tee /etc/rc.conf << 'EOF'
iocage_enable="YES"
EOF

# 在 ZFS 池上激活 iocage（通常是 zroot）
iocage activate zroot
```

## 快速开始

### 1. 创建 R Commerce Jail

```bash
# 获取 FreeBSD 版本
iocage fetch --release 14.2-RELEASE

# 创建 jail
iocage create --name rcommerce \
  --release 14.2-RELEASE \
  --ip4_addr="lo1|192.168.1.100/24" \
  --resolver="nameserver 8.8.8.8" \
  --boot=on

# 启动 jail
iocage start rcommerce
```

### 2. 配置 Jail

```bash
# 进入 jail
iocage exec rcommerce /bin/sh

# 更新软件包
pkg update
pkg upgrade -y

# 安装依赖
pkg install -y postgresql15-server redis nginx ca_root_nss

# 创建用户
pw useradd -n rcommerce -s /bin/sh -d /usr/local/rcommerce -m

# 退出 jail
exit
```

### 3. 部署 R Commerce

从主机将二进制文件复制到 jail：

```bash
# 下载并安装 R Commerce
iocage exec rcommerce fetch -o /usr/local/bin/rcommerce \
  "https://github.com/creativebastard/rcommerce/releases/latest/download/rcommerce-freebsd-amd64"
iocage exec rcommerce chmod +x /usr/local/bin/rcommerce
```

### 4. 配置

```bash
# 创建配置目录
iocage exec rcommerce mkdir -p /usr/local/etc/rcommerce

# 创建配置
iocage exec rcommerce tee /usr/local/etc/rcommerce/config.toml << 'EOF'
[server]
host = "127.0.0.1"
port = 8080

[database]
db_type = "Postgres"
host = "localhost"
port = 5432
database = "rcommerce"
username = "rcommerce"
password = "${DB_PASSWORD}"

[cache]
cache_type = "Memory"
max_size_mb = 100
EOF
```

### 5. rc.d 服务脚本

在 jail 内创建服务脚本：

```bash
iocage exec rcommerce tee /usr/local/etc/rc.d/rcommerce << 'EOF'
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

command="/usr/local/bin/rcommerce"
procname="/usr/local/bin/rcommerce"

start_cmd="rcommerce_start"
stop_cmd="rcommerce_stop"
status_cmd="rcommerce_status"

rcommerce_start() {
    echo "Starting ${name}."
    export RCOMMERCE_CONFIG=${rcommerce_config}
    /usr/sbin/daemon -u ${rcommerce_user} -p /var/run/${name}.pid \
        ${command} server
}

rcommerce_stop() {
    echo "Stopping ${name}."
    if [ -f /var/run/${name}.pid ]; then
        kill $(cat /var/run/${name}.pid)
    fi
}

rcommerce_status() {
    if [ -f /var/run/${name}.pid ] && kill -0 $(cat /var/run/${name}.pid) 2>/dev/null; then
        echo "${name} is running as pid $(cat /var/run/${name}.pid)."
    else
        echo "${name} is not running."
    fi
}

run_rc_command "$1"
EOF

iocage exec rcommerce chmod +x /usr/local/etc/rc.d/rcommerce
iocage exec rcommerce sysrc rcommerce_enable=YES
```

### 6. PF 配置（主机）

在主机上配置 PF 以进行 jail 网络：

```bash
tee -a /etc/pf.conf << 'EOF'
# NAT for jails
nat on em0 from 192.168.1.0/24 to any -> (em0)

# Redirect HTTP/HTTPS to jail
rdr pass on em0 inet proto tcp from any to any port 80 -> 192.168.1.100 port 80
rdr pass on em0 inet proto tcp from any to any port 443 -> 192.168.1.100 port 443

# Allow jail traffic
pass in on lo1 from 192.168.1.0/24 to any
pass out on lo1 from any to 192.168.1.0/24
EOF

pfctl -f /etc/pf.conf
```

## 使用 iocage 管理 Jail

### 基本命令

```bash
# 列出 jails
iocage list

# 启动/停止/重启
iocage start rcommerce
iocage stop rcommerce
iocage restart rcommerce

# 进入 jail 进行维护
iocage console rcommerce

# 在 jail 中执行命令
iocage exec rcommerce ps aux

# 查看 jail 属性
iocage get all rcommerce
```

### 资源限制

```bash
# 设置内存限制（4GB）
iocage set memoryuse=4G rcommerce

# 设置 CPU 限制（2 核）
iocage set pcpu=200 rcommerce

# 设置磁盘配额（50GB）
iocage set quota=50G rcommerce
```

### 快照和克隆

```bash
# 创建快照
iocage snapshot rcommerce

# 列出快照
iocage snaplist rcommerce

# 回滚到快照
iocage rollback rcommerce@snapshot_name

# 克隆 jail
iocage create --name rcommerce-dev --clone rcommerce
```

## 多 Jail 设置

为不同组件创建单独的 jails：

```bash
# 数据库 jail
iocage create --name rcommerce-db \
  --release 14.2-RELEASE \
  --ip4_addr="lo1|192.168.1.101/24" \
  --boot=on

iocage exec rcommerce-db pkg install -y postgresql15-server
iocage exec rcommerce-db sysrc postgresql_enable=YES
iocage exec rcommerce-db service postgresql initdb
iocage exec rcommerce-db service postgresql start

# Redis jail
iocage create --name rcommerce-cache \
  --release 14.2-RELEASE \
  --ip4_addr="lo1|192.168.1.102/24" \
  --boot=on

iocage exec rcommerce-cache pkg install -y redis
iocage exec rcommerce-cache sysrc redis_enable=YES
iocage exec rcommerce-cache service redis start

# 应用 jail
iocage create --name rcommerce-app \
  --release 14.2-RELEASE \
  --ip4_addr="lo1|192.168.1.103/24" \
  --boot=on

# 更新配置以使用单独的 jails
cat > /usr/local/etc/rcommerce/config.toml << 'EOF'
[database]
host = "192.168.1.101"
port = 5432

[cache]
redis_url = "redis://192.168.1.102:6379"
EOF
```

## 备份策略

```bash
# 快照 jail
iocage snapshot rcommerce

# 导出 jail 到文件
iocage export rcommerce

# 通过 cron 自动备份
0 2 * * * /usr/local/bin/iocage snapshot rcommerce
```

## 故障排除

| 问题 | 解决方案 |
|-------|----------|
| Jail 无法启动 | 检查 `iocage list` 和 `iocage console rcommerce` |
| 网络不可达 | 验证 PF 规则和 jail IP 配置 |
| 权限被拒绝 | 使用 `iocage exec` 检查 jail 文件所有权 |
| 内存不足 | 使用 `iocage set memoryuse` 调整 jail 限制 |
| ZFS 问题 | 使用 `zpool status` 检查池状态 |

## 从 ezjail 迁移

如果您当前正在使用 ezjail：

```bash
# 停止 ezjail
service ezjail stop

# 导出现有 jail 数据
cp -a /usr/jails/rcommerce /tmp/rcommerce-backup

# 使用相同配置创建新的 iocage jail
iocage create --name rcommerce --release 14.1-RELEASE ...

# 复制数据到新 jail
cp -a /tmp/rcommerce-backup/* /zroot/iocage/jails/rcommerce/root/

# 更新 rc.conf 以禁用 ezjail
sysrc ezjail_enable=NO
sysrc iocage_enable=YES
```

## 另请参阅

- [独立 FreeBSD 部署](standalone.md) - 不使用 jails 部署
- [rc.d 服务](rc.d.md) - 传统 rc.d 服务管理
- [运维概览](../../deployment/index.md)
