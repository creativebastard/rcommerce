// R Commerce Demo Frontend - Checkout v2 with Agnostic Payment API
// This version uses the backend API for all payment processing

// Initialize checkout page
document.addEventListener('DOMContentLoaded', async () => {
    initCart();
    updateAuthUI();
    loadCheckoutItems();
    
    // Pre-fill customer email if logged in
    const customer = getCustomer();
    if (customer && customer.email) {
        document.getElementById('customerEmail').value = customer.email;
    }
    
    // Load available payment methods
    await loadPaymentMethods();
    
    // Handle payment submission
    document.getElementById('submitPayment').addEventListener('click', handlePayment);
});

// Load available payment methods from API
async function loadPaymentMethods() {
    const container = document.getElementById('paymentMethods');
    const total = cart.reduce((sum, item) => sum + (item.price * item.quantity), 0);
    
    try {
        const response = await fetch(`${API_BASE_URL}/v2/payments/methods`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                currency: 'usd',
                amount: total.toFixed(2)
            })
        });
        
        if (!response.ok) {
            throw new Error('Failed to load payment methods');
        }
        
        const gateways = await response.json();
        
        // Render payment methods
        let html = '';
        gateways.forEach(gateway => {
            gateway.payment_methods.forEach(method => {
                html += renderPaymentMethod(gateway, method);
            });
        });
        
        container.innerHTML = html;
        
        // Add event listeners for payment method selection
        document.querySelectorAll('input[name="payment_method"]').forEach(radio => {
            radio.addEventListener('change', (e) => {
                const methodType = e.target.value;
                const gatewayId = e.target.dataset.gateway;
                showPaymentForm(gatewayId, methodType);
            });
        });
        
    } catch (error) {
        console.error('Failed to load payment methods:', error);
        container.innerHTML = '<p class="error">Failed to load payment methods. Please try again.</p>';
    }
}

// Render a payment method option
function renderPaymentMethod(gateway, method) {
    const methodId = `${gateway.gateway_id}_${method.method_type}`;
    
    return `
        <div class="payment-method-option">
            <label class="payment-method-label">
                <input type="radio" name="payment_method" value="${method.method_type}" data-gateway="${gateway.gateway_id}" id="${methodId}">
                <span class="payment-method-icon">${getPaymentMethodIcon(method.method_type)}</span>
                <span class="payment-method-name">${method.display_name}</span>
            </label>
            <div class="payment-method-form" id="form_${methodId}" style="display: none;">
                ${renderPaymentFields(method)}
            </div>
        </div>
    `;
}

// Get icon for payment method type
function getPaymentMethodIcon(type) {
    const icons = {
        card: '💳',
        google_pay: 'G',
        apple_pay: '🍎',
        alipay: 'A',
        wechat_pay: 'W',
        paypal: 'P',
        bank_transfer: '🏦',
        installments: '💰',
        crypto: '₿',
        cash: '💵'
    };
    return icons[type] || '💳';
}

// Render payment fields based on method configuration
function renderPaymentFields(method) {
    if (method.required_fields.length === 0) {
        return '<p>No additional information required.</p>';
    }
    
    return method.required_fields.map(field => {
        const inputType = getInputType(field.field_type);
        const pattern = field.pattern ? `pattern="${field.pattern}"` : '';
        const placeholder = field.placeholder ? `placeholder="${field.placeholder}"` : '';
        const helpText = field.help_text ? `<small class="help-text">${field.help_text}</small>` : '';
        
        return `
            <div class="form-group">
                <label for="field_${field.name}">${field.label}</label>
                <input 
                    type="${inputType}" 
                    id="field_${field.name}"
                    name="${field.name}"
                    class="payment-field"
                    data-field-type="${field.field_type}"
                    ${pattern}
                    ${placeholder}
                    required
                >
                ${helpText}
            </div>
        `;
    }).join('');
}

// Get HTML input type from field type
function getInputType(fieldType) {
    switch (fieldType) {
        case 'card_number':
        case 'expiry_date':
        case 'cvc':
            return 'text';
        case 'number':
            return 'number';
        case 'checkbox':
            return 'checkbox';
        case 'hidden':
            return 'hidden';
        default:
            return 'text';
    }
}

// Show payment form for selected method
function showPaymentForm(gatewayId, methodType) {
    // Hide all forms
    document.querySelectorAll('.payment-method-form').forEach(form => {
        form.style.display = 'none';
    });
    
    // Show selected form
    const formId = `form_${gatewayId}_${methodType}`;
    const form = document.getElementById(formId);
    if (form) {
        form.style.display = 'block';
    }
    
    // Store selected gateway and method
    window.selectedGateway = gatewayId;
    window.selectedMethod = methodType;
}

// Load cart items in checkout
function loadCheckoutItems() {
    const container = document.getElementById('checkoutItems');
    
    if (cart.length === 0) {
        window.location.href = 'cart.html';
        return;
    }
    
    container.innerHTML = cart.map(item => `
        <div class="checkout-item">
            <span class="checkout-item-name">${escape(item.name)}</span>
            <span class="checkout-item-qty">x${item.quantity}</span>
            <span class="checkout-item-price">$${(item.price * item.quantity).toFixed(2)}</span>
        </div>
    `).join('');
    
    const subtotal = cart.reduce((sum, item) => sum + (item.price * item.quantity), 0);
    document.getElementById('subtotal').textContent = `$${subtotal.toFixed(2)}`;
    document.getElementById('total').textContent = `$${subtotal.toFixed(2)}`;
}

// Handle payment submission
async function handlePayment() {
    const submitBtn = document.getElementById('submitPayment');
    const email = document.getElementById('customerEmail').value;
    
    if (!email) {
        showMessage('Please enter your email', 'error');
        return;
    }
    
    if (!window.selectedGateway || !window.selectedMethod) {
        showMessage('Please select a payment method', 'error');
        return;
    }
    
    submitBtn.disabled = true;
    submitBtn.textContent = 'Processing...';
    
    try {
        // Calculate total
        const total = cart.reduce((sum, item) => sum + (item.price * item.quantity), 0);
        
        // Collect payment method data
        const paymentMethodData = collectPaymentMethodData(window.selectedMethod);
        
        // Create payment via API
        const paymentResponse = await fetch(`${API_BASE_URL}/v2/payments`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                gateway_id: window.selectedGateway,
                amount: total.toFixed(2),
                currency: 'usd',
                payment_method_type: window.selectedMethod,
                order_id: null, // Will be created after payment
                customer_email: email,
                payment_method_data: paymentMethodData,
                save_payment_method: false,
                description: `Order from ${window.location.hostname}`
            })
        });
        
        if (!paymentResponse.ok) {
            const error = await paymentResponse.json();
            throw new Error(error.message || 'Payment failed');
        }
        
        const paymentResult = await paymentResponse.json();
        
        // Handle different response types
        switch (paymentResult.status) {
            case 'success':
                // Payment succeeded immediately
                await createOrder(paymentResult.payment_id, email, total);
                break;
                
            case 'requires_action':
                // Additional action required (3DS, redirect, etc.)
                await handlePaymentAction(paymentResult);
                break;
                
            case 'failed':
                // Payment failed
                throw new Error(paymentResult.error_message || 'Payment failed');
                
            default:
                throw new Error('Unexpected payment response');
        }
        
    } catch (error) {
        console.error('Payment failed:', error);
        showMessage(error.message, 'error');
        submitBtn.disabled = false;
        submitBtn.textContent = 'Pay Now';
    }
}

// Collect payment method data from form
function collectPaymentMethodData(methodType) {
    const data = { type: methodType };
    
    // Collect fields based on method type
    document.querySelectorAll('.payment-field').forEach(field => {
        if (field.closest('.payment-method-form').style.display !== 'none') {
            data[field.name] = field.value;
        }
    });
    
    return data;
}

// Handle payment that requires additional action
async function handlePaymentAction(paymentResult) {
    const { payment_id, action_type, action_data } = paymentResult;
    
    switch (action_type) {
        case 'three_d_secure':
            // Handle 3D Secure
            if (action_data.redirect_url) {
                // Redirect to 3DS page
                window.location.href = action_data.redirect_url;
            } else if (action_data.use_stripe_sdk) {
                // Handle Stripe 3DS via SDK (would need Stripe.js for this)
                showMessage('3D Secure authentication required. This demo does not support 3DS.', 'error');
            }
            break;
            
        case 'redirect':
            // Redirect to payment provider (PayPal, Alipay, etc.)
            if (action_data.redirect_url) {
                window.location.href = action_data.redirect_url;
            }
            break;
            
        case 'challenge':
            // Show challenge/OTP form
            showMessage('Additional verification required. This demo does not support challenges.', 'error');
            break;
            
        default:
            showMessage('Unknown payment action required', 'error');
    }
}

// Create order after successful payment
async function createOrder(paymentId, email, total) {
    try {
        const orderData = {
            items: cart.map(item => ({
                product_id: item.productId,
                quantity: item.quantity,
                price: item.price
            })),
            total: total,
            email: email,
            payment_id: paymentId
        };
        
        // Add auth header if logged in
        const headers = {
            'Content-Type': 'application/json'
        };
        if (isAuthenticated()) {
            headers['Authorization'] = `Bearer ${getToken()}`;
        }
        
        const response = await fetch(`${API_BASE_URL}/orders`, {
            method: 'POST',
            headers: headers,
            body: JSON.stringify(orderData)
        });
        
        let order;
        if (response.ok) {
            order = await response.json();
        } else {
            // Demo mode - create mock order
            order = {
                order: {
                    id: 'ord_' + Math.random().toString(36).substr(2, 9),
                    status: 'confirmed',
                    total: total,
                    created_at: new Date().toISOString()
                }
            };
        }
        
        // Clear cart
        cart = [];
        saveCart();
        updateCartCount();
        
        // Redirect to confirmation page
        const orderId = order.order?.id || order.id;
        window.location.href = `confirmation.html?order_id=${orderId}`;
        
    } catch (error) {
        console.error('Order creation failed:', error);
        // Still redirect to confirmation in demo mode
        window.location.href = `confirmation.html?order_id=demo_${Date.now()}`;
    }
}

// Utility function to escape HTML
function escape(html) {
    const div = document.createElement('div');
    div.textContent = html;
    return div.innerHTML;
}

// Show message to user
function showMessage(message, type = 'info') {
    const container = document.getElementById('messageContainer') || document.body;
    const messageEl = document.createElement('div');
    messageEl.className = `message message-${type}`;
    messageEl.textContent = message;
    container.appendChild(messageEl);
    
    setTimeout(() => {
        messageEl.remove();
    }, 5000);
}
