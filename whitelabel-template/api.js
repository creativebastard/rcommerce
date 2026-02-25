// R Commerce API Client for Whitelabel Template
// This uses relative URLs - works with rcommerce-frontend-server

const API_BASE = '/api/v1';

const api = {
    // Products
    async getProducts() {
        const res = await fetch(`${API_BASE}/products`);
        if (!res.ok) throw new Error('Failed to fetch products');
        const data = await res.json();
        return data.products || [];
    },

    // Cart
    async addToCart(productId, quantity = 1) {
        // For demo, use localStorage
        let cart = JSON.parse(localStorage.getItem('cart') || '[]');
        const existing = cart.find(item => item.product_id === productId);
        if (existing) {
            existing.quantity += quantity;
        } else {
            cart.push({ product_id: productId, quantity });
        }
        localStorage.setItem('cart', JSON.stringify(cart));
        alert('Added to cart!');
        return cart;
    },

    // Auth
    async login(email, password) {
        const res = await fetch(`${API_BASE}/auth/login`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email, password })
        });
        if (!res.ok) throw new Error('Login failed');
        const data = await res.json();
        localStorage.setItem('token', data.access_token);
        return data;
    }
};
