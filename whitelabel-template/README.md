# R Commerce Dynamic Frontend Template

A dynamic, server-side rendered storefront template for R Commerce.

## Features

- **Server-Side Rendering (SSR)**: Tera templates rendered on the server
- **Dynamic Routing**: `/products/:id`, `/categories/:slug`, `/pages/:slug`
- **API Integration**: Fetches data from R Commerce backend
- **Edge Caching**: CloudFlare/CDN optimized
- **Redis Caching**: Optional Redis for API response caching

## Directory Structure

```
whitelabel-template/
├── templates/           # Tera HTML templates
│   ├── base.html       # Base layout
│   ├── index.html      # Homepage
│   ├── product.html    # Product detail
│   ├── category.html   # Category listing
│   ├── cart.html       # Shopping cart
│   └── page.html       # CMS pages
├── static/             # Static assets
│   ├── style.css
│   └── app.js
└── README.md
```

## Dynamic Routes

| Route | Template | Description |
|-------|----------|-------------|
| `/` | `index.html` | Homepage with featured products |
| `/products/:id` | `product.html` | Product detail page |
| `/categories/:slug` | `category.html` | Category listing |
| `/pages/:slug` | `page.html` | CMS pages (about, contact, etc) |
| `/cart` | `cart.html` | Shopping cart |

## Template Variables

### Global (available in all templates)
- `store_name` - Store name
- `current_year` - Current year
- `api_url` - API base URL

### index.html
- `title` - Page title
- `products` - Array of featured products

### product.html
- `title` - Product name
- `product` - Product object with id, name, price, description, images, etc.

### category.html
- `title` - Category name
- `category` - Category slug
- `products` - Array of products
- `page` - Current page number

## Running the Server

```bash
# Using rcommerce-frontend
rcommerce-frontend \
  --api-url https://api.yourstore.com \
  --api-key ak_your_api_key \
  --template-dir ./templates \
  --static-dir ./static \
  --redis-url redis://localhost:6379  # Optional
```

### Configuration File

Create `frontend-server.toml`:

```toml
bind = "0.0.0.0:3000"
api_url = "https://api.yourstore.com"
api_key = "ak_your_api_key"
template_dir = "templates"
static_dir = "static"

# Optional Redis caching
redis_url = "redis://localhost:6379"
cache_ttl_secs = 300

# Rate limiting
rate_limit_per_minute = 60

# Development mode (template hot-reload)
dev_mode = false
```

## Template Syntax

Uses [Tera](https://tera.netlify.app/) template engine:

```html
{# Comments #}
{{ variable }}
{{ variable | filter }}
{% if condition %}...{% endif %}
{% for item in items %}...{% endfor %}
{% extends "base.html" %}
{% block content %}...{% endblock %}
```

### Available Filters

- `| capitalize` - Capitalize string
- `| truncate(length=100)` - Truncate text
- `| replace(from="-", to=" ")` - Replace text
- `| default(value="fallback")` - Default value

## Deployment

### With CloudFlare

1. Deploy `rcommerce-frontend` server
2. Point domain to server
3. Enable CloudFlare proxy
4. Static assets cached for 30 days automatically

### Environment Variables

```bash
FRONTEND_BIND=0.0.0.0:3000
FRONTEND_API_URL=https://api.yourstore.com
FRONTEND_API_KEY=ak_your_api_key
FRONTEND_REDIS_URL=redis://localhost:6379
FRONTEND_DEV_MODE=false
```

## Customization

1. Edit templates in `templates/` directory
2. Modify styles in `static/style.css`
3. Add custom JavaScript in `static/app.js`
4. Restart server (or enable dev_mode for auto-reload)

## API Data Structure

### Product Object
```json
{
  "id": "uuid",
  "name": "Product Name",
  "slug": "product-name",
  "description": "...",
  "price": 29.99,
  "compare_at_price": 39.99,
  "sku": "PROD-001",
  "images": [{"url": "..."}],
  "category": {"id": "...", "name": "..."},
  "inventory": {"quantity": 100}
}
```
