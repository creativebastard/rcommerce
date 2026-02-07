# Migration Guides

Comprehensive guides for migrating from popular ecommerce platforms to R Commerce.

## Available Migration Guides

### [Shopify](./shopify.md)
Migrate from Shopify including:
- Product and variant migration
- Customer and order history
- Theme and frontend transition
- SEO preservation
- App ecosystem alternatives

### [WooCommerce](./woocommerce.md)
Migrate from WooCommerce including:
- WordPress integration removal
- Product and attribute migration
- Customer and order migration
- Plugin ecosystem mapping
- Payment gateway transition

### [Magento](./magento.md)
Migrate from Magento including:
- Complex product structure migration
- Customer group mapping
- Order and invoice migration
- Extension ecosystem alternatives
- B2B feature migration

### [Medusa](./medusa.md)
Migrate from Medusa.js including:
- API compatibility considerations
- Direct migration path
- Feature comparison
- Migration script examples

## Migration Strategies

### Big Bang Migration
All data migrated at once during a planned downtime.

**Pros:**
- Simple execution
- Clean cutover
- Lower complexity

**Cons:**
- Requires downtime
- Higher risk
- Rollback can be difficult

**Best for:** Small stores, off-season migration, new launches

### Phased Migration
Migrate in stages over time with zero or minimal downtime.

**Pros:**
- Zero downtime
- Lower risk
- Easy rollback
- Test each phase

**Cons:**
- More complex
- Longer timeline
- Running dual systems

**Best for:** Large stores, high-traffic stores, complex integrations

## Import Tool

R Commerce includes a built-in import tool for migrating from popular platforms:

### Quick Start

```bash
# Import from Shopify
rcommerce import platform shopify \
  -c config.toml \
  --api-url https://your-store.myshopify.com \
  --api-key YOUR_API_KEY \
  --api-secret YOUR_API_PASSWORD

# Import from WooCommerce
rcommerce import platform woocommerce \
  -c config.toml \
  --api-url https://your-store.com \
  --api-key YOUR_CONSUMER_KEY \
  --api-secret YOUR_CONSUMER_SECRET
```

### Supported Platforms

| Platform | Status | Entities |
|----------|--------|----------|
| Shopify | ✅ Full | Products, Customers, Orders |
| WooCommerce | ✅ Full | Products, Customers, Orders |
| Magento | 🚧 Planned | Products, Customers, Orders |
| Medusa | 🚧 Planned | Products, Customers, Orders |

### File Import

Import from exported files:

```bash
# CSV import
rcommerce import file -c config.toml --file products.csv --format csv --entity products

# JSON import
rcommerce import file -c config.toml --file customers.json --format json --entity customers
```

### Dry Run Mode

Always validate first with `--dry-run`:

```bash
rcommerce import platform shopify ... --dry-run
```

This validates all data without modifying your database.

See the [CLI Reference](../development/cli-reference.md#import) for complete documentation.

## Migration Checklist

### Pre-Migration
- [ ] Audit current platform data
- [ ] Clean up unused products/customers/orders
- [ ] Export all data from current platform
- [ ] Set up R Commerce environment
- [ ] Choose migration strategy
- [ ] Create migration timeline
- [ ] Prepare rollback plan
- [ ] Test import with `--dry-run`

### Migration Execution
- [ ] Import products and categories
- [ ] Import customers
- [ ] Import orders (optional, for reporting)
- [ ] Set up payment gateways
- [ ] Configure shipping
- [ ] Set up tax rules
- [ ] Test checkout flow
- [ ] Test order management
- [ ] Test customer accounts

### Post-Migration
- [ ] Verify data integrity
- [ ] Test all integrations
- [ ] Update DNS/point domain
- [ ] Monitor performance
- [ ] Test backup/restore
- [ ] Train staff on new system
- [ ] Update documentation
- [ ] Archive old platform data

## Data Mapping Considerations

### Product Complexity
Different platforms handle products differently:

| Platform | Product Model |
|----------|---------------|
| Shopify | Simple products with variants |
| WooCommerce | Variable products with attributes |
| Magento | Complex types (configurable, bundled, grouped) |
| R Commerce | Unified model with variants and attributes |

### Customer Accounts
- **Passwords**: Usually cannot be migrated (require reset)
- **Groups**: Map to R Commerce customer groups
- **Loyalty**: May need custom integration
- **Subscriptions**: Requires special handling

### Order History
- Order numbering schemes may differ
- Status mapping required
- Partial refunds/exchanges
- Multi-currency orders

### SEO Preservation
Critical for maintaining search rankings:
- URL redirects (essential!)
- Meta data migration
- Sitemap structure
- Search engine re-indexing

## Common Pitfalls

### 1. Underestimating Time
- Large catalogs take days, not hours
- Order history migration is slow
- Testing phase often takes longer than expected

### 2. Data Loss Risks
- Always backup before migrating
- Test integrity after migration
- Keep original data until confirmed

### 3. SEO Impact
- Missing redirects = lost rankings
- Content changes = re-indexing time
- Plan for temporary traffic drop

### 4. Integration Breakage
- Payment webhooks need updating
- Shipping integrations may break
- CRM/email marketing connections
- Accounting system integrations

### 5. Performance Issues
- New platform may need tuning
- Different caching strategies
- Database optimization needed

## Testing Checklist

- [ ] Product catalog complete (count matches)
- [ ] Product details accurate (prices, images, descriptions)
- [ ] Customer data migrated (emails, addresses)
- [ ] Order history accessible
- [ ] Payment processing works
- [ ] Shipping calculations accurate
- [ ] Tax rules applied correctly
- [ ] Email notifications sent
- [ ] Webhooks received
- [ ] Mobile responsive
- [ ] Admin functions work
- [ ] Reports generate correctly
- [ ] Backup and restore tested
- [ ] Performance acceptable
- [ ] SEO redirects working

## Support

For migration assistance:

1. **Documentation**: Review platform-specific guides
2. **Community**: [GitHub Discussions](https://github.com/creativebastard/rcommerce/discussions)
3. **Professional Services**: Contact sales@rcommerce.app for enterprise migration support

## See Also

- [Architecture Overview](../architecture/overview.md)
- [Deployment Guide](../deployment/index.md)
- [CLI Reference](../development/cli-reference.md)
