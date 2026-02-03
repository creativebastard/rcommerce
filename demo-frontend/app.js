// R Commerce Demo Frontend - Main App
// Reads all data from the R Commerce API

const API_BASE_URL = 'http://localhost:8080/api/v1';
let cart = JSON.parse(localStorage.getItem('cart')) || [];

document.addEventListener('DOMContentLoaded', () => {
    initCart();
    loadProducts();
});

function initCart() {
    updateCartCount();
}

function updateCartCount() {
    const count = cart.reduce((sum, item) => sum + item.quantity, 0);
    const el = document.getElementById('cartCount');
    if (el) el.textContent = count;
}

async function loadProducts() {
    const grid = document.getElementById('productsGrid');
    if (!grid) return;

    // Check authentication first
    if (!isAuthenticated()) {
        grid.innerHTML = `
            <div class="auth-required">
                <p>Please login to view products</p>
                <button class="btn btn-primary" onclick="showLoginModal()">Login</button>
            </div>`;
        return;
    }

    grid.innerHTML = '<p class="loading">Loading products...</p>';

    try {
        const response = await fetch(`${API_BASE_URL}/products`, {
            headers: {
                'Authorization': `Bearer ${getToken()}`
            }
        });
        
        if (response.status === 401) {
            logout();
            grid.innerHTML = `
                <div class="auth-required">
                    <p>Session expired. Please login again.</p>
                    <button class="btn btn-primary" onclick="showLoginModal()">Login</button>
                </div>`;
            return;
        }
        
        if (!response.ok) throw new Error(`HTTP ${response.status}`);
        
        const data = await response.json();
        const products = data.products || data.data || data;
        
        if (!products || products.length === 0) {
            grid.innerHTML = '<p class="empty">No products available.</p>';
            return;
        }
        
        renderProducts(products);
    } catch (error) {
        console.error('Error loading products:', error);
        grid.innerHTML = `
            <div class="error-message">
                <p>Unable to load products from API.</p>
                <p>Make sure R Commerce backend is running at ${API_BASE_URL}</p>
            </div>`;
    }
}

function renderProducts(products) {
    const grid = document.getElementById('productsGrid');
    grid.innerHTML = products.map(p => `
        <div class="product-card">
            <div class="product-image">${getEmoji(p.title)}</div>
            <div class="product-info">
                <h3 class="product-name">${escape(p.title)}</h3>
                <p class="product-description">${truncate(p.description, 100)}</p>
                <div class="product-footer">
                    <span class="product-price">$${parseFloat(p.price).toFixed(2)}</span>
                    <a href="product.html?id=${p.id}" class="btn btn-primary">View</a>
                </div>
            </div>
        </div>
    `).join('');
}

function getEmoji(title) {
    const t = (title || '').toLowerCase();
    if (t.includes('headphone')) return '🎧';
    if (t.includes('watch')) return '⌚';
    if (t.includes('speaker')) return '🔊';
    return '📦';
}

function escape(text) {
    if (!text) return '';
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

function truncate(text, len) {
    if (!text) return '';
    return text.length > len ? text.substring(0, len) + '...' : text;
}

function addToCart(id, name, price, qty = 1) {
    const item = cart.find(i => i.productId === id);
    if (item) item.quantity += qty;
    else cart.push({ productId: id, name, price: parseFloat(price), quantity: qty });
    saveCart();
    updateCartCount();
    showMessage('Added to cart!', 'success');
}

function removeFromCart(id) {
    cart = cart.filter(i => i.productId !== id);
    saveCart();
    updateCartCount();
    renderCart();
}

function updateQuantity(id, qty) {
    const item = cart.find(i => i.productId === id);
    if (!item) return;
    if (qty <= 0) removeFromCart(id);
    else {
        item.quantity = qty;
        saveCart();
        updateCartCount();
        renderCart();
    }
}

function saveCart() {
    localStorage.setItem('cart', JSON.stringify(cart));
}

function renderCart() {
    const items = document.getElementById('cartItems');
    const empty = document.getElementById('cartEmpty');
    const summary = document.getElementById('cartSummary');
    
    if (!items) return;
    
    console.log('Rendering cart:', cart);

    if (cart.length === 0) {
        items.style.display = 'none';
        if (summary) summary.style.display = 'none';
        if (empty) empty.style.display = 'block';
        return;
    }

    items.style.display = 'block';
    if (summary) summary.style.display = 'block';
    if (empty) empty.style.display = 'none';

    items.innerHTML = cart.map(item => `
        <div class="cart-item">
            <div class="cart-item-image">${getEmoji(item.name)}</div>
            <div class="cart-item-info">
                <h3>${escape(item.name)}</h3>
                <p>$${item.price.toFixed(2)} each</p>
            </div>
            <div class="quantity-selector">
                <button class="quantity-btn" onclick="updateQuantity('${item.productId}', ${item.quantity - 1})">-</button>
                <input type="number" class="quantity-input" value="${item.quantity}" min="1" onchange="updateQuantity('${item.productId}', parseInt(this.value))">
                <button class="quantity-btn" onclick="updateQuantity('${item.productId}', ${item.quantity + 1})">+</button>
            </div>
            <div class="cart-item-total">$${(item.price * item.quantity).toFixed(2)}</div>
        </div>
    `).join('');

    const subtotal = cart.reduce((sum, i) => sum + (i.price * i.quantity), 0);
    const totalEl = document.getElementById('cartTotal');
    if (totalEl) totalEl.textContent = `$${subtotal.toFixed(2)}`;
}

function showMessage(text, type = 'success') {
    const msg = document.createElement('div');
    msg.className = `message message-${type}`;
    msg.textContent = text;
    const main = document.querySelector('.main');
    if (main) {
        main.insertBefore(msg, main.firstChild);
        setTimeout(() => msg.remove(), 3000);
    }
}

async function checkout() {
    if (cart.length === 0) {
        showMessage('Your cart is empty!', 'error');
        return;
    }
    
    // Check if user is authenticated
    if (!isAuthenticated()) {
        showMessage('Please login to complete your order', 'error');
        showLoginModal();
        return;
    }
    
    try {
        const orderData = {
            items: cart.map(i => ({ product_id: i.productId, quantity: i.quantity, price: i.price })),
            total: cart.reduce((sum, i) => sum + (i.price * i.quantity), 0)
        };
        
        const response = await fetch(`${API_BASE_URL}/orders`, {
            method: 'POST',
            headers: { 
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${getToken()}`
            },
            body: JSON.stringify(orderData)
        });
        
        if (response.ok) {
            showMessage('Order placed successfully!', 'success');
            cart = [];
            saveCart();
            updateCartCount();
            renderCart();
        } else if (response.status === 401) {
            showMessage('Session expired. Please login again.', 'error');
            logout();
            showLoginModal();
        } else {
            const errorData = await response.json().catch(() => ({}));
            throw new Error(errorData.error || 'Order failed');
        }
    } catch (error) {
        showMessage(error.message || 'Order failed. Please try again.', 'error');
    }
}
