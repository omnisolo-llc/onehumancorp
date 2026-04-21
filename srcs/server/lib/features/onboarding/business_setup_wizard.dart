import 'dart:ui';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:http/http.dart' as http;

class BusinessSetupState {
  final int step;
  final String companyName;
  final String industry;
  final String size;
  final String language;
  final List<String> goals;
  final String deploymentMode;
  final String adminName;
  final String adminEmail;
  final String adminPassword;
  final bool isLoading;

  const BusinessSetupState({
    this.step = 0,
    this.companyName = '',
    this.industry = 'Tech',
    this.size = 'M',
    this.language = 'English',
    this.goals = const [],
    this.deploymentMode = 'Cloud',
    this.adminName = '',
    this.adminEmail = '',
    this.adminPassword = '',
    this.isLoading = false,
  });

  BusinessSetupState copyWith({
    int? step,
    String? companyName,
    String? industry,
    String? size,
    String? language,
    List<String>? goals,
    String? deploymentMode,
    String? adminName,
    String? adminEmail,
    String? adminPassword,
    bool? isLoading,
  }) {
    return BusinessSetupState(
      step: step ?? this.step,
      companyName: companyName ?? this.companyName,
      industry: industry ?? this.industry,
      size: size ?? this.size,
      language: language ?? this.language,
      goals: goals ?? this.goals,
      deploymentMode: deploymentMode ?? this.deploymentMode,
      adminName: adminName ?? this.adminName,
      adminEmail: adminEmail ?? this.adminEmail,
      adminPassword: adminPassword ?? this.adminPassword,
      isLoading: isLoading ?? this.isLoading,
    );
  }
}

class BusinessSetupNotifier extends Notifier<BusinessSetupState> {
  @override
  BusinessSetupState build() => const BusinessSetupState();

  void nextStep() {
    if (state.step < 5) {
      state = state.copyWith(step: state.step + 1);
      _saveState();
    }
  }

  void prevStep() {
    if (state.step > 0) {
      state = state.copyWith(step: state.step - 1);
    }
  }

  void updateCompanyName(String val) => state = state.copyWith(companyName: val);
  void updateIndustry(String val) => state = state.copyWith(industry: val);
  void updateSize(String val) => state = state.copyWith(size: val);
  void updateLanguage(String val) => state = state.copyWith(language: val);

  void toggleGoal(String goal) {
    final goals = List<String>.from(state.goals);
    if (goals.contains(goal)) {
      goals.remove(goal);
    } else {
      goals.add(goal);
    }
    state = state.copyWith(goals: goals);
  }

  void updateDeploymentMode(String val) => state = state.copyWith(deploymentMode: val);
  void updateAdminName(String val) => state = state.copyWith(adminName: val);
  void updateAdminEmail(String val) => state = state.copyWith(adminEmail: val);
  void updateAdminPassword(String val) => state = state.copyWith(adminPassword: val);

  Future<void> _saveState() async {
    final stateData = {
      'step': state.step,
      'companyName': state.companyName,
      'industry': state.industry,
      'size': state.size,
      'language': state.language,
      'goals': state.goals,
      'deploymentMode': state.deploymentMode,
      'adminName': state.adminName,
      'adminEmail': state.adminEmail,
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
}

final businessSetupProvider = NotifierProvider<BusinessSetupNotifier, BusinessSetupState>(() {
  return BusinessSetupNotifier();
});

class BusinessSetupWizard extends ConsumerWidget {
  const BusinessSetupWizard({super.key});

  Widget _buildGlassmorphism({required Widget child}) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: BackdropFilter(
        filter: ImageFilter.compose(
                outer: const ColorFilter.matrix(<double>[
                  1.168, -0.153, -0.015, 0, 0,
                  -0.046, 1.061, -0.015, 0, 0,
                  -0.046, -0.152, 1.198, 0, 0,
                  0, 0, 0, 1, 0,
                ]),
                inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
              ),
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

  Widget _buildProfile(BusinessSetupState state, BusinessSetupNotifier notifier) {
    return _buildGlassmorphism(
      child: Column(
        children: [
          TextField(
            decoration: const InputDecoration(labelText: 'Company name'),
            onChanged: notifier.updateCompanyName,
            controller: TextEditingController(text: state.companyName)..selection = TextSelection.collapsed(offset: state.companyName.length),
          ),
          DropdownButtonFormField<String>(
            value: state.industry,
            items: ['Tech', 'Healthcare', 'Finance', 'Retail', 'Other'].map((i) => DropdownMenuItem(value: i, child: Text(i))).toList(),
            onChanged: (v) => notifier.updateIndustry(v!),
            decoration: const InputDecoration(labelText: 'Industry'),
          ),
          DropdownButtonFormField<String>(
            value: state.size,
            items: ['S', 'M', 'L', 'Enterprise'].map((s) => DropdownMenuItem(value: s, child: Text(s))).toList(),
            onChanged: (v) => notifier.updateSize(v!),
            decoration: const InputDecoration(labelText: 'Company Size'),
          ),
          DropdownButtonFormField<String>(
            value: state.language,
            items: ['English', 'Spanish', 'French', 'German'].map((l) => DropdownMenuItem(value: l, child: Text(l))).toList(),
            onChanged: (v) => notifier.updateLanguage(v!),
            decoration: const InputDecoration(labelText: 'Primary Language'),
          ),
        ],
      ),
    );
  }

  Widget _buildGoals(BusinessSetupState state, BusinessSetupNotifier notifier) {
    final allGoals = [
      'Automate customer support',
      'Build software faster',
      'Generate marketing content',
      'Analyze data',
      'Custom',
    ];

    return _buildGlassmorphism(
      child: Column(
        children: allGoals.map((k) => CheckboxListTile(
          title: Text(k, style: const TextStyle(fontFamily: 'Inter')),
          value: state.goals.contains(k),
          onChanged: (v) => notifier.toggleGoal(k),
        )).toList(),
      ),
    );
  }

  Widget _buildDeployment(BusinessSetupState state, BusinessSetupNotifier notifier) {
    return _buildGlassmorphism(
      child: Column(
        children: ['Cloud', 'Self-hosted Desktop', 'Mobile-only'].map((m) => RadioListTile<String>(
          title: Text(m, style: const TextStyle(fontFamily: 'Inter')),
          value: m,
          groupValue: state.deploymentMode,
          onChanged: (v) => notifier.updateDeploymentMode(v!),
        )).toList(),
      ),
    );
  }

  Widget _buildAdmin(BusinessSetupState state, BusinessSetupNotifier notifier) {
    return _buildGlassmorphism(
      child: Column(
        children: [
          TextField(
            decoration: const InputDecoration(labelText: 'Name'),
            onChanged: notifier.updateAdminName,
            controller: TextEditingController(text: state.adminName)..selection = TextSelection.collapsed(offset: state.adminName.length),
          ),
          TextField(
            decoration: const InputDecoration(labelText: 'Email'),
            onChanged: notifier.updateAdminEmail,
            controller: TextEditingController(text: state.adminEmail)..selection = TextSelection.collapsed(offset: state.adminEmail.length),
          ),
          TextField(
            obscureText: true,
            decoration: const InputDecoration(labelText: 'Password'),
            onChanged: notifier.updateAdminPassword,
            controller: TextEditingController(text: state.adminPassword)..selection = TextSelection.collapsed(offset: state.adminPassword.length),
          ),
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

  Widget _buildReview(BusinessSetupState state) {
    return _buildGlassmorphism(
      child: Column(
        children: [
          const Text('Review & Launch', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold)),
          const SizedBox(height: 16),
          ListTile(title: const Text('Company'), subtitle: Text(state.companyName)),
          ListTile(title: const Text('Deployment'), subtitle: Text(state.deploymentMode)),
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
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(businessSetupProvider);
    final notifier = ref.read(businessSetupProvider.notifier);

    return Scaffold(
      body: Stepper(
        currentStep: state.step,
        onStepContinue: notifier.nextStep,
        onStepCancel: notifier.prevStep,
        steps: [
          Step(title: const Text('Welcome', style: TextStyle(fontFamily: 'Outfit')), content: _buildWelcome()),
          Step(title: const Text('Business Profile', style: TextStyle(fontFamily: 'Outfit')), content: _buildProfile(state, notifier)),
          Step(title: const Text('Goal Selection', style: TextStyle(fontFamily: 'Outfit')), content: _buildGoals(state, notifier)),
          Step(title: const Text('Deployment', style: TextStyle(fontFamily: 'Outfit')), content: _buildDeployment(state, notifier)),
          Step(title: const Text('Admin Account', style: TextStyle(fontFamily: 'Outfit')), content: _buildAdmin(state, notifier)),
          Step(title: const Text('Review & Launch', style: TextStyle(fontFamily: 'Outfit')), content: _buildReview(state)),
        ],
      ),
    );
  }
}
