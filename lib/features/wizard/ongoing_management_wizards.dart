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

class UpgradeWizard extends StatefulWidget {
  const UpgradeWizard({super.key});
  @override
  State<UpgradeWizard> createState() => _UpgradeWizardState();
}

class _UpgradeWizardState extends State<UpgradeWizard> {
  bool _isUpgrading = false;
  double _progress = 0.0;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Upgrade Wizard')),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(color: Colors.blue.withOpacity(0.1), borderRadius: BorderRadius.circular(12)),
              child: const Row(
                children: [
                  Icon(Icons.star, color: Colors.blue),
                  SizedBox(width: 8),
                  Text("What's new ✨: Supercharged AI capabilities!", style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
                ],
              ),
            ),
            const SizedBox(height: 32),
            if (!_isUpgrading)
              ElevatedButton(
                onPressed: () {
                  setState(() => _isUpgrading = true);
                  Future.delayed(const Duration(milliseconds: 500), () => setState(() => _progress = 0.5));
                  Future.delayed(const Duration(milliseconds: 1000), () => setState(() => _progress = 1.0));
                },
                child: const Text('Upgrade in 1 click', style: TextStyle(fontFamily: 'Inter')),
              )
            else ...[
              const Text('Upgrading...', style: TextStyle(fontFamily: 'Inter')),
              const SizedBox(height: 8),
              LinearProgressIndicator(value: _progress),
              const SizedBox(height: 16),
              TextButton(onPressed: () => setState(() { _isUpgrading = false; _progress = 0.0; }), child: const Text('Rollback', style: TextStyle(fontFamily: 'Inter', color: Colors.red))),
            ]
          ],
        ),
      ),
    );
  }
}

class BillingWizard extends StatelessWidget {
  const BillingWizard({super.key});
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Billing & Credits Wizard')),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text('How much does this cost?', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold)),
            const SizedBox(height: 16),
            ListTile(
              title: const Text('Current Usage', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
              trailing: const Text('\$42.50', style: TextStyle(fontFamily: 'Outfit', fontSize: 20)),
              tileColor: Colors.grey.withOpacity(0.1),
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
            ),
            const SizedBox(height: 8),
            ListTile(
              title: const Text('Projected Monthly Cost', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
              trailing: const Text('\$120.00', style: TextStyle(fontFamily: 'Outfit', fontSize: 20)),
              tileColor: Colors.grey.withOpacity(0.1),
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
            ),
            const SizedBox(height: 32),
            Row(
              children: [
                ElevatedButton(onPressed: () {}, child: const Text('Add Credits', style: TextStyle(fontFamily: 'Inter'))),
                const SizedBox(width: 16),
                OutlinedButton(onPressed: () {}, child: const Text('Switch Plan', style: TextStyle(fontFamily: 'Inter'))),
              ],
            )
          ],
        ),
      ),
    );
  }
}
