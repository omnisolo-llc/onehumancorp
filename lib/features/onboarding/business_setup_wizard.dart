import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';

class BusinessSetupWizard extends ConsumerStatefulWidget {
  const BusinessSetupWizard({super.key});

  @override
  ConsumerState<BusinessSetupWizard> createState() => _BusinessSetupWizardState();
}

class _BusinessSetupWizardState extends ConsumerState<BusinessSetupWizard> {
  int _step = 0;
  bool _isLoading = false;

  final _companyNameController = TextEditingController();
  String _selectedIndustry = 'Technology';
  String _selectedSize = 'Small';
  String _selectedLanguage = 'English';

  final Map<String, bool> _goals = {
    'Automate customer support': false,
    'Build software faster': false,
    'Generate marketing content': false,
    'Analyze data': false,
    'Custom': false,
  };

  String _selectedDeployment = 'Cloud (managed)';

  final _adminNameController = TextEditingController();
  final _adminEmailController = TextEditingController();
  final _adminPasswordController = TextEditingController();
  final _formKey = GlobalKey<FormState>();

  @override
  void initState() {
    super.initState();
    _loadState();
  }

  Future<void> _loadState() async {
    setState(() => _isLoading = true);
    try {
      final res = await http.get(Uri.parse(const String.fromEnvironment('PROVISION_API_URL', defaultValue: 'http://localhost:8080/api/provision')));
      if (res.statusCode == 200) {
        final data = jsonDecode(res.body);
        if (data['step'] != null) {
          setState(() {
            _step = data['step'];
            if (data['profile'] != null) {
              _companyNameController.text = data['profile']['name'] ?? '';
            }
          });
        }
      }
    } catch (e) {
      debugPrint('Error loading state: $e');
    } finally {
      if (mounted) setState(() => _isLoading = false);
    }
  }


  Widget _buildWelcomeStep() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Hero(tag: 'ohc_logo', child: Icon(Icons.auto_awesome, size: 64, color: Theme.of(context).colorScheme.primary)),
        const SizedBox(height: 16),
        const Text('Welcome to OHC', style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 8),
        const Text('Your AI team, ready in minutes.', style: TextStyle(fontSize: 16, fontFamily: 'Inter')),
        const SizedBox(height: 24),
      ],
    );
  }

  Widget _buildBusinessProfileStep() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Business Profile', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        TextFormField(
          controller: _companyNameController,
          validator: (val) => val!.isEmpty ? 'Required' : null,
          decoration: const InputDecoration(labelText: 'Company Name', border: OutlineInputBorder()),
        ),
        const SizedBox(height: 16),
        DropdownButtonFormField<String>(
          value: _selectedIndustry,
          decoration: const InputDecoration(labelText: 'Industry', border: OutlineInputBorder()),
          items: ['Technology', 'Finance', 'Healthcare', 'Retail', 'Other']
              .map((label) => DropdownMenuItem(value: label, child: Row(children: [const Icon(Icons.business, size: 16), const SizedBox(width: 8), Text(label)])))
              .toList(),
          onChanged: (val) => setState(() => _selectedIndustry = val!),
        ),
        const SizedBox(height: 16),
        DropdownButtonFormField<String>(
          value: _selectedSize,
          decoration: const InputDecoration(labelText: 'Company Size', border: OutlineInputBorder()),
          items: ['Small', 'Medium', 'Large', 'Enterprise']
              .map((label) => DropdownMenuItem(value: label, child: Row(children: [const Icon(Icons.business, size: 16), const SizedBox(width: 8), Text(label)])))
              .toList(),
          onChanged: (val) => setState(() => _selectedSize = val!),
        ),
        const SizedBox(height: 16),
        DropdownButtonFormField<String>(
          value: _selectedLanguage,
          decoration: const InputDecoration(labelText: 'Primary Language', border: OutlineInputBorder()),
          items: ['English', 'Spanish', 'French', 'German']
              .map((label) => DropdownMenuItem(value: label, child: Row(children: [const Icon(Icons.business, size: 16), const SizedBox(width: 8), Text(label)])))
              .toList(),
          onChanged: (val) => setState(() => _selectedLanguage = val!),
        ),
      ],
    );
  }

  Widget _buildGoalSelectionStep() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('What are your goals?', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        ..._goals.keys.map((goal) {
          return CheckboxListTile(
            secondary: const Icon(Icons.flag),
            title: Text(goal, style: const TextStyle(fontFamily: 'Inter')),
            value: _goals[goal],
            onChanged: (val) {
              if (val != null) setState(() => _goals[goal] = val);
            },
          );
        }).toList(),
      ],
    );
  }

  Widget _buildDeploymentPreferenceStep() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Deployment Preference', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        Tooltip(message: 'Recommended for most users', child: RadioListTile<String>(
          title: const Text('Cloud (managed)'),
          subtitle: const Text('Fully managed by OHC'),
          value: 'Cloud (managed)',
          groupValue: _selectedDeployment,
          onChanged: (val) => setState(() => _selectedDeployment = val!),
        )),
        Tooltip(message: 'Requires local installation', child: RadioListTile<String>(
          title: const Text('Self-hosted Desktop'),
          subtitle: const Text('Run locally on your machine'),
          value: 'Self-hosted Desktop',
          groupValue: _selectedDeployment,
          onChanged: (val) => setState(() => _selectedDeployment = val!),
        )),
        Tooltip(message: 'Limited capabilities', child: RadioListTile<String>(
          title: const Text('Mobile-only'),
          subtitle: const Text('Access your swarm on the go'),
          value: 'Mobile-only',
          groupValue: _selectedDeployment,
          onChanged: (val) => setState(() => _selectedDeployment = val!),
        )),
      ],
    );
  }

  Widget _buildAdminAccountStep() {
    return Form(
      key: _formKey,
      child: Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Administrator Account', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        TextFormField(
          controller: _adminNameController,
          validator: (val) => val!.isEmpty ? 'Required' : null,
          decoration: const InputDecoration(labelText: 'Name', border: OutlineInputBorder()),
        ),
        const SizedBox(height: 16),
        TextFormField(
          controller: _adminEmailController,
          validator: (val) => val!.isEmpty ? 'Required' : null,
          decoration: const InputDecoration(labelText: 'Email', border: OutlineInputBorder()),
        ),
        const SizedBox(height: 16),
        TextFormField(
          controller: _adminPasswordController,
          validator: (val) => val!.isEmpty || val.length < 8 ? 'Min 8 chars' : null,
          decoration: const InputDecoration(labelText: 'Password', border: OutlineInputBorder()),
          obscureText: true,
          onChanged: (val) => setState(() {}),
        ),
        const SizedBox(height: 8),
        LinearProgressIndicator(
          value: _adminPasswordController.text.length / 12,
          color: _adminPasswordController.text.length > 8 ? Colors.green : Colors.orange,
        ),
        const SizedBox(height: 8),
        const Text('Optional: Sign in with provider', style: TextStyle(fontSize: 12)),
        Row(
          children: [
            ElevatedButton.icon(onPressed: () {}, icon: const Icon(Icons.g_mobiledata), label: const Text('Google')),
            const SizedBox(width: 8),
            ElevatedButton.icon(onPressed: () {}, icon: const Icon(Icons.code), label: const Text('GitHub')),
          ],
        ),
      ],
    ),
    );
  }

  Widget _buildReviewStep() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Review & Launch', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        ClipRRect(
          borderRadius: BorderRadius.circular(16),
          child: BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            child: Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Theme.of(context).colorScheme.surface.withOpacity(0.1),
                borderRadius: BorderRadius.circular(16),
                border: Border.all(color: Theme.of(context).colorScheme.onSurface.withOpacity(0.2)),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text('Company: ${_companyNameController.text.isNotEmpty ? _companyNameController.text : "N/A"}', style: const TextStyle(fontFamily: 'Inter')),
                  Text('Industry: $_selectedIndustry', style: const TextStyle(fontFamily: 'Inter')),
                  Text('Size: $_selectedSize', style: const TextStyle(fontFamily: 'Inter')),
                  Text('Deployment: $_selectedDeployment', style: const TextStyle(fontFamily: 'Inter')),
                  const SizedBox(height: 8),
                  Text('Admin: ${_adminNameController.text}', style: const TextStyle(fontFamily: 'Inter')),
                ],
              ),
            ),
          ),
        ),
      ],
    );
  }


  Future<void> _saveStepState() async {
    try {
      await http.post(
        Uri.parse(const String.fromEnvironment('PROVISION_API_URL', defaultValue: 'http://localhost:8080/api/provision')),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({
          'step': _step,
          'profile': {
            'name': _companyNameController.text,
            'industry': _selectedIndustry,
            'size': _selectedSize,
            'language': _selectedLanguage,
          },
        }),
      );
    } catch (e) {
      debugPrint('Failed to save step state: $e');
    }
  }

  Future<void> _handleLaunch() async {
    setState(() => _isLoading = true);

    final List<String> selectedGoals = _goals.entries.where((e) => e.value).map((e) => e.key).toList();

    try {
      final res = await http.post(
        Uri.parse(const String.fromEnvironment('PROVISION_API_URL', defaultValue: 'http://localhost:8080/api/provision')),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({
          'profile': {
            'name': _companyNameController.text,
            'industry': _selectedIndustry,
            'size': _selectedSize,
            'language': _selectedLanguage,
          },
          'goals': selectedGoals,
          'deployment': _selectedDeployment,
          'admin': {
            'name': _adminNameController.text,
            'email': _adminEmailController.text,
            'password': _adminPasswordController.text,
          }
        }),
      );

      if (res.statusCode == 200) {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Swarm Launched!')));
        }
      } else {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Error: ${res.body}')));
        }
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Error: $e')));
      }
    } finally {
      if (mounted) {
        setState(() => _isLoading = false);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Stepper(
      type: StepperType.vertical,
      currentStep: _step,
      onStepContinue: () {
        if (_step < 5) {
          if (_step == 1 && _companyNameController.text.isEmpty) { ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Company name is required'))); return; }
          if (_step == 4 && !(_formKey.currentState?.validate() ?? false)) { return; }
          setState(() => _step += 1);
          _saveStepState();
        }
      },
      onStepCancel: () {
        if (_step > 0) {
          setState(() => _step -= 1);
        }
      },
      controlsBuilder: (context, details) {
        return Padding(
          padding: const EdgeInsets.only(top: 24.0),
          child: Row(
            children: [
              if (_step < 5)
                ElevatedButton(
                  onPressed: details.onStepContinue,
                  child: const Text('Next'),
                )
              else
                ElevatedButton(
                  onPressed: _isLoading ? null : _handleLaunch,
                  child: _isLoading
                    ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2))
                    : AnimatedContainer(
                      duration: const Duration(milliseconds: 500),
                      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                      decoration: BoxDecoration(
                        boxShadow: [
                          BoxShadow(
                            color: Theme.of(context).colorScheme.primary.withOpacity(_step == 5 ? 0.6 : 0.0),
                            blurRadius: 12,
                            spreadRadius: 2,
                          )
                        ]
                      ),
                      child: const Text('Launch My AI Team →'),
                    ),
                ),
              const SizedBox(width: 12),
              if (_step > 0)
                TextButton(
                  onPressed: details.onStepCancel,
                  child: const Text('Back'),
                ),
            ],
          ),
        );
      },
      steps: [
        Step(title: const Text('Welcome'), isActive: _step >= 0, content: _buildWelcomeStep()),
        Step(title: const Text('Business Profile'), isActive: _step >= 1, content: _buildBusinessProfileStep()),
        Step(title: const Text('Goals'), isActive: _step >= 2, content: _buildGoalSelectionStep()),
        Step(title: const Text('Deployment'), isActive: _step >= 3, content: _buildDeploymentPreferenceStep()),
        Step(title: const Text('Admin Account'), isActive: _step >= 4, content: _buildAdminAccountStep()),
        Step(title: const Text('Review'), isActive: _step >= 5, content: _buildReviewStep()),
      ],
    );
  }
}
