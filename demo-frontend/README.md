# R Commerce Demo Frontend

A lightweight, vanilla HTML/JavaScript frontend for demonstrating R Commerce functionality.

## Features

- Product listing with 3 demo products
- Product detail pages
- Shopping cart functionality
- Checkout flow
- Responsive design
- Works with or without backend API

## Quick Start

1. Open `index.html` in a browser
2. Or serve with a simple HTTP server:

```bash
# Python 3
python -m http.server 3000

# Node.js
npx serve .

# PHP
php -S localhost:3000
```

Then open http://localhost:3000

## Configuration

Edit `api.js` to configure the API endpoint and API key:

```javascript
const API_BASE_URL = 'http://localhost:8080/api/v1';
const API_KEY = 'ak_yourprefix.yoursecret';  // Get from R Commerce CLI
```

### Getting an API Key

1. Start your R Commerce server
2. Create an API key using the CLI:
   ```bash
   rcommerce api-key create --name "Demo Frontend" --scopes "products:read,orders:write,carts:write"
   ```
3. Copy the generated key to `api.js`

**Note:** API keys are for service-to-service authentication. Customer authentication (login/register) uses JWT tokens.

## Demo Products

1. **Premium Wireless Headphones** - $299.99
2. **Smart Watch Pro** - $399.99
3. **Portable Bluetooth Speaker** - $149.99

## Cart Persistence

Cart data is stored in browser localStorage and persists across sessions.

## Browser Compatibility

- Chrome/Edge (latest)
- Firefox (latest)
- Safari (latest)

## Integration with R Commerce

When the R Commerce backend is running, the frontend will:
- Fetch products from `/api/v1/products`
- Create orders via `/api/v1/orders`

If the backend is unavailable, it falls back to demo data.
