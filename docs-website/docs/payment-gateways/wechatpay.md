# WeChat Pay Integration

WeChat Pay is integrated into China's most popular messaging app with over 1.2 billion monthly active users.

## Features

- **In-App Payments**: Native payment within WeChat mini programs
- **QR Code Payments**: Merchant displays QR, customer scans
- **H5 Payments**: Mobile web payments for non-WeChat browsers
- **Native App**: Deep-link to WeChat app for payment
- **Red Packets**: Marketing and promotional features

## Prerequisites

- WeChat merchant account
- Business license for verification
- ICP license for websites (China)
- API credentials from WeChat Pay platform

## Configuration

### Environment Variables

```bash
WECHATPAY_MCH_ID=your_merchant_id
WECHATPAY_APP_ID=your_app_id
WECHATPAY_API_KEY=your_api_key
WECHATPAY_CERT_PATH=/path/to/apiclient_cert.pem
WECHATPAY_KEY_PATH=/path/to/apiclient_key.pem
```

### Config File

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

## API Usage

### Native Payment (QR Code)

```http
POST /api/v1/payments
Content-Type: application/json
Authorization: Bearer <token>

{
  "gateway": "wechatpay",
  "amount": "199.00",
  "currency": "CNY",
  "payment_method": "native",
  "description": "Product Purchase",
  "notify_url": "https://yoursite.com/webhooks/wechatpay"
}
```

**Response:**

```json
{
  "id": "wx...",
  "code_url": "weixin://wxpay/bizpayurl?pr=...",
  "qr_code": "https://api.qrserver.com/v1/...",
  "status": "pending"
}
```

Display the QR code for customers to scan with WeChat.

### JSAPI Payment (Mini Program / Official Account)

```http
POST /api/v1/payments
{
  "gateway": "wechatpay",
  "amount": "199.00",
  "currency": "CNY",
  "payment_method": "jsapi",
  "openid": "user_openid_from_wechat",
  "description": "Product Purchase"
}
```

**Response:**

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

Pass these parameters to WeChat JS-SDK `chooseWXPay` method.

### H5 Payment (Mobile Web)

```http
POST /api/v1/payments
{
  "gateway": "wechatpay",
  "amount": "199.00",
  "currency": "CNY",
  "payment_method": "h5",
  "description": "Product Purchase",
  "scene_info": {
    "h5_info": {
      "type": "Wap",
      "wap_url": "https://yoursite.com",
      "wap_name": "Your Store"
    }
  }
}
```

**Response:**

```json
{
  "id": "wx...",
  "h5_url": "https://wx.tenpay.com/cgi-bin/...",
  "status": "pending"
}
```

Redirect customer to `h5_url` to complete payment.

## Webhook Notifications

WeChat Pay sends payment notifications to your `notify_url`:

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

Verify the signature and respond with:

```xml
<xml>
  <return_code><![CDATA[SUCCESS]]></return_code>
  <return_msg><![CDATA[OK]]></return_msg>
</xml>
```

## Payment Status

| Status | Description |
|--------|-------------|
| `NOTPAY` | Not paid |
| `USERPAYING` | User paying (password entry) |
| `SUCCESS` | Payment successful |
| `CLOSED` | Order closed |
| `REVOKED` | Payment revoked |
| `PAYERROR` | Payment failed |

## Refunds

```http
POST /api/v1/payments/{payment_id}/refund
{
  "amount": "199.00",
  "reason": "Customer request",
  "notify_url": "https://yoursite.com/webhooks/wechatpay/refund"
}
```

## Testing

Use WeChat Pay sandbox:

```toml
[payment.wechatpay]
sandbox = true
```

Note: Sandbox requires special test accounts from WeChat.

## Best Practices

1. **Certificate Management**: Keep API certificates secure and rotated
2. **OpenID**: Cache user openid to avoid repeated authorization
3. **Notification Handling**: Process notifications idempotently
4. **Query Fallback**: Poll payment status if notification delayed
5. **Error Messages**: Display user-friendly messages in Chinese
