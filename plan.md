1. **Claim the Proactive Mission File**
   - Execute `run_in_bash_session` to run `sed -i 's/status: DONE/status: IN_PROGRESS/' .agent-task/missions/1780000001_palette_proactive_glassmorphism_cards.md`
   - Execute `run_in_bash_session` to run `cat .agent-task/missions/1780000001_palette_proactive_glassmorphism_cards.md` to verify the state was correctly updated.
2. **Create `GlassCard` Widget**
   - Execute `write_file` to create `srcs/app/lib/widgets/glass_card.dart` with the `GlassCard` `StatefulWidget` implementation. It should have properties `child`, `color`, `borderColor`, `margin`, `padding`, `shape`, `elevation`, etc to maintain compatibility with `Card`. It should manage hover scale with `MouseRegion` and use `BackdropFilter` with the matrix and blur values specified.
   - Execute `run_in_bash_session` to run `cat srcs/app/lib/widgets/glass_card.dart` to verify successful creation.
   - Execute `run_in_bash_session` to run `echo "export 'glass_card.dart';" >> srcs/app/lib/widgets/BUILD.bazel` to update bazel. (Actually I'll use sed: `sed -i 's/\["\*.dart"\],/\["\*.dart"\],/g' srcs/app/lib/widgets/BUILD.bazel` wait, BUILD.bazel just uses `glob(["*.dart"])` so it automatically picks it up! I will verify this).
3. **Refactor Codebase to use `GlassCard`**
   - I already ran `grep -rn "Card(" srcs/app/lib/` and found the files. I will execute `run_in_bash_session` to run a Python script that reads those specific files, replaces `Card(` with `GlassCard(`, and adds `import 'package:ohc_app/widgets/glass_card.dart';`.
   ```python
   import glob
   files = ['srcs/app/lib/screens/handoffs_screen.dart', 'srcs/app/lib/screens/integrations_screen.dart', 'srcs/app/lib/screens/scaling_screen.dart', 'srcs/app/lib/screens/security_screen.dart', 'srcs/app/lib/screens/settings_screen.dart', 'srcs/app/lib/screens/service_screen.dart', 'srcs/app/lib/screens/meetings_screen.dart', 'srcs/app/lib/screens/cost_dashboard_screen.dart', 'srcs/app/lib/screens/channels_screen.dart']
   for f in files:
       with open(f, 'r') as file:
           content = file.read()
       content = content.replace('Card(', 'GlassCard(')
       if "import 'package:ohc_app/widgets/glass_card.dart';" not in content:
           content = content.replace("import 'package:flutter/material.dart';", "import 'package:flutter/material.dart';\nimport 'package:ohc_app/widgets/glass_card.dart';")
       with open(f, 'w') as file:
           file.write(content)
   ```
   - Execute `run_in_bash_session` to run `cat srcs/app/lib/screens/handoffs_screen.dart` to verify that the change was successfully applied.
4. **Update Mission File Status to DONE**
   - Execute `run_in_bash_session` to run `sed -i 's/status: IN_PROGRESS/status: DONE/' .agent-task/missions/1780000001_palette_proactive_glassmorphism_cards.md`.
   - Execute `run_in_bash_session` to run `cat .agent-task/missions/1780000001_palette_proactive_glassmorphism_cards.md` to verify the update.
5. **Testing**
   - Execute `run_in_bash_session` to run `cd srcs/app && flutter test` to verify all tests still pass.
6. **Pre-commit Steps**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
7. **Submit PR**
   - Execute `submit` with branch `palette-glasscards`, title `🎨 Palette: [Hybrid UX improvement] Apply Glassmorphism to remaining Cards`, and description.
