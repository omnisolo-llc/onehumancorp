import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../widgets/glass_card.dart';

class BusinessSetupWizardScreen extends ConsumerStatefulWidget {
  const BusinessSetupWizardScreen({super.key});

  @override
  ConsumerState<BusinessSetupWizardScreen> createState() => _BusinessSetupWizardScreenState();
}

class _BusinessSetupWizardScreenState extends ConsumerState<BusinessSetupWizardScreen> {
  int _step = 0;
  String _companyName = '';
  String _industry = '';
  String _size = 'S';
  List<String> _goals = [];
  String _deployment = 'Cloud';
  String _adminName = '';
  String _adminEmail = '';
  String _adminPassword = '';

  void _nextStep() {
    if (_step < 4) {
      setState(() => _step++);
    } else {
      _launch();
    }
  }

  void _launch() {
    // Perform launch logic
    // Mocking launch logic here since the actual API might not exist yet based on exploration.
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 600),
          child: GlassCard(
            child: Padding(
              padding: const EdgeInsets.all(24.0),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Text(
                    'Business Setup',
                    style: const TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 24,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  const SizedBox(height: 16),
                  if (_step == 0) ...[
                    const Text('Welcome! Your AI team, ready in minutes.', style: TextStyle(fontFamily: 'Inter')),
                    const SizedBox(height: 16),
                  ] else if (_step == 1) ...[
                    TextField(
                      decoration: const InputDecoration(labelText: 'Company Name'),
                      onChanged: (v) => _companyName = v,
                      style: const TextStyle(fontFamily: 'Inter'),
                    ),
                  ] else if (_step == 2) ...[
                     const Text('Select Goals', style: TextStyle(fontFamily: 'Inter')),
                  ] else if (_step == 3) ...[
                     const Text('Deployment Preference', style: TextStyle(fontFamily: 'Inter')),
                  ] else if (_step == 4) ...[
                    TextField(
                      decoration: const InputDecoration(labelText: 'Admin Name'),
                      onChanged: (v) => _adminName = v,
                      style: const TextStyle(fontFamily: 'Inter'),
                    ),
                  ],
                  ElevatedButton(
                    onPressed: _nextStep,
                    child: Text(_step == 4 ? 'Launch My AI Team →' : 'Next', style: const TextStyle(fontFamily: 'Inter')),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
