# OHC Hybrid Agentic OS v0.4.30

We are excited to announce the v0.4.30 release of the OHC Hybrid Agentic OS! This release continues our commitment to delivering a scalable cloud experience alongside a secure, offline-capable standalone desktop binary.

## 🚀 Cloud Scaling Improvements (v0.4.30)
- Enhanced multi-tenant Kubernetes pod routing for improved scaling.
- Performance optimization in agent mesh telemetry.
- Improved cost auditing tracking capabilities.

## 🔒 Standalone Privacy & Offline Improvements (v0.4.30+1)
- Strengthened local data isolation for improved privacy.
- Enhanced offline state synchronization (StateSync MCP).
- Streamlined desktop binary packaging.

*Enjoy the best of both worlds with OHC!*

# release_notes.md

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
