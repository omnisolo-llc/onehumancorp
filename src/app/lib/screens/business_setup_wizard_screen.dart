import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

// --- State Models ---

class BusinessSetupState {
  final int currentStep;
  final String companyName;
  final String industry;
  final String size;
  final Set<String> selectedGoals;
  final String deploymentPreference;
  final String adminName;
  final String adminEmail;
  final String adminPassword;

  BusinessSetupState({
    this.currentStep = 0,
    this.companyName = '',
    this.industry = '',
    this.size = '',
    this.selectedGoals = const {},
    this.deploymentPreference = '',
    this.adminName = '',
    this.adminEmail = '',
    this.adminPassword = '',
  });

  BusinessSetupState copyWith({
    int? currentStep,
    String? companyName,
    String? industry,
    String? size,
    Set<String>? selectedGoals,
    String? deploymentPreference,
    String? adminName,
    String? adminEmail,
    String? adminPassword,
  }) {
    return BusinessSetupState(
      currentStep: currentStep ?? this.currentStep,
      companyName: companyName ?? this.companyName,
      industry: industry ?? this.industry,
      size: size ?? this.size,
      selectedGoals: selectedGoals ?? this.selectedGoals,
      deploymentPreference: deploymentPreference ?? this.deploymentPreference,
      adminName: adminName ?? this.adminName,
      adminEmail: adminEmail ?? this.adminEmail,
      adminPassword: adminPassword ?? this.adminPassword,
    );
  }
}

class BusinessSetupNotifier extends StateNotifier<BusinessSetupState> {
  BusinessSetupNotifier() : super(BusinessSetupState());

  void setStep(int step) => state = state.copyWith(currentStep: step);
  void nextStep() => state = state.copyWith(currentStep: state.currentStep + 1);
  void prevStep() => state = state.copyWith(currentStep: state.currentStep > 0 ? state.currentStep - 1 : 0);

  void setBusinessProfile({String? name, String? industry, String? size}) {
    state = state.copyWith(
      companyName: name,
      industry: industry,
      size: size,
    );
  }

  void toggleGoal(String goal) {
    final newGoals = Set<String>.from(state.selectedGoals);
    if (newGoals.contains(goal)) {
      newGoals.remove(goal);
    } else {
      newGoals.add(goal);
    }
    state = state.copyWith(selectedGoals: newGoals);
  }

  void setDeployment(String deployment) {
    state = state.copyWith(deploymentPreference: deployment);
  }

  void setAdmin({String? name, String? email, String? password}) {
    state = state.copyWith(
      adminName: name,
      adminEmail: email,
      adminPassword: password,
    );
  }
}

final businessSetupProvider = StateNotifierProvider<BusinessSetupNotifier, BusinessSetupState>((ref) {
  return BusinessSetupNotifier();
});

// --- Screens ---

class BusinessSetupWizardScreen extends ConsumerWidget {
  const BusinessSetupWizardScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(businessSetupProvider);

    return Scaffold(
      backgroundColor: const Color(0xFFF5F5F7), // Light Gray/White Background
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 600),
          child: Container(
            margin: const EdgeInsets.symmetric(vertical: 40, horizontal: 24),
            child: ClipRRect(
              borderRadius: BorderRadius.circular(16),
              child: BackdropFilter(
                filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
                child: Container(
                  decoration: BoxDecoration(
                    color: Colors.white.withOpacity(0.65),
                    borderRadius: BorderRadius.circular(16),
                    border: Border.all(
                      color: Colors.white.withOpacity(0.4),
                      width: 1,
                    ),
                  ),
                  child: SingleChildScrollView(
                    child: AnimatedSwitcher(
                      duration: const Duration(milliseconds: 300),
                      child: _buildStepContent(context, ref, state),
                    ),
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildStepContent(BuildContext context, WidgetRef ref, BusinessSetupState state) {
    switch (state.currentStep) {
      case 0:
        return _WelcomeStep(key: const ValueKey(0));
      case 1:
        return _BusinessProfileStep(key: const ValueKey(1));
      case 2:
        return _GoalSelectionStep(key: const ValueKey(2));
      case 3:
        return _DeploymentStep(key: const ValueKey(3));
      case 4:
        return _AdminAccountStep(key: const ValueKey(4));
      case 5:
        return _ReviewStep(key: const ValueKey(5));
      default:
        return const SizedBox.shrink();
    }
  }
}

// --- Step 0: Welcome ---

class _WelcomeStep extends ConsumerWidget {
  const _WelcomeStep({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Padding(
      padding: const EdgeInsets.all(40),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          Container(
            padding: const EdgeInsets.all(24),
            decoration: BoxDecoration(
              color: const Color(0xFF0066FF).withOpacity(0.1),
              shape: BoxShape.circle,
            ),
            child: const Icon(
              Icons.auto_awesome,
              size: 64,
              color: Color(0xFF0066FF),
            ),
          ),
          const SizedBox(height: 32),
          const Text(
            'Welcome to One Human Corp',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 32,
              fontWeight: FontWeight.bold,
              color: Color(0xFF1D1D1F),
            ),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 16),
          Text(
            'We will set up your personalized AI team in just a few clicks. No technical knowledge required.',
            style: TextStyle(
              fontFamily: 'Inter',
              fontSize: 16,
              color: Colors.grey[600],
            ),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 48),
          ElevatedButton(
            onPressed: () => ref.read(businessSetupProvider.notifier).nextStep(),
            style: ElevatedButton.styleFrom(
              backgroundColor: const Color(0xFF0066FF),
              foregroundColor: Colors.white,
              padding: const EdgeInsets.symmetric(vertical: 18, horizontal: 32),
              minimumSize: const Size(double.infinity, 54),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(8),
              ),
              elevation: 0,
            ),
            child: const Text(
              'Get Started',
              style: TextStyle(
                fontFamily: 'Inter',
                fontSize: 16,
                fontWeight: FontWeight.bold,
              ),
            ),
          ),
        ],
      ),
    );
  }
}

// --- Step 1: Business Profile ---

class _BusinessProfileStep extends ConsumerStatefulWidget {
  const _BusinessProfileStep({Key? key}) : super(key: key);

  @override
  _BusinessProfileStepState createState() => _BusinessProfileStepState();
}

class _BusinessProfileStepState extends ConsumerState<_BusinessProfileStep> {
  final _formKey = GlobalKey<FormState>();
  late TextEditingController _nameController;
  late TextEditingController _industryController;
  late TextEditingController _sizeController;

  @override
  void initState() {
    super.initState();
    final state = ref.read(businessSetupProvider);
    _nameController = TextEditingController(text: state.companyName);
    _industryController = TextEditingController(text: state.industry);
    _sizeController = TextEditingController(text: state.size);
  }

  @override
  void dispose() {
    _nameController.dispose();
    _industryController.dispose();
    _sizeController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(40),
      child: Form(
        key: _formKey,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Business Profile',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 28,
                fontWeight: FontWeight.bold,
                color: Color(0xFF1D1D1F),
              ),
            ),
            const SizedBox(height: 8),
            Text(
              'Tell us a bit about your business.',
              style: TextStyle(fontFamily: 'Inter', color: Colors.grey[600]),
            ),
            const SizedBox(height: 32),
            TextFormField(
              controller: _nameController,
              decoration: _inputDecoration('Company Name'),
              validator: (v) => v!.isEmpty ? 'Required' : null,
              onChanged: (v) => ref.read(businessSetupProvider.notifier).setBusinessProfile(name: v),
            ),
            const SizedBox(height: 16),
            TextFormField(
              controller: _industryController,
              decoration: _inputDecoration('Industry'),
              onChanged: (v) => ref.read(businessSetupProvider.notifier).setBusinessProfile(industry: v),
            ),
            const SizedBox(height: 16),
            TextFormField(
              controller: _sizeController,
              decoration: _inputDecoration('Company Size (e.g. 1-10)'),
              onChanged: (v) => ref.read(businessSetupProvider.notifier).setBusinessProfile(size: v),
            ),
            const SizedBox(height: 40),
            Row(
              children: [
                TextButton(
                  onPressed: () => ref.read(businessSetupProvider.notifier).prevStep(),
                  child: const Text('Back', style: TextStyle(color: Colors.grey, fontFamily: 'Inter')),
                ),
                const Spacer(),
                ElevatedButton(
                  onPressed: () {
                    if (_formKey.currentState!.validate()) {
                      ref.read(businessSetupProvider.notifier).nextStep();
                    }
                  },
                  style: _primaryButtonStyle(),
                  child: const Text('Next'),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

// --- Step 2: Goal Selection ---

class _GoalSelectionStep extends ConsumerWidget {
  const _GoalSelectionStep({Key? key}) : super(key: key);

  final List<Map<String, dynamic>> _goals = const [
    {'id': 'support', 'label': 'Customer Support', 'icon': Icons.headset_mic},
    {'id': 'build', 'label': 'Build Software', 'icon': Icons.code},
    {'id': 'marketing', 'label': 'Marketing', 'icon': Icons.campaign},
    {'id': 'data', 'label': 'Data Analysis', 'icon': Icons.analytics},
    {'id': 'custom', 'label': 'Custom Operations', 'icon': Icons.settings},
  ];

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(businessSetupProvider);

    return Padding(
      padding: const EdgeInsets.all(40),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            'What are your primary goals?',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 28,
              fontWeight: FontWeight.bold,
              color: Color(0xFF1D1D1F),
            ),
          ),
          const SizedBox(height: 8),
          Text(
            'Select all that apply to help us configure your agents.',
            style: TextStyle(fontFamily: 'Inter', color: Colors.grey[600]),
          ),
          const SizedBox(height: 32),
          Wrap(
            spacing: 12,
            runSpacing: 12,
            children: _goals.map((goal) {
              final isSelected = state.selectedGoals.contains(goal['id']);
              return InkWell(
                onTap: () => ref.read(businessSetupProvider.notifier).toggleGoal(goal['id']),
                borderRadius: BorderRadius.circular(8),
                child: Container(
                  padding: const EdgeInsets.symmetric(vertical: 12, horizontal: 16),
                  decoration: BoxDecoration(
                    color: isSelected ? const Color(0xFF0066FF).withOpacity(0.1) : Colors.white.withOpacity(0.5),
                    border: Border.all(
                      color: isSelected ? const Color(0xFF0066FF) : Colors.grey[300]!,
                      width: 1,
                    ),
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Icon(
                        goal['icon'],
                        color: isSelected ? const Color(0xFF0066FF) : Colors.grey[600],
                        size: 20,
                      ),
                      const SizedBox(width: 8),
                      Text(
                        goal['label'],
                        style: TextStyle(
                          fontFamily: 'Inter',
                          color: isSelected ? const Color(0xFF0066FF) : const Color(0xFF1D1D1F),
                          fontWeight: isSelected ? FontWeight.bold : FontWeight.normal,
                        ),
                      ),
                    ],
                  ),
                ),
              );
            }).toList(),
          ),
          const SizedBox(height: 40),
          Row(
            children: [
              TextButton(
                onPressed: () => ref.read(businessSetupProvider.notifier).prevStep(),
                child: const Text('Back', style: TextStyle(color: Colors.grey, fontFamily: 'Inter')),
              ),
              const Spacer(),
              ElevatedButton(
                onPressed: () => ref.read(businessSetupProvider.notifier).nextStep(),
                style: _primaryButtonStyle(),
                child: const Text('Next'),
              ),
            ],
          ),
        ],
      ),
    );
  }
}

// --- Step 3: Deployment Preference ---

class _DeploymentStep extends ConsumerWidget {
  const _DeploymentStep({Key? key}) : super(key: key);

  final List<Map<String, String>> _options = const [
    {'id': 'cloud', 'label': 'Cloud', 'desc': 'Hosted automatically by One Human Corp.'},
    {'id': 'desktop', 'label': 'Desktop', 'desc': 'Run locally on this machine.'},
    {'id': 'mobile', 'label': 'Mobile-only', 'desc': 'Manage everything from your phone.'},
  ];

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(businessSetupProvider);

    return Padding(
      padding: const EdgeInsets.all(40),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            'Deployment Preference',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 28,
              fontWeight: FontWeight.bold,
              color: Color(0xFF1D1D1F),
            ),
          ),
          const SizedBox(height: 8),
          Text(
            'Where should your agents live?',
            style: TextStyle(fontFamily: 'Inter', color: Colors.grey[600]),
          ),
          const SizedBox(height: 32),
          Column(
            children: _options.map((opt) {
              final isSelected = state.deploymentPreference == opt['id'];
              return Padding(
                padding: const EdgeInsets.only(bottom: 12),
                child: InkWell(
                  onTap: () => ref.read(businessSetupProvider.notifier).setDeployment(opt['id']!),
                  borderRadius: BorderRadius.circular(12),
                  child: Container(
                    padding: const EdgeInsets.all(16),
                    decoration: BoxDecoration(
                      color: isSelected ? const Color(0xFF0066FF).withOpacity(0.05) : Colors.white.withOpacity(0.5),
                      border: Border.all(
                        color: isSelected ? const Color(0xFF0066FF) : Colors.grey[300]!,
                        width: 1,
                      ),
                      borderRadius: BorderRadius.circular(12),
                    ),
                    child: Row(
                      children: [
                        Icon(
                          isSelected ? Icons.radio_button_checked : Icons.radio_button_unchecked,
                          color: isSelected ? const Color(0xFF0066FF) : Colors.grey[400],
                        ),
                        const SizedBox(width: 16),
                        Expanded(
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                opt['label']!,
                                style: TextStyle(
                                  fontFamily: 'Outfit',
                                  fontSize: 16,
                                  fontWeight: FontWeight.bold,
                                  color: isSelected ? const Color(0xFF0066FF) : const Color(0xFF1D1D1F),
                                ),
                              ),
                              const SizedBox(height: 4),
                              Text(
                                opt['desc']!,
                                style: TextStyle(
                                  fontFamily: 'Inter',
                                  fontSize: 14,
                                  color: Colors.grey[600],
                                ),
                              ),
                            ],
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              );
            }).toList(),
          ),
          const SizedBox(height: 28),
          Row(
            children: [
              TextButton(
                onPressed: () => ref.read(businessSetupProvider.notifier).prevStep(),
                child: const Text('Back', style: TextStyle(color: Colors.grey, fontFamily: 'Inter')),
              ),
              const Spacer(),
              ElevatedButton(
                onPressed: state.deploymentPreference.isEmpty ? null : () => ref.read(businessSetupProvider.notifier).nextStep(),
                style: _primaryButtonStyle(),
                child: const Text('Next'),
              ),
            ],
          ),
        ],
      ),
    );
  }
}

// --- Step 4: Administrator Account ---

class _AdminAccountStep extends ConsumerStatefulWidget {
  const _AdminAccountStep({Key? key}) : super(key: key);

  @override
  _AdminAccountStepState createState() => _AdminAccountStepState();
}

class _AdminAccountStepState extends ConsumerState<_AdminAccountStep> {
  final _formKey = GlobalKey<FormState>();
  late TextEditingController _nameController;
  late TextEditingController _emailController;
  late TextEditingController _pwdController;

  @override
  void initState() {
    super.initState();
    final state = ref.read(businessSetupProvider);
    _nameController = TextEditingController(text: state.adminName);
    _emailController = TextEditingController(text: state.adminEmail);
    _pwdController = TextEditingController(text: state.adminPassword);
  }

  @override
  void dispose() {
    _nameController.dispose();
    _emailController.dispose();
    _pwdController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(40),
      child: Form(
        key: _formKey,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Administrator Account',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 28,
                fontWeight: FontWeight.bold,
                color: Color(0xFF1D1D1F),
              ),
            ),
            const SizedBox(height: 8),
            Text(
              'Create your main login.',
              style: TextStyle(fontFamily: 'Inter', color: Colors.grey[600]),
            ),
            const SizedBox(height: 32),
            TextFormField(
              controller: _nameController,
              decoration: _inputDecoration('Full Name'),
              validator: (v) => v!.isEmpty ? 'Required' : null,
              onChanged: (v) => ref.read(businessSetupProvider.notifier).setAdmin(name: v),
            ),
            const SizedBox(height: 16),
            TextFormField(
              controller: _emailController,
              decoration: _inputDecoration('Email Address'),
              validator: (v) => v!.isEmpty || !v.contains('@') ? 'Valid email required' : null,
              onChanged: (v) => ref.read(businessSetupProvider.notifier).setAdmin(email: v),
            ),
            const SizedBox(height: 16),
            TextFormField(
              controller: _pwdController,
              decoration: _inputDecoration('Password'),
              obscureText: true,
              validator: (v) => v!.length < 6 ? 'Min 6 chars' : null,
              onChanged: (v) => ref.read(businessSetupProvider.notifier).setAdmin(password: v),
            ),
            const SizedBox(height: 40),
            Row(
              children: [
                TextButton(
                  onPressed: () => ref.read(businessSetupProvider.notifier).prevStep(),
                  child: const Text('Back', style: TextStyle(color: Colors.grey, fontFamily: 'Inter')),
                ),
                const Spacer(),
                ElevatedButton(
                  onPressed: () {
                    if (_formKey.currentState!.validate()) {
                      ref.read(businessSetupProvider.notifier).nextStep();
                    }
                  },
                  style: _primaryButtonStyle(),
                  child: const Text('Next'),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

// --- Step 5: Review & Launch ---

class _ReviewStep extends ConsumerStatefulWidget {
  const _ReviewStep({Key? key}) : super(key: key);

  @override
  _ReviewStepState createState() => _ReviewStepState();
}

class _ReviewStepState extends ConsumerState<_ReviewStep> with SingleTickerProviderStateMixin {
  late AnimationController _pulseController;
  bool _isLaunching = false;

  @override
  void initState() {
    super.initState();
    _pulseController = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 2),
    );
    // Only repeat in non-test environments to prevent pumpAndSettle timeouts
    if (!const bool.hasEnvironment('FLUTTER_TEST')) {
      _pulseController.repeat(reverse: true);
    }
  }

  @override
  void dispose() {
    _pulseController.dispose();
    super.dispose();
  }

  void _launch() async {
    setState(() => _isLaunching = true);

    // Mock API Call delay
    await Future.delayed(const Duration(seconds: 2));

    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('AI Team Launched Successfully!'), backgroundColor: Color(0xFF34C759)),
      );
      // Reset or navigate away
      ref.read(businessSetupProvider.notifier).setStep(0);
      Navigator.of(context).pop(); // Attempt to pop, or redirect appropriately.
    }
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(businessSetupProvider);

    return Padding(
      padding: const EdgeInsets.all(40),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            'Review & Launch',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 28,
              fontWeight: FontWeight.bold,
              color: Color(0xFF1D1D1F),
            ),
          ),
          const SizedBox(height: 8),
          Text(
            'Everything looks great. Ready to go?',
            style: TextStyle(fontFamily: 'Inter', color: Colors.grey[600]),
          ),
          const SizedBox(height: 32),
          Container(
            padding: const EdgeInsets.all(16),
            decoration: BoxDecoration(
              color: Colors.white.withOpacity(0.8),
              borderRadius: BorderRadius.circular(12),
              border: Border.all(color: Colors.grey[200]!),
            ),
            child: Column(
              children: [
                _buildSummaryRow('Company', state.companyName),
                const Divider(),
                _buildSummaryRow('Goals', state.selectedGoals.isEmpty ? 'None' : state.selectedGoals.join(', ')),
                const Divider(),
                _buildSummaryRow('Deployment', state.deploymentPreference),
                const Divider(),
                _buildSummaryRow('Admin', state.adminEmail),
              ],
            ),
          ),
          const SizedBox(height: 40),
          Row(
            children: [
              if (!_isLaunching)
                TextButton(
                  onPressed: () => ref.read(businessSetupProvider.notifier).prevStep(),
                  child: const Text('Back', style: TextStyle(color: Colors.grey, fontFamily: 'Inter')),
                ),
              const Spacer(),
              AnimatedBuilder(
                animation: _pulseController,
                builder: (context, child) {
                  return Container(
                    decoration: BoxDecoration(
                      borderRadius: BorderRadius.circular(8),
                      boxShadow: [
                        BoxShadow(
                          color: const Color(0xFF0066FF).withOpacity(_pulseController.value * 0.5),
                          blurRadius: 20 * _pulseController.value,
                          spreadRadius: 5 * _pulseController.value,
                        ),
                      ],
                    ),
                    child: child,
                  );
                },
                child: ElevatedButton(
                  onPressed: _isLaunching ? null : _launch,
                  style: ElevatedButton.styleFrom(
                    backgroundColor: const Color(0xFF0066FF),
                    foregroundColor: Colors.white,
                    padding: const EdgeInsets.symmetric(vertical: 18, horizontal: 32),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(8),
                    ),
                    elevation: 0,
                  ),
                  child: _isLaunching
                      ? const SizedBox(
                          width: 20,
                          height: 20,
                          child: CircularProgressIndicator(color: Colors.white, strokeWidth: 2),
                        )
                      : const Text(
                          'Launch My AI Team',
                          style: TextStyle(
                            fontFamily: 'Inter',
                            fontSize: 16,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildSummaryRow(String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(label, style: const TextStyle(fontFamily: 'Inter', color: Colors.grey)),
          Text(value, style: const TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Color(0xFF1D1D1F))),
        ],
      ),
    );
  }
}

// --- Helpers ---

InputDecoration _inputDecoration(String label) {
  return InputDecoration(
    labelText: label,
    labelStyle: TextStyle(fontFamily: 'Inter', color: Colors.grey[600]),
    filled: true,
    fillColor: Colors.white.withOpacity(0.5),
    border: OutlineInputBorder(
      borderRadius: BorderRadius.circular(8),
      borderSide: BorderSide(color: Colors.grey[300]!),
    ),
    enabledBorder: OutlineInputBorder(
      borderRadius: BorderRadius.circular(8),
      borderSide: BorderSide(color: Colors.grey[300]!),
    ),
    focusedBorder: OutlineInputBorder(
      borderRadius: BorderRadius.circular(8),
      borderSide: const BorderSide(color: Color(0xFF0066FF)),
    ),
  );
}

ButtonStyle _primaryButtonStyle() {
  return ElevatedButton.styleFrom(
    backgroundColor: const Color(0xFF0066FF),
    foregroundColor: Colors.white,
    padding: const EdgeInsets.symmetric(vertical: 16, horizontal: 24),
    shape: RoundedRectangleBorder(
      borderRadius: BorderRadius.circular(8),
    ),
    elevation: 0,
  );
}
