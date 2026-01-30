# 二进制部署

将 R Commerce 作为独立二进制文件部署，无需容器。

## 下载

```bash
# Linux AMD64
curl -L -o rcommerce https://github.com/captainjez/gocart/releases/latest/download/rcommerce-linux-amd64

# Linux ARM64
curl -L -o rcommerce https://github.com/captainjez/gocart/releases/latest/download/rcommerce-linux-arm64

# macOS AMD64
curl -L -o rcommerce https://github.com/captainjez/gocart/releases/latest/download/rcommerce-darwin-amd64

# macOS ARM64
curl -L -o rcommerce https://github.com/captainjez/gocart/releases/latest/download/rcommerce-darwin-arm64

chmod +x rcommerce
```

## 运行

```bash
# 设置配置路径
export RCOMMERCE_CONFIG=/etc/rcommerce/config.toml

# 启动服务器
./rcommerce server

# 运行迁移
./rcommerce migrate
```

请参阅平台特定指南进行服务管理：
- [Linux systemd](./linux/systemd.md)
- [FreeBSD](./freebsd/jails.md)
- [macOS](./macos/launchd.md)
