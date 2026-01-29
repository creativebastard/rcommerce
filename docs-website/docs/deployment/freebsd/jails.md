# FreeBSD Jail Deployment

Deploy R Commerce in a FreeBSD jail for enhanced security and isolation.

## Why Jails?

- **Security**: Process isolation prevents escape
- **Resource Control**: CPU/memory limits per jail
- **Easy Management**: Create/destroy/clone jails quickly
- **Network Isolation**: Separate IP addresses and firewall rules

## Prerequisites

```bash
# Install ezjail for jail management
pkg install ezjail

# Enable jails in rc.conf
echo 'jail_enable="YES"' >> /etc/rc.conf
echo 'ezjail_enable="YES"' >> /etc/rc.conf
```

## Setup

### 1. Configure ezjail

```bash
# Edit ezjail configuration
cat >> /usr/local/etc/ezjail.conf << 'EOF'
ezjail_jaildir=/usr/jails
ezjail_use_zfs="YES"
ezjail_use_zfs_for_jails="YES"
ezjail_jailzfs="zroot/jails"
EOF
```

### 2. Create Base Jail

```bash
# Fetch FreeBSD base system
ezjail-admin install -p

# Update base jail
ezjail-admin update -u
```

### 3. Create R Commerce Jail

```bash
# Create jail
ezjail-admin create -f basic rcommerce 'lo1|192.168.1.100'

# Start jail
ezjail-admin start rcommerce

# Enter jail
jexec rcommerce /bin/sh
```

### 4. Inside the Jail

```bash
# Update packages
pkg update
pkg upgrade -y

# Install dependencies
pkg install -y postgresql15-server redis nginx

# Create user
pw useradd -n rcommerce -s /bin/sh -d /usr/local/rcommerce -m

# Install R Commerce
fetch -o /usr/local/bin/rcommerce \
  https://github.com/captainjez/gocart/releases/latest/download/rcommerce-freebsd-amd64
chmod +x /usr/local/bin/rcommerce
```

### 5. Configuration

```bash
mkdir -p /usr/local/etc/rcommerce
cat > /usr/local/etc/rcommerce/config.toml << 'EOF'
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

### 6. rc.d Script

Create `/usr/local/etc/rc.d/rcommerce`:

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
```

```bash
chmod +x /usr/local/etc/rc.d/rcommerce
echo 'rcommerce_enable="YES"' >> /etc/rc.conf
```

### 7. PF Configuration (Host)

On the host, configure PF for jail networking:

```bash
cat >> /etc/pf.conf << 'EOF'
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

## Jail Management

```bash
# List jails
jls

# Start/stop/restart
ezjail-admin start rcommerce
ezjail-admin stop rcommerce
ezjail-admin restart rcommerce

# Enter jail for maintenance
jexec rcommerce /bin/sh

# View jail resources
rctl -h jail:rcommerce

# Update jail
ezjail-admin update -j rcommerce
```

## Multiple Jails Setup

Create separate jails for different components:

```bash
# Database jail
ezjail-admin create -f basic rcommerce-db 'lo1|192.168.1.101'

# Redis jail  
ezjail-admin create -f basic rcommerce-cache 'lo1|192.168.1.102'

# App jail
ezjail-admin create -f basic rcommerce-app 'lo1|192.168.1.103'

# Update config to use separate jails
[database]
host = "192.168.1.101"

[cache]
redis_url = "redis://192.168.1.102:6379"
```

## Backup Strategy

```bash
# Snapshot jail
zfs snapshot zroot/jails/rcommerce@backup-$(date +%Y%m%d)

# Send to remote
zfs send zroot/jails/rcommerce@backup-20260128 | ssh backup-server zfs receive tank/backups/rcommerce

# Automated in cron
0 2 * * * /root/scripts/backup-rcommerce.sh
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Jail won't start | Check `ezjail-admin list` and logs |
| Network unreachable | Verify PF rules and jail IP |
| Permission denied | Check jail file ownership |
| Out of memory | Adjust jail limits with rctl |
