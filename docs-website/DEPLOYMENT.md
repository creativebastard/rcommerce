# Documentation Website Deployment Guide

This guide covers deploying the R Commerce documentation website to various hosting platforms.

## Quick Start

```bash
# 1. Install dependencies
pip install -r requirements.txt

# 2. Serve locally for development
mkdocs serve

# 3. Build for production
mkdocs build

# 4. Deploy the 'site/' directory to your hosting provider
```

## Deployment Options

### 1. GitHub Pages (Recommended)

The repository includes a GitHub Actions workflow for automatic deployment:

1. Push your changes to the `master` or `main` branch
2. The workflow will automatically build and deploy
3. Your site will be available at `https://yourusername.github.io/gocart`

**Manual Setup:**

1. Go to repository Settings → Pages
2. Set Source to "GitHub Actions"
3. The workflow in `.github/workflows/deploy.yml` will handle the rest

### 2. Netlify

**Option A: Drag and Drop**

1. Build locally: `mkdocs build`
2. Drag the `site/` folder to Netlify's deploy page

**Option B: Git Integration**

1. Connect your Git repository to Netlify
2. Set build command: `pip install -r requirements.txt && mkdocs build`
3. Set publish directory: `site`
4. The `netlify.toml` file is already configured

**Option C: Netlify CLI**

```bash
# Install Netlify CLI
npm install -g netlify-cli

# Login
netlify login

# Initialize and deploy
netlify init
netlify deploy --prod --dir=site
```

### 3. Vercel

**Using Vercel CLI:**

```bash
# Install Vercel CLI
npm install -g vercel

# Login
vercel login

# Deploy
vercel --prod
```

**Configuration:**
- Build Command: `pip install -r requirements.txt && mkdocs build`
- Output Directory: `site`
- The `vercel.json` file is already configured

### 4. AWS S3 + CloudFront

```bash
# Build the site
mkdocs build

# Sync to S3 (requires AWS CLI)
aws s3 sync site/ s3://your-bucket-name --delete

# Invalidate CloudFront cache (optional)
aws cloudfront create-invalidation --distribution-id YOUR_DISTRIBUTION_ID --paths "/*"
```

### 5. Docker Deployment

```bash
# Build the image
docker build -t rcommerce-docs .

# Run locally
docker run -p 8000:80 rcommerce-docs
```

**Dockerfile:**

```dockerfile
FROM python:3.11-slim as builder

WORKDIR /app
COPY requirements.txt .
RUN pip install -r requirements.txt

COPY . .
RUN mkdocs build

FROM nginx:alpine
COPY --from=builder /app/site /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
```

### 6. Self-Hosted (Nginx)

```bash
# Build the site
mkdocs build

# Copy to web server
sudo cp -r site/* /var/www/html/

# Or use rsync for efficient updates
rsync -avz --delete site/ user@server:/var/www/html/
```

**Nginx Configuration:**

```nginx
server {
    listen 80;
    server_name docs.rcommerce.app;
    root /var/www/html;
    index index.html;

    # Enable gzip compression
    gzip on;
    gzip_types text/plain text/css application/json application/javascript;

    # Cache static assets
    location ~* \.(css|js|png|jpg|jpeg|gif|ico|svg)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # Handle 404s
    error_page 404 /404.html;

    # SPA routing
    location / {
        try_files $uri $uri/ /index.html;
    }
}
```

## Environment Variables

You can customize the build using environment variables:

```bash
# Site URL (for sitemap and canonical links)
export SITE_URL=https://docs.rcommerce.app

# Google Analytics
export GOOGLE_ANALYTICS_KEY=G-XXXXXXXXXX

# Build
mkdocs build
```

## Custom Domain Setup

### GitHub Pages

1. Add a `CNAME` file in the `docs/` directory:
   ```
   docs.rcommerce.app
   ```

2. Configure DNS:
   - For apex domain: Add A records pointing to GitHub Pages IPs
   - For subdomain: Add CNAME record pointing to `username.github.io`

### Netlify

1. Go to Site settings → Domain management
2. Add custom domain
3. Configure DNS as instructed

### Vercel

1. Go to Project settings → Domains
2. Add custom domain
3. Configure DNS as instructed

## Versioning (Optional)

To maintain multiple versions of documentation:

```bash
# Install mike
pip install mike

# Deploy a version
mike deploy 1.0 latest

# Set default version
mike set-default latest

# Push to GitHub Pages
mike deploy --push 1.0 latest
```

## SEO Optimization

The documentation includes:

-  Sitemap generation (`sitemap.xml`)
-  robots.txt
-  Structured data (JSON-LD)
-  Open Graph meta tags
-  Twitter Card meta tags
-  Canonical URLs
-  Clean URLs (no `.html` extension)

To submit to search engines:

```bash
# Google
# Submit sitemap at: https://search.google.com/search-console

# Bing
# Submit sitemap at: https://www.bing.com/webmasters
```

## Analytics

### Google Analytics

Add to `mkdocs.yml`:

```yaml
extra:
  analytics:
    provider: google
    property: G-XXXXXXXXXX
```

### Plausible (Privacy-Friendly)

Add to `mkdocs.yml`:

```yaml
extra:
  analytics:
    provider: plausible
    domain: docs.rcommerce.app
```

## Troubleshooting

### Build Errors

```bash
# Clean build
rm -rf site/
mkdocs build --clean

# Verbose output
mkdocs build -v
```

### Search Not Working

1. Ensure `plugins.search` is enabled in `mkdocs.yml`
2. Build must complete successfully for search index to generate
3. Check browser console for JavaScript errors

### Changes Not Reflected

1. Clear browser cache (Ctrl+Shift+R or Cmd+Shift+R)
2. Check if build completed successfully
3. Verify file paths in `mkdocs.yml` navigation

## Support

For issues with:
- **MkDocs**: https://www.mkdocs.org/
- **Material Theme**: https://squidfunk.github.io/mkdocs-material/
- **R Commerce**: https://gitee.com/captainjez/gocart/issues
