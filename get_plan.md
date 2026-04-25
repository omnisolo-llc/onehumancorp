1. **Welcome Checklist (Post-Onboarding)**:
   - Create a `Welcome Checklist` banner or card widget directly on the `DashboardScreen`.
   - The checklist should contain these actionable items:
     - ✅ Business live (marked checked by default since they arrived here via onboarding).
     - ⬜ Add 3 more products (links to product add flow).
     - ⬜ Connect Instagram (links to integrations flow).
     - ⬜ Share your link with a friend (shows share dialog or copies to clipboard).
   - *Modification*: Modify `src/app/lib/screens/dashboard_screen.dart` to include this checklist conditionally (or persistently) right below the stats wrap or upgrade banner.
   - *Modification*: I will create a `src/app/lib/widgets/welcome_checklist_widget.dart` for the widget itself to keep the dashboard code clean.

2. **Pre-commit Checks**
   - Run pre-commit instructions.

3. **Submit Changes**
