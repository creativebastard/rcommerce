# R Commerce 扩展

通过水平和垂直扩展 R Commerce 来处理增加的流量和负载。

## 扩展策略

### 垂直扩展（纵向扩展）

增加单台服务器的资源：

| 资源 | 影响 | 限制 |
|----------|--------|-------|
| CPU | 更多并发请求 | 单机限制 |
| 内存 | 更大的缓存，更多连接 | 成本增加 |
| 磁盘 I/O | 更快的数据库查询 | 需要 SSD/NVMe |
| 网络 | 更高吞吐量 | 网卡带宽 |

### 水平扩展（横向扩展）

在负载均衡器后添加更多服务器：

```
                    ┌─────────────────┐
                    │   负载均衡器     │
                    │    (nginx)      │
                    └────────┬────────┘
                             │
            ┌────────────────┼────────────────┐
            │                │                │
     ┌──────▼──────┐  ┌──────▼──────┐  ┌──────▼──────┐
     │  R Commerce │  │  R Commerce │  │  R Commerce │
     │   实例 1     │  │   实例 2     │  │   实例 3     │
     └──────┬──────┘  └──────┬──────┘  └──────┬──────┘
            │                │                │
            └────────────────┼────────────────┘
                             │
                    ┌────────▼────────┐
                    │   PostgreSQL    │
                    │    (主库)        │
                    └─────────────────┘
                             │
                    ┌────────▼────────┐
                    │     Redis       │
                    │    (共享)        │
                    └─────────────────┘
```

## 无状态架构

R Commerce 设计为无状态架构：

- **无本地会话**：使用 Redis 进行会话存储
- **共享存储**：使用 S3/MinIO 进行文件上传
- **集中配置**：所有实例共享相同配置

```toml
[cache]
cache_type = "Redis"
redis_url = "redis://redis-cluster:6379"

[media]
storage_type = "S3"
s3_bucket = "rcommerce-uploads"
s3_region = "us-east-1"
```

## 数据库扩展

### 只读副本

将读取查询分布到副本：

```toml
[database]
# 写入主库
host = "postgres-primary.internal"
port = 5432

# 从副本读取（应用层）
read_replicas = [
    "postgres-replica-1.internal:5432",
    "postgres-replica-2.internal:5432"
]
```

### 连接池

使用 PgBouncer 进行连接池管理：

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

### 数据库分片

对于非常大的数据集，按租户分片：

```rust
// 基于 tenant_id 的分片键
fn get_shard(tenant_id: Uuid) -> &'static str {
    match tenant_id.as_u128() % 4 {
        0 => "postgres-shard-0",
        1 => "postgres-shard-1",
        2 => "postgres-shard-2",
        _ => "postgres-shard-3",
    }
}
```

## 缓存策略

### 多级缓存

```
┌─────────────────────────────────────┐
│           客户端浏览器               │
│         (HTTP 缓存, 本地)            │
└───────────────┬─────────────────────┘
                │
┌───────────────▼─────────────────────┐
│           CDN (CloudFlare)          │
│      (边缘缓存, 5分钟 TTL)           │
└───────────────┬─────────────────────┘
                │
┌───────────────▼─────────────────────┐
│         应用缓存                     │
│    (内存缓存, 请求级别)               │
└───────────────┬─────────────────────┘
                │
┌───────────────▼─────────────────────┐
│           Redis 集群                 │
│    (分布式缓存, 1小时 TTL)           │
└───────────────┬─────────────────────┘
                │
┌───────────────▼─────────────────────┐
│           数据库                     │
│      (真实数据源)                    │
└─────────────────────────────────────┘
```

### 缓存配置

```toml
[cache]
cache_type = "Redis"
redis_url = "redis://redis-node-1:6379,redis-node-2:6379,redis-node-3:6379"
redis_cluster = true

# 缓存 TTL
product_cache_ttl = 300      # 5 分钟
customer_cache_ttl = 600     # 10 分钟
session_cache_ttl = 86400    # 24 小时
```

## 负载均衡器配置

### nginx 上游配置

```nginx
upstream rcommerce {
    least_conn;
    
    server 10.0.1.10:8080 weight=5 max_fails=3 fail_timeout=30s;
    server 10.0.1.11:8080 weight=5 max_fails=3 fail_timeout=30s;
    server 10.0.1.12:8080 weight=5 max_fails=3 fail_timeout=30s;
    
    keepalive 32;
}
```

### 健康检查

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

## 自动扩展

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

### AWS 自动扩展

```yaml
# CloudFormation 模板
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

## 性能基准测试

### 单实例

| 实例类型 | RPS | p99 延迟 | 并发用户 |
|---------------|-----|-------------|------------------|
| 1 vCPU, 1GB | 500 | 50ms | 1,000 |
| 2 vCPU, 4GB | 2,000 | 20ms | 5,000 |
| 4 vCPU, 8GB | 5,000 | 10ms | 15,000 |
| 8 vCPU, 16GB | 12,000 | 8ms | 40,000 |

### 水平扩展

| 实例数 | RPS | 数据库连接数 |
|-----------|-----|---------------------|
| 1 | 5,000 | 20 |
| 3 | 15,000 | 60 |
| 10 | 50,000 | 200 |
| 20 | 100,000 | 400 |

## 监控扩展

### 关键指标

```promql
# 请求速率
rate(http_requests_total[5m])

# 响应时间
histogram_quantile(0.99, rate(http_request_duration_seconds_bucket[5m]))

# 错误率
rate(http_requests_total{status=~"5.."}[5m])

# 活跃连接数
count(increase(http_requests_total[1m]) > 0)

# 数据库连接数
pg_stat_activity_count{state="active"}
```

### 告警

```yaml
# 高请求速率 - 需要扩展
- alert: HighRequestRate
  expr: rate(http_requests_total[5m]) > 10000
  for: 2m
  labels:
    severity: warning
  annotations:
    summary: "高请求速率 - 考虑扩展"

# 高延迟 - 需要调查
- alert: HighLatency
  expr: histogram_quantile(0.99, rate(http_request_duration_seconds_bucket[5m])) > 0.1
  for: 5m
  labels:
    severity: critical
```

## 最佳实践

1. **从小开始**：先进行垂直扩展，再进行水平扩展
2. **先监控**：在扩展之前建立基线
3. **积极缓存**：减少数据库负载
4. **数据库优先**：在扩展应用之前先扩展数据库
5. **负载测试**：使用负载测试找到限制
6. **优雅关闭**：处理 SIGTERM 以实现零停机部署
7. **熔断器**：当依赖项宕机时快速失败
