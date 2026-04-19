import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class FixThisWizard extends ConsumerStatefulWidget {
  const FixThisWizard({super.key});

  @override
  ConsumerState<FixThisWizard> createState() => _FixThisWizardState();
}

class _FixThisWizardState extends ConsumerState<FixThisWizard> {
  int _step = 0;
  bool _isFixing = false;

  Widget _buildGlassmorphism({required Widget child}) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
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

  Widget _buildStep1() {
    return _buildGlassmorphism(
      child: const Text('Agent lost connection to the database. This usually means the credentials rotated or the network dropped.', style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
    );
  }

  Widget _buildStep2() {
    return _buildGlassmorphism(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('Suggested Fix:', style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold)),
          const SizedBox(height: 8),
          const Text('Rotate credentials and restart the agent connection pool.', style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
          const SizedBox(height: 16),
          ElevatedButton(
            onPressed: () async {
              setState(() => _isFixing = true);
              await Future.delayed(const Duration(seconds: 1));
              if (mounted) setState(() { _isFixing = false; _step = 2; });
            },
            child: _isFixing ? const CircularProgressIndicator() : const Text('Apply fix', style: TextStyle(fontFamily: 'Inter')),
          )
        ],
      ),
    );
  }

  Widget _buildStep3() {
    return _buildGlassmorphism(
      child: const Text('Fix applied successfully! The agent is healthy again.', style: TextStyle(fontFamily: 'Inter', fontSize: 16, color: Colors.green)),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Stepper(
      currentStep: _step,
      onStepContinue: () {
        if (_step < 2) setState(() => _step += 1);
      },
      onStepCancel: () {
        if (_step > 0) setState(() => _step -= 1);
      },
      controlsBuilder: (context, details) {
        return Padding(
          padding: const EdgeInsets.only(top: 16.0),
          child: Row(
            children: [
              if (_step < 1) ElevatedButton(onPressed: details.onStepContinue, child: const Text('Next')),
              const SizedBox(width: 8),
              if (_step > 0 && _step < 2) TextButton(onPressed: details.onStepCancel, child: const Text('Back')),
            ],
          ),
        );
      },
      steps: [
        Step(title: const Text('Error Summary', style: TextStyle(fontFamily: 'Outfit')), isActive: _step >= 0, content: _buildStep1()),
        Step(title: const Text('Suggested Fix', style: TextStyle(fontFamily: 'Outfit')), isActive: _step >= 1, content: _buildStep2()),
        Step(title: const Text('Confirmation', style: TextStyle(fontFamily: 'Outfit')), isActive: _step >= 2, content: _buildStep3()),
      ],
    );
  }
}

class UpgradeWizard extends ConsumerStatefulWidget {
  const UpgradeWizard({super.key});

  @override
  ConsumerState<UpgradeWizard> createState() => _UpgradeWizardState();
}

class _UpgradeWizardState extends ConsumerState<UpgradeWizard> {
  int _step = 0;
  bool _isUpgrading = false;

  Widget _buildGlassmorphism({required Widget child}) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
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

  Widget _buildStep1() {
    return _buildGlassmorphism(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: const [
          Text('What\'s new ✨', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold)),
          SizedBox(height: 8),
          Text('- Better performance\n- New UI components\n- Bug fixes', style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
        ],
      ),
    );
  }

  Widget _buildStep2() {
    return _buildGlassmorphism(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('Ready to upgrade?', style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold)),
          const SizedBox(height: 16),
          ElevatedButton(
            onPressed: () async {
              setState(() => _isUpgrading = true);
              await Future.delayed(const Duration(seconds: 1));
              if (mounted) setState(() { _isUpgrading = false; _step = 2; });
            },
            child: _isUpgrading ? const CircularProgressIndicator() : const Text('Upgrade in 1 click', style: TextStyle(fontFamily: 'Inter')),
          ),
          const SizedBox(height: 8),
          TextButton(
            onPressed: () {},
            child: const Text('Rollback option available after upgrade', style: TextStyle(fontFamily: 'Inter', fontSize: 12)),
          )
        ],
      ),
    );
  }

  Widget _buildStep3() {
    return _buildGlassmorphism(
      child: const Text('Upgrade complete!', style: TextStyle(fontFamily: 'Inter', fontSize: 16, color: Colors.green)),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Stepper(
      currentStep: _step,
      onStepContinue: () {
        if (_step < 2) setState(() => _step += 1);
      },
      onStepCancel: () {
        if (_step > 0) setState(() => _step -= 1);
      },
      controlsBuilder: (context, details) {
        return Padding(
          padding: const EdgeInsets.only(top: 16.0),
          child: Row(
            children: [
              if (_step < 1) ElevatedButton(onPressed: details.onStepContinue, child: const Text('Next')),
              const SizedBox(width: 8),
              if (_step > 0 && _step < 2) TextButton(onPressed: details.onStepCancel, child: const Text('Back')),
            ],
          ),
        );
      },
      steps: [
        Step(title: const Text('Release Notes', style: TextStyle(fontFamily: 'Outfit')), isActive: _step >= 0, content: _buildStep1()),
        Step(title: const Text('Upgrade', style: TextStyle(fontFamily: 'Outfit')), isActive: _step >= 1, content: _buildStep2()),
        Step(title: const Text('Complete', style: TextStyle(fontFamily: 'Outfit')), isActive: _step >= 2, content: _buildStep3()),
      ],
    );
  }
}

class BillingWizard extends ConsumerStatefulWidget {
  const BillingWizard({super.key});

  @override
  ConsumerState<BillingWizard> createState() => _BillingWizardState();
}

class _BillingWizardState extends ConsumerState<BillingWizard> {
  int _step = 0;

  Widget _buildGlassmorphism({required Widget child}) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
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

  Widget _buildStep1() {
    return _buildGlassmorphism(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: const [
          Text('Current Usage', style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold)),
          SizedBox(height: 8),
          Text('150 hours used', style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
          SizedBox(height: 8),
          LinearProgressIndicator(value: 0.75),
        ],
      ),
    );
  }

  Widget _buildStep2() {
    return _buildGlassmorphism(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: const [
          Text('Projected Monthly Cost', style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold)),
          SizedBox(height: 8),
          Text('\$45.00', style: TextStyle(fontFamily: 'Inter', fontSize: 24, fontWeight: FontWeight.bold)),
        ],
      ),
    );
  }

  Widget _buildStep3() {
    return _buildGlassmorphism(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('Add Credits / Switch Plan', style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold)),
          const SizedBox(height: 16),
          ElevatedButton(onPressed: () {}, child: const Text('Add \$50 Credits', style: TextStyle(fontFamily: 'Inter'))),
          const SizedBox(height: 8),
          OutlinedButton(onPressed: () {}, child: const Text('Switch to Pro Plan', style: TextStyle(fontFamily: 'Inter'))),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Stepper(
      currentStep: _step,
      onStepContinue: () {
        if (_step < 2) setState(() => _step += 1);
      },
      onStepCancel: () {
        if (_step > 0) setState(() => _step -= 1);
      },
      steps: [
        Step(title: const Text('Usage', style: TextStyle(fontFamily: 'Outfit')), isActive: _step >= 0, content: _buildStep1()),
        Step(title: const Text('Projection', style: TextStyle(fontFamily: 'Outfit')), isActive: _step >= 1, content: _buildStep2()),
        Step(title: const Text('Manage', style: TextStyle(fontFamily: 'Outfit')), isActive: _step >= 2, content: _buildStep3()),
      ],
    );
  }
}
