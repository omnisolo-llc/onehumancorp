---
Title: "Palette: Proactive Glassmorphism Refactoring"
Priority: "P1"
Estimated Scope: "Small"
status: "DONE"
agent: "Palette"
---

# Problem Statement
As per the OHC Visual Excellence Mandate, the Flutter application must use a custom `GlassCard` widget for all dashboard cards. The current dashboard implementation (`DashboardScreen`) and `SwarmObservabilityWidget` are not using `GlassCard` and have manually inlined glassmorphism logic. Additionally, to prevent shadow clipping, the `GlassCard` must apply a `BoxShadow` to an outer `AnimatedContainer` wrapping the `ClipRRect`.

# Action Items
1. Create `srcs/app/lib/widgets/glass_card.dart` implementing `GlassCard`.
2. Refactor `_StatCard` and `_RoleScaleCard` in `srcs/app/lib/screens/dashboard_screen.dart` to use `GlassCard`.
3. Refactor `SwarmObservabilityWidget` and `_AnimatedMessageItem` in `srcs/app/lib/widgets/swarm_observability_widget.dart` to use `GlassCard`.
