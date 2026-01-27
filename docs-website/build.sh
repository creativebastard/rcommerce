#!/bin/bash

# R Commerce Documentation Build Script

set -e

echo "🚀 Building R Commerce Documentation..."

# Check if virtual environment exists
if [ ! -d "venv" ]; then
    echo "📦 Creating virtual environment..."
    python3 -m venv venv
fi

# Activate virtual environment
echo "🔧 Activating virtual environment..."
source venv/bin/activate

# Install dependencies
echo "📥 Installing dependencies..."
pip install -r requirements.txt --quiet

# Build the documentation
echo "🏗️  Building site..."
mkdocs build

# Check if build succeeded
if [ -d "site" ]; then
    echo "✅ Build successful!"
    echo ""
    echo "📊 Build Statistics:"
    echo "  - Total files: $(find site -type f | wc -l)"
    echo "  - HTML files: $(find site -name '*.html' | wc -l)"
    echo "  - CSS files: $(find site -name '*.css' | wc -l)"
    echo "  - JS files: $(find site -name '*.js' | wc -l)"
    echo "  - Total size: $(du -sh site | cut -f1)"
    echo ""
    echo "🌐 To preview locally, run: mkdocs serve"
    echo "📦 To deploy, upload the 'site/' directory to your hosting provider"
else
    echo "❌ Build failed!"
    exit 1
fi
