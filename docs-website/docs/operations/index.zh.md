# 运维指南

自信地在生产环境中运行 R Commerce。本节涵盖日常运维、监控、扩展和维护。

## 主题

### 基础设施

- [反向代理](./reverse-proxies/caddy.md) - Caddy、Nginx、HAProxy、Traefik
- [扩展](./scaling.md) - 水平和垂直扩展
- [监控](./monitoring.md) - 指标、日志、告警
- [Redis](./redis.md) - 缓存和会话管理

### 维护

- [备份](./backups.md) - 数据库和文件备份
- [安全](./security.md) - 加固和最佳实践

## 快速参考

### 健康检查

```bash
# API 健康
curl http://localhost:8080/health

# 数据库健康
curl http://localhost:8080/health/db

# 完整状态
curl http://localhost:8080/health/detailed
```

### 常用命令

```bash
# 检查服务状态
systemctl status rcommerce  # Linux
launchctl list | grep rcommerce  # macOS
service rcommerce status  # FreeBSD

# 查看日志
journalctl -u rcommerce -f  # Linux
tail -f /var/log/rcommerce.log  # FreeBSD

# 数据库操作
rcommerce db status
rcommerce db migrate
rcommerce db reset
```

### 性能指标

监控这些关键指标：

| 指标 | 目标 | 告警阈值 |
|--------|--------|-----------------|
| 响应时间 | < 10ms p50 | > 50ms |
| 错误率 | < 0.1% | > 1% |
| CPU 使用率 | < 70% | > 90% |
| 内存使用率 | < 80% | > 95% |
| 数据库连接 | < 80% 池 | > 95% 池 |

## 支持

对于运维问题：

1. 首先检查日志
2. 查看监控仪表板
3. 查阅特定指南部分
4. 在 GitHub 上提交 issue
