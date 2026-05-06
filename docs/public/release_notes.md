# release_notes.md

## v0.4.29 (Cloud) / v0.4.29+1 (Standalone)
### Cloud Scaling Improvements
- 🧹 Cleaner: Fixed power sync orchestrator initialization bug relying on static standalone config rather than dynamic is_cloud state (#11312)

### Privacy/Offline Improvements
- 🧹 Cleaner: Added PII payload redaction unit testing and ensured telemetry data is correctly redacted (#11312)

## v0.4.28 (Cloud) / v0.4.28+1 (Standalone)
### Cloud Scaling Improvements
- ✍️ Scribe: Scaled the Help Center & Tooltip Documentation System for multi-tenant cloud environments (#11267)

### Privacy/Offline Improvements
- ✍️ Scribe: Enabled offline-first support for the Help Center & Tooltip Documentation System in standalone mode (#11267)

## v0.4.27 (Cloud) / v0.4.27+1 (Standalone)

- Scaling (Cloud): 🎨 Canvas: Refactored the MCP LocalProxyClient to use an abstract BlobProvider with S3 support for cloud multitenant scaling.
- Privacy and Offline (Standalone): 🎨 Canvas: Added LocalBlobProvider implementation to ensure privacy and offline capabilities for the MCP proxy.
