# 微信支付集成

微信支付集成到中国最受欢迎的通讯应用中，拥有超过 12 亿月活跃用户。

## 功能

- **应用内支付**: 微信小程序内的原生支付
- **二维码支付**: 商家展示二维码，客户扫码
- **H5 支付**: 非微信浏览器的移动网页支付
- **原生应用**: 深度链接到微信应用进行支付
- **红包**: 营销和促销功能

## 先决条件

- 微信商户账户
- 商业执照进行验证
- 网站的 ICP 许可证（中国）
- 来自微信支付平台的 API 凭证

## 配置

### 环境变量

```bash
WECHATPAY_MCH_ID=your_merchant_id
WECHATPAY_APP_ID=your_app_id
WECHATPAY_API_KEY=your_api_key
WECHATPAY_CERT_PATH=/path/to/apiclient_cert.pem
WECHATPAY_KEY_PATH=/path/to/apiclient_key.pem
```

### 配置文件

```toml
[payment.wechatpay]
enabled = true
mch_id = "${WECHATPAY_MCH_ID}"
app_id = "${WECHATPAY_APP_ID}"
api_key = "${WECHATPAY_API_KEY}"
cert_path = "${WECHATPAY_CERT_PATH}"
key_path = "${WECHATPAY_KEY_PATH}"
sandbox = false
```

## API 使用

### 原生支付（二维码）

```http
POST /api/v1/payments
Content-Type: application/json
Authorization: Bearer <token>

{
  "gateway": "wechatpay",
  "amount": "199.00",
  "currency": "CNY",
  "payment_method": "native",
  "description": "商品购买",
  "notify_url": "https://yoursite.com/webhooks/wechatpay"
}
```

**响应：**

```json
{
  "id": "wx...",
  "code_url": "weixin://wxpay/bizpayurl?pr=...",
  "qr_code": "https://api.qrserver.com/v1/...",
  "status": "pending"
}
```

展示二维码供客户用微信扫描。

### JSAPI 支付（小程序 / 公众号）

```http
POST /api/v1/payments
{
  "gateway": "wechatpay",
  "amount": "199.00",
  "currency": "CNY",
  "payment_method": "jsapi",
  "openid": "user_openid_from_wechat",
  "description": "商品购买"
}
```

**响应：**

```json
{
  "id": "wx...",
  "appId": "wx...",
  "timeStamp": "1706448000",
  "nonceStr": "random_string",
  "package": "prepay_id=wx...",
  "signType": "RSA",
  "paySign": "signature"
}
```

将这些参数传递给微信 JS-SDK `chooseWXPay` 方法。

### H5 支付（移动网页）

```http
POST /api/v1/payments
{
  "gateway": "wechatpay",
  "amount": "199.00",
  "currency": "CNY",
  "payment_method": "h5",
  "description": "商品购买",
  "scene_info": {
    "h5_info": {
      "type": "Wap",
      "wap_url": "https://yoursite.com",
      "wap_name": "Your Store"
    }
  }
}
```

**响应：**

```json
{
  "id": "wx...",
  "h5_url": "https://wx.tenpay.com/cgi-bin/...",
  "status": "pending"
}
```

将客户重定向到 `h5_url` 以完成支付。

## Webhook 通知

微信支付发送支付通知到您的 `notify_url`：

```xml
<xml>
  <appid><![CDATA[wx...]]></appid>
  <mch_id><![CDATA[123...]]></mch_id>
  <out_trade_no><![CDATA[your_order_id]]></out_trade_no>
  <transaction_id><![CDATA[wx...]]></transaction_id>
  <result_code><![CDATA[SUCCESS]]></result_code>
  <total_fee>19900</total_fee>
  <sign><![CDATA[...]]></sign>
</xml>
```

验证签名并响应：

```xml
<xml>
  <return_code><![CDATA[SUCCESS]]></return_code>
  <return_msg><![CDATA[OK]]></return_msg>
</xml>
```

## 支付状态

| 状态 | 说明 |
|--------|-------------|
| `NOTPAY` | 未支付 |
| `USERPAYING` | 用户支付中（输入密码） |
| `SUCCESS` | 支付成功 |
| `CLOSED` | 订单已关闭 |
| `REVOKED` | 支付已撤销 |
| `PAYERROR` | 支付失败 |

## 退款

```http
POST /api/v1/payments/{payment_id}/refund
{
  "amount": "199.00",
  "reason": "客户要求",
  "notify_url": "https://yoursite.com/webhooks/wechatpay/refund"
}
```

## 测试

使用微信支付沙盒：

```toml
[payment.wechatpay]
sandbox = true
```

注意：沙盒需要微信提供的特殊测试账户。

## 最佳实践

1. **证书管理**: 保持 API 证书安全并定期轮换
2. **OpenID**: 缓存用户 openid 以避免重复授权
3. **通知处理**: 幂等地处理通知
4. **查询回退**: 如果通知延迟，轮询支付状态
5. **错误消息**: 用中文显示用户友好的消息
