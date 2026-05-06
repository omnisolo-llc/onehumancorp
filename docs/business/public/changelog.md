# OHC Hybrid Agentic OS Changelog

## v0.4.25 (Cloud) / v0.4.25+1 (Standalone)

### Cloud Scaling Improvements

- Formalized multi-tenant K8s deployment pipelines to ensure isolated workload execution per tenant and resilient agent auto-scaling under heavy traffic.

### Privacy and Offline Improvements

- Engineered a fully encapsulated desktop binary wrapper enforcing strict local data residency via SQLite, completely bypassing external cloud services for maximum data privacy.

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
