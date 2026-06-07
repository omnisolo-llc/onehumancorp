# In-Store QR Code Generator: Bridging Offline Traffic to Online Growth Loops

## Overview
OneHumanCorp (OHC) empowers non-technical business owners to serve customers across multiple channels. However, offline businesses (food carts, boutiques, in-person tutors) often fail to capture walk-up foot traffic into their digital growth loops.

The **In-Store QR Code Generator** solves this by letting owners generate beautiful, printable display cards (posters, table tents) featuring a scannable QR code.

## Persona Alignment
- **Fatima (Food Cart Operator)**: Can print a "Skip the Line, Order Here" QR poster to put on her food cart window, instantly turning foot traffic into digital pre-orders.
- **Priya (Boutique Operator)**: Can print "Join our VIP list for 10% off" cards to place next to the tap-to-pay terminal.
- **Leo (Creator & Tutor)**: Can hand out business cards with a QR code directing students straight to his booking calendar.

## The Growth Loop
1. **Offline Capture**: The physical QR code acts as the top of the funnel for anonymous in-store foot traffic.
2. **Digital Transition**: Scanning the code routes the customer through a tracking endpoint (`/api/v1/growth/qr/scan`) which records the physical attribution before redirecting.
3. **Virality/Branding**: The printed display itself features a "⚡ Powered by OHC" footer. This exposes the OHC brand to other aspiring local business owners who see the professional setup. Owners can remove this watermark by upgrading to the Pro plan (soft paywall).

## Architecture

### Backend (Rust/Axum)
- A new route `GET /api/v1/growth/qr/scan` will track the scan event in the OHC telemetry hub, then issue an HTTP 302 redirect to the target URL (e.g., the store's link-in-bio, checkout page, or review portal).

### Frontend (Next.js Prototype / Tauri equivalent)
- A new route `/qr-generator` serves as the configuration dashboard.
- Uses the `qrcode.react` package to render high-quality SVG QR codes locally.
- Provides real-time preview of the printable asset.
- Exposes a print sheet feature (`window.print()`) styled cleanly via `@media print`.

## Future Considerations
- Dynamic destination routing (change the link destination without re-printing the QR code).
- Deep integration with POS receipts.