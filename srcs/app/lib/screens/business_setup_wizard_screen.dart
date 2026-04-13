import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import '../widgets/glass_card.dart';
import '../services/api_service.dart';

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
  final List<String> _goals = [];
  String _deployment = 'Cloud';
  String _adminName = '';
  String _adminEmail = '';
  String _adminPassword = '';
  bool _isLoading = false;

  void _nextStep() {
    if (_step < 4) {
      setState(() => _step++);
    } else {
      _launch();
    }
  }

  Future<void> _launch() async {
    setState(() => _isLoading = true);
    try {
      await ref.read(apiServiceProvider)?.submitBusinessSetup({
        'companyName': _companyName,
        'industry': _industry,
        'size': _size,
        'goals': _goals,
        'deployment': _deployment,
        'adminName': _adminName,
        'adminEmail': _adminEmail,
        'adminPassword': _adminPassword,
      });
      if (mounted) {
        context.go('/dashboard');
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Error: \$e')));
      }
    } finally {
      if (mounted) {
        setState(() => _isLoading = false);
      }
    }
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
                  const Text(
                    'Business Setup',
                    style: TextStyle(
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
                    const SizedBox(height: 8),
                    TextField(
                      decoration: const InputDecoration(labelText: 'Industry'),
                      onChanged: (v) => _industry = v,
                      style: const TextStyle(fontFamily: 'Inter'),
                    ),
                    const SizedBox(height: 8),
                    DropdownButtonFormField<String>(
                      value: _size,
                      items: ['S', 'M', 'L', 'Enterprise'].map((s) => DropdownMenuItem(value: s, child: Text(s))).toList(),
                      onChanged: (v) => setState(() => _size = v!),
                      decoration: const InputDecoration(labelText: 'Size'),
                    ),
                  ] else if (_step == 2) ...[
                     const Text('Select Goals', style: TextStyle(fontFamily: 'Inter')),
                     Wrap(
                       spacing: 8.0,
                       children: ['Support', 'Build software', 'Marketing', 'Data', 'Custom'].map((goal) {
                         return FilterChip(
                           label: Text(goal),
                           selected: _goals.contains(goal),
                           onSelected: (selected) {
                             setState(() {
                               if (selected) {
                                 _goals.add(goal);
                               } else {
                                 _goals.remove(goal);
                               }
                             });
                           },
                         );
                       }).toList(),
                     ),
                  ] else if (_step == 3) ...[
                     const Text('Deployment Preference', style: TextStyle(fontFamily: 'Inter')),
                     DropdownButtonFormField<String>(
                       value: _deployment,
                       items: ['Cloud', 'Desktop', 'Mobile-only'].map((s) => DropdownMenuItem(value: s, child: Text(s))).toList(),
                       onChanged: (v) => setState(() => _deployment = v!),
                     ),
                  ] else if (_step == 4) ...[
                    TextField(
                      decoration: const InputDecoration(labelText: 'Admin Name'),
                      onChanged: (v) => _adminName = v,
                      style: const TextStyle(fontFamily: 'Inter'),
                    ),
                    const SizedBox(height: 8),
                    TextField(
                      decoration: const InputDecoration(labelText: 'Admin Email'),
                      onChanged: (v) => _adminEmail = v,
                      style: const TextStyle(fontFamily: 'Inter'),
                    ),
                    const SizedBox(height: 8),
                    TextField(
                      obscureText: true,
                      decoration: const InputDecoration(labelText: 'Admin Password'),
                      onChanged: (v) => _adminPassword = v,
                      style: const TextStyle(fontFamily: 'Inter'),
                    ),
                  ],
                  const SizedBox(height: 16),
                  if (_isLoading)
                    const CircularProgressIndicator()
                  else
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
