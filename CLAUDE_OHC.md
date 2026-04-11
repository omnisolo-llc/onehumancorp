# OHC Hybrid Architecture (OHC-HA)

## Overview
One Human Corp (OHC) provides a platform that operates in two primary modes:
1. **Cloud-Native**: PostgreSQL backend, multi-tenant, horizontal scaling.
2. **Standalone Desktop**: SQLite backend, single-user, local-first degradation.

This requires careful abstraction of database logic and synchronization boundaries.
