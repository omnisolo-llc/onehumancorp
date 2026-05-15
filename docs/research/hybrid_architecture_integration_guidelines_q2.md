# Hybrid Architecture Integration Guidelines (Q2)

## Goal
To ensure tool integrations work seamlessly across OHC's operating modes: **Cloud**, **Headless**, **Standalone**, and **Local Stack**.

---

## 1. Authentication Patterns
- **Cloud Mode**: Standard server-side secure connection flow.
- **Standalone Mode**: OHC implements an "Auth Reflector" that securely relays tokens back to the user's local machine without storing them on OHC servers.

## 2. Real-Time Updates (Automatic Updates)
- **Problem**: Local machines behind firewalls cannot receive updates from Square or Buffer.
- **OHC Solution**: We use a cloud-side event mesh that the Standalone client subscribes to, ensuring real-time updates reach the desktop app instantly.

## 3. Data Privacy & SIPDB
- Tool data should be cached in the local SIPDB (SQLite) for Standalone users, supporting "Offline-First" capability.
- In Cloud mode, tool data is isolated per OHC tenant at the database level.

## 4. Performance (The Bolt Standard)
- **Asynchronous Processing**: No external communication should block the OHC user interface.
- **Optimistic UI**: Show the "Sent" state for a message immediately while the background process handles the delivery.
- **Resource Management**: Standalone mode integration must be lightweight to preserve the user's computer performance.
