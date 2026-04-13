import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../widgets/glass_card.dart';

class BusinessSetupState {
  final int step;
  final String companyName;
  final String industry;
  final String size;
  final List<String> goals;
  final String deployment;
  final String adminName;
  final String adminEmail;
  final String adminPassword;

  const BusinessSetupState({
    this.step = 0,
    this.companyName = '',
    this.industry = '',
    this.size = 'S',
    this.goals = const [],
    this.deployment = 'Cloud',
    this.adminName = '',
    this.adminEmail = '',
    this.adminPassword = '',
  });

  BusinessSetupState copyWith({
    int? step,
    String? companyName,
    String? industry,
    String? size,
    List<String>? goals,
    String? deployment,
    String? adminName,
    String? adminEmail,
    String? adminPassword,
  }) {
    return BusinessSetupState(
      step: step ?? this.step,
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

class BusinessSetupNotifier extends Notifier<BusinessSetupState> {
  @override
  BusinessSetupState build() => const BusinessSetupState();

  void nextStep() {
    if (state.step < 5) {
      state = state.copyWith(step: state.step + 1);
    } else {
      _launch();
    }
  }

  void updateCompany(String name) => state = state.copyWith(companyName: name);
  void updateIndustry(String val) => state = state.copyWith(industry: val);
  void updateSize(String val) => state = state.copyWith(size: val);
  void toggleGoal(String goal) {
    final goals = List<String>.from(state.goals);
    if (goals.contains(goal)) {
      goals.remove(goal);
    } else {
      goals.add(goal);
    }
    state = state.copyWith(goals: goals);
  }
  void updateDeployment(String val) => state = state.copyWith(deployment: val);
  void updateAdminName(String name) => state = state.copyWith(adminName: name);
  void updateAdminEmail(String val) => state = state.copyWith(adminEmail: val);
  void updateAdminPassword(String val) => state = state.copyWith(adminPassword: val);

  void _launch() {
    // Perform launch logic
  }
}

final businessSetupProvider = NotifierProvider<BusinessSetupNotifier, BusinessSetupState>(() {
  return BusinessSetupNotifier();
});

class BusinessSetupWizardScreen extends ConsumerWidget {
  const BusinessSetupWizardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(businessSetupProvider);
    final notifier = ref.read(businessSetupProvider.notifier);

    return Scaffold(
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Color(0xFF0F172A), Color(0xFF1E293B)],
          ),
        ),
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 600),
            child: GlassCard(
              child: Padding(
                padding: const EdgeInsets.all(32.0),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    const Text(
                      'Business Setup',
                      style: TextStyle(fontFamily: 'Outfit', fontSize: 28, fontWeight: FontWeight.bold, color: Colors.white),
                      textAlign: TextAlign.center,
                    ),
                    const SizedBox(height: 24),
                    _buildStepContent(state, notifier),
                    const SizedBox(height: 32),
                    ElevatedButton(
                      style: ElevatedButton.styleFrom(
                        backgroundColor: const Color(0xFF3B82F6),
                        padding: const EdgeInsets.symmetric(vertical: 16),
                        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                      ),
                      onPressed: notifier.nextStep,
                      child: Text(
                        state.step == 5 ? 'Launch My AI Team →' : 'Next Step',
                        style: const TextStyle(fontFamily: 'Inter', fontSize: 16, fontWeight: FontWeight.w600, color: Colors.white),
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildStepContent(BusinessSetupState state, BusinessSetupNotifier notifier) {
    switch (state.step) {
      case 0:
        return const Column(
          children: [
            Icon(Icons.rocket_launch, size: 64, color: Colors.blueAccent),
            SizedBox(height: 16),
            Text(
              'Welcome! Your AI team is ready to be configured in minutes.',
              style: TextStyle(fontFamily: 'Inter', fontSize: 16, color: Colors.white70),
              textAlign: TextAlign.center,
            ),
          ],
        );
      case 1:
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text('Business Profile', style: TextStyle(color: Colors.white, fontSize: 18, fontFamily: 'Outfit')),
            const SizedBox(height: 16),
            _buildTextField('Company Name', notifier.updateCompany),
            const SizedBox(height: 12),
            _buildTextField('Industry', notifier.updateIndustry),
            const SizedBox(height: 12),
            _buildDropdown('Size', ['S', 'M', 'L', 'Enterprise'], state.size, notifier.updateSize),
          ],
        );
      case 2:
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text('Select Goals', style: TextStyle(color: Colors.white, fontSize: 18, fontFamily: 'Outfit')),
            const SizedBox(height: 16),
            Wrap(
              spacing: 8,
              runSpacing: 8,
              children: ['Support', 'Build software', 'Marketing', 'Data', 'Custom']
                  .map((goal) => ChoiceChip(
                        label: Text(goal),
                        selected: state.goals.contains(goal),
                        onSelected: (_) => notifier.toggleGoal(goal),
                        selectedColor: Colors.blueAccent.withOpacity(0.3),
                        backgroundColor: Colors.white10,
                        labelStyle: const TextStyle(color: Colors.white),
                      ))
                  .toList(),
            ),
          ],
        );
      case 3:
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text('Deployment Preference', style: TextStyle(color: Colors.white, fontSize: 18, fontFamily: 'Outfit')),
            const SizedBox(height: 16),
            ...['Cloud', 'Desktop', 'Mobile-only'].map((val) => RadioListTile<String>(
                  title: Text(val, style: const TextStyle(color: Colors.white)),
                  value: val,
                  groupValue: state.deployment,
                  onChanged: (v) => notifier.updateDeployment(v!),
                  activeColor: Colors.blueAccent,
                )),
          ],
        );
      case 4:
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text('Administrator Account', style: TextStyle(color: Colors.white, fontSize: 18, fontFamily: 'Outfit')),
            const SizedBox(height: 16),
            _buildTextField('Admin Name', notifier.updateAdminName),
            const SizedBox(height: 12),
            _buildTextField('Admin Email', notifier.updateAdminEmail),
            const SizedBox(height: 12),
            _buildTextField('Admin Password', notifier.updateAdminPassword, obscureText: true),
          ],
        );
      case 5:
        return Container(
          padding: const EdgeInsets.all(16),
          decoration: BoxDecoration(
            color: Colors.white10,
            borderRadius: BorderRadius.circular(8),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text('Review & Launch', style: TextStyle(color: Colors.white, fontSize: 18, fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
              const SizedBox(height: 12),
              _summaryRow('Company', state.companyName),
              _summaryRow('Deployment', state.deployment),
              _summaryRow('Admin Email', state.adminEmail),
            ],
          ),
        );
      default:
        return const SizedBox.shrink();
    }
  }

  Widget _buildTextField(String label, Function(String) onChanged, {bool obscureText = false}) {
    return TextField(
      decoration: InputDecoration(
        labelText: label,
        labelStyle: const TextStyle(color: Colors.white54),
        enabledBorder: const OutlineInputBorder(borderSide: BorderSide(color: Colors.white24)),
        focusedBorder: const OutlineInputBorder(borderSide: BorderSide(color: Colors.blueAccent)),
      ),
      style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
      onChanged: onChanged,
      obscureText: obscureText,
    );
  }

  Widget _buildDropdown(String label, List<String> items, String value, Function(String) onChanged) {
    return DropdownButtonFormField<String>(
      value: value,
      decoration: InputDecoration(
        labelText: label,
        labelStyle: const TextStyle(color: Colors.white54),
        enabledBorder: const OutlineInputBorder(borderSide: BorderSide(color: Colors.white24)),
        focusedBorder: const OutlineInputBorder(borderSide: BorderSide(color: Colors.blueAccent)),
      ),
      dropdownColor: const Color(0xFF1E293B),
      style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
      items: items.map((item) => DropdownMenuItem(value: item, child: Text(item))).toList(),
      onChanged: (v) => onChanged(v!),
    );
  }

  Widget _summaryRow(String label, String value) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 8.0),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(label, style: const TextStyle(color: Colors.white54)),
          Text(value.isNotEmpty ? value : '-', style: const TextStyle(color: Colors.white, fontWeight: FontWeight.bold)),
        ],
      ),
    );
  }
}
