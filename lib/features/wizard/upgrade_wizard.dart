import 'dart:ui';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:http/http.dart' as http;

class UpgradeWizard extends ConsumerStatefulWidget {
  const UpgradeWizard({super.key});

  @override
  ConsumerState<UpgradeWizard> createState() => _UpgradeWizardState();
}

class _UpgradeWizardState extends ConsumerState<UpgradeWizard> {
  int _step = 0;
  bool _isUpgrading = false;
  bool _isRollingBack = false;

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

  Widget _buildStep1WhatsNew() {
    return _buildGlassmorphism(
      child: const Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('What\'s new ✨', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
          SizedBox(height: 16),
          Text('• Improved autoDream consolidation\n• KAIROS shared task list enhancements\n• General bug fixes', style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
        ],
      ),
    );
  }

  Widget _buildStep2Progress() {
    return _buildGlassmorphism(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('Upgrading System...', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
          const SizedBox(height: 16),
          const LinearProgressIndicator(),
          const SizedBox(height: 16),
          ElevatedButton(
            onPressed: _isUpgrading ? null : _handleUpgrade,
            child: _isUpgrading ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2)) : const Text('Upgrade in 1 click', style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
          ),
        ],
      ),
    );
  }

  Widget _buildStep3Confirmation() {
    return _buildGlassmorphism(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Row(
            children: [
              Icon(Icons.check_circle, color: Colors.green, size: 28),
              SizedBox(width: 8),
              Text('Upgrade Complete', style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
            ],
          ),
          const SizedBox(height: 16),
          ElevatedButton(
            onPressed: _isRollingBack ? null : _handleRollback,
            style: ElevatedButton.styleFrom(backgroundColor: Colors.redAccent),
            child: _isRollingBack ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2)) : const Text('Rollback', style: TextStyle(fontFamily: 'Inter', fontSize: 16, color: Colors.white)),
          ),
        ],
      ),
    );
  }

  Future<void> _handleUpgrade() async {
    setState(() => _isUpgrading = true);
    try {
      final uri = Uri.tryParse('/api/wizard/upgrade');
      final targetUri = uri != null && uri.hasScheme ? uri : Uri.parse('http://localhost:8080/api/wizard/upgrade');
      final response = await http.post(targetUri, headers: {'Content-Type': 'application/json'}, body: jsonEncode({}));
      if (!mounted) return;
      if (response.statusCode == 200) {
        setState(() {
          _isUpgrading = false;
          _step = 2;
        });
      } else {
        setState(() => _isUpgrading = false);
        ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Failed to upgrade.')));
      }
    } catch (e) {
      if (!mounted) return;
      setState(() => _isUpgrading = false);
      ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Network error.')));
    }
  }

  Future<void> _handleRollback() async {
    setState(() => _isRollingBack = true);
    try {
      final uri = Uri.tryParse('/api/wizard/rollback');
      final targetUri = uri != null && uri.hasScheme ? uri : Uri.parse('http://localhost:8080/api/wizard/rollback');
      final response = await http.post(targetUri, headers: {'Content-Type': 'application/json'}, body: jsonEncode({}));
      if (!mounted) return;
      if (response.statusCode == 200) {
        setState(() {
          _isRollingBack = false;
          _step = 0;
        });
        ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Rollback successful.')));
      } else {
        setState(() => _isRollingBack = false);
        ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Failed to rollback.')));
      }
    } catch (e) {
      if (!mounted) return;
      setState(() => _isRollingBack = false);
      ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Network error.')));
    }
  }

  @override
  Widget build(BuildContext context) {
    return Stepper(
      currentStep: _step,
      onStepContinue: () {
        if (_step == 0) setState(() => _step++);
        else if (_step == 1) _handleUpgrade();
      },
      onStepCancel: () {
        if (_step > 0 && _step < 2) setState(() => _step--);
      },
      controlsBuilder: (context, details) {
        if (_step == 2) return const SizedBox.shrink();
        return Padding(
          padding: const EdgeInsets.only(top: 16.0),
          child: Row(
            children: [
              if (_step == 0) ElevatedButton(onPressed: details.onStepContinue, child: const Text('Next')),
              if (_step == 1) const SizedBox.shrink(),
              const SizedBox(width: 12),
              if (_step > 0) TextButton(onPressed: details.onStepCancel, child: const Text('Back')),
            ],
          ),
        );
      },
      steps: [
        Step(title: const Text('What\'s New', style: TextStyle(fontFamily: 'Outfit')), content: _buildStep1WhatsNew(), isActive: _step >= 0),
        Step(title: const Text('Upgrade', style: TextStyle(fontFamily: 'Outfit')), content: _buildStep2Progress(), isActive: _step >= 1),
        Step(title: const Text('Confirmation', style: TextStyle(fontFamily: 'Outfit')), content: _buildStep3Confirmation(), isActive: _step >= 2),
      ],
    );
  }
}
