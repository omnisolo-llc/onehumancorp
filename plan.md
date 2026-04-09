1. **Implement `GlassCard` Widget**:
   - Write `srcs/app/lib/widgets/glass_card.dart` containing `GlassCard` as a `StatefulWidget` using `MouseRegion` for hover detection, `AnimatedScale` for 1.02 scale on hover, and `BackdropFilter` with `ImageFilter.compose(outer: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0), inner: ColorFilter.matrix(<double>[...]))` (or just simple standard properties if compose is tricky) and a background of `Colors.white.withValues(alpha: 0.03)` as per the Mandate.
   - Verify file creation with `cat srcs/app/lib/widgets/glass_card.dart`.
2. **Refactor Existing Flat Cards**:
   - Use targeted `sed` commands or a python script to replace `Card` with `GlassCard` in the specific files identified in the trace: `srcs/app/lib/screens/dashboard_screen.dart`, `srcs/app/lib/screens/handoffs_screen.dart`, `srcs/app/lib/screens/referrals_dashboard_screen.dart`, `srcs/app/lib/screens/pipelines_screen.dart`, `srcs/app/lib/screens/integrations_screen.dart`, `srcs/app/lib/screens/scaling_screen.dart`, `srcs/app/lib/screens/security_screen.dart`, `srcs/app/lib/screens/ai_config_screen.dart`, `srcs/app/lib/screens/settings_screen.dart`, `srcs/app/lib/screens/agents_screen.dart`, `srcs/app/lib/screens/service_screen.dart`, `srcs/app/lib/screens/meetings_screen.dart`, `srcs/app/lib/screens/cost_dashboard_screen.dart`, `srcs/app/lib/screens/swarm_memory_screen.dart`, `srcs/app/lib/screens/skills_screen.dart`, `srcs/app/lib/screens/channels_screen.dart`, `srcs/app/lib/screens/wizard_screen.dart`.
   - Also use a python script to inject `import '../widgets/glass_card.dart';` into these files (handling nested screen depths appropriately).
   - Verify the changes with `git diff`.
3. **Run Tests**:
   - Run `bazelisk test //...` to verify everything passes.
4. **Pre-commit Steps**:
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
5. **Submit PR**:
   - Submit the PR with title `🎨 Palette: [Hybrid UX improvement] Apply Glassmorphism to remaining Cards`.
