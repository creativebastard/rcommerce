# Documentation Fix Plan

## Phase 1: Critical Link Fixes (Must Do)

### 1.1 Fix Broken Links in docs/README.md
- [ ] Fix `../development-roadmap.md` link (file doesn't exist)

### 1.2 Fix or Create Missing Architecture File
- [ ] Either create `docs/architecture/03-api-design.md` OR
- [ ] Update all references to point to `docs/api/01-api-design.md`

### 1.3 Fix Relative Paths in architecture/01-overview.md
- [ ] Fix `[API Design](../api/01-api-design.md)` path
- [ ] Fix other incorrect relative paths

### 1.4 Fix CLI Reference Links
- [ ] Fix `../getting-started/configuration.md` (line ~748)
- [ ] Fix `../api-reference/authentication.md`
- [ ] Fix `../deployment/index.md`

### 1.5 Fix Migration Guide Links
- [ ] Fix Migration Tool Repository link (may not exist)
- [ ] Fix R commerce Documentation link

## Phase 2: CLI Documentation Accuracy

### 2.1 Fix Import Command Syntax
- [ ] Change `--platform <PLATFORM>` to `<PLATFORM>` (positional arg)
- [ ] Update both docs/ and docs-website/

### 2.2 Fix Missing --limit Option
- [ ] Add `--limit` to `ImportCommands::File` in CLI code, OR
- [ ] Remove `--limit` from file import documentation

### 2.3 Document db reset --force
- [ ] Add `--force` flag documentation

## Phase 3: Content Consistency

### 3.1 Align Migration Index Files
- [ ] Add legacy tool section to docs-website (or note about deprecation)
- [ ] Add Platform-Specific Considerations to docs-website
- [ ] Standardize support links
- [ ] Add missing checklist items to docs-website

### 3.2 Standardize Naming
- [ ] Use consistent "R Commerce" branding

## Phase 4: Structure Improvements

### 4.1 Create Missing Index Files
- [ ] docs/api/index.md
- [ ] docs/architecture/index.md
- [ ] docs/deployment/index.md
- [ ] docs/development/index.md

### 4.2 Fix Duplicate Numbering
- [ ] Rename 08-product-types-and-subscriptions.md to 09-*

### 4.3 Archive Temporary Files
- [ ] Move docs/compilation/ to docs/internal/ or archive
- [ ] Move docs/project/ to docs/internal/ or archive

## Phase 5: Translation Coverage

### 5.1 High Priority Translations
- [ ] architecture/overview.md.zh
- [ ] architecture/data-model.md.zh
- [ ] development/cli-reference.md.zh
- [ ] development/index.md.zh

### 5.2 Medium Priority Translations
- [ ] migration/*.zh.md (all 5 files)
- [ ] deployment/scaling.md.zh
- [ ] deployment/redis.md.zh

## Phase 6: Content Completion

### 6.1 Complete Placeholder Pages
- [ ] deployment/linux/manual.md - add actual content
- [ ] deployment/freebsd/rc.d.md - add actual content, fix typo

