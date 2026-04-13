import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../widgets/glass_card.dart';

// State model for the wizard
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
  final bool isLaunching;

  BusinessSetupState({
    this.step = 0,
    this.companyName = '',
    this.industry = '',
    this.size = 'S',
    this.goals = const [],
    this.deployment = 'Cloud',
    this.adminName = '',
    this.adminEmail = '',
    this.adminPassword = '',
    this.isLaunching = false,
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
    bool? isLaunching,
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
      isLaunching: isLaunching ?? this.isLaunching,
    );
  }
}

// Riverpod provider for state management
class BusinessSetupNotifier extends StateNotifier<BusinessSetupState> {
  BusinessSetupNotifier() : super(BusinessSetupState());

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

  void updateCompanyInfo(String name, String ind, String sz) {
    state = state.copyWith(companyName: name, industry: ind, size: sz);
  }

  void toggleGoal(String goal) {
    final currentGoals = List<String>.from(state.goals);
    if (currentGoals.contains(goal)) {
      currentGoals.remove(goal);
    } else {
      currentGoals.add(goal);
    }
    state = state.copyWith(goals: currentGoals);
  }

  void setDeployment(String dep) {
    state = state.copyWith(deployment: dep);
  }

  void updateAdminInfo(String name, String email, String password) {
    state = state.copyWith(adminName: name, adminEmail: email, adminPassword: password);
  }

  Future<void> launch() async {
    state = state.copyWith(isLaunching: true);
    // Locally mock the API integration via Future.delayed
    await Future.delayed(const Duration(seconds: 2));
    state = state.copyWith(isLaunching: false);
    // Ideally, navigate to the dashboard here
  }
}

final businessSetupProvider = StateNotifierProvider<BusinessSetupNotifier, BusinessSetupState>((ref) {
  return BusinessSetupNotifier();
});

class BusinessSetupWizardScreen extends ConsumerWidget {
  const BusinessSetupWizardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(businessSetupProvider);
    final notifier = ref.watch(businessSetupProvider.notifier);

    Widget buildStepContent() {
      switch (state.step) {
        case 0:
          return Column(
            children: [
              const Hero(
                tag: 'logo',
                child: Icon(Icons.rocket_launch, size: 80, color: Colors.blueAccent),
              ),
              const SizedBox(height: 24),
              const Text(
                'Welcome! Your AI team, ready in minutes.',
                style: TextStyle(fontFamily: 'Inter', fontSize: 18),
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 16),
              const Text(
                'Configure your platform with zero jargon.',
                style: TextStyle(fontFamily: 'Inter', fontSize: 14, color: Colors.grey),
                textAlign: TextAlign.center,
              ),
            ],
          );
        case 1:
          return Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text('Company Profile', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
              const SizedBox(height: 16),
              TextFormField(
                initialValue: state.companyName,
                decoration: const InputDecoration(labelText: 'Company Name', border: OutlineInputBorder()),
                onChanged: (v) => notifier.updateCompanyInfo(v, state.industry, state.size),
                style: const TextStyle(fontFamily: 'Inter'),
              ),
              const SizedBox(height: 16),
              TextFormField(
                initialValue: state.industry,
                decoration: const InputDecoration(labelText: 'Industry', border: OutlineInputBorder()),
                onChanged: (v) => notifier.updateCompanyInfo(state.companyName, v, state.size),
                style: const TextStyle(fontFamily: 'Inter'),
              ),
              const SizedBox(height: 16),
              DropdownButtonFormField<String>(
                value: state.size,
                items: ['S', 'M', 'L', 'Enterprise'].map((s) => DropdownMenuItem(value: s, child: Text(s))).toList(),
                onChanged: (v) {
                  if (v != null) notifier.updateCompanyInfo(state.companyName, state.industry, v);
                },
                decoration: const InputDecoration(labelText: 'Size', border: OutlineInputBorder()),
              )
            ],
          );
        case 2:
          final goals = ['Support', 'Build software', 'Marketing', 'Data', 'Custom'];
          return Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text('Select Goals', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
              const SizedBox(height: 16),
              Wrap(
                spacing: 8,
                runSpacing: 8,
                children: goals.map((goal) {
                  final isSelected = state.goals.contains(goal);
                  return ChoiceChip(
                    label: Text(goal),
                    selected: isSelected,
                    onSelected: (_) => notifier.toggleGoal(goal),
                    selectedColor: Colors.blueAccent.withValues(alpha: 0.3),
                  );
                }).toList(),
              )
            ],
          );
        case 3:
          final deployments = ['Cloud', 'Desktop', 'Mobile-only'];
          return Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text('Deployment Preference', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
              const SizedBox(height: 16),
              ...deployments.map((dep) => RadioListTile<String>(
                title: Text(dep, style: const TextStyle(fontFamily: 'Inter')),
                value: dep,
                groupValue: state.deployment,
                onChanged: (v) {
                  if (v != null) notifier.setDeployment(v);
                },
              )),
            ],
          );
        case 4:
          return Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text('Administrator Account', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
              const SizedBox(height: 16),
              TextFormField(
                initialValue: state.adminName,
                decoration: const InputDecoration(labelText: 'Admin Name', border: OutlineInputBorder()),
                onChanged: (v) => notifier.updateAdminInfo(v, state.adminEmail, state.adminPassword),
                style: const TextStyle(fontFamily: 'Inter'),
              ),
              const SizedBox(height: 16),
              TextFormField(
                initialValue: state.adminEmail,
                decoration: const InputDecoration(labelText: 'Admin Email', border: OutlineInputBorder()),
                onChanged: (v) => notifier.updateAdminInfo(state.adminName, v, state.adminPassword),
                style: const TextStyle(fontFamily: 'Inter'),
              ),
              const SizedBox(height: 16),
              TextFormField(
                initialValue: state.adminPassword,
                decoration: const InputDecoration(labelText: 'Admin Password', border: OutlineInputBorder()),
                obscureText: true,
                onChanged: (v) => notifier.updateAdminInfo(state.adminName, state.adminEmail, v),
                style: const TextStyle(fontFamily: 'Inter'),
              ),
            ],
          );
        case 5:
          return Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text('Review & Launch', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, fontSize: 18)),
              const SizedBox(height: 16),
              Text('Company: ${state.companyName} (${state.industry}, ${state.size})', style: const TextStyle(fontFamily: 'Inter')),
              const SizedBox(height: 8),
              Text('Goals: ${state.goals.join(", ")}', style: const TextStyle(fontFamily: 'Inter')),
              const SizedBox(height: 8),
              Text('Deployment: ${state.deployment}', style: const TextStyle(fontFamily: 'Inter')),
              const SizedBox(height: 8),
              Text('Admin: ${state.adminName} (${state.adminEmail})', style: const TextStyle(fontFamily: 'Inter')),
            ],
          );
        default:
          return const SizedBox.shrink();
      }
    }

    return Scaffold(
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 600),
          child: GlassCard(
            child: Padding(
              padding: const EdgeInsets.all(32.0),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  const Text(
                    'Business Setup',
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 28,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  const SizedBox(height: 32),
                  buildStepContent(),
                  const SizedBox(height: 32),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      if (state.step > 0)
                        TextButton(
                          onPressed: state.isLaunching ? null : notifier.previousStep,
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
                        _PulsingCTA(
                          onPressed: state.isLaunching ? null : () => notifier.launch(),
                          isLaunching: state.isLaunching,
                        )
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

class _PulsingCTA extends StatefulWidget {
  final VoidCallback? onPressed;
  final bool isLaunching;

  const _PulsingCTA({required this.onPressed, required this.isLaunching});

  @override
  State<_PulsingCTA> createState() => _PulsingCTAState();
}

class _PulsingCTAState extends State<_PulsingCTA> with SingleTickerProviderStateMixin {
  late AnimationController _controller;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 1500),
    )..repeat(reverse: true);
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: _controller,
      builder: (context, child) {
        final scale = 1.0 + (_controller.value * 0.05);
        return Transform.scale(
          scale: widget.isLaunching ? 1.0 : scale,
          child: ElevatedButton(
            style: ElevatedButton.styleFrom(
              backgroundColor: Colors.blueAccent,
              foregroundColor: Colors.white,
              padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
            ),
            onPressed: widget.onPressed,
            child: widget.isLaunching
              ? const SizedBox(
                  width: 20, height: 20,
                  child: CircularProgressIndicator(color: Colors.white, strokeWidth: 2),
                )
              : const Text('Launch My AI Team →', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
          ),
        );
      },
    );
  }
}
