# R Commerce Whitelabel Template

A customizable storefront template for R Commerce.

## Quick Start

1. **Copy this directory** to your project:
   ```bash
   cp -r whitelabel-template my-store
   cd my-store
   ```

2. **Customize the template** by editing the variables at the top of `index.html`:
   - `{{STORE_NAME}}` - Your store name
   - `{{PRIMARY_COLOR}}` - Brand color (e.g., #EB4F27)
   - `{{HERO_TITLE}}` - Main headline
   - `{{HERO_SUBTITLE}}` - Subheadline

3. **Deploy with rcommerce-frontend-server**:
   ```bash
   rcommerce-frontend \
     --api-url https://api.yourdomain.com \
     --api-key ak_your_api_key \
     --static-dir ./
   ```

## Features

- ✅ Responsive design (mobile-friendly)
- ✅ Tailwind CSS styling
- ✅ No build step required
- ✅ Works with rcommerce-frontend-server
- ✅ Easy to customize

## Customization

### Colors
Edit the CSS variables in the `<style>` section:
```css
:root {
    --primary: #EB4F27;    /* Your brand color */
    --secondary: #1F2937;  /* Secondary color */
}
```

### Pages
Add new HTML files for additional pages:
- `products.html` - Product listing
- `cart.html` - Shopping cart
- `checkout.html` - Checkout flow

### API Integration
The included `api.js` provides:
- `api.getProducts()` - Fetch products
- `api.addToCart(id, qty)` - Add to cart
- `api.login(email, pass)` - Customer login

## Deployment

### With rcommerce-frontend-server (Recommended)
```bash
# Install the server
cargo install --git https://github.com/creativebastard/rcommerce rcommerce-demo-server

# Run with your config
rcommerce-frontend --config config.toml
```

### With CloudFlare
1. Deploy `rcommerce-frontend-server` to your server
2. Point your domain to the server
3. Enable CloudFlare proxy
4. Configure page rules for caching static assets

## Support

For help, see the [R Commerce Documentation](https://docs.rcommerce.app).
