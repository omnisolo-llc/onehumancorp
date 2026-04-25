import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
import 'dart:ui';
import '../services/auth_service.dart';
import '../services/settings_service.dart';
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
  final String whatTheySell;
  final String paymentPreferences;
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
    this.whatTheySell = '',
    this.paymentPreferences = '',
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
    String? whatTheySell,
    String? paymentPreferences,
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
      whatTheySell: whatTheySell ?? this.whatTheySell,
      paymentPreferences: paymentPreferences ?? this.paymentPreferences,
      isLoading: isLoading ?? this.isLoading,
      errorMessage: errorMessage ?? this.errorMessage,
    );
  }
}

class BusinessSetupNotifier extends Notifier<BusinessSetupState> {
  @override
  BusinessSetupState build() {
    Future.microtask(() => loadState());
    return const BusinessSetupState();
  }

  Future<void> loadState() async {
    final url = ref.read(backendUrlProvider);
    try {
      final response = await http.get(Uri.parse('$url/api/wizard/status'));
      if (response.statusCode == 200) {
        final data = jsonDecode(response.body);
        if (data['extras'] != null) {
          final extras = data['extras'] as Map<String, dynamic>;
          state = state.copyWith(
            step: int.tryParse(extras['step']?.toString() ?? '0') ?? 0,
            companyName: extras['companyName']?.toString() ?? '',
            industry: extras['industry']?.toString() ?? '',
            whatTheySell: extras['whatTheySell']?.toString() ?? '',
            paymentPreferences: extras['paymentPreferences']?.toString() ?? '',
          );
        }
      }
    } catch (_) {}
  }

  Future<void> saveAndNextStep() async {
    if (state.step < 4) {
      final nextStep = state.step + 1;
      state = state.copyWith(step: nextStep);

      final url = ref.read(backendUrlProvider);
      try {
        await http.post(
          Uri.parse('$url/api/wizard/configure'),
          headers: {'Content-Type': 'application/json'},
          body: jsonEncode({
            'extras': {
              'step': nextStep.toString(),
              'companyName': state.companyName,
              'industry': state.industry,
              'whatTheySell': state.whatTheySell,
              'paymentPreferences': state.paymentPreferences,
            },
          }),
        );
      } catch (_) {}
    }
  }

  void updateWhatTheySell(String v) => state = state.copyWith(whatTheySell: v);
  void updatePaymentPreferences(String v) =>
      state = state.copyWith(paymentPreferences: v);

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
  void updateAdminPassword(String val) =>
      state = state.copyWith(adminPassword: val);

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
        },
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
          state = state.copyWith(
            isLoading: false,
            errorMessage: 'Configuration failed: ${res.statusCode}',
          );
          return;
        }
      } catch (e) {
        state = state.copyWith(
          isLoading: false,
          errorMessage: 'Network error: $e',
        );
        return;
      }
    }

    state = state.copyWith(isLoading: false);

    if (context.mounted) {
      GoRouter.of(context).go('/dashboard');
    }
  }
}

final businessSetupProvider =
    NotifierProvider<BusinessSetupNotifier, BusinessSetupState>(() {
      return BusinessSetupNotifier();
    });

class BusinessSetupWizardScreen extends ConsumerStatefulWidget {
  const BusinessSetupWizardScreen({super.key});

  @override
  ConsumerState<BusinessSetupWizardScreen> createState() =>
      _BusinessSetupWizardScreenState();
}

class _BusinessSetupWizardScreenState
    extends ConsumerState<BusinessSetupWizardScreen> {
  bool _obscurePassword = true;

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(businessSetupProvider);
    final notifier = ref.read(businessSetupProvider.notifier);
    final clientSettings = ref.watch(clientSettingsProvider).valueOrNull;
    final isStandalone = clientSettings?.standaloneMode ?? false;

    return Scaffold(
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Color(0xFF0D0D1A), Color(0xFF1A1A33)],
          ),
        ),
        child: Center(
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
                        color: Colors.white,
                      ),
                    ),
                    const SizedBox(height: 16),
                    if (state.errorMessage != null) ...[
                      Text(
                        state.errorMessage!,
                        style: const TextStyle(color: Colors.red),
                      ),
                      const SizedBox(height: 16),
                    ],
                    AnimatedSwitcher(
                      duration: const Duration(milliseconds: 300),
                      transitionBuilder: (
                        Widget child,
                        Animation<double> animation,
                      ) {
                        return FadeTransition(opacity: animation, child: child);
                      },
                      child: Container(
                        key: ValueKey<int>(state.step),
                        child: Column(
                          mainAxisSize: MainAxisSize.min,
                          children: [
                            if (state.step == 0) ...[
                              const Text(
                                'Welcome! Your AI team, ready in minutes.',
                                style: TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                  fontSize: 16,
                                ),
                              ),
                            ] else if (state.step == 1) ...[
                              TextField(
                                decoration: const InputDecoration(
                                  labelText: 'Business Name',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updateCompany,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                                controller: TextEditingController(
                                    text: state.companyName,
                                  )
                                  ..selection = TextSelection.collapsed(
                                    offset: state.companyName.length,
                                  ),
                              ),
                              const SizedBox(height: 16),
                              TextField(
                                decoration: const InputDecoration(
                                  labelText: 'Business Type (Industry)',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updateIndustry,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                                controller: TextEditingController(
                                    text: state.industry,
                                  )
                                  ..selection = TextSelection.collapsed(
                                    offset: state.industry.length,
                                  ),
                              ),
                            ] else if (state.step == 2) ...[
                              TextField(
                                decoration: const InputDecoration(
                                  labelText: 'What do you sell?',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updateWhatTheySell,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                                controller: TextEditingController(
                                    text: state.whatTheySell,
                                  )
                                  ..selection = TextSelection.collapsed(
                                    offset: state.whatTheySell.length,
                                  ),
                              ),
                            ] else if (state.step == 3) ...[
                              TextField(
                                decoration: const InputDecoration(
                                  labelText:
                                      'Payment Preferences (e.g. Stripe)',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updatePaymentPreferences,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                                controller: TextEditingController(
                                    text: state.paymentPreferences,
                                  )
                                  ..selection = TextSelection.collapsed(
                                    offset: state.paymentPreferences.length,
                                  ),
                              ),
                            ] else if (state.step == 4) ...[
                              TextField(
                                decoration: const InputDecoration(
                                  labelText: 'Admin Name',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updateAdminName,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                              ),
                              const SizedBox(height: 16),
                              TextField(
                                decoration: const InputDecoration(
                                  labelText: 'Admin Email',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updateAdminEmail,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                              ),
                              const SizedBox(height: 16),
                              TextField(
                                obscureText: _obscurePassword,
                                onChanged: notifier.updateAdminPassword,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                                decoration: InputDecoration(
                                  labelText: 'Admin Password',
                                  labelStyle: const TextStyle(
                                    color: Colors.white70,
                                  ),
                                  suffixIcon: IconButton(
                                    icon: Icon(
                                      _obscurePassword
                                          ? Icons.visibility
                                          : Icons.visibility_off,
                                      color: Colors.white70,
                                    ),
                                    onPressed: () {
                                      setState(() {
                                        _obscurePassword = !_obscurePassword;
                                      });
                                    },
                                  ),
                                ),
                              ),
                            ],
                          ],
                        ),
                      ),
                    ),
                    const SizedBox(height: 24),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        if (state.step > 0)
                          TextButton(
                            onPressed:
                                state.isLoading ? null : notifier.prevStep,
                            child: const Text(
                              'Back',
                              style: TextStyle(fontFamily: 'Inter'),
                            ),
                          )
                        else
                          const SizedBox(),
                        ElevatedButton(
                          onPressed:
                              state.isLoading
                                  ? null
                                  : () {
                                    if (state.step < 4) {
                                      notifier.saveAndNextStep();
                                    } else {
                                      notifier.launch(context, ref);
                                    }
                                  },
                          child:
                              state.isLoading
                                  ? const SizedBox(
                                    width: 20,
                                    height: 20,
                                    child: CircularProgressIndicator(
                                      strokeWidth: 2,
                                    ),
                                  )
                                  : Text(
                                    state.step == 4
                                        ? 'Launch My AI Team →'
                                        : 'Next',
                                    style: const TextStyle(fontFamily: 'Inter'),
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
      ),
    );
  }
}
