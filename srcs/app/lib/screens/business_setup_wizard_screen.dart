import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
import '../services/auth_service.dart';
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
  final bool isLoading;
  final String? errorMessage;

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
    this.isLoading = false,
    this.errorMessage,
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
    bool? isLoading,
    String? errorMessage,
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
      isLoading: isLoading ?? this.isLoading,
      errorMessage: errorMessage ?? this.errorMessage,
    );
  }
}

class BusinessSetupNotifier extends Notifier<BusinessSetupState> {
  @override
  BusinessSetupState build() => const BusinessSetupState();

  void nextStep() {
    if (state.step < 4) {
      state = state.copyWith(step: state.step + 1);
    }
  }

  void prevStep() {
    if (state.step > 0) {
      state = state.copyWith(step: state.step - 1);
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

  Future<void> launch(BuildContext context, WidgetRef ref) async {
    final user = ref.read(authStateProvider).valueOrNull;
    final baseUrl = ref.read(backendUrlProvider);

    state = state.copyWith(isLoading: true, errorMessage: null);

    if (user != null && baseUrl.isNotEmpty) {
      final body = {
        'extras': {
          'company_name': state.companyName,
          'industry': state.industry,
          'company_size': state.size,
          'goals': state.goals.join(','),
          'deployment_preference': state.deployment,
          'admin_name': state.adminName,
          'admin_email': state.adminEmail,
        }
      };

      try {
        final res = await http.post(
          Uri.parse('$baseUrl/api/wizard/configure'),
          headers: {
            'Authorization': 'Bearer ${user.token}',
            'Content-Type': 'application/json',
          },
          body: jsonEncode(body),
        );

        if (res.statusCode != 200) {
          state = state.copyWith(isLoading: false, errorMessage: 'Configuration failed: ${res.statusCode}');
          return;
        }
      } catch (e) {
        state = state.copyWith(isLoading: false, errorMessage: 'Network error: $e');
        return;
      }
    }

    state = state.copyWith(isLoading: false);

    if (context.mounted) {
      GoRouter.of(context).go('/dashboard');
    }
  }
}

final businessSetupProvider = NotifierProvider<BusinessSetupNotifier, BusinessSetupState>(() {
  return BusinessSetupNotifier();
});

class BusinessSetupWizardScreen extends ConsumerStatefulWidget {
  const BusinessSetupWizardScreen({super.key});

  @override
  ConsumerState<BusinessSetupWizardScreen> createState() => _BusinessSetupWizardScreenState();
}

class _BusinessSetupWizardScreenState extends ConsumerState<BusinessSetupWizardScreen> {
  bool _obscurePassword = true;

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(businessSetupProvider);
    final notifier = ref.read(businessSetupProvider.notifier);

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
                  if (state.errorMessage != null) ...[
                    Text(state.errorMessage!, style: const TextStyle(color: Colors.red)),
                    const SizedBox(height: 16),
                  ],
                  if (state.step == 0) ...[
                    const Text('Welcome! Your AI team, ready in minutes.', style: TextStyle(fontFamily: 'Inter')),
                  ] else if (state.step == 1) ...[
                    TextField(
                      decoration: const InputDecoration(labelText: 'Company Name'),
                      onChanged: notifier.updateCompany,
                      style: const TextStyle(fontFamily: 'Inter'),
                    ),
                    const SizedBox(height: 16),
                    TextField(
                      decoration: const InputDecoration(labelText: 'Industry'),
                      onChanged: notifier.updateIndustry,
                      style: const TextStyle(fontFamily: 'Inter'),
                    ),
                    const SizedBox(height: 16),
                    DropdownButtonFormField<String>(
                      value: state.size,
                      decoration: const InputDecoration(labelText: 'Size'),
                      items: const [
                        DropdownMenuItem(value: 'S', child: Text('Small')),
                        DropdownMenuItem(value: 'M', child: Text('Medium')),
                        DropdownMenuItem(value: 'L', child: Text('Large')),
                      ],
                      onChanged: (val) {
                        if (val != null) notifier.updateSize(val);
                      },
                    ),
                  ] else if (state.step == 2) ...[
                     const Text('Select Goals', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
                     ...['Support', 'Build software', 'Marketing', 'Data', 'Custom'].map((goal) => CheckboxListTile(
                      title: Text(goal, style: const TextStyle(fontFamily: 'Inter')),
                      value: state.goals.contains(goal),
                      onChanged: (bool? value) {
                        notifier.toggleGoal(goal);
                      },
                    )),
                  ] else if (state.step == 3) ...[
                     const Text('Deployment Preference', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
                     ...['Cloud', 'Desktop', 'Mobile-only'].map((dep) => RadioListTile<String>(
                      title: Text(dep, style: const TextStyle(fontFamily: 'Inter')),
                      value: dep,
                      groupValue: state.deployment,
                      onChanged: (String? value) {
                        if (value != null) notifier.updateDeployment(value);
                      },
                    )),
                  ] else if (state.step == 4) ...[
                    TextField(
                      decoration: const InputDecoration(labelText: 'Admin Name'),
                      onChanged: notifier.updateAdminName,
                      style: const TextStyle(fontFamily: 'Inter'),
                    ),
                    const SizedBox(height: 16),
                    TextField(
                      decoration: const InputDecoration(labelText: 'Admin Email'),
                      onChanged: notifier.updateAdminEmail,
                      style: const TextStyle(fontFamily: 'Inter'),
                    ),
                    const SizedBox(height: 16),
                    TextField(
                      obscureText: _obscurePassword,
                      onChanged: notifier.updateAdminPassword,
                      style: const TextStyle(fontFamily: 'Inter'),
                      decoration: InputDecoration(
                        labelText: 'Admin Password',
                        suffixIcon: IconButton(
                          icon: Icon(_obscurePassword ? Icons.visibility : Icons.visibility_off),
                          tooltip: _obscurePassword ? 'Show Password' : 'Hide Password',
                          onPressed: () {
                            setState(() {
                              _obscurePassword = !_obscurePassword;
                            });
                          },
                        ),
                      ),
                    ),
                  ],
                  const SizedBox(height: 24),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      if (state.step > 0)
                        Semantics(
                          label: 'Go back to previous step',
                          child: Tooltip(
                            message: 'Previous step',
                            child: TextButton(
                              onPressed: state.isLoading ? null : notifier.prevStep,
                              child: const Text('Back', style: TextStyle(fontFamily: 'Inter')),
                            ),
                          ),
                        )
                      else
                        const SizedBox(),
                      Semantics(
                        label: state.step == 4 ? 'Launch the business setup' : 'Proceed to next step',
                        child: Tooltip(
                          message: state.step == 4 ? 'Complete setup and launch dashboard' : 'Next step',
                          child: ElevatedButton(
                            onPressed: state.isLoading ? null : () {
                              if (state.step < 4) {
                                notifier.nextStep();
                              } else {
                                notifier.launch(context, ref);
                              }
                            },
                            child: state.isLoading
                                ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2))
                                : Text(state.step == 4 ? 'Launch My AI Team →' : 'Next', style: const TextStyle(fontFamily: 'Inter')),
                          ),
                        ),
                      ),
                    ],
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
