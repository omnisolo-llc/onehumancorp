import 'dart:ui';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:http/http.dart' as http;

class FixThisWizard extends ConsumerStatefulWidget {
  const FixThisWizard({super.key});

  @override
  ConsumerState<FixThisWizard> createState() => _FixThisWizardState();
}

class _FixThisWizardState extends ConsumerState<FixThisWizard> {
  int _step = 0;
  bool _isFixing = false;
  bool _expertMode = false;
  String _errorSummary = "The agent is experiencing high latency and connection drops to the external provider.";
  String _suggestedFix = "Restart the agent and clear its current session cache.";
  String _rawError = "Error 504 Gateway Timeout: connection pool exhausted. Trace ID: 9f8a8b1c";

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

  Widget _buildStep1Error() {
    return _buildGlassmorphism(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text('Error Summary', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
              Row(
                children: [
                  const Text('Expert Mode', style: TextStyle(fontFamily: 'Inter', fontSize: 12)),
                  Switch(value: _expertMode, onChanged: (v) => setState(() => _expertMode = v)),
                ],
              ),
            ],
          ),
          const SizedBox(height: 16),
          Text(_errorSummary, style: const TextStyle(fontFamily: 'Inter', fontSize: 16)),
          if (_expertMode) ...[
            const SizedBox(height: 16),
            const Text('Raw Log:', style: TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Inter')),
            const SizedBox(height: 8),
            Container(
              padding: const EdgeInsets.all(8),
              color: Colors.black.withOpacity(0.3),
              child: Text(_rawError, style: const TextStyle(fontFamily: 'monospace', color: Colors.redAccent)),
            ),
          ]
        ],
      ),
    );
  }

  Widget _buildStep2Fix() {
    return _buildGlassmorphism(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('Suggested Fix', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
          const SizedBox(height: 16),
          Text(_suggestedFix, style: const TextStyle(fontFamily: 'Inter', fontSize: 16)),
          if (_expertMode) ...[
            const SizedBox(height: 16),
            const Text('Action:', style: TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Inter')),
            const SizedBox(height: 8),
            const Text('POST /api/wizard/fix { "agent_id": "current", "action": "restart_and_clear_cache" }', style: TextStyle(fontFamily: 'monospace')),
          ],
          const SizedBox(height: 24),
          ElevatedButton(
            onPressed: _isFixing ? null : _handleFix,
            style: ElevatedButton.styleFrom(
              padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
            ),
            child: _isFixing ? const SizedBox(width: 24, height: 24, child: CircularProgressIndicator(strokeWidth: 2)) : const Text('Apply fix', style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
          ),
        ],
      ),
    );
  }

  Widget _buildStep3Confirmation() {
    return _buildGlassmorphism(
      child: const Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(Icons.check_circle, color: Colors.green, size: 28),
              SizedBox(width: 8),
              Text('Agent Restored', style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
            ],
          ),
          SizedBox(height: 16),
          Text('The fix was applied successfully and the agent is healthy again.', style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
        ],
      ),
    );
  }

  Future<void> _handleFix() async {
    setState(() => _isFixing = true);

    try {
      // In a real app this would read from a configuration provider,
      // but matching the exact pattern used in other files (like business_setup_wizard.dart)
      // which assumes a same-origin proxy is serving /api. We fallback to localhost for tests.
      final uri = Uri.tryParse('/api/wizard/fix');
      final targetUri = uri != null && uri.hasScheme ? uri : Uri.parse('http://localhost:8080/api/wizard/fix');

      final response = await http.post(
        targetUri,
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({
          'agent_id': 'current',
          'action': 'restart_and_clear_cache',
          'expert_mode': _expertMode.toString(),
        }),
      );

      if (!mounted) return;

      if (response.statusCode == 200) {
        setState(() {
          _isFixing = false;
          _step = 2; // Move to confirmation step
        });
      } else {
        setState(() => _isFixing = false);
        ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Failed to apply fix.')));
      }
    } catch (e) {
      if (!mounted) return;
      setState(() => _isFixing = false);
      ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Network error.')));
    }
  }

  @override
  Widget build(BuildContext context) {
    return Stepper(
      currentStep: _step,
      onStepContinue: () {
        if (_step == 0) {
          setState(() => _step++);
        } else if (_step == 1) {
          _handleFix();
        }
      },
      onStepCancel: () {
        if (_step > 0 && _step < 2) {
          setState(() => _step--);
        }
      },
      controlsBuilder: (context, details) {
        if (_step == 2) return const SizedBox.shrink(); // Hide controls on confirmation

        return Padding(
          padding: const EdgeInsets.only(top: 16.0),
          child: Row(
            children: [
              if (_step == 0)
                ElevatedButton(onPressed: details.onStepContinue, child: const Text('Next')),
              if (_step == 1)
                const SizedBox.shrink(), // Button is inside the step content
              const SizedBox(width: 12),
              if (_step > 0)
                TextButton(onPressed: details.onStepCancel, child: const Text('Back')),
            ],
          ),
        );
      },
      steps: [
        Step(title: const Text('Issue Overview', style: TextStyle(fontFamily: 'Outfit')), content: _buildStep1Error(), isActive: _step >= 0),
        Step(title: const Text('Resolution', style: TextStyle(fontFamily: 'Outfit')), content: _buildStep2Fix(), isActive: _step >= 1),
        Step(title: const Text('Confirmation', style: TextStyle(fontFamily: 'Outfit')), content: _buildStep3Confirmation(), isActive: _step >= 2),
      ],
    );
  }
}
