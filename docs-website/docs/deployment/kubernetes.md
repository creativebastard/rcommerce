# Kubernetes Deployment

Deploy R Commerce on Kubernetes for production workloads.

## Architecture

```
┌─────────────────────────────────────────┐
│              Ingress                     │
│         (TLS termination)               │
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

## Quick Start

```bash
kubectl apply -f k8s/namespace.yaml
kubectl apply -f k8s/configmap.yaml
kubectl apply -f k8s/secret.yaml
kubectl apply -f k8s/postgres.yaml
kubectl apply -f k8s/redis.yaml
kubectl apply -f k8s/rcommerce.yaml
kubectl apply -f k8s/ingress.yaml
```

See [Scaling](../operations/scaling.md) for HPA configuration.
