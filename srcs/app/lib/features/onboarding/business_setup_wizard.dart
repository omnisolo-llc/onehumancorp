import 'dart:ui';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:http/http.dart' as http;

class BusinessSetupWizard extends ConsumerStatefulWidget {
  const BusinessSetupWizard({super.key});
  @override
  ConsumerState<BusinessSetupWizard> createState() => _BusinessSetupWizardState();
}

class _BusinessSetupWizardState extends ConsumerState<BusinessSetupWizard> {
  int _step = 0;
  bool _isLoading = false;

  final _companyNameCtrl = TextEditingController();
  String _selectedIndustry = 'Tech';
  String _selectedSize = 'M';
  String _selectedLanguage = 'English';

  final Map<String, bool> _goals = {
    'Automate customer support': false,
    'Build software faster': false,
    'Generate marketing content': false,
    'Analyze data': false,
    'Custom': false,
  };

  String _deploymentMode = 'Cloud';
  final _adminNameCtrl = TextEditingController();
  final _adminEmailCtrl = TextEditingController();
  final _adminPasswordCtrl = TextEditingController();

  Future<void> _saveState() async {
    final stateData = {
      'step': _step,
      'companyName': _companyNameCtrl.text,
      'industry': _selectedIndustry,
      'size': _selectedSize,
      'language': _selectedLanguage,
      'goals': _goals.entries.where((e) => e.value).map((e) => e.key).toList(),
      'deploymentMode': _deploymentMode,
      'adminName': _adminNameCtrl.text,
      'adminEmail': _adminEmailCtrl.text,
    };

    try {
      await http.post(
        Uri.parse('/api/wizard/state/save'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode(stateData),
      );
    } catch (e) {
      debugPrint("Failed to save state: \$e");
    }
  }

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

  Widget _buildWelcome() {
    return _buildGlassmorphism(
      child: const Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text('Your AI team, ready in minutes', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold)),
            SizedBox(height: 16),
            Text('Set up your business profile to get started.', style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
          ],
        ),
      ),
    );
  }

  Widget _buildProfile() {
    return _buildGlassmorphism(
      child: Column(
        children: [
          TextField(controller: _companyNameCtrl, decoration: const InputDecoration(labelText: 'Company name')),
          DropdownButtonFormField<String>(
            value: _selectedIndustry,
            items: ['Tech', 'Healthcare', 'Finance', 'Retail', 'Other'].map((i) => DropdownMenuItem(value: i, child: Text(i))).toList(),
            onChanged: (v) => setState(() => _selectedIndustry = v!),
            decoration: const InputDecoration(labelText: 'Industry'),
          ),
          DropdownButtonFormField<String>(
            value: _selectedSize,
            items: ['S', 'M', 'L', 'Enterprise'].map((s) => DropdownMenuItem(value: s, child: Text(s))).toList(),
            onChanged: (v) => setState(() => _selectedSize = v!),
            decoration: const InputDecoration(labelText: 'Company Size'),
          ),
          DropdownButtonFormField<String>(
            value: _selectedLanguage,
            items: ['English', 'Spanish', 'French', 'German'].map((l) => DropdownMenuItem(value: l, child: Text(l))).toList(),
            onChanged: (v) => setState(() => _selectedLanguage = v!),
            decoration: const InputDecoration(labelText: 'Primary Language'),
          ),
        ],
      ),
    );
  }

  Widget _buildGoals() {
    return _buildGlassmorphism(
      child: Column(
        children: _goals.keys.map((k) => CheckboxListTile(
          title: Text(k, style: const TextStyle(fontFamily: 'Inter')),
          value: _goals[k],
          onChanged: (v) => setState(() => _goals[k] = v!),
        )).toList(),
      ),
    );
  }

  Widget _buildDeployment() {
    return _buildGlassmorphism(
      child: Column(
        children: ['Cloud', 'Self-hosted Desktop', 'Mobile-only'].map((m) => RadioListTile<String>(
          title: Text(m, style: const TextStyle(fontFamily: 'Inter')),
          value: m,
          groupValue: _deploymentMode,
          onChanged: (v) => setState(() => _deploymentMode = v!),
        )).toList(),
      ),
    );
  }

  Widget _buildAdmin() {
    return _buildGlassmorphism(
      child: Column(
        children: [
          TextField(controller: _adminNameCtrl, decoration: const InputDecoration(labelText: 'Name')),
          TextField(controller: _adminEmailCtrl, decoration: const InputDecoration(labelText: 'Email')),
          TextField(controller: _adminPasswordCtrl, obscureText: true, decoration: const InputDecoration(labelText: 'Password')),
          const SizedBox(height: 8),
          const LinearProgressIndicator(value: 0.5, backgroundColor: Colors.grey, color: Colors.green),
          const SizedBox(height: 8),
          const Text('Password Strength: Medium', style: TextStyle(fontFamily: 'Inter', fontSize: 12)),
          const SizedBox(height: 16),
          ElevatedButton.icon(onPressed: () {}, icon: const Icon(Icons.login), label: const Text('Sign in with Google')),
          ElevatedButton.icon(onPressed: () {}, icon: const Icon(Icons.code), label: const Text('Sign in with GitHub')),
        ],
      ),
    );
  }

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

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Stepper(
        currentStep: _step,
        onStepContinue: () {
          if (_step < 5) {
            setState(() => _step++);
            _saveState();
          }
        },
        onStepCancel: () {
          if (_step > 0) {
            setState(() => _step--);
          }
        },
        steps: [
          Step(title: const Text('Welcome', style: TextStyle(fontFamily: 'Outfit')), content: _buildWelcome()),
          Step(title: const Text('Business Profile', style: TextStyle(fontFamily: 'Outfit')), content: _buildProfile()),
          Step(title: const Text('Goal Selection', style: TextStyle(fontFamily: 'Outfit')), content: _buildGoals()),
          Step(title: const Text('Deployment', style: TextStyle(fontFamily: 'Outfit')), content: _buildDeployment()),
          Step(title: const Text('Admin Account', style: TextStyle(fontFamily: 'Outfit')), content: _buildAdmin()),
          Step(title: const Text('Review & Launch', style: TextStyle(fontFamily: 'Outfit')), content: _buildReview()),
        ],
      ),
    );
  }
}
