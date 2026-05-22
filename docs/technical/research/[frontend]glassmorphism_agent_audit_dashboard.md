# [frontend] Glassmorphism Agent Audit Dashboard

## Problem Statement
Administrators and users need a "Premium" visual interface to monitor the OHC swarm. The dashboard should display agent health, recent violations, and real-time execution logs with OHC's signature aesthetic.

## Research Report
- **Competitive Analysis**: Most competitors use CLI-only or basic web TUIs. OHC aims for a "Mission Control" experience.
- **Visual Mandate**: Glassmorphism, 20px blur, Outfit/Inter typography, and smooth animations (Framer Motion/Tauri).

## Design Doc
- **Layout**: A grid of "Audit Cards" showing real-time agent status.
- **Violation Feed**: A scrolling sidebar with red-tinted glassmorphism alerts for sandbox violations.
- **State Machine Visualizer**: A Mermaid.js or custom SVG graph showing current task transitions.
- **Cost Tracker**: A real-time counter showing USD spend across the organization.

## Implementation Prompt
1. Build a new React component for the packaged frontend and wire it through the Tauri desktop shell.
2. Apply `Glassmorphism` styling: `box-shadow`, `backdrop-filter: blur(20px)`, `border: 1px solid rgba(255, 255, 255, 0.1)`.
3. Integrate with the Telemetry API to fetch real-time metrics.
4. Implement a "Live Log" view that streams `stdout/stderr` from the `bash_sandbox` via WebSockets.
5. Ensure the dashboard is responsive and works in both Cloud-Native (multi-tenant) and Standalone (single-user) modes.

## Priority
P2

## Estimated Scope
Medium
