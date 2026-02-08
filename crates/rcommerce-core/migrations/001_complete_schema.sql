-- ============================================================================
-- R Commerce Complete Database Schema
-- ============================================================================
-- This is a comprehensive, single-file migration that creates the ENTIRE
-- database schema from scratch. It drops all existing tables and recreates
-- everything in the correct dependency order.
--
-- Run this on a fresh database or use it to reset an existing database.
-- ============================================================================

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ============================================================================
-- STEP 1: DROP ALL EXISTING TABLES (Clean Slate)
-- ============================================================================

DROP TABLE IF EXISTS dunning_emails CASCADE;
DROP TABLE IF EXISTS payment_retry_attempts CASCADE;
DROP TABLE IF EXISTS subscription_invoices CASCADE;
DROP TABLE IF EXISTS subscription_items CASCADE;
DROP TABLE IF EXISTS subscriptions CASCADE;
DROP TABLE IF EXISTS license_keys CASCADE;
DROP TABLE IF EXISTS order_item_downloads CASCADE;
DROP TABLE IF EXISTS bundle_components CASCADE;
DROP TABLE IF EXISTS collection_products CASCADE;
DROP TABLE IF EXISTS collections CASCADE;
DROP TABLE IF EXISTS product_categories CASCADE;
DROP TABLE IF EXISTS order_notes CASCADE;
DROP TABLE IF EXISTS fulfillments CASCADE;
DROP TABLE IF EXISTS refunds CASCADE;
DROP TABLE IF EXISTS payments CASCADE;
DROP TABLE IF EXISTS order_items CASCADE;
DROP TABLE IF EXISTS orders CASCADE;
DROP TABLE IF EXISTS cart_items CASCADE;
DROP TABLE IF EXISTS carts CASCADE;
DROP TABLE IF EXISTS coupon_usages CASCADE;
DROP TABLE IF EXISTS coupon_applications CASCADE;
DROP TABLE IF EXISTS coupons CASCADE;
DROP TABLE IF EXISTS api_keys CASCADE;
DROP TABLE IF EXISTS addresses CASCADE;
DROP TABLE IF EXISTS customers CASCADE;
DROP TABLE IF EXISTS product_option_values CASCADE;
DROP TABLE IF EXISTS product_options CASCADE;
DROP TABLE IF EXISTS product_images CASCADE;
DROP TABLE IF EXISTS product_variants CASCADE;
DROP TABLE IF EXISTS products CASCADE;

-- Drop all enum types
DROP TYPE IF EXISTS dunning_email_type CASCADE;
DROP TYPE IF EXISTS invoice_status CASCADE;
DROP TYPE IF EXISTS cancellation_reason CASCADE;
DROP TYPE IF EXISTS subscription_status CASCADE;
DROP TYPE IF EXISTS customer_role CASCADE;
DROP TYPE IF EXISTS discount_type CASCADE;
DROP TYPE IF EXISTS payment_method_type CASCADE;
DROP TYPE IF EXISTS payment_status CASCADE;
DROP TYPE IF EXISTS fulfillment_status CASCADE;
DROP TYPE IF EXISTS order_status CASCADE;
DROP TYPE IF EXISTS order_type CASCADE;
DROP TYPE IF EXISTS subscription_interval CASCADE;
DROP TYPE IF EXISTS bundle_pricing_strategy CASCADE;
DROP TYPE IF EXISTS product_type CASCADE;
DROP TYPE IF EXISTS inventory_policy CASCADE;
DROP TYPE IF EXISTS length_unit CASCADE;
DROP TYPE IF EXISTS weight_unit CASCADE;
DROP TYPE IF EXISTS currency CASCADE;
DROP TYPE IF EXISTS address_type CASCADE;

-- ============================================================================
-- STEP 2: CREATE ENUM TYPES
-- ============================================================================

-- Currency type
CREATE TYPE currency AS ENUM ('USD', 'EUR', 'GBP', 'JPY', 'AUD', 'CAD', 'CNY', 'HKD', 'SGD');

-- Weight unit type
CREATE TYPE weight_unit AS ENUM ('g', 'kg', 'oz', 'lb');

-- Length unit type
CREATE TYPE length_unit AS ENUM ('cm', 'm', 'in', 'ft');

-- Inventory policy type
CREATE TYPE inventory_policy AS ENUM ('deny', 'continue');

-- Product type
CREATE TYPE product_type AS ENUM ('simple', 'variable', 'subscription', 'digital', 'bundle');

-- Bundle pricing strategy
CREATE TYPE bundle_pricing_strategy AS ENUM ('fixed', 'sum', 'percentage_discount');

-- Subscription interval type
CREATE TYPE subscription_interval AS ENUM ('daily', 'weekly', 'bi_weekly', 'monthly', 'quarterly', 'bi_annually', 'annually');

-- Order type
CREATE TYPE order_type AS ENUM ('one_time', 'subscription_initial', 'subscription_renewal');

-- Order status type
CREATE TYPE order_status AS ENUM ('pending', 'confirmed', 'processing', 'on_hold', 'completed', 'cancelled', 'refunded');

-- Fulfillment status type
CREATE TYPE fulfillment_status AS ENUM ('pending', 'processing', 'partial', 'shipped', 'delivered', 'cancelled', 'returned');

-- Payment status type
CREATE TYPE payment_status AS ENUM ('pending', 'authorized', 'paid', 'failed', 'cancelled', 'refunded');

-- Payment method type
CREATE TYPE payment_method_type AS ENUM ('credit_card', 'debit_card', 'bank_transfer', 'cash_on_delivery', 'digital_wallet', 'cryptocurrency');

-- Discount type
CREATE TYPE discount_type AS ENUM ('percentage', 'fixed_amount', 'free_shipping', 'buy_x_get_y');

-- Customer role type
CREATE TYPE customer_role AS ENUM ('customer', 'manager', 'admin');

-- Address type
CREATE TYPE address_type AS ENUM ('shipping', 'billing');

-- Subscription status type
CREATE TYPE subscription_status AS ENUM ('active', 'paused', 'cancelled', 'expired', 'past_due', 'trialing', 'pending');

-- Cancellation reason type
CREATE TYPE cancellation_reason AS ENUM ('customer_requested', 'payment_failed', 'fraudulent', 'too_expensive', 'not_useful', 'other');

-- Invoice status type
CREATE TYPE invoice_status AS ENUM ('pending', 'billed', 'paid', 'failed', 'past_due', 'cancelled');

-- Dunning email type
CREATE TYPE dunning_email_type AS ENUM ('first_failure', 'retry_failure', 'final_notice', 'cancellation_notice', 'payment_recovered');

-- ============================================================================
-- STEP 3: CREATE INDEPENDENT TABLES (No Foreign Keys)
-- ============================================================================

-- Products table
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    sku VARCHAR(100),
    product_type product_type NOT NULL DEFAULT 'simple',
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
    -- Subscription fields
    subscription_interval subscription_interval,
    subscription_interval_count INTEGER,
    subscription_trial_days INTEGER,
    subscription_setup_fee DECIMAL(20, 2),
    subscription_min_cycles INTEGER,
    subscription_max_cycles INTEGER,
    -- Digital product fields
    file_url TEXT,
    file_size BIGINT,
    file_hash TEXT,
    download_limit INTEGER,
    license_key_enabled BOOLEAN DEFAULT FALSE,
    download_expiry_days INTEGER,
    -- Bundle product fields
    bundle_pricing_strategy bundle_pricing_strategy,
    bundle_discount_percentage DECIMAL(5,2),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    published_at TIMESTAMPTZ
);

-- Customers table
CREATE TABLE customers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL UNIQUE,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone VARCHAR(50),
    accepts_marketing BOOLEAN NOT NULL DEFAULT false,
    tax_exempt BOOLEAN NOT NULL DEFAULT false,
    currency currency NOT NULL DEFAULT 'USD',
    confirmed_at TIMESTAMPTZ,
    timezone VARCHAR(50),
    marketing_opt_in BOOLEAN NOT NULL DEFAULT false,
    email_notifications BOOLEAN NOT NULL DEFAULT true,
    sms_notifications BOOLEAN NOT NULL DEFAULT false,
    push_notifications BOOLEAN NOT NULL DEFAULT false,
    password_hash VARCHAR(255),
    is_verified BOOLEAN NOT NULL DEFAULT false,
    last_login_at TIMESTAMPTZ,
    role customer_role NOT NULL DEFAULT 'customer',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Coupons table
CREATE TABLE coupons (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) NOT NULL UNIQUE,
    description TEXT,
    discount_type discount_type NOT NULL,
    discount_value DECIMAL(19, 4) NOT NULL,
    minimum_purchase DECIMAL(19, 4),
    maximum_discount DECIMAL(19, 4),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    starts_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    usage_limit INTEGER,
    usage_limit_per_customer INTEGER,
    usage_count INTEGER NOT NULL DEFAULT 0,
    applies_to_specific_products BOOLEAN NOT NULL DEFAULT FALSE,
    applies_to_specific_collections BOOLEAN NOT NULL DEFAULT FALSE,
    can_combine BOOLEAN NOT NULL DEFAULT FALSE,
    created_by UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- API Keys table
CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID REFERENCES customers(id) ON DELETE CASCADE,
    key_prefix VARCHAR(16) NOT NULL UNIQUE,
    key_hash VARCHAR(64) NOT NULL,
    name VARCHAR(100) NOT NULL DEFAULT 'API Key',
    scopes TEXT[] NOT NULL DEFAULT ARRAY['read'],
    expires_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    last_used_ip VARCHAR(45),
    rate_limit_per_minute INTEGER,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_at TIMESTAMPTZ,
    revoked_reason TEXT
);

-- Product Categories table
CREATE TABLE product_categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    parent_id UUID REFERENCES product_categories(id) ON DELETE SET NULL,
    image_url TEXT,
    seo_title VARCHAR(255),
    seo_description TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Collections table
CREATE TABLE collections (
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
    published_scope VARCHAR(20) NOT NULL DEFAULT 'web',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- STEP 4: CREATE TABLES WITH FOREIGN KEYS (Level 1 - Direct deps on independent tables)
-- ============================================================================

-- Product Variants table
CREATE TABLE product_variants (
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

-- Product Images table
CREATE TABLE product_images (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    variant_id UUID REFERENCES product_variants(id) ON DELETE CASCADE,
    position INTEGER NOT NULL DEFAULT 0,
    src VARCHAR(500) NOT NULL,
    alt_text VARCHAR(255),
    width INTEGER,
    height INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Product Options table
CREATE TABLE product_options (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    position INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Addresses table
CREATE TABLE addresses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    company VARCHAR(100),
    address1 VARCHAR(255) NOT NULL,
    address2 VARCHAR(255),
    city VARCHAR(100) NOT NULL,
    province VARCHAR(100),
    country VARCHAR(100) NOT NULL,
    zip VARCHAR(20) NOT NULL,
    phone VARCHAR(50),
    is_default_shipping BOOLEAN NOT NULL DEFAULT false,
    is_default_billing BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Carts table
CREATE TABLE carts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID REFERENCES customers(id) ON DELETE SET NULL,
    session_token VARCHAR(255),
    currency currency NOT NULL DEFAULT 'USD',
    subtotal DECIMAL(19, 4) NOT NULL DEFAULT 0,
    discount_total DECIMAL(19, 4) NOT NULL DEFAULT 0,
    tax_total DECIMAL(19, 4) NOT NULL DEFAULT 0,
    shipping_total DECIMAL(19, 4) NOT NULL DEFAULT 0,
    total DECIMAL(19, 4) NOT NULL DEFAULT 0,
    coupon_code VARCHAR(50),
    email VARCHAR(255),
    shipping_address_id UUID REFERENCES addresses(id) ON DELETE SET NULL,
    billing_address_id UUID REFERENCES addresses(id) ON DELETE SET NULL,
    shipping_method VARCHAR(100),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    converted_to_order BOOLEAN NOT NULL DEFAULT FALSE,
    order_id UUID
);

-- Orders table
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_number VARCHAR(50) NOT NULL UNIQUE,
    customer_id UUID REFERENCES customers(id) ON DELETE SET NULL,
    email VARCHAR(255) NOT NULL,
    currency currency NOT NULL DEFAULT 'USD',
    order_type order_type NOT NULL DEFAULT 'one_time',
    subtotal DECIMAL(20, 2) NOT NULL DEFAULT 0,
    tax_total DECIMAL(20, 2) NOT NULL DEFAULT 0,
    shipping_total DECIMAL(20, 2) NOT NULL DEFAULT 0,
    discount_total DECIMAL(20, 2) NOT NULL DEFAULT 0,
    total DECIMAL(20, 2) NOT NULL DEFAULT 0,
    status order_status NOT NULL DEFAULT 'pending',
    payment_status payment_status NOT NULL DEFAULT 'pending',
    fulfillment_status fulfillment_status,
    shipping_address_id UUID REFERENCES addresses(id) ON DELETE SET NULL,
    billing_address_id UUID REFERENCES addresses(id) ON DELETE SET NULL,
    shipping_address JSONB,
    billing_address JSONB,
    payment_method VARCHAR(100),
    shipping_method VARCHAR(100),
    notes TEXT,
    tags TEXT[] DEFAULT ARRAY[]::TEXT[],
    metadata JSONB DEFAULT '{}'::JSONB,
    draft BOOLEAN NOT NULL DEFAULT false,
    subscription_id UUID,
    billing_cycle INTEGER,
    coupon_code VARCHAR(50),
    coupon_id UUID REFERENCES coupons(id) ON DELETE SET NULL,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ
);

-- Fix carts foreign key to orders (now that orders table exists)
ALTER TABLE carts ADD CONSTRAINT fk_carts_order_id 
    FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE SET NULL;

-- Subscriptions table
CREATE TABLE subscriptions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    variant_id UUID REFERENCES product_variants(id) ON DELETE SET NULL,
    status subscription_status NOT NULL DEFAULT 'pending',
    interval subscription_interval NOT NULL,
    interval_count INTEGER NOT NULL DEFAULT 1,
    currency currency NOT NULL DEFAULT 'USD',
    amount DECIMAL(20, 2) NOT NULL,
    setup_fee DECIMAL(20, 2),
    trial_days INTEGER NOT NULL DEFAULT 0,
    trial_ends_at TIMESTAMPTZ,
    current_cycle INTEGER NOT NULL DEFAULT 1,
    min_cycles INTEGER,
    max_cycles INTEGER,
    starts_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    next_billing_at TIMESTAMPTZ NOT NULL,
    last_billing_at TIMESTAMPTZ,
    ends_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    cancellation_reason cancellation_reason,
    payment_method_id VARCHAR(255),
    gateway VARCHAR(100) NOT NULL DEFAULT 'stripe',
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- STEP 5: CREATE JUNCTION AND CHILD TABLES (Level 2)
-- ============================================================================

-- Product Option Values table
CREATE TABLE product_option_values (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    option_id UUID NOT NULL REFERENCES product_options(id) ON DELETE CASCADE,
    variant_id UUID NOT NULL REFERENCES product_variants(id) ON DELETE CASCADE,
    value VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Cart Items table
CREATE TABLE cart_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cart_id UUID NOT NULL REFERENCES carts(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    variant_id UUID REFERENCES product_variants(id) ON DELETE SET NULL,
    quantity INTEGER NOT NULL CHECK (quantity > 0 AND quantity <= 9999),
    unit_price DECIMAL(19, 4) NOT NULL,
    original_price DECIMAL(19, 4) NOT NULL,
    subtotal DECIMAL(19, 4) NOT NULL,
    discount_amount DECIMAL(19, 4) NOT NULL DEFAULT 0,
    total DECIMAL(19, 4) NOT NULL,
    sku VARCHAR(255),
    title VARCHAR(500) NOT NULL,
    variant_title VARCHAR(500),
    image_url TEXT,
    requires_shipping BOOLEAN NOT NULL DEFAULT TRUE,
    is_gift_card BOOLEAN NOT NULL DEFAULT FALSE,
    custom_attributes JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Order Items table
CREATE TABLE order_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id UUID REFERENCES products(id) ON DELETE SET NULL,
    variant_id UUID REFERENCES product_variants(id) ON DELETE SET NULL,
    title VARCHAR(255) NOT NULL,
    variant_title VARCHAR(255),
    sku VARCHAR(100),
    quantity INTEGER NOT NULL,
    price DECIMAL(20, 2) NOT NULL,
    subtotal DECIMAL(20, 2) NOT NULL DEFAULT 0,
    tax_amount DECIMAL(20, 2) NOT NULL DEFAULT 0,
    total DECIMAL(20, 2) NOT NULL,
    requires_shipping BOOLEAN NOT NULL DEFAULT true,
    is_gift_card BOOLEAN NOT NULL DEFAULT false,
    weight DECIMAL(10, 2),
    weight_unit weight_unit,
    image_url VARCHAR(500),
    is_digital BOOLEAN DEFAULT FALSE,
    download_url TEXT,
    license_key TEXT,
    bundle_parent_id UUID REFERENCES order_items(id) ON DELETE SET NULL,
    is_bundle_component BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Coupon Applications table
CREATE TABLE coupon_applications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    coupon_id UUID NOT NULL REFERENCES coupons(id) ON DELETE CASCADE,
    product_id UUID REFERENCES products(id) ON DELETE CASCADE,
    collection_id UUID REFERENCES collections(id) ON DELETE CASCADE,
    is_exclusion BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_coupon_application_target CHECK (
        (product_id IS NOT NULL AND collection_id IS NULL) OR
        (product_id IS NULL AND collection_id IS NOT NULL)
    )
);

-- Coupon Usages table
CREATE TABLE coupon_usages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    coupon_id UUID NOT NULL REFERENCES coupons(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES customers(id) ON DELETE SET NULL,
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    discount_amount DECIMAL(19, 4) NOT NULL,
    used_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Collection Products table (junction)
CREATE TABLE collection_products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    collection_id UUID NOT NULL REFERENCES collections(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    position INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(collection_id, product_id)
);

-- Bundle Components table
CREATE TABLE bundle_components (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    bundle_product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    component_product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL DEFAULT 1,
    is_optional BOOLEAN DEFAULT FALSE,
    sort_order INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(bundle_product_id, component_product_id)
);

-- Subscription Items table
CREATE TABLE subscription_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    subscription_id UUID NOT NULL REFERENCES subscriptions(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    variant_id UUID REFERENCES product_variants(id) ON DELETE SET NULL,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    price DECIMAL(20, 2) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- STEP 6: CREATE REMAINING CHILD TABLES (Level 3)
-- ============================================================================

-- Payments table
CREATE TABLE payments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    amount DECIMAL(20, 2) NOT NULL,
    currency currency NOT NULL DEFAULT 'USD',
    status payment_status NOT NULL DEFAULT 'pending',
    gateway VARCHAR(50) NOT NULL,
    gateway_payment_id VARCHAR(255),
    payment_method payment_method_type,
    card_last_four VARCHAR(4),
    card_brand VARCHAR(50),
    error_message TEXT,
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Refunds table
CREATE TABLE refunds (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    payment_id UUID NOT NULL REFERENCES payments(id) ON DELETE CASCADE,
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    amount DECIMAL(20, 2) NOT NULL,
    currency currency NOT NULL DEFAULT 'USD',
    reason TEXT,
    status payment_status NOT NULL DEFAULT 'pending',
    gateway_refund_id VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Fulfillments table
CREATE TABLE fulfillments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    status fulfillment_status NOT NULL DEFAULT 'pending',
    tracking_number VARCHAR(255),
    tracking_url TEXT,
    tracking_company VARCHAR(100),
    shipped_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Order Notes table
CREATE TABLE order_notes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    author VARCHAR(255),
    note TEXT NOT NULL,
    is_customer_notified BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Subscription Invoices table
CREATE TABLE subscription_invoices (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    subscription_id UUID NOT NULL REFERENCES subscriptions(id) ON DELETE CASCADE,
    order_id UUID REFERENCES orders(id) ON DELETE SET NULL,
    cycle_number INTEGER NOT NULL,
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    subtotal DECIMAL(20, 2) NOT NULL DEFAULT 0,
    tax_total DECIMAL(20, 2) NOT NULL DEFAULT 0,
    total DECIMAL(20, 2) NOT NULL DEFAULT 0,
    status invoice_status NOT NULL DEFAULT 'pending',
    paid_at TIMESTAMPTZ,
    payment_id VARCHAR(255),
    failed_attempts INTEGER NOT NULL DEFAULT 0,
    last_failed_at TIMESTAMPTZ,
    failure_reason TEXT,
    next_retry_at TIMESTAMPTZ,
    retry_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Order Item Downloads table
CREATE TABLE order_item_downloads (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_item_id UUID NOT NULL REFERENCES order_items(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES customers(id) ON DELETE SET NULL,
    download_token TEXT UNIQUE NOT NULL,
    download_count INTEGER DEFAULT 0,
    download_limit INTEGER,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- License Keys table
CREATE TABLE license_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    order_item_id UUID REFERENCES order_items(id) ON DELETE SET NULL,
    customer_id UUID REFERENCES customers(id) ON DELETE SET NULL,
    license_key TEXT NOT NULL,
    is_used BOOLEAN DEFAULT FALSE,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(product_id, license_key)
);

-- Payment Retry Attempts table
CREATE TABLE payment_retry_attempts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    subscription_id UUID NOT NULL REFERENCES subscriptions(id) ON DELETE CASCADE,
    invoice_id UUID NOT NULL REFERENCES subscription_invoices(id) ON DELETE CASCADE,
    attempt_number INTEGER NOT NULL,
    attempted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    succeeded BOOLEAN NOT NULL DEFAULT FALSE,
    error_message TEXT,
    error_code VARCHAR(100),
    next_retry_at TIMESTAMPTZ,
    payment_method_id VARCHAR(255),
    gateway_transaction_id VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Dunning Emails table
CREATE TABLE dunning_emails (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    subscription_id UUID NOT NULL REFERENCES subscriptions(id) ON DELETE CASCADE,
    invoice_id UUID NOT NULL REFERENCES subscription_invoices(id) ON DELETE CASCADE,
    email_type dunning_email_type NOT NULL,
    subject VARCHAR(500) NOT NULL,
    body_html TEXT NOT NULL,
    body_text TEXT NOT NULL,
    sent_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    opened_at TIMESTAMPTZ,
    clicked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- STEP 7: CREATE INDEXES
-- ============================================================================

-- Products indexes
CREATE INDEX idx_products_slug ON products(slug);
CREATE INDEX idx_products_is_active ON products(is_active);
CREATE INDEX idx_products_created_at ON products(created_at);
CREATE INDEX idx_products_price ON products(price);
CREATE INDEX idx_products_product_type ON products(product_type);

-- Product Variants indexes
CREATE INDEX idx_product_variants_product_id ON product_variants(product_id);

-- Product Images indexes
CREATE INDEX idx_product_images_product_id ON product_images(product_id);

-- Product Options indexes
CREATE INDEX idx_product_options_product_id ON product_options(product_id);

-- Product Option Values indexes
CREATE INDEX idx_product_option_values_option_id ON product_option_values(option_id);
CREATE INDEX idx_product_option_values_variant_id ON product_option_values(variant_id);

-- Customers indexes
CREATE INDEX idx_customers_email ON customers(email);
CREATE INDEX idx_customers_created_at ON customers(created_at);

-- Addresses indexes
CREATE INDEX idx_addresses_customer_id ON addresses(customer_id);

-- Carts indexes
CREATE INDEX idx_carts_customer_id ON carts(customer_id) WHERE customer_id IS NOT NULL;
CREATE INDEX idx_carts_session_token ON carts(session_token) WHERE session_token IS NOT NULL;
CREATE INDEX idx_carts_expires_at ON carts(expires_at) WHERE expires_at IS NOT NULL;
CREATE INDEX idx_carts_converted ON carts(converted_to_order) WHERE converted_to_order = TRUE;

-- Cart Items indexes
CREATE INDEX idx_cart_items_cart_id ON cart_items(cart_id);
CREATE INDEX idx_cart_items_product_id ON cart_items(product_id);
CREATE UNIQUE INDEX idx_cart_items_unique 
    ON cart_items(cart_id, product_id, COALESCE(variant_id, '00000000-0000-0000-0000-000000000000'));

-- Orders indexes
CREATE INDEX idx_orders_customer_id ON orders(customer_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_created_at ON orders(created_at);
CREATE INDEX idx_orders_order_number ON orders(order_number);
CREATE INDEX idx_orders_coupon ON orders(coupon_id) WHERE coupon_id IS NOT NULL;

-- Order Items indexes
CREATE INDEX idx_order_items_order_id ON order_items(order_id);
CREATE INDEX idx_order_items_bundle_parent ON order_items(bundle_parent_id) WHERE bundle_parent_id IS NOT NULL;

-- Payments indexes
CREATE INDEX idx_payments_order_id ON payments(order_id);
CREATE INDEX idx_payments_status ON payments(status);

-- Refunds indexes
CREATE INDEX idx_refunds_payment_id ON refunds(payment_id);
CREATE INDEX idx_refunds_order_id ON refunds(order_id);

-- Coupons indexes
CREATE INDEX idx_coupons_active ON coupons(is_active) WHERE is_active = TRUE;
CREATE INDEX idx_coupons_code ON coupons(code);

-- Coupon Applications indexes
CREATE INDEX idx_coupon_applications_coupon ON coupon_applications(coupon_id);
CREATE INDEX idx_coupon_applications_product ON coupon_applications(product_id) WHERE product_id IS NOT NULL;
CREATE INDEX idx_coupon_applications_collection ON coupon_applications(collection_id) WHERE collection_id IS NOT NULL;

-- Coupon Usages indexes
CREATE INDEX idx_coupon_usages_coupon ON coupon_usages(coupon_id);
CREATE INDEX idx_coupon_usages_customer ON coupon_usages(customer_id) WHERE customer_id IS NOT NULL;
CREATE INDEX idx_coupon_usages_order ON coupon_usages(order_id);

-- API Keys indexes
CREATE INDEX idx_api_keys_customer_id ON api_keys(customer_id) WHERE customer_id IS NOT NULL;
CREATE INDEX idx_api_keys_key_prefix ON api_keys(key_prefix);
CREATE INDEX idx_api_keys_is_active ON api_keys(is_active) WHERE is_active = true;
CREATE INDEX idx_api_keys_expires_at ON api_keys(expires_at) WHERE expires_at IS NOT NULL;

-- Subscriptions indexes
CREATE INDEX idx_subscriptions_customer_id ON subscriptions(customer_id);
CREATE INDEX idx_subscriptions_status ON subscriptions(status);
CREATE INDEX idx_subscriptions_next_billing ON subscriptions(next_billing_at);

-- Subscription Items indexes
CREATE INDEX idx_subscription_items_subscription_id ON subscription_items(subscription_id);
CREATE INDEX idx_subscription_items_product_id ON subscription_items(product_id);

-- Subscription Invoices indexes
CREATE INDEX idx_subscription_invoices_subscription ON subscription_invoices(subscription_id);
CREATE INDEX idx_subscription_invoices_status ON subscription_invoices(status);
CREATE INDEX idx_subscription_invoices_next_retry ON subscription_invoices(next_retry_at) WHERE next_retry_at IS NOT NULL;

-- Fulfillments indexes
CREATE INDEX idx_fulfillments_order_id ON fulfillments(order_id);
CREATE INDEX idx_fulfillments_status ON fulfillments(status);
CREATE INDEX idx_fulfillments_tracking_number ON fulfillments(tracking_number) WHERE tracking_number IS NOT NULL;

-- Order Notes indexes
CREATE INDEX idx_order_notes_order_id ON order_notes(order_id);
CREATE INDEX idx_order_notes_created_at ON order_notes(created_at);

-- Product Categories indexes
CREATE INDEX idx_product_categories_slug ON product_categories(slug);
CREATE INDEX idx_product_categories_parent_id ON product_categories(parent_id) WHERE parent_id IS NOT NULL;
CREATE INDEX idx_product_categories_is_active ON product_categories(is_active) WHERE is_active = true;

-- Collections indexes
CREATE INDEX idx_collections_handle ON collections(handle);
CREATE INDEX idx_collections_published_at ON collections(published_at) WHERE published_at IS NOT NULL;

-- Collection Products indexes
CREATE INDEX idx_collection_products_collection_id ON collection_products(collection_id);
CREATE INDEX idx_collection_products_product_id ON collection_products(product_id);

-- Bundle Components indexes
CREATE INDEX idx_bundle_components_bundle ON bundle_components(bundle_product_id);
CREATE INDEX idx_bundle_components_component ON bundle_components(component_product_id);

-- Order Item Downloads indexes
CREATE INDEX idx_order_item_downloads_token ON order_item_downloads(download_token);
CREATE INDEX idx_order_item_downloads_order_item ON order_item_downloads(order_item_id);
CREATE INDEX idx_order_item_downloads_customer ON order_item_downloads(customer_id) WHERE customer_id IS NOT NULL;

-- License Keys indexes
CREATE INDEX idx_license_keys_product ON license_keys(product_id);
CREATE INDEX idx_license_keys_order_item ON license_keys(order_item_id) WHERE order_item_id IS NOT NULL;
CREATE INDEX idx_license_keys_customer ON license_keys(customer_id) WHERE customer_id IS NOT NULL;
CREATE INDEX idx_license_keys_key ON license_keys(license_key);

-- Payment Retry Attempts indexes
CREATE INDEX idx_payment_retry_subscription ON payment_retry_attempts(subscription_id);
CREATE INDEX idx_payment_retry_invoice ON payment_retry_attempts(invoice_id);
CREATE INDEX idx_payment_retry_attempted_at ON payment_retry_attempts(attempted_at);

-- Dunning Emails indexes
CREATE INDEX idx_dunning_emails_subscription ON dunning_emails(subscription_id);
CREATE INDEX idx_dunning_emails_invoice ON dunning_emails(invoice_id);
CREATE INDEX idx_dunning_emails_sent_at ON dunning_emails(sent_at);

-- ============================================================================
-- STEP 8: CREATE TRIGGERS FOR UPDATED_AT
-- ============================================================================

-- Create the trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Products trigger
CREATE TRIGGER products_updated_at
    BEFORE UPDATE ON products
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Product Variants trigger
CREATE TRIGGER product_variants_updated_at
    BEFORE UPDATE ON product_variants
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Product Images trigger
CREATE TRIGGER product_images_updated_at
    BEFORE UPDATE ON product_images
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Product Options trigger
CREATE TRIGGER product_options_updated_at
    BEFORE UPDATE ON product_options
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Product Option Values trigger
CREATE TRIGGER product_option_values_updated_at
    BEFORE UPDATE ON product_option_values
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Customers trigger
CREATE TRIGGER customers_updated_at
    BEFORE UPDATE ON customers
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Addresses trigger
CREATE TRIGGER addresses_updated_at
    BEFORE UPDATE ON addresses
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Orders trigger
CREATE TRIGGER orders_updated_at
    BEFORE UPDATE ON orders
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Order Items trigger
CREATE TRIGGER order_items_updated_at
    BEFORE UPDATE ON order_items
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Payments trigger
CREATE TRIGGER payments_updated_at
    BEFORE UPDATE ON payments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Refunds trigger
CREATE TRIGGER refunds_updated_at
    BEFORE UPDATE ON refunds
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Carts trigger
CREATE TRIGGER carts_updated_at
    BEFORE UPDATE ON carts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Cart Items trigger
CREATE TRIGGER cart_items_updated_at
    BEFORE UPDATE ON cart_items
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Coupons trigger
CREATE TRIGGER coupons_updated_at
    BEFORE UPDATE ON coupons
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- API Keys trigger
CREATE TRIGGER api_keys_updated_at
    BEFORE UPDATE ON api_keys
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Subscriptions trigger
CREATE TRIGGER subscriptions_updated_at
    BEFORE UPDATE ON subscriptions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Subscription Items trigger
CREATE TRIGGER subscription_items_updated_at
    BEFORE UPDATE ON subscription_items
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Subscription Invoices trigger
CREATE TRIGGER subscription_invoices_updated_at
    BEFORE UPDATE ON subscription_invoices
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Fulfillments trigger
CREATE TRIGGER fulfillments_updated_at
    BEFORE UPDATE ON fulfillments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Product Categories trigger
CREATE TRIGGER product_categories_updated_at
    BEFORE UPDATE ON product_categories
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Collections trigger
CREATE TRIGGER collections_updated_at
    BEFORE UPDATE ON collections
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Bundle Components trigger
CREATE TRIGGER bundle_components_updated_at
    BEFORE UPDATE ON bundle_components
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Order Item Downloads trigger
CREATE TRIGGER order_item_downloads_updated_at
    BEFORE UPDATE ON order_item_downloads
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- License Keys trigger
CREATE TRIGGER license_keys_updated_at
    BEFORE UPDATE ON license_keys
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- SCHEMA CREATION COMPLETE
-- ============================================================================
