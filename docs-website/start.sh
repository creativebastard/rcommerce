#!/bin/bash

# R Commerce Documentation - Quick Start Script

set -e

echo "🚀 R Commerce Documentation Website"
echo "===================================="
echo ""

# Check Python version
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 is required but not installed."
    exit 1
fi

PYTHON_VERSION=$(python3 --version 2>&1 | awk '{print $2}')
echo "✅ Python version: $PYTHON_VERSION"

# Create virtual environment if it doesn't exist
if [ ! -d "venv" ]; then
    echo "📦 Creating virtual environment..."
    python3 -m venv venv
fi

# Activate virtual environment
echo "🔧 Activating virtual environment..."
source venv/bin/activate

# Install dependencies
echo "📥 Installing dependencies..."
pip install -q -r requirements.txt

echo ""
echo "✅ Setup complete!"
echo ""
echo "Available commands:"
echo "  ./start.sh serve    - Start development server"
echo "  ./start.sh build    - Build for production"
echo "  ./start.sh deploy   - Show deployment options"
echo ""

# Handle command
case "${1:-serve}" in
    serve)
        echo "🌐 Starting development server at http://127.0.0.1:8000"
        echo "Press Ctrl+C to stop"
        echo ""
        mkdocs serve
        ;;
    build)
        echo "🏗️  Building site..."
        mkdocs build
        echo ""
        echo "✅ Build complete! Site is in the 'site/' directory"
        ;;
    deploy)
        echo "🚀 Deployment Options:"
        echo ""
        echo "1. GitHub Pages (automatic on push to main)"
        echo "2. Netlify:       npx netlify-cli deploy --prod --dir=site"
        echo "3. Vercel:        npx vercel --prod"
        echo "4. Docker:        docker-compose up -d"
        echo ""
        echo "See DEPLOYMENT.md for detailed instructions"
        ;;
    *)
        echo "Unknown command: $1"
        echo "Usage: ./start.sh [serve|build|deploy]"
        exit 1
        ;;
esac
