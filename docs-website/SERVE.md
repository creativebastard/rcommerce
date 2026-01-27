# Serve Documentation Locally

## Option 1: Using Python (Simplest)

```bash
cd docs-website/site
python3 -m http.server 8000
```

Visit: http://localhost:8000

## Option 2: Using MkDocs (With Live Reload)

```bash
cd docs-website
source venv/bin/activate
mkdocs serve
```

Visit: http://127.0.0.1:8000

Changes to markdown files will auto-reload.

## Option 3: Using Docker

```bash
cd docs-website
docker-compose up docs
```

Visit: http://localhost:8080

## Option 4: Using Node (npx)

```bash
cd docs-website/site
npx serve -p 8000
```

Visit: http://localhost:8000
