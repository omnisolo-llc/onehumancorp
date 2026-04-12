import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../widgets/glass_card.dart';
import '../providers/business_setup_wizard_provider.dart';
import '../services/api_service.dart';

class BusinessSetupWizardScreen extends ConsumerStatefulWidget {
  const BusinessSetupWizardScreen({super.key});

  @override
  ConsumerState<BusinessSetupWizardScreen> createState() => _BusinessSetupWizardScreenState();
}

class _BusinessSetupWizardScreenState extends ConsumerState<BusinessSetupWizardScreen> {
  final List<String> _availableGoals = ['Support', 'Build software', 'Marketing', 'Data', 'Custom'];
  final List<String> _deploymentOptions = ['Cloud', 'Desktop', 'Mobile-only'];
  final List<String> _sizes = ['1-10', '11-50', '51-200', '201-500', '500+'];

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(businessSetupWizardProvider);
    final notifier = ref.read(businessSetupWizardProvider.notifier);

    return Scaffold(
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 600),
          child: GlassCard(
            child: Padding(
              padding: const EdgeInsets.all(24.0),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  const Text(
                    'Business Setup',
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 24,
                      fontWeight: FontWeight.bold,
                    ),
                    textAlign: TextAlign.center,
                  ),
                  const SizedBox(height: 24),
                  if (state.step == 0) ...[
                    const Icon(Icons.auto_awesome, size: 64, color: Colors.blue),
                    const SizedBox(height: 16),
                    const Text(
                      'Welcome! Your AI team, ready in minutes.',
                      style: TextStyle(fontFamily: 'Inter', fontSize: 18),
                      textAlign: TextAlign.center,
                    ),
                    const SizedBox(height: 8),
                    const Text(
                      'Let\'s configure OHC to perfectly match your business needs.',
                      style: TextStyle(fontFamily: 'Inter', color: Colors.grey),
                      textAlign: TextAlign.center,
                    ),
                    const SizedBox(height: 24),
                  ] else if (state.step == 1) ...[
                    TextField(
                      decoration: const InputDecoration(labelText: 'Company Name'),
                      onChanged: notifier.setCompanyName,
                      style: const TextStyle(fontFamily: 'Inter'),
                    ),
                    const SizedBox(height: 16),
                    TextField(
                      decoration: const InputDecoration(labelText: 'Industry'),
                      onChanged: notifier.setIndustry,
                      style: const TextStyle(fontFamily: 'Inter'),
                    ),
                    const SizedBox(height: 16),
                    DropdownButtonFormField<String>(
                      decoration: const InputDecoration(labelText: 'Size'),
                      value: state.size,
                      items: _sizes.map((s) => DropdownMenuItem(value: s, child: Text(s))).toList(),
                      onChanged: (v) => notifier.setSize(v ?? '1-10'),
                    ),
                  ] else if (state.step == 2) ...[
                     const Text('Select Goals', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
                     const SizedBox(height: 16),
                     Wrap(
                       spacing: 8.0,
                       children: _availableGoals.map((goal) {
                         return ChoiceChip(
                           label: Text(goal),
                           selected: state.goals.contains(goal),
                           onSelected: (selected) {
                             if (selected) {
                               notifier.addGoal(goal);
                             } else {
                               notifier.removeGoal(goal);
                             }
                           },
                         );
                       }).toList(),
                     )
                  ] else if (state.step == 3) ...[
                     const Text('Deployment Preference', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
                     const SizedBox(height: 16),
                     ..._deploymentOptions.map((opt) => RadioListTile<String>(
                       title: Text(opt),
                       value: opt,
                       groupValue: state.deployment,
                       onChanged: (v) => notifier.setDeployment(v ?? 'Cloud'),
                     )),
                  ] else if (state.step == 4) ...[
                    TextField(
                      decoration: const InputDecoration(labelText: 'Admin Name'),
                      onChanged: notifier.setAdminName,
                      style: const TextStyle(fontFamily: 'Inter'),
                    ),
                    const SizedBox(height: 16),
                    TextField(
                      decoration: const InputDecoration(labelText: 'Admin Email'),
                      onChanged: notifier.setAdminEmail,
                      style: const TextStyle(fontFamily: 'Inter'),
                    ),
                    const SizedBox(height: 16),
                    TextField(
                      decoration: const InputDecoration(labelText: 'Admin Password'),
                      obscureText: true,
                      onChanged: notifier.setAdminPassword,
                      style: const TextStyle(fontFamily: 'Inter'),
                    ),
                  ] else if (state.step == 5) ...[
                    const Text('Review & Launch', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, fontSize: 18)),
                    const SizedBox(height: 16),
                    Text('Company: ${state.companyName}', style: const TextStyle(fontFamily: 'Inter')),
                    Text('Industry: ${state.industry}', style: const TextStyle(fontFamily: 'Inter')),
                    Text('Size: ${state.size}', style: const TextStyle(fontFamily: 'Inter')),
                    Text('Goals: ${state.goals.join(', ')}', style: const TextStyle(fontFamily: 'Inter')),
                    Text('Deployment: ${state.deployment}', style: const TextStyle(fontFamily: 'Inter')),
                    Text('Admin: ${state.adminName} (${state.adminEmail})', style: const TextStyle(fontFamily: 'Inter')),
                    const SizedBox(height: 24),
                  ],
                  ElevatedButton(
                    onPressed: () {
                      if (state.step < 5) {
                        notifier.nextStep();
                      } else {
                        _launch(state);
                      }
                    },
                    style: ElevatedButton.styleFrom(
                      padding: const EdgeInsets.symmetric(vertical: 16),
                    ),
                    child: Text(state.step == 5 ? 'Launch My AI Team →' : 'Next', style: const TextStyle(fontFamily: 'Inter', fontSize: 16)),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  void _launch(BusinessSetupState state) async {
    final apiService = ref.read(apiServiceProvider);
    try {
      await apiService!.submitBusinessSetup({
        'companyName': state.companyName,
        'industry': state.industry,
        'size': state.size,
        'goals': state.goals,
        'deployment': state.deployment,
        'adminName': state.adminName,
        'adminEmail': state.adminEmail,
        'adminPassword': state.adminPassword,
      });
      // Handle success, maybe navigate
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Setup complete!')));
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Error: $e')));
    }
  }
}
