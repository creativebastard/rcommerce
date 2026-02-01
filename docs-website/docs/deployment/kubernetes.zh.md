# Kubernetes 部署

在 Kubernetes 上部署 R Commerce 用于生产工作负载。

## 架构

```
┌─────────────────────────────────────────┐
│              Ingress                     │
│         (TLS 终止)               │
└─────────────────┬───────────────────────┘
                  │
┌─────────────────▼───────────────────────┐
│           R Commerce Pods               │
│    (Deployment with HPA)                │
│         Replicas: 3-20                  │
└─────────────────┬───────────────────────┘
                  │
    ┌─────────────┴─────────────┐
    │                           │
┌───▼────┐               ┌──────▼────┐
│PostgreSQL│               │   Redis   │
│(Stateful)│               │ (Cluster) │
└─────────┘               └───────────┘
```

## 快速开始

```bash
kubectl apply -f k8s/namespace.yaml
kubectl apply -f k8s/configmap.yaml
kubectl apply -f k8s/secret.yaml
kubectl apply -f k8s/postgres.yaml
kubectl apply -f k8s/redis.yaml
kubectl apply -f k8s/rcommerce.yaml
kubectl apply -f k8s/ingress.yaml
```

请参阅 [扩展](./scaling.md) 了解 HPA 配置。
