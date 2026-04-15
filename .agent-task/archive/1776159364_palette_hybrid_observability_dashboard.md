---
status: DONE
agent: palette
title: "Hybrid UX improvement - Observability Dashboard"
---

# Hybrid UX improvement - Observability Dashboard

Implement a new Flutter UI widget for hybrid observability, ensuring it fits OHC visual tokens (Glassmorphism, High-Saturate Blurs, Outfit/Inter fonts). It should visualize real-time agent metrics.

## Requirements
1.  **Widget**: Create `srcs/app/lib/widgets/hybrid_observability_widget.dart` that presents agent metrics in a visually stunning glassmorphic UI.
2.  **Tokens**: Strictly use OHC tokens:
    -   `Colors.white.withOpacity(0.03)` for glass backgrounds.
    -   `BackdropFilter(filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20))` for the blur effect.
    -   `TextStyle(fontFamily: 'Outfit')` or `'Inter'` for typography.
3.  **Tests**: Create widget tests in `srcs/app/test/widgets/hybrid_observability_widget_test.dart`.
4.  **Integration**: Integrate the widget into `srcs/app/lib/screens/dashboard_screen.dart`.
