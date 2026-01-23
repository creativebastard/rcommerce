-- Initial schema migration for R Commerce
-- Creates core tables for products, customers, orders

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Currency type
CREATE TYPE currency AS ENUM ('USD', 'EUR', 'GBP', 'JPY', 'AUD', 'CAD', 'CNY', 'HKD', 'SGD');

-- Weight unit type
CREATE TYPE weight_unit AS ENUM ('g', 'kg', 'oz', 'lb');

-- Length unit type
CREATE TYPE length_unit AS ENUM ('cm', 'm', 'in', 'ft');

-- Inventory policy type
CREATE TYPE inventory_policy AS ENUM ('deny', 'continue');

-- Order status type
CREATE TYPE order_status AS ENUM ('pending', 'confirmed', 'processing', 'on_hold', 'completed', 'cancelled', 'refunded');

-- Fulfillment status type
CREATE TYPE fulfillment_status AS ENUM ('pending', 'processing', 'partial', 'shipped', 'delivered', 'cancelled', 'returned');

-- Payment status type
CREATE TYPE payment_status AS ENUM ('pending', 'authorized', 'paid', 'failed', 'cancelled', 'refunded');

-- Payment method type
CREATE TYPE payment_method_type AS ENUM ('credit_card', 'debit_card', 'bank_transfer', 'cash_on_delivery', 'digital_wallet', 'cryptocurrency');

-- ====================
-- PRODUCT TABLES
-- ====================

-- Products
CREATE TABLE IF NOT EXISTS products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    sku VARCHAR(100),
    price DECIMAL(20, 2) NOT NULL,
    compare_at_price DECIMAL(20, 2),
    cost_price DECIMAL(20, 2),
    currency currency NOT NULL DEFAULT 'USD',
    inventory_quantity INTEGER NOT NULL DEFAULT 0,
    inventory_policy inventory_policy NOT NULL DEFAULT 'deny',
    inventory_management BOOLEAN NOT NULL DEFAULT false,
    continues_selling_when_out_of_stock BOOLEAN NOT NULL DEFAULT false,
    weight DECIMAL(10, 4),
    weight_unit weight_unit,
    requires_shipping BOOLEAN NOT NULL DEFAULT true,
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_featured BOOLEAN NOT NULL DEFAULT false,
    seo_title VARCHAR(255),
    seo_description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    published_at TIMESTAMPTZ
);

CREATE INDEX idx_products_slug ON products(slug);
CREATE INDEX idx_products_is_active ON products(is_active);
CREATE INDEX idx_products_created_at ON products(created_at);
CREATE INDEX idx_products_price ON products(price);

-- Product variants
CREATE TABLE IF NOT EXISTS product_variants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    sku VARCHAR(100),
    price DECIMAL(20, 2) NOT NULL,
    compare_at_price DECIMAL(20, 2),
    cost_price DECIMAL(20, 2),
    currency currency NOT NULL DEFAULT 'USD',
    inventory_quantity INTEGER NOT NULL DEFAULT 0,
    inventory_policy inventory_policy NOT NULL DEFAULT 'deny',
    weight DECIMAL(10, 4),
    weight_unit weight_unit,
    requires_shipping BOOLEAN NOT NULL DEFAULT true,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_product_variants_product_id ON product_variants(product_id);
CREATE INDEX idx_product_variants_sku ON product_variants(sku);

-- Product images
CREATE TABLE IF NOT EXISTS product_images (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    variant_id UUID REFERENCES product_variants(id) ON DELETE CASCADE,
    position INTEGER NOT NULL DEFAULT 1,
    src VARCHAR(500) NOT NULL,
    alt_text VARCHAR(255),
    width INTEGER,
    height INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_product_images_product_id ON product_images(product_id);
CREATE INDEX idx_product_images_variant_id ON product_images(variant_id);

-- Product categories
CREATE TABLE IF NOT EXISTS product_categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    parent_id UUID REFERENCES product_categories(id) ON DELETE CASCADE,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_product_categories_parent_id ON product_categories(parent_id);
CREATE INDEX idx_product_categories_sort_order ON product_categories(sort_order);

-- Product tags
CREATE TABLE IF NOT EXISTS product_tags (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Product-tag relationships
CREATE TABLE IF NOT EXISTS product_tag_relations (
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES product_tags(id) ON DELETE CASCADE,
    PRIMARY KEY (product_id, tag_id)
);

-- Product-category relationships
CREATE TABLE IF NOT EXISTS product_category_relations (
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    category_id UUID NOT NULL REFERENCES product_categories(id) ON DELETE CASCADE,
    PRIMARY KEY (product_id, category_id)
);

-- Collections
CREATE TABLE IF NOT EXISTS collections (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    handle VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    seo_title VARCHAR(255),
    seo_description TEXT,
    sort_order VARCHAR(50) NOT NULL DEFAULT 'manual',
    published_at TIMESTAMPTZ,
    template_suffix VARCHAR(100),
    disjunctive BOOLEAN NOT NULL DEFAULT false,
    published_scope VARCHAR(50) NOT NULL DEFAULT 'web',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_collections_handle ON collections(handle);
CREATE INDEX idx_collections_published_at ON collections(published_at);

-- Collection-product relationships
CREATE TABLE IF NOT EXISTS collection_product_relations (
    collection_id UUID NOT NULL REFERENCES collections(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    position INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (collection_id, product_id)
);

CREATE INDEX idx_collection_products_position ON collection_product_relations(position);

-- ====================
-- CUSTOMER TABLES
-- ====================

-- Customers
CREATE TABLE IF NOT EXISTS customers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL UNIQUE,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    phone VARCHAR(20),
    accepts_marketing BOOLEAN NOT NULL DEFAULT false,
    tax_exempt BOOLEAN NOT NULL DEFAULT false,
    currency currency NOT NULL DEFAULT 'USD',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    confirmed_at TIMESTAMPTZ
);

CREATE INDEX idx_customers_email ON customers(email);
CREATE INDEX idx_customers_created_at ON customers(created_at);

-- Customer addresses
CREATE TABLE IF NOT EXISTS addresses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    company VARCHAR(255),
    phone VARCHAR(20),
    address1 VARCHAR(255) NOT NULL,
    address2 VARCHAR(255),
    city VARCHAR(100) NOT NULL,
    state VARCHAR(100),
    country VARCHAR(2) NOT NULL,
    zip VARCHAR(20) NOT NULL,
    is_default_shipping BOOLEAN NOT NULL DEFAULT false,
    is_default_billing BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_addresses_customer_id ON addresses(customer_id);
CREATE INDEX idx_addresses_country ON addresses(country);

-- Customer preferences
CREATE TABLE IF NOT EXISTS customer_preferences (
    customer_id UUID PRIMARY KEY REFERENCES customers(id) ON DELETE CASCADE,
    language VARCHAR(10),
    currency currency,
    timezone VARCHAR(100),
    marketing_opt_in BOOLEAN NOT NULL DEFAULT false,
    email_notifications BOOLEAN NOT NULL DEFAULT true,
    sms_notifications BOOLEAN NOT NULL DEFAULT false,
    push_notifications BOOLEAN NOT NULL DEFAULT false
);

-- ====================
-- ORDER TABLES
-- ====================

-- Orders
CREATE TABLE IF NOT EXISTS orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_number VARCHAR(50) NOT NULL UNIQUE,
    customer_id UUID REFERENCES customers(id) ON DELETE SET NULL,
    email VARCHAR(255) NOT NULL,
    currency currency NOT NULL DEFAULT 'USD',
    subtotal DECIMAL(20, 2) NOT NULL DEFAULT 0,
    tax_total DECIMAL(20, 2) NOT NULL DEFAULT 0,
    shipping_total DECIMAL(20, 2) NOT NULL DEFAULT 0,
    discount_total DECIMAL(20, 2) NOT NULL DEFAULT 0,
    total DECIMAL(20, 2) NOT NULL DEFAULT 0,
    status order_status NOT NULL DEFAULT 'pending',
    fulfillment_status fulfillment_status NOT NULL DEFAULT 'pending',
    payment_status payment_status NOT NULL DEFAULT 'pending',
    shipping_address_id UUID REFERENCES addresses(id) ON DELETE SET NULL,
    billing_address_id UUID REFERENCES addresses(id) ON DELETE SET NULL,
    payment_method VARCHAR(100),
    shipping_method VARCHAR(100),
    notes TEXT,
    tags TEXT[], -- Array of strings
    draft BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

CREATE INDEX idx_orders_order_number ON orders(order_number);
CREATE INDEX idx_orders_customer_id ON orders(customer_id);
CREATE INDEX idx_orders_email ON orders(email);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_fulfillment_status ON orders(fulfillment_status);
CREATE INDEX idx_orders_payment_status ON orders(payment_status);
CREATE INDEX idx_orders_created_at ON orders(created_at);
CREATE INDEX idx_orders_draft ON orders(draft);

-- Order items
CREATE TABLE IF NOT EXISTS order_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE RESTRICT,
    variant_id UUID REFERENCES product_variants(id) ON DELETE RESTRICT,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    price DECIMAL(20, 2) NOT NULL,
    subtotal DECIMAL(20, 2) NOT NULL,
    tax_amount DECIMAL(20, 2) NOT NULL DEFAULT 0,
    total DECIMAL(20, 2) NOT NULL,
    sku VARCHAR(100),
    title VARCHAR(255) NOT NULL,
    variant_title VARCHAR(255),
    requires_shipping BOOLEAN NOT NULL DEFAULT true,
    is_gift_card BOOLEAN NOT NULL DEFAULT false,
    weight DECIMAL(10, 4),
    weight_unit weight_unit,
    image_url VARCHAR(500),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_order_items_order_id ON order_items(order_id);
CREATE INDEX idx_order_items_product_id ON order_items(product_id);
CREATE INDEX idx_order_items_variant_id ON order_items(variant_id);

-- Order fulfillments
CREATE TABLE IF NOT EXISTS fulfillments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    status fulfillment_status NOT NULL DEFAULT 'pending',
    tracking_number VARCHAR(100),
    tracking_url VARCHAR(500),
    tracking_company VARCHAR(100),
    shipped_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_fulfillments_order_id ON fulfillments(order_id);
CREATE INDEX idx_fulfillments_tracking_number ON fulfillments(tracking_number);

-- Order notes
CREATE TABLE IF NOT EXISTS order_notes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    author VARCHAR(255),
    note TEXT NOT NULL,
    is_customer_notified BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_order_notes_order_id ON order_notes(order_id);

-- ====================
-- PAYMENT TABLES
-- ====================

-- Payment attempts
CREATE TABLE IF NOT EXISTS payments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    gateway VARCHAR(50) NOT NULL,
    amount DECIMAL(20, 2) NOT NULL,
    currency currency NOT NULL,
    status payment_status NOT NULL DEFAULT 'pending',
    transaction_id VARCHAR(255),
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payments_order_id ON payments(order_id);
CREATE INDEX idx_payments_transaction_id ON payments(transaction_id);
CREATE INDEX idx_payments_status ON payments(status);

-- ====================
-- AUDIT LOGS
-- ====================

-- Audit logs
CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_type VARCHAR(50) NOT NULL,
    entity_id UUID NOT NULL,
    action VARCHAR(50) NOT NULL,
    user_id UUID,
    ip_address INET,
    user_agent TEXT,
    changes JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_logs_entity ON audit_logs(entity_type, entity_id);
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);

-- ====================
-- API KEYS
-- ====================

-- API keys for authentication
CREATE TABLE IF NOT EXISTS api_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    key_prefix VARCHAR(20) NOT NULL,
    key_hash VARCHAR(255) NOT NULL, -- Hashed secret key
    user_id UUID,
    name VARCHAR(255) NOT NULL,
    permissions TEXT[] NOT NULL DEFAULT '{}', -- Array of permission strings
    is_active BOOLEAN NOT NULL DEFAULT true,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_api_keys_key_prefix ON api_keys(key_prefix);
CREATE INDEX idx_api_keys_user_id ON api_keys(user_id);
CREATE INDEX idx_api_keys_is_active ON api_keys(is_active) WHERE is_active = true;

-- ====================
-- UPDATED AT TRIGGERS
-- ====================

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Add triggers for updated_at columns
CREATE TRIGGER update_products_updated_at BEFORE UPDATE ON products FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_product_variants_updated_at BEFORE UPDATE ON product_variants FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_product_images_updated_at BEFORE UPDATE ON product_images FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_product_categories_updated_at BEFORE UPDATE ON product_categories FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_product_tags_updated_at BEFORE UPDATE ON product_tags FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_collections_updated_at BEFORE UPDATE ON collections FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_customers_updated_at BEFORE UPDATE ON customers FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_addresses_updated_at BEFORE UPDATE ON addresses FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_orders_updated_at BEFORE UPDATE ON orders FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_order_items_updated_at BEFORE UPDATE ON order_items FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_fulfillments_updated_at BEFORE UPDATE ON fulfillments FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_payments_updated_at BEFORE UPDATE ON payments FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_api_keys_updated_at BEFORE UPDATE ON api_keys FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ====================
-- SEED DATA
-- ====================

-- Insert default product tags
INSERT INTO product_tags (name) VALUES 
    ('New'),
    ('Sale'),
    ('Featured'),
    ('Bestseller')
ON CONFLICT (name) DO NOTHING;

-- Insert default categories
INSERT INTO product_categories (name, slug, sort_order) VALUES
    ('Uncategorized', 'uncategorized', 0)
ON CONFLICT (slug) DO NOTHING;