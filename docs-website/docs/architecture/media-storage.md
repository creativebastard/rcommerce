# Media Storage

## Overview

R Commerce supports multiple storage backends for product images and other media files.

## Storage Backends

### Local Filesystem
Store files on local disk:

```toml
[media]
storage_type = "Local"
local_path = "./uploads"
base_url = "https://cdn.example.com/media"
```

### S3-Compatible
AWS S3, MinIO, DigitalOcean Spaces:

```toml
[media]
storage_type = "S3"
bucket = "my-store-media"
region = "us-east-1"
access_key = "AKIA..."
secret_key = "secret"
endpoint = "https://s3.amazonaws.com"  # Optional for S3-compatible
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

## Image Processing

Automatic image processing:
- Format conversion (WebP, JPEG, PNG)
- Size variants (thumbnail, medium, large)
- Compression optimization
- Responsive image generation

## Upload Handling

```rust
// Upload product image
let image = media_service
    .upload(file_data, filename, content_type)
    .await?;

// Associate with product
product_service.add_image(product_id, image.id).await?;
```

## CDN Integration

Serve media via CDN for optimal performance:
- CloudFlare
- CloudFront
- Fastly
- KeyCDN

## Security

- File type validation
- Size limits
- Virus scanning (optional)
- Signed URLs for private files

## See Also

- [Configuration](../getting-started/configuration.md)
