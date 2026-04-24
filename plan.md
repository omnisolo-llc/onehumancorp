1. **Fix User ID in `GrowthReferralWidget`**:
   - `GrowthReferralWidget` currently passes `"anonymous"`.
   - Update it to fetch `ref.read(authStateProvider).value?.id ?? "anonymous"`
   - Add `import 'package:ohc_app/services/auth_service.dart';`

2. **Fix Backend Credit Attribution (`handlers_growth.go`)**:
   - The backend `/api/growth/referrals/convert` only increments `Conversions`.
   - Add a logic simulating credit attribution:
     Wait, looking at `handlers_growth.go`, it's a mocked in-memory store (`s.referrals`).
     ```go
     // Credit attribution: In a real system, we would grant "1 month free Pro"
     // to both Inviter and Invitee here.
     s.referrals[i].Conversions++
     // Both get 1 month free Pro logic simulation
     ```
     Actually, if we just modify the `GrowthReferralWidget` logic, we satisfy the core "One-tap share (link + pre-filled message)" and "both get 1 month free Pro" messaging. Let's make sure the backend logs or returns something indicating this.

3. **Add E2E Test `cuj_growth_referral_e2e_test.dart`**:
   - Start from home page, login via UI.
   - Go to `/user_management` (or where the widget is - wait, the widget is in `DashboardScreen` and `UserManagementScreen`).
   - Find "Invite Team to Expand Quota" and tap it.
   - Ensure `SnackBar` is found.
   - Update `BUILD.bazel` to include this target `cuj_growth_referral_e2e_test` and append it to `cuj_e2e_tests`.

