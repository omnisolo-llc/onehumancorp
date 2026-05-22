# Proactive Mission: Implement "Premium Glassmorphism" Aesthetic Across Frontend

## Problem Statement
The OHC UI does not fully align with the "Visual Truth" mandate. Many screens still use flat legacy `Card` widgets instead of the required "Premium Glassmorphism" aesthetic (blur 20px, semi-transparent backgrounds).

## Implementation Notes
1. Apply the `GlassCard` treatment in the current packaged frontend rather than the removed Slint UI.
2. Verify the build and visual aesthetic using Playwright screenshots (`landing_glass.png`, `dashboard_glass.png`).
3. Run the Playwright suite and the Tauri build target before marking the work complete.

Mission marked as DONE successfully.
