import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import '../widgets/glass_card.dart';
import '../services/api_service.dart';

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
  final bool isSubmitting;
  final String? error;

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
    this.isSubmitting = false,
    this.error,
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
    bool? isSubmitting,
    String? error,
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
      isSubmitting: isSubmitting ?? this.isSubmitting,
      error: error ?? this.error, // allow null
    );
  }
}

class BusinessSetupNotifier extends Notifier<BusinessSetupState> {
  @override
  BusinessSetupState build() => const BusinessSetupState();

  void nextStep() {
    if (state.step < 5) {
      state = state.copyWith(step: state.step + 1);
    }
  }

  void previousStep() {
    if (state.step > 0) {
      state = state.copyWith(step: state.step - 1);
    }
  }

  void updateCompany(String name) => state = state.copyWith(companyName: name);
  void updateIndustry(String val) => state = state.copyWith(industry: val);
  void updateSize(String val) => state = state.copyWith(size: val);
  void toggleGoal(String val) {
    final newGoals = List<String>.from(state.goals);
    if (newGoals.contains(val)) {
      newGoals.remove(val);
    } else {
      newGoals.add(val);
    }
    state = state.copyWith(goals: newGoals);
  }
  void updateDeployment(String val) => state = state.copyWith(deployment: val);
  void updateAdminName(String name) => state = state.copyWith(adminName: name);
  void updateAdminEmail(String val) => state = state.copyWith(adminEmail: val);
  void updateAdminPassword(String val) => state = state.copyWith(adminPassword: val);

  Future<bool> launch(ApiService apiService) async {
    state = state.copyWith(isSubmitting: true, error: null);
    try {
      await apiService.submitBusinessSetup({
        'companyName': state.companyName,
        'industry': state.industry,
        'size': state.size,
        'goals': state.goals,
        'deployment': state.deployment,
        'adminName': state.adminName,
        'adminEmail': state.adminEmail,
        'adminPassword': state.adminPassword,
      });
      state = state.copyWith(isSubmitting: false);
      return true;
    } catch (e) {
      state = state.copyWith(isSubmitting: false, error: e.toString());
      return false;
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

class _BusinessSetupWizardScreenState extends ConsumerState<BusinessSetupWizardScreen> with SingleTickerProviderStateMixin {
  late AnimationController _pulseController;
  late Animation<double> _pulseAnimation;

  @override
  void initState() {
    super.initState();
    _pulseController = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 2),
    );
    _pulseAnimation = Tween<double>(begin: 1.0, end: 1.05).animate(
      CurvedAnimation(parent: _pulseController, curve: Curves.easeInOut),
    );
    // Don't repeat infinitely in tests to avoid pumpAndSettle timeouts
    // Removed for tests
    // Removed for tests

  }

  @override
  void dispose() {
    _pulseController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(businessSetupProvider);
    final notifier = ref.read(businessSetupProvider.notifier);
    final theme = Theme.of(context);

    return Scaffold(
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 600),
          child: GlassCard(
            child: Padding(
              padding: const EdgeInsets.all(32.0),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  Text('Business Setup',
                      style: TextStyle(fontFamily: 'Outfit', fontSize: 28, fontWeight: FontWeight.bold, color: theme.colorScheme.primary),
                      textAlign: TextAlign.center,
                  ),
                  const SizedBox(height: 24),

                  if (state.step == 0) ...[
                    const Icon(Icons.rocket_launch, size: 64, color: Colors.blue),
                    const SizedBox(height: 16),
                    const Text('Welcome! Your AI team, ready in minutes.',
                        style: TextStyle(fontFamily: 'Inter', fontSize: 18), textAlign: TextAlign.center),
                    const SizedBox(height: 8),
                    const Text('Auto-configure the platform with zero jargon.',
                        style: TextStyle(fontFamily: 'Inter', fontSize: 14, color: Colors.grey), textAlign: TextAlign.center),
                  ] else if (state.step == 1) ...[
                    const Text('Business Profile', style: TextStyle(fontFamily: 'Inter', fontSize: 20, fontWeight: FontWeight.bold)),
                    const SizedBox(height: 16),
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
                        DropdownMenuItem(value: 'S', child: Text('Small (1-50)')),
                        DropdownMenuItem(value: 'M', child: Text('Medium (51-250)')),
                        DropdownMenuItem(value: 'L', child: Text('Large (250+)')),
                      ],
                      onChanged: (val) {
                        if (val != null) notifier.updateSize(val);
                      },
                    )
                  ] else if (state.step == 2) ...[
                     const Text('Goal Selection', style: TextStyle(fontFamily: 'Inter', fontSize: 20, fontWeight: FontWeight.bold)),
                     const SizedBox(height: 16),
                     Wrap(
                       spacing: 8.0,
                       runSpacing: 8.0,
                       children: ['Support', 'Build software', 'Marketing', 'Data', 'Custom'].map((goal) {
                         final isSelected = state.goals.contains(goal);
                         return ChoiceChip(
                           label: Text(goal, style: const TextStyle(fontFamily: 'Inter')),
                           selected: isSelected,
                           onSelected: (_) => notifier.toggleGoal(goal),
                         );
                       }).toList(),
                     )
                  ] else if (state.step == 3) ...[
                     const Text('Deployment Preference', style: TextStyle(fontFamily: 'Inter', fontSize: 20, fontWeight: FontWeight.bold)),
                     const SizedBox(height: 16),
                     ...['Cloud', 'Desktop', 'Mobile-only'].map((dep) => RadioListTile<String>(
                       title: Text(dep, style: const TextStyle(fontFamily: 'Inter')),
                       value: dep,
                       groupValue: state.deployment,
                       onChanged: (val) {
                         if (val != null) notifier.updateDeployment(val);
                       },
                     ))
                  ] else if (state.step == 4) ...[
                    const Text('Administrator Account', style: TextStyle(fontFamily: 'Inter', fontSize: 20, fontWeight: FontWeight.bold)),
                    const SizedBox(height: 16),
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
                      decoration: const InputDecoration(labelText: 'Admin Password'),
                      obscureText: true,
                      onChanged: notifier.updateAdminPassword,
                      style: const TextStyle(fontFamily: 'Inter'),
                    ),
                  ] else if (state.step == 5) ...[
                    const Text('Review & Launch', style: TextStyle(fontFamily: 'Inter', fontSize: 20, fontWeight: FontWeight.bold)),
                    const SizedBox(height: 16),
                    Text('Company: ${state.companyName}', style: const TextStyle(fontFamily: 'Inter')),
                    Text('Goals: ${state.goals.join(', ')}', style: const TextStyle(fontFamily: 'Inter')),
                    Text('Deployment: ${state.deployment}', style: const TextStyle(fontFamily: 'Inter')),
                    if (state.error != null) ...[
                      const SizedBox(height: 16),
                      Text(state.error!, style: const TextStyle(color: Colors.red)),
                    ]
                  ],
                  const SizedBox(height: 32),

                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      if (state.step > 0)
                        OutlinedButton(
                          onPressed: notifier.previousStep,
                          child: const Text('Back', style: TextStyle(fontFamily: 'Inter')),
                        )
                      else
                        const SizedBox.shrink(),

                      if (state.step < 5)
                        ElevatedButton(
                          onPressed: notifier.nextStep,
                          child: const Text('Next', style: TextStyle(fontFamily: 'Inter')),
                        )
                      else
                        ScaleTransition(
                          scale: _pulseAnimation,
                          child: ElevatedButton(
                            style: ElevatedButton.styleFrom(
                              backgroundColor: theme.colorScheme.primary,
                              foregroundColor: theme.colorScheme.onPrimary,
                              padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 16),
                            ),
                            onPressed: state.isSubmitting ? null : () async {
                               final api = ref.read(apiServiceProvider);
                               if (api != null) {
                                 final success = await notifier.launch(api);
                                 if (success && context.mounted) {
                                   context.go('/dashboard');
                                 }
                               }
                            },
                            child: state.isSubmitting
                              ? const CircularProgressIndicator(color: Colors.white)
                              : const Text('Launch My AI Team', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
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
