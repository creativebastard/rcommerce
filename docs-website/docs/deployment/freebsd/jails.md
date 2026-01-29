# FreeBSD Jail Deployment with iocage

Deploy R Commerce in a FreeBSD jail for enhanced security and isolation using iocage, the modern jail management framework.

## Supported FreeBSD Versions

- **FreeBSD 14.2** - Latest production release (recommended)
- **FreeBSD 15.0** - Current stable branch

## Why Jails?

- **Security**: Process isolation prevents escape
- **Resource Control**: CPU/memory limits per jail
- **Easy Management**: Create/destroy/clone jails quickly
- **Network Isolation**: Separate IP addresses and firewall rules
- **ZFS Integration**: Built-in snapshots and cloning

## Prerequisites

```bash
# Install iocage
pkg install iocage

# Enable iocage service
tee /etc/rc.conf << 'EOF'
iocage_enable="YES"
EOF

# Activate iocage on ZFS pool (usually zroot)
iocage activate zroot
```

## Quick Start

### 1. Create R Commerce Jail

```bash
# Fetch FreeBSD release
iocage fetch --release 14.2-RELEASE

# Create jail
iocage create --name rcommerce \
  --release 14.2-RELEASE \
  --ip4_addr="lo1|192.168.1.100/24" \
  --resolver="nameserver 8.8.8.8" \
  --boot=on

# Start jail
iocage start rcommerce
```

### 2. Configure Jail

```bash
# Enter jail
iocage exec rcommerce /bin/sh

# Update packages
pkg update
pkg upgrade -y

# Install dependencies
pkg install -y postgresql15-server redis nginx ca_root_nss

# Create user
pw useradd -n rcommerce -s /bin/sh -d /usr/local/rcommerce -m

# Exit jail
exit
```

### 3. Deploy R Commerce

From the host, copy the binary into the jail:

```bash
# Download and install R Commerce
iocage exec rcommerce fetch -o /usr/local/bin/rcommerce \
  "https://github.com/creativebastard/rcommerce/releases/latest/download/rcommerce-freebsd-amd64"
iocage exec rcommerce chmod +x /usr/local/bin/rcommerce
```

### 4. Configuration

```bash
# Create config directory
iocage exec rcommerce mkdir -p /usr/local/etc/rcommerce

# Create configuration
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

### 5. rc.d Service Script

Create the service script inside the jail:

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

### 6. PF Configuration (Host)

Configure PF on the host for jail networking:

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

## Jail Management with iocage

### Basic Commands

```bash
# List jails
iocage list

# Start/stop/restart
iocage start rcommerce
iocage stop rcommerce
iocage restart rcommerce

# Enter jail for maintenance
iocage console rcommerce

# Execute command in jail
iocage exec rcommerce ps aux

# View jail properties
iocage get all rcommerce
```

### Resource Limits

```bash
# Set memory limit (4GB)
iocage set memoryuse=4G rcommerce

# Set CPU limit (2 cores)
iocage set pcpu=200 rcommerce

# Set disk quota (50GB)
iocage set quota=50G rcommerce
```

### Snapshots and Clones

```bash
# Create snapshot
iocage snapshot rcommerce

# List snapshots
iocage snaplist rcommerce

# Rollback to snapshot
iocage rollback rcommerce@snapshot_name

# Clone jail
iocage create --name rcommerce-dev --clone rcommerce
```

## Multiple Jails Setup

Create separate jails for different components:

```bash
# Database jail
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

# App jail
iocage create --name rcommerce-app \
  --release 14.2-RELEASE \
  --ip4_addr="lo1|192.168.1.103/24" \
  --boot=on

# Update config to use separate jails
cat > /usr/local/etc/rcommerce/config.toml << 'EOF'
[database]
host = "192.168.1.101"
port = 5432

[cache]
redis_url = "redis://192.168.1.102:6379"
EOF
```

## Backup Strategy

```bash
# Snapshot jail
iocage snapshot rcommerce

# Export jail to file
iocage export rcommerce

# Automated backups via cron
0 2 * * * /usr/local/bin/iocage snapshot rcommerce
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Jail won't start | Check `iocage list` and `iocage console rcommerce` |
| Network unreachable | Verify PF rules and jail IP configuration |
| Permission denied | Check jail file ownership with `iocage exec` |
| Out of memory | Adjust jail limits with `iocage set memoryuse` |
| ZFS issues | Check pool status with `zpool status` |

## Migration from ezjail

If you're currently using ezjail:

```bash
# Stop ezjail
service ezjail stop

# Export existing jail data
cp -a /usr/jails/rcommerce /tmp/rcommerce-backup

# Create new iocage jail with same configuration
iocage create --name rcommerce --release 14.1-RELEASE ...

# Copy data to new jail
cp -a /tmp/rcommerce-backup/* /zroot/iocage/jails/rcommerce/root/

# Update rc.conf to disable ezjail
sysrc ezjail_enable=NO
sysrc iocage_enable=YES
```

## See Also

- [Standalone FreeBSD Deployment](standalone.md) - Deploy without jails
- [rc.d Service](rc.d.md) - Traditional rc.d service management
- [Operations Overview](../../operations/index.md)
