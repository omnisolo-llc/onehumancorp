1. **Fix Glassmorphism & add Expert Mode to `business_setup_wizard.dart`:**
   - Use `replace_with_git_merge_diff` on `lib/features/onboarding/business_setup_wizard.dart` to apply this diff:
```dart
<<<<<<< SEARCH
class _BusinessSetupWizardState extends ConsumerState<BusinessSetupWizard> {
  int _step = 0;
  bool _isLoading = false;

  final _companyNameCtrl = TextEditingController();
=======
class _BusinessSetupWizardState extends ConsumerState<BusinessSetupWizard> {
  int _step = 0;
  bool _isLoading = false;
  bool _expertMode = false;

  final _companyNameCtrl = TextEditingController();
>>>>>>> REPLACE
```
   - Use `replace_with_git_merge_diff` on `lib/features/onboarding/business_setup_wizard.dart` to apply this diff:
```dart
<<<<<<< SEARCH
  Widget _buildReview() {
    return _buildGlassmorphism(
      child: Column(
        children: [
          const Text('Review & Launch', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold)),
          const SizedBox(height: 16),
          ListTile(title: const Text('Company'), subtitle: Text(_companyNameCtrl.text)),
          ListTile(title: const Text('Deployment'), subtitle: Text(_deploymentMode)),
          const SizedBox(height: 16),
          ElevatedButton(
            onPressed: () {},
            style: ElevatedButton.styleFrom(
              padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 16),
            ),
            child: const Text('Launch My AI Team →', style: TextStyle(fontFamily: 'Inter', fontSize: 18)),
          ),
        ],
      ),
    );
  }
=======
  Widget _buildReview() {
    return _buildGlassmorphism(
      child: Column(
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text('Review & Launch', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold)),
              Row(
                children: [
                  const Text('Expert Mode', style: TextStyle(fontFamily: 'Inter', fontSize: 12)),
                  Switch(value: _expertMode, onChanged: (v) => setState(() => _expertMode = v)),
                ],
              ),
            ],
          ),
          const SizedBox(height: 16),
          ListTile(title: const Text('Company'), subtitle: Text(_companyNameCtrl.text)),
          ListTile(title: const Text('Deployment'), subtitle: Text(_deploymentMode)),
          if (_expertMode) ...[
             const SizedBox(height: 16),
             Container(
               color: Colors.black12,
               padding: const EdgeInsets.all(8),
               child: Text('Raw Config Fields: \ncompanyName: ${_companyNameCtrl.text}\nindustry: $_selectedIndustry\nsize: $_selectedSize\nlanguage: $_selectedLanguage\ngoals: ${_goals.entries.where((e) => e.value).map((e) => e.key).toList()}\ndeploymentMode: $_deploymentMode', style: const TextStyle(fontFamily: 'monospace')),
             )
          ],
          const SizedBox(height: 16),
          ElevatedButton(
            onPressed: () {},
            style: ElevatedButton.styleFrom(
              padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 16),
            ),
            child: const Text('Launch My AI Team →', style: TextStyle(fontFamily: 'Inter', fontSize: 18)),
          ),
        ],
      ),
    );
  }
>>>>>>> REPLACE
```
   - Use `run_in_bash_session` to execute: `sed -i 's/filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),/filter: ImageFilter.compose(outer: ColorFilter.matrix([2.0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 1, 0]), inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0)),/' lib/features/onboarding/business_setup_wizard.dart`.
2. **Verify `business_setup_wizard.dart`:**
   - Run `cat lib/features/onboarding/business_setup_wizard.dart` to verify changes.
3. **Fix Glassmorphism in `ai_agent_config_wizard.dart`:**
   - Use `run_in_bash_session` to execute: `sed -i 's/filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),/filter: ImageFilter.compose(outer: ColorFilter.matrix([2.0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 1, 0]), inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0)),/' lib/features/wizard/ai_agent_config_wizard.dart`.
4. **Verify `ai_agent_config_wizard.dart`:**
   - Run `cat lib/features/wizard/ai_agent_config_wizard.dart` to verify.
5. **Fix Glassmorphism & add Expert Mode to `prompt_tuning_wizard.dart`:**
   - Use `replace_with_git_merge_diff` on `lib/features/wizard/prompt_tuning_wizard.dart` to apply this diff:
```dart
<<<<<<< SEARCH
class _PromptTuningWizardState extends ConsumerState<PromptTuningWizard> {
  int _step = 0;
  bool _isSaving = false;

  String _personality = 'Friendly';
=======
class _PromptTuningWizardState extends ConsumerState<PromptTuningWizard> {
  int _step = 0;
  bool _isSaving = false;
  bool _expertMode = false;

  String _personality = 'Friendly';
>>>>>>> REPLACE
```
   - Use `replace_with_git_merge_diff` on `lib/features/wizard/prompt_tuning_wizard.dart` to apply this diff:
```dart
<<<<<<< SEARCH
  Widget _buildStep4Preview() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Step 4 — Live Preview', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        ClipRRect(
          borderRadius: BorderRadius.circular(16),
          child: BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            child: Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Theme.of(context).colorScheme.surface.withOpacity(0.1),
                border: Border.all(color: Theme.of(context).colorScheme.onSurface.withOpacity(0.2)),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text('System Prompt Generated:', style: TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
                  const SizedBox(height: 8),
                  Text('You are a $_personality AI assistant.\nRules:\n${_domainFocus.join('\n')}\nExamples:\n${_examples.map((e) => "User: ${e['q']}\nAgent: ${e['a']}").join('\n')}', style: const TextStyle(fontFamily: 'Inter')),
                ],
              ),
            ),
          ),
        ),
      ],
    );
  }
=======
  Widget _buildStep4Preview() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text('Step 4 — Live Preview', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
              Row(
                children: [
                  const Text('Expert Mode', style: TextStyle(fontFamily: 'Inter', fontSize: 12)),
                  Switch(value: _expertMode, onChanged: (v) => setState(() => _expertMode = v)),
                ],
              ),
            ],
          ),
        const SizedBox(height: 16),
        ClipRRect(
          borderRadius: BorderRadius.circular(16),
          child: BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            child: Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Theme.of(context).colorScheme.surface.withOpacity(0.1),
                border: Border.all(color: Theme.of(context).colorScheme.onSurface.withOpacity(0.2)),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text('System Prompt Generated:', style: TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
                  const SizedBox(height: 8),
                  if (_expertMode)
                    const Text('RAW JSON PAYLOAD: \n{"model":"claude-v3", "tokens": 4096}', style: TextStyle(fontFamily: 'monospace')),
                  Text('You are a $_personality AI assistant.\nRules:\n${_domainFocus.join('\n')}\nExamples:\n${_examples.map((e) => "User: ${e['q']}\nAgent: ${e['a']}").join('\n')}', style: const TextStyle(fontFamily: 'Inter')),
                ],
              ),
            ),
          ),
        ),
      ],
    );
  }
>>>>>>> REPLACE
```
   - Use `run_in_bash_session` to execute: `sed -i 's/filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),/filter: ImageFilter.compose(outer: ColorFilter.matrix([2.0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 1, 0]), inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0)),/' lib/features/wizard/prompt_tuning_wizard.dart`.
6. **Verify `prompt_tuning_wizard.dart`:**
   - Run `cat lib/features/wizard/prompt_tuning_wizard.dart` to verify.
7. **Create `ongoing_management_wizards.dart`:**
   - Use `run_in_bash_session` to run:
```bash
cat << 'EOF2' > lib/features/wizard/ongoing_management_wizards.dart
import 'dart:ui';
import 'package:flutter/material.dart';

class FixThisWizard extends StatefulWidget {
  const FixThisWizard({super.key});
  @override
  State<FixThisWizard> createState() => _FixThisWizardState();
}

class _FixThisWizardState extends State<FixThisWizard> {
  int _step = 0;

  Widget _buildGlassmorphism(Widget child) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: BackdropFilter(
        filter: ImageFilter.compose(
          outer: const ColorFilter.matrix([
            2.0, 0, 0, 0, 0,
            0, 2.0, 0, 0, 0,
            0, 0, 2.0, 0, 0,
            0, 0, 0, 1, 0,
          ]),
          inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        ),
        child: Container(
          padding: const EdgeInsets.all(16),
          decoration: BoxDecoration(
            color: Colors.white.withOpacity(0.05),
            borderRadius: BorderRadius.circular(12),
            border: Border.all(color: Colors.white.withOpacity(0.1)),
          ),
          child: child,
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Fix This Wizard')),
      body: Stepper(
        currentStep: _step,
        onStepContinue: () { if (_step < 2) setState(() => _step++); },
        onStepCancel: () { if (_step > 0) setState(() => _step--); },
        steps: [
          Step(title: const Text('Error Summary', style: TextStyle(fontFamily: 'Outfit')), content: _buildGlassmorphism(const Text('Something went wrong.', style: TextStyle(fontFamily: 'Inter')))),
          Step(title: const Text('Suggested Fix', style: TextStyle(fontFamily: 'Outfit')), content: _buildGlassmorphism(const Text('Apply fix now.', style: TextStyle(fontFamily: 'Inter')))),
          Step(title: const Text('Confirmation', style: TextStyle(fontFamily: 'Outfit')), content: _buildGlassmorphism(const Text('Fixed.', style: TextStyle(fontFamily: 'Inter')))),
        ],
      )
    );
  }
}

class UpgradeWizard extends StatelessWidget {
  const UpgradeWizard({super.key});
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Upgrade Wizard')),
      body: const Center(child: Text('Upgrade Wizard', style: TextStyle(fontFamily: 'Outfit'))),
    );
  }
}

class BillingWizard extends StatelessWidget {
  const BillingWizard({super.key});
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Billing Wizard')),
      body: const Center(child: Text('Billing Wizard', style: TextStyle(fontFamily: 'Outfit'))),
    );
  }
}
EOF2
```
8. **Verify `ongoing_management_wizards.dart`:**
   - Run `cat lib/features/wizard/ongoing_management_wizards.dart`.
9. **Create `ongoing_management_wizards_test.dart`:**
   - Use `run_in_bash_session` to run:
```bash
cat << 'EOF2' > lib/features/wizard/ongoing_management_wizards_test.dart
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'ongoing_management_wizards.dart';

void main() {
  testWidgets('FixThisWizard renders', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: FixThisWizard()));
    expect(find.text('Fix This Wizard'), findsOneWidget);
  });
}
EOF2
```
10. **Verify `ongoing_management_wizards_test.dart`:**
    - Run `cat lib/features/wizard/ongoing_management_wizards_test.dart`.
11. **Update `BUILD.bazel`:**
    - Use `run_in_bash_session` to run:
```bash
cat << 'EOF2' > lib/features/wizard/BUILD.bazel
load("@rules_flutter//flutter:defs.bzl", "flutter_library", "flutter_test")

flutter_library(
    name = "ongoing_management_wizards",
    srcs = ["ongoing_management_wizards.dart"],
)

flutter_test(
    name = "ongoing_management_wizards_test",
    srcs = ["ongoing_management_wizards_test.dart"],
    deps = [":ongoing_management_wizards"],
)
EOF2
```
12. **Verify `BUILD.bazel`:**
    - Run `cat lib/features/wizard/BUILD.bazel`.
13. **Run Tests:**
    - Run `bazelisk test //...`.
14. **Pre Commit Steps:**
    - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
15. **Submit:**
    - Submit the PR with the message `🧙 Wizard: Implement and refine wizard and onboarding experience`.
