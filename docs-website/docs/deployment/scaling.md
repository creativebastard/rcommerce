# Scaling R Commerce

Scale R Commerce horizontally and vertically to handle increased traffic and load.

## Scaling Strategies

### Vertical Scaling (Scale Up)

Increase resources on a single server:

| Resource | Impact | Limit |
|----------|--------|-------|
| CPU | More concurrent requests | Single machine limit |
| RAM | Larger cache, more connections | Cost increases |
| Disk I/O | Faster database queries | SSD/NVMe required |
| Network | Higher throughput | NIC bandwidth |

### Horizontal Scaling (Scale Out)

Add more servers behind a load balancer:

```
                    ┌─────────────────┐
                    │   Load Balancer │
                    │    (nginx)      │
                    └────────┬────────┘
                             │
            ┌────────────────┼────────────────┐
            │                │                │
     ┌──────▼──────┐  ┌──────▼──────┐  ┌──────▼──────┐
     │  R Commerce │  │  R Commerce │  │  R Commerce │
     │   Instance 1│  │   Instance 2│  │   Instance 3│
     └──────┬──────┘  └──────┬──────┘  └──────┬──────┘
            │                │                │
            └────────────────┼────────────────┘
                             │
                    ┌────────▼────────┐
                    │   PostgreSQL    │
                    │    (Primary)    │
                    └─────────────────┘
                             │
                    ┌────────▼────────┐
                    │     Redis       │
                    │    (Shared)     │
                    └─────────────────┘
```

## Stateless Architecture

R Commerce is designed to be stateless:

- **No local sessions**: Use Redis for session storage
- **Shared storage**: Use S3/MinIO for uploads
- **Centralized config**: All instances share same config

```toml
[cache]
cache_type = "Redis"
redis_url = "redis://redis-cluster:6379"

[media]
storage_type = "S3"
s3_bucket = "rcommerce-uploads"
s3_region = "us-east-1"
```

## Database Scaling

### Read Replicas

Distribute read queries across replicas:

```toml
[database]
# Write to primary
host = "postgres-primary.internal"
port = 5432

# Read from replicas (application-level)
read_replicas = [
    "postgres-replica-1.internal:5432",
    "postgres-replica-2.internal:5432"
]
```

### Connection Pooling

Use PgBouncer for connection pooling:

```ini
; pgbouncer.ini
[databases]
rcommerce = host=postgres-primary port=5432 dbname=rcommerce

[pgbouncer]
listen_port = 6432
listen_addr = 0.0.0.0
auth_type = md5
pool_mode = transaction
max_client_conn = 10000
default_pool_size = 25
```

### Database Sharding

For very large datasets, shard by tenant:

```rust
// Shard key based on tenant_id
fn get_shard(tenant_id: Uuid) -> &'static str {
    match tenant_id.as_u128() % 4 {
        0 => "postgres-shard-0",
        1 => "postgres-shard-1",
        2 => "postgres-shard-2",
        _ => "postgres-shard-3",
    }
}
```

## Caching Strategy

### Multi-Level Cache

```
┌─────────────────────────────────────┐
│           Client Browser            │
│         (HTTP Cache, Local)         │
└───────────────┬─────────────────────┘
                │
┌───────────────▼─────────────────────┐
│           CDN (CloudFlare)          │
│      (Edge Cache, 5min TTL)         │
└───────────────┬─────────────────────┘
                │
┌───────────────▼─────────────────────┐
│         Application Cache           │
│    (In-Memory, Request-Level)       │
└───────────────┬─────────────────────┘
                │
┌───────────────▼─────────────────────┐
│           Redis Cluster             │
│    (Distributed Cache, 1hr TTL)     │
└───────────────┬─────────────────────┘
                │
┌───────────────▼─────────────────────┐
│           Database                  │
│      (Source of Truth)              │
└─────────────────────────────────────┘
```

### Cache Configuration

```toml
[cache]
cache_type = "Redis"
redis_url = "redis://redis-node-1:6379,redis-node-2:6379,redis-node-3:6379"
redis_cluster = true

# Cache TTLs
product_cache_ttl = 300      # 5 minutes
customer_cache_ttl = 600     # 10 minutes
session_cache_ttl = 86400    # 24 hours
```

## Load Balancer Configuration

### nginx Upstream

```nginx
upstream rcommerce {
    least_conn;
    
    server 10.0.1.10:8080 weight=5 max_fails=3 fail_timeout=30s;
    server 10.0.1.11:8080 weight=5 max_fails=3 fail_timeout=30s;
    server 10.0.1.12:8080 weight=5 max_fails=3 fail_timeout=30s;
    
    keepalive 32;
}
```

### Health Checks

```nginx
upstream rcommerce {
    zone upstream_rcommerce 64k;
    
    server 10.0.1.10:8080;
    server 10.0.1.11:8080;
    
    check interval=5000 rise=2 fall=3 timeout=3000 type=http;
    check_http_send "GET /health HTTP/1.0\r\n\r\n";
    check_http_expect_alive http_2xx http_3xx;
}
```

## Auto-Scaling

### Kubernetes HPA

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: rcommerce-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: rcommerce
  minReplicas: 3
  maxReplicas: 20
  metrics:
    - type: Resource
      resource:
        name: cpu
        target:
          type: Utilization
          averageUtilization: 70
    - type: Resource
      resource:
        name: memory
        target:
          type: Utilization
          averageUtilization: 80
  behavior:
    scaleUp:
      stabilizationWindowSeconds: 60
      policies:
        - type: Percent
          value: 100
          periodSeconds: 15
    scaleDown:
      stabilizationWindowSeconds: 300
      policies:
        - type: Percent
          value: 10
          periodSeconds: 60
```

### AWS Auto Scaling

```yaml
# CloudFormation template
Resources:
  RCommerceAutoScalingGroup:
    Type: AWS::AutoScaling::AutoScalingGroup
    Properties:
      MinSize: 2
      MaxSize: 10
      DesiredCapacity: 3
      LaunchTemplate:
        LaunchTemplateId: !Ref RCommerceLaunchTemplate
        Version: !GetAtt RCommerceLaunchTemplate.LatestVersionNumber
      TargetGroupARNs:
        - !Ref RCommerceTargetGroup
      HealthCheckType: ELB
      HealthCheckGracePeriod: 300
      
  RCommerceScalingPolicy:
    Type: AWS::AutoScaling::ScalingPolicy
    Properties:
      AutoScalingGroupName: !Ref RCommerceAutoScalingGroup
      PolicyType: TargetTrackingScaling
      TargetTrackingConfiguration:
        PredefinedMetricSpecification:
          PredefinedMetricType: ASGAverageCPUUtilization
        TargetValue: 70.0
```

## Performance Benchmarks

### Single Instance

| Instance Type | RPS | Latency p99 | Concurrent Users |
|---------------|-----|-------------|------------------|
| 1 vCPU, 1GB | 500 | 50ms | 1,000 |
| 2 vCPU, 4GB | 2,000 | 20ms | 5,000 |
| 4 vCPU, 8GB | 5,000 | 10ms | 15,000 |
| 8 vCPU, 16GB | 12,000 | 8ms | 40,000 |

### Horizontal Scaling

| Instances | RPS | Database Connections |
|-----------|-----|---------------------|
| 1 | 5,000 | 20 |
| 3 | 15,000 | 60 |
| 10 | 50,000 | 200 |
| 20 | 100,000 | 400 |

## Monitoring Scaling

### Key Metrics

```promql
# Request rate
rate(http_requests_total[5m])

# Response time
histogram_quantile(0.99, rate(http_request_duration_seconds_bucket[5m]))

# Error rate
rate(http_requests_total{status=~"5.."}[5m])

# Active connections
count(increase(http_requests_total[1m]) > 0)

# Database connections
pg_stat_activity_count{state="active"}
```

### Alerts

```yaml
# High request rate - scale up
- alert: HighRequestRate
  expr: rate(http_requests_total[5m]) > 10000
  for: 2m
  labels:
    severity: warning
  annotations:
    summary: "High request rate - consider scaling"

# High latency - investigate
- alert: HighLatency
  expr: histogram_quantile(0.99, rate(http_request_duration_seconds_bucket[5m])) > 0.1
  for: 5m
  labels:
    severity: critical
```

## Best Practices

1. **Start Small**: Begin with vertical scaling, then horizontal
2. **Monitor First**: Establish baselines before scaling
3. **Cache Aggressively**: Reduce database load
4. **Database First**: Scale database before application
5. **Test Load**: Use load testing to find limits
6. **Graceful Shutdown**: Handle SIGTERM for zero-downtime deploys
7. **Circuit Breakers**: Fail fast when dependencies are down
