# Architecture Brief: Mobile-First Review & Performance

## Title
OHC Mobile-First Contract: Performance, Resilience, and "Grandmother Test" Audit

## Problem Statement
Small business owners (Carlos, Fatima, Maya) are mobile-only or mobile-primary. They operate in high-distraction environments (bakeries, repair sites, food carts) and often on low-end Android devices or poor 4G/5G connections. If the OHC dashboard is slow to load or fails when offline, Carlos can't send a quote, and Fatima loses a sale. OHC must be as fast and reliable as a native calculator app.

## Research Report
- **The "Grandmother Test"**: If a user has to wait more than 2 seconds for a screen to load, or more than 1 second for a button to respond, they assume the app is "broken."
- **Payload Bloat**: Traditional SaaS dashboards (Shopify/Wix) often fetch megabytes of JS and JSON, leading to LCP > 3s on 4G networks.
- **Offline Gaps**: Most web-based builders require a constant internet connection. OHC's hybrid nature (Local SQLite/SIPDB) provides a unique opportunity to allow "Offline Drafting."

## Design Doc

### Mobile-First Performance Targets
| Metric | Target | Why? |
| :--- | :--- | :--- |
| **LCP (Largest Contentful Paint)** | < 1.5s (4G) | Essential for "Activation" and perceived speed. |
| **FID (First Input Delay)** | < 100ms | Buttons must feel "native" and responsive. |
| **Bundle Size (Core UI)** | < 500KB | Fast download on low-data plans (Fatima). |
| **Touch Target Size** | ≥ 44x44px | Accessible for all users, especially in active work environments. |

### Architectural Decisions for Mobile Resilience
1.  **Lightweight Dashboard Service**: Implement `GetLightweightDashboard` in gRPC/Proto to return ONLY the critical counts (Orders, Agents, Messages) instead of full resource lists for the initial paint.
2.  **Optimistic UI with "Mesh Sync"**: All user actions (e.g., "Approve Quote") update the local UI state immediately. The KAIROS Orchestrator handles the background sync to the Teammate Mesh.
3.  **Offline-First Drafting**: Users can draft products or messages while offline. These are stored in the local SQLite SIPDB and auto-synced by the `SyncDaemon` once connectivity is restored.
4.  **Adaptive Asset Loading**: AI-generated images for the storefront are served via progressive JPEGs/WebP with mobile-responsive `srcset` variants.

### Mobile UX Flow (375px First)
- **Bottom Navigation**: Primary actions (Home, Orders, Agents, Settings) are reachable with one thumb.
- **Glassmorphism Shimmer**: Use skeleton loading states (shimmer effect) to maintain visual continuity during data retrieval.
- **Jargon-Free UI**: Replace "API Config" with "Helper Settings," "CNAME" with "Website Address."

## Implementation Prompt
**To Implementer Agent:**
Audit the current Slint/Rust frontend against the Mobile-First Performance targets. Implement "Skeleton Loading" (shimmer effect) for the `StatCard` and `AgentFeed` components. Update the `DashboardService` to support a `mobile_optimized: true` flag that returns a lightweight payload for the initial mobile paint. Implement "Optimistic Updates" in the Task List: when a user approves an agent draft, the UI should immediately reflect the "Approved" status and show a non-blocking background sync indicator. Ensure all touch targets in the `WebsiteBuilder` are at least 44x44px and use native mobile keyboards (numeric for prices, etc.).

## Priority
P0

## Estimated Scope
Medium

### Critical SaaS Journeys

#### Offline Reliability
When Fatima is in a dead zone, the OHC app must queue actions instead of displaying errors.
- A generic "No Internet" toast is insufficient.
- The app must clearly distinguish between "Syncing" and "Draft" states.
- Local SQLite allows reading the catalog unconditionally.
- Writes (like Sold Out toggles) are captured locally and synced later.

#### Memory Constraints
Older Android devices aggressively kill background apps.
- The app must resume precisely where Carlos left off, even if suspended while drafting a complex quote.
- This requires deep persistence of UI state, not just data models.

#### Navigation Hierarchy
- The traditional "Hamburger menu -> Settings -> Submenu" is a desktop anti-pattern applied to mobile.
- OHC must use a shallow navigation structure. 90% of a persona's daily tasks must be reachable within a maximum of two taps from the home screen.
- For Maya: The home screen is her inbox.
- For Fatima: The home screen is the order queue.
- For Priya: The home screen is the POS scanner.

### The App Size Contract
- The downloaded bundle size must remain under 15MB.
- Heavy assets (fonts, non-critical images) must be deferred.
- Every MB added reduces conversion rates in low-bandwidth regions by an estimated 1-3%.

### Accessibility
- **Target Touch Area**: Minimum 48x48dp for all interactive elements.
- **Contrast**: WCAG AAA standard for primary actions. Fatima needs to see the "Sold Out" button in direct sunlight.
- **Typography**: Dynamic Type support. If Carlos increases his system font size because he forgot his reading glasses, the app layout must adapt without truncating text or breaking buttons.
