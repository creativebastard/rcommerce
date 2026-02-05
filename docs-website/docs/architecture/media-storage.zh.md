# 媒体存储

## 概述

R Commerce 支持多种存储后端，用于产品图片和其他媒体文件。

## 存储后端

### 本地文件系统
在本地磁盘存储文件：

```toml
[media]
storage_type = "Local"
local_path = "./uploads"
base_url = "https://cdn.example.com/media"
```

### S3 兼容
AWS S3、MinIO、DigitalOcean Spaces：

```toml
[media]
storage_type = "S3"
bucket = "my-store-media"
region = "us-east-1"
access_key = "AKIA..."
secret_key = "secret"
endpoint = "https://s3.amazonaws.com"  # S3 兼容可选
```

### Google Cloud Storage

```toml
[media]
storage_type = "GCS"
bucket = "my-store-media"
credentials_path = "/etc/gcp/credentials.json"
```

### Azure Blob Storage

```toml
[media]
storage_type = "Azure"
account = "mystore"
access_key = "key"
container = "media"
```

## 图片处理

自动图片处理：
- 格式转换（WebP、JPEG、PNG）
- 尺寸变体（缩略图、中等、大图）
- 压缩优化
- 响应式图片生成

## 上传处理

```rust
// 上传产品图片
let image = media_service
    .upload(file_data, filename, content_type)
    .await?;

// 关联到产品
product_service.add_image(product_id, image.id).await?;
```

## CDN 集成

通过 CDN 提供媒体以获得最佳性能：
- CloudFlare
- CloudFront
- Fastly
- KeyCDN

## 安全性

- 文件类型验证
- 大小限制
- 病毒扫描（可选）
- 私有文件签名 URL

## 另请参阅

- [配置指南](../getting-started/configuration.md)
