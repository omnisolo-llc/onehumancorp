# release_notes.md

## v0.4.40 (Cloud) / v0.4.40+1 (Standalone)

### Cloud Scaling Improvements
- Optimize Sub-Agent Queue polling intervals to reduce Postgres connection pressure.
- 🔨 Forge: Implemented KAIROS Shared Task List Backend supporting multi-tenant Postgres schema and tenant isolation for cloud deployments (#12855).

### Privacy/Offline Improvements
- Implement offline-first local vector embeddings cache for the OHC Swarm.
- 🔨 Forge: Implemented KAIROS Shared Task List Backend with full in-memory and local SQLite capabilities for standalone isolation (#12855).

## v0.4.38 (Cloud) / v0.4.38+1 (Standalone)

### Cloud Scaling Improvements
- Enhance multi-tenant onboarding flow tests for the Welcome Checklist to ensure reliable scaling.

### Privacy/Offline Improvements
- Bolster Standalone Wizard state test coverage for improved offline reliability and progressive disclosure validation.

## v0.4.37 (Cloud) / v0.4.37+1 (Standalone)

### Cloud Scaling Improvements
- 🔗 Link: Interop Mesh Comprehensive Test Coverage to improve distributed lock resilience (#12496)

### Privacy/Offline Improvements
- 🔗 Link: Ensured graceful interop mesh protocol handling for malformed offline mesh payloads (#12496)


## v0.4.32 (Cloud) / v0.4.32+1 (Standalone)

### Cloud Scaling Improvements
- 🛡️ Sentry: Health Guardianship /api/v1/health improvements for multi-tenant state sync

### Privacy/Offline Improvements
- 🛡️ Sentry: Health Guardianship /api/v1/health improvements for standalone isolated node switching

## v0.4.30 (Cloud) / v0.4.30+1 (Standalone)
### Cloud Scaling Improvements
- 🔨 Forge: Refactor GrowthReferralWidget to use GlassCard for premium aesthetic (#11347)

### Privacy/Offline Improvements
- 🔨 Forge: Refactor GrowthReferralWidget to use GlassCard for premium aesthetic (#11347)

## v0.4.29 (Cloud) / v0.4.29+1 (Standalone)

### Cloud Scaling Improvements
- 🔗 Link: Implemented Teammate Mesh Communication Layer and Distributed Locks (#11313)

### Privacy/Offline Improvements
- 🔗 Link: Ensured mesh communication layer degrades gracefully into isolated standalone instances (#11313)


## v0.4.28 (Cloud) / v0.4.28+1 (Standalone)
### Cloud Scaling Improvements
- ✍️ Scribe: Scaled the Help Center & Tooltip Documentation System for multi-tenant cloud environments (#11267)

### Privacy/Offline Improvements
- ✍️ Scribe: Enabled offline-first support for the Help Center & Tooltip Documentation System in standalone mode (#11267)

## v0.4.27 (Cloud) / v0.4.27+1 (Standalone)

- Scaling (Cloud): 🎨 Canvas: Refactored the MCP LocalProxyClient to use an abstract BlobProvider with S3 support for cloud multitenant scaling.
- Privacy and Offline (Standalone): 🎨 Canvas: Added LocalBlobProvider implementation to ensure privacy and offline capabilities for the MCP proxy.