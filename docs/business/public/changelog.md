# OHC Hybrid Agentic OS Changelog

## v0.4.30 (Cloud) / v0.4.30+1 (Standalone)

### Cloud Scaling Improvements
- 🔨 Forge: Add Unit Test for Mission Handover Protocol (#11345)

### Privacy/Offline Improvements
- 🔨 Forge: Add Unit Test for Mission Handover Protocol (#11345)


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

### Cloud Scaling Improvements

- 🎨 Canvas: Refactored the MCP LocalProxyClient to use an abstract BlobProvider with S3 support for cloud multitenant scaling.

### Privacy and Offline Improvements

- 🎨 Canvas: Added LocalBlobProvider implementation to ensure privacy and offline capabilities for the MCP proxy.

## v0.3.6 (Cloud) / v0.3.6+1 (Standalone)

### Cloud Scaling Improvements

- Implemented storage compression and token budget management tools for cost optimization in Kubernetes deployments.

### Privacy and Offline Improvements

- Enabled offline-compatible storage compression reducing local disk footprint for standalone environments.

## v0.3.5 (Cloud) / v0.3.5+1 (Standalone)

### Cloud Scaling Improvements

- Enhanced Teammate Mesh APIs and AutoDream worker logic for more scalable Kubernetes pod communications.

### Privacy and Offline Improvements

- Continued stabilization of the offline KAIROS state machine functionality via SQLite fallbacks.

## v0.3.4 (Cloud) / v0.3.4+1 (Standalone)

### Cloud Scaling Improvements

- Enhanced cloud multi-tenant architecture and hybrid teammate mesh APIs for improved coordination across Kubernetes pods.

### Privacy and Offline Improvements

- Implemented a fully offline-capable KAIROS state machine via SQLite with safe fallbacks.

## v0.3.3 (Cloud) / v0.3.3+1 (Standalone)

### Cloud Scaling Improvements

- Enhanced cloud multi-tenant architecture with robust onboarding tests and removed obsolete test files for cleaner CI/CD execution.

### Privacy and Offline Improvements

- Improved standalone offline test parity by ensuring onboarding integration tests run smoothly in isolated local environments without heavy cloud dependencies.
