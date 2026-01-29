// R Commerce Demo Frontend - Main App

// API Configuration
const API_BASE_URL = 'http://localhost:8080/api/v1';

// Demo Products (fallback if API is not available)
const DEMO_PRODUCTS = [
    {
        id: 'prod-1',
        name: 'Premium Wireless Headphones',
        description: 'High-quality wireless headphones with noise cancellation and 30-hour battery life. Perfect for music lovers and professionals.',
        price: 299.99,
        emoji: '🎧'
    },
    {
        id: 'prod-2',
        name: 'Smart Watch Pro',
        description: 'Advanced fitness tracking, heart rate monitoring, and smartphone notifications. Water-resistant up to 50 meters.',
        price: 399.99,
        emoji: '⌚'
    },
    {
        id: 'prod-3',
        name: 'Portable Bluetooth Speaker',
        description: '360-degree sound, 20-hour battery, waterproof design. Take your music anywhere with this compact powerhouse.',
        price: 149.99,
        emoji: '🔊'
    }
];

// Cart State
let cart = JSON.parse(localStorage.getItem('cart')) || [];

// Initialize App
document.addEventListener('DOMContentLoaded', () => {
    initCart();
    loadProducts();
});

// Initialize Cart Count
function initCart() {
    updateCartCount();
}

// Update Cart Count in Header
function updateCartCount() {
    const count = cart.reduce((sum, item) => sum + item.quantity, 0);
    const countElement = document.getElementById('cartCount');
    if (countElement) {
        countElement.textContent = count;
    }
}

// Load Products
async function loadProducts() {
    const grid = document.getElementById('productsGrid');
    if (!grid) return;

    try {
        // Try to fetch from API
        const response = await fetch(`${API_BASE_URL}/products`);
        
        if (response.ok) {
            const data = await response.json();
            renderProducts(data.data || data);
        } else {
            // Use demo products if API fails
            renderProducts(DEMO_PRODUCTS);
        }
    } catch (error) {
        console.log('API not available, using demo products');
        renderProducts(DEMO_PRODUCTS);
    }
}

// Render Products Grid
function renderProducts(products) {
    const grid = document.getElementById('productsGrid');
    if (!grid) return;

    grid.innerHTML = products.map(product => `
        <div class="product-card">
            <div class="product-image">${product.emoji || '📦'}</div>
            <div class="product-info">
                <h3 class="product-name">${product.name || product.title}</h3>
                <p class="product-description">${product.description}</p>
                <div class="product-footer">
                    <span class="product-price">$${product.price}</span>
                    <a href="product.html?id=${product.id}" class="btn btn-primary">View</a>
                </div>
            </div>
        </div>
    `).join('');
}

// Add to Cart
function addToCart(productId, name, price, quantity = 1) {
    const existingItem = cart.find(item => item.productId === productId);
    
    if (existingItem) {
        existingItem.quantity += quantity;
    } else {
        cart.push({
            productId,
            name,
            price,
            quantity
        });
    }
    
    saveCart();
    updateCartCount();
    showMessage('Added to cart!', 'success');
}

// Remove from Cart
function removeFromCart(productId) {
    cart = cart.filter(item => item.productId !== productId);
    saveCart();
    updateCartCount();
    renderCart();
}

// Update Cart Quantity
function updateQuantity(productId, quantity) {
    const item = cart.find(item => item.productId === productId);
    if (item) {
        if (quantity <= 0) {
            removeFromCart(productId);
        } else {
            item.quantity = quantity;
            saveCart();
            updateCartCount();
            renderCart();
        }
    }
}

// Save Cart to LocalStorage
function saveCart() {
    localStorage.setItem('cart', JSON.stringify(cart));
}

// Render Cart Page
function renderCart() {
    const cartItems = document.getElementById('cartItems');
    const cartEmpty = document.getElementById('cartEmpty');
    const cartSummary = document.getElementById('cartSummary');
    
    if (!cartItems) return;

    if (cart.length === 0) {
        cartItems.style.display = 'none';
        if (cartSummary) cartSummary.style.display = 'none';
        if (cartEmpty) cartEmpty.style.display = 'block';
        return;
    }

    cartItems.style.display = 'block';
    if (cartSummary) cartSummary.style.display = 'block';
    if (cartEmpty) cartEmpty.style.display = 'none';

    cartItems.innerHTML = cart.map(item => `
        <div class="cart-item">
            <div class="cart-item-image">📦</div>
            <div class="cart-item-info">
                <h3>${item.name}</h3>
                <p>$${item.price} each</p>
            </div>
            <div class="quantity-selector">
                <button class="quantity-btn" onclick="updateQuantity('${item.productId}', ${item.quantity - 1})">-</button>
                <input type="number" class="quantity-input" value="${item.quantity}" min="1" onchange="updateQuantity('${item.productId}', parseInt(this.value))">
                <button class="quantity-btn" onclick="updateQuantity('${item.productId}', ${item.quantity + 1})">+</button>
            </div>
            <div class="cart-item-total">$${(item.price * item.quantity).toFixed(2)}</div>
        </div>
    `).join('');

    // Update summary
    const subtotal = cart.reduce((sum, item) => sum + (item.price * item.quantity), 0);
    const totalElement = document.getElementById('cartTotal');
    if (totalElement) {
        totalElement.textContent = `$${subtotal.toFixed(2)}`;
    }
}

// Show Message
function showMessage(text, type = 'success') {
    const message = document.createElement('div');
    message.className = `message message-${type}`;
    message.textContent = text;
    
    const main = document.querySelector('.main');
    if (main) {
        main.insertBefore(message, main.firstChild);
        setTimeout(() => message.remove(), 3000);
    }
}

// Format Price
function formatPrice(price) {
    return `$${parseFloat(price).toFixed(2)}`;
}
