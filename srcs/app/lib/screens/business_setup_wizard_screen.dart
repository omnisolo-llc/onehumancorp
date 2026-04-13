import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../widgets/glass_card.dart';

class WizardState {
  final String companyName;
  final String industry;
  final String size;
  final List<String> goals;
  final String deployment;
  final String adminName;
  final String adminEmail;
  final String adminPassword;

  WizardState({
    this.companyName = '',
    this.industry = '',
    this.size = 'S',
    this.goals = const [],
    this.deployment = 'Cloud',
    this.adminName = '',
    this.adminEmail = '',
    this.adminPassword = '',
  });

  WizardState copyWith({
    String? companyName,
    String? industry,
    String? size,
    List<String>? goals,
    String? deployment,
    String? adminName,
    String? adminEmail,
    String? adminPassword,
  }) {
    return WizardState(
      companyName: companyName ?? this.companyName,
      industry: industry ?? this.industry,
      size: size ?? this.size,
      goals: goals ?? this.goals,
      deployment: deployment ?? this.deployment,
      adminName: adminName ?? this.adminName,
      adminEmail: adminEmail ?? this.adminEmail,
      adminPassword: adminPassword ?? this.adminPassword,
    );
  }
}

final wizardProvider = StateNotifierProvider<WizardNotifier, WizardState>((ref) => WizardNotifier());

class WizardNotifier extends StateNotifier<WizardState> {
  WizardNotifier() : super(WizardState());
  void updateCompany(String name) => state = state.copyWith(companyName: name);
  void updateIndustry(String ind) => state = state.copyWith(industry: ind);
  void updateSize(String s) => state = state.copyWith(size: s);
  void toggleGoal(String goal) {
    final goals = List<String>.from(state.goals);
    if (goals.contains(goal)) {
      goals.remove(goal);
    } else {
      goals.add(goal);
    }
    state = state.copyWith(goals: goals);
  }
  void updateDeployment(String dep) => state = state.copyWith(deployment: dep);
  void updateAdminName(String name) => state = state.copyWith(adminName: name);
  void updateAdminEmail(String email) => state = state.copyWith(adminEmail: email);
  void updateAdminPassword(String pass) => state = state.copyWith(adminPassword: pass);
}

class BusinessSetupWizardScreen extends ConsumerStatefulWidget {
  const BusinessSetupWizardScreen({super.key});
  @override
  ConsumerState<BusinessSetupWizardScreen> createState() => _BusinessSetupWizardScreenState();
}

class _BusinessSetupWizardScreenState extends ConsumerState<BusinessSetupWizardScreen> {
  int _step = 0;

  void _nextStep() {
    if (_step < 5) {
      setState(() => _step++);
    } else {
      // Backend integration placeholder
    }
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(wizardProvider);
    final notifier = ref.read(wizardProvider.notifier);

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
                  const Text('Business Setup', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold)),
                  const SizedBox(height: 16),
                  if (_step == 0) ...[
                    const Text('Welcome! Your AI team, ready in minutes.', style: TextStyle(fontFamily: 'Inter')),
                  ] else if (_step == 1) ...[
                    TextField(decoration: const InputDecoration(labelText: 'Company Name'), onChanged: notifier.updateCompany, style: const TextStyle(fontFamily: 'Inter')),
                    TextField(decoration: const InputDecoration(labelText: 'Industry'), onChanged: notifier.updateIndustry, style: const TextStyle(fontFamily: 'Inter')),
                    DropdownButton<String>(
                      value: state.size,
                      items: ['S', 'M', 'L'].map((s) => DropdownMenuItem(value: s, child: Text(s))).toList(),
                      onChanged: (v) => notifier.updateSize(v!),
                    ),
                  ] else if (_step == 2) ...[
                    const Text('Select Goals', style: TextStyle(fontFamily: 'Inter')),
                    Wrap(
                      children: ['Support', 'Build software', 'Marketing', 'Data', 'Custom'].map((g) =>
                        ChoiceChip(
                          label: Text(g),
                          selected: state.goals.contains(g),
                          onSelected: (_) => notifier.toggleGoal(g),
                        )
                      ).toList(),
                    ),
                  ] else if (_step == 3) ...[
                    const Text('Deployment Preference', style: TextStyle(fontFamily: 'Inter')),
                    DropdownButton<String>(
                      value: state.deployment,
                      items: ['Cloud', 'Desktop', 'Mobile-only'].map((s) => DropdownMenuItem(value: s, child: Text(s))).toList(),
                      onChanged: (v) => notifier.updateDeployment(v!),
                    ),
                  ] else if (_step == 4) ...[
                    TextField(decoration: const InputDecoration(labelText: 'Admin Name'), onChanged: notifier.updateAdminName, style: const TextStyle(fontFamily: 'Inter')),
                    TextField(decoration: const InputDecoration(labelText: 'Admin Email'), onChanged: notifier.updateAdminEmail, style: const TextStyle(fontFamily: 'Inter')),
                    TextField(decoration: const InputDecoration(labelText: 'Admin Password'), obscureText: true, onChanged: notifier.updateAdminPassword, style: const TextStyle(fontFamily: 'Inter')),
                  ] else if (_step == 5) ...[
                    const Text('Review & Launch', style: TextStyle(fontFamily: 'Inter')),
                    Text('Company: ${state.companyName}', style: const TextStyle(fontFamily: 'Inter')),
                  ],
                  const SizedBox(height: 16),
                  ElevatedButton(
                    onPressed: _nextStep,
                    child: Text(_step == 5 ? 'Launch My AI Team' : 'Next', style: const TextStyle(fontFamily: 'Inter')),
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
