import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../providers/wizard_provider.dart';
import '../main.dart'; // For GlassContainer
import '../widgets/contextual_tooltip.dart';
import '../widgets/walkthrough_overlay.dart';
import 'help/help_center_screen.dart';

enum EnvironmentMode { cloud, standaloneDesktop }

class BusinessSetupWizardScreen extends ConsumerStatefulWidget {
  final EnvironmentMode environmentMode;

  const BusinessSetupWizardScreen({
    super.key,
    this.environmentMode = EnvironmentMode.cloud,
  });

  @override
  ConsumerState<BusinessSetupWizardScreen> createState() => _BusinessSetupWizardScreenState();
}

class _BusinessSetupWizardScreenState extends ConsumerState<BusinessSetupWizardScreen> with TickerProviderStateMixin {
  final _companyNameController = TextEditingController();
  final _adminNameController = TextEditingController();
  final _adminEmailController = TextEditingController();
  final _adminPasswordController = TextEditingController();

  late AnimationController _heroAnimationController;
  late Animation<double> _heroAnimation;

  late AnimationController _pulseAnimationController;
  late Animation<double> _pulseAnimation;

  bool _showWalkthrough = true;

  @override
  void initState() {
    super.initState();
    _heroAnimationController = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 2),
    )..repeat(reverse: true);

    _heroAnimation = Tween<double>(begin: -10, end: 10).animate(
      CurvedAnimation(parent: _heroAnimationController, curve: Curves.easeInOut),
    );

    _pulseAnimationController = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 1),
    )..repeat(reverse: true);

    _pulseAnimation = Tween<double>(begin: 1.0, end: 1.05).animate(
      CurvedAnimation(parent: _pulseAnimationController, curve: Curves.easeInOut),
    );
  }

  @override
  void dispose() {
    _heroAnimationController.dispose();
    _pulseAnimationController.dispose();
    _companyNameController.dispose();
    _adminNameController.dispose();
    _adminEmailController.dispose();
    _adminPasswordController.dispose();
    super.dispose();
  }

  void _nextStep() {
    ref.read(wizardProvider.notifier).nextStep();
  }

  void _prevStep() {
    ref.read(wizardProvider.notifier).prevStep();
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(wizardProvider);

    if (state.currentStep == 6) {
      return const DashboardScreen();
    }

    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      body: Stack(
        children: [
          Center(
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 400),
              child: Padding(
                padding: const EdgeInsets.all(20),
                child: _buildCurrentStep(state.currentStep, state),
              ),
            ),
          ),
          Positioned(
            top: 40,
            right: 20,
            child: IconButton(
              icon: const Icon(Icons.help_outline, color: Colors.white, size: 30),
              onPressed: () {
                Navigator.push(context, MaterialPageRoute(builder: (context) => const HelpCenterScreen()));
              },
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildCurrentStep(int step, WizardState state) {
    switch (step) {
      case 0:
        return _buildWelcomeScreen();
      case 1:
        return _buildBusinessProfileScreen(state);
      case 2:
        return _buildGoalSelectionScreen(state);
      case 3:
        return _buildAIGenerationReviewScreen(state);
      case 4:
        return _buildPaymentSetupScreen(state);
      case 5:
        return _buildReviewAndLaunchScreen(state);
      default:
        return const SizedBox.shrink();
    }
  }

  Widget _buildWelcomeScreen() {
    return SingleChildScrollView(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          AnimatedBuilder(
            animation: _heroAnimation,
            builder: (context, child) {
              return Transform.translate(
                offset: Offset(0, _heroAnimation.value),
                child: child,
              );
            },
            child: const Icon(Icons.rocket_launch, size: 80, color: Color(0xFF6B4EFF)),
          ),
          const SizedBox(height: 30),
          const Text(
            'Welcome to One Human Corp',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 32,
              fontWeight: FontWeight.bold,
              color: Colors.white,
            ),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 20),
          const Text(
            'Create your account to start your business.',
            style: TextStyle(
              fontFamily: 'Inter',
              fontSize: 16,
              color: Colors.white70,
            ),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 20),
          GlassContainer(
            child: TextField(
              key: const Key('signupEmailField'),
              style: const TextStyle(color: Colors.white),
              decoration: const InputDecoration(
                labelText: 'Email',
                labelStyle: TextStyle(color: Colors.white70),
                border: InputBorder.none,
              ),
            ),
          ),
          const SizedBox(height: 10),
          GlassContainer(
            child: TextField(
              key: const Key('signupPasswordField'),
              obscureText: true,
              style: const TextStyle(color: Colors.white),
              decoration: const InputDecoration(
                labelText: 'Password',
                labelStyle: TextStyle(color: Colors.white70),
                border: InputBorder.none,
              ),
            ),
          ),
          const SizedBox(height: 20),
          WalkthroughHighlight(
            showHighlight: _showWalkthrough,
            speechBubbleText: "Start setting up your store here!",
            onDismiss: () => setState(() => _showWalkthrough = false),
            child: ElevatedButton(
              key: const Key('signupBtn'),
              onPressed: () {
                setState(() => _showWalkthrough = false);
                _nextStep();
              },
              style: ElevatedButton.styleFrom(
                backgroundColor: const Color(0xFF6B4EFF),
                padding: const EdgeInsets.symmetric(vertical: 20),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(15),
                ),
              ),
              child: const Text('Sign Up & Continue', style: TextStyle(fontSize: 18, color: Colors.white, fontWeight: FontWeight.bold)),
            ),
          ),
          const SizedBox(height: 15),
          ElevatedButton.icon(
            onPressed: () {
              setState(() => _showWalkthrough = false);
              _nextStep();
            },
            icon: const Icon(Icons.g_mobiledata, color: Colors.white, size: 30),
            label: const Text('Continue with Google', style: TextStyle(fontSize: 16, color: Colors.white)),
            style: ElevatedButton.styleFrom(
              backgroundColor: const Color(0xFF1E293B),
              padding: const EdgeInsets.symmetric(vertical: 15),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(15),
              ),
            ),
          ),
          const SizedBox(height: 10),
          ElevatedButton.icon(
            onPressed: () {
              setState(() => _showWalkthrough = false);
              _nextStep();
            },
            icon: const Icon(Icons.apple, color: Colors.white, size: 30),
            label: const Text('Continue with Apple', style: TextStyle(fontSize: 16, color: Colors.white)),
            style: ElevatedButton.styleFrom(
              backgroundColor: const Color(0xFF1E293B),
              padding: const EdgeInsets.symmetric(vertical: 15),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(15),
              ),
            ),
          ),
          const SizedBox(height: 15),
          Center(
            child: TextButton(
              onPressed: () {
                ScaffoldMessenger.of(context).showSnackBar(
                  const SnackBar(content: Text('Verification email resent!')),
                );
              },
              child: const Text('Resend Verification Email', style: TextStyle(color: Color(0xFF6B4EFF), fontSize: 14)),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildBusinessProfileScreen(WizardState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Business Profile',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 20),
        GlassContainer(
          child: TextField(
            key: const Key('companyNameField'),
            controller: _companyNameController,
            style: const TextStyle(color: Colors.white),
            decoration: const InputDecoration(
              labelText: 'Company Name',
              labelStyle: TextStyle(color: Colors.white70),
              border: InputBorder.none,
            ),
            onChanged: (value) {
              ref.read(wizardProvider.notifier).updateBusinessProfile(companyName: value);
            },
          ),
        ),
        const SizedBox(height: 15),
        GlassContainer(
          child: DropdownButtonHideUnderline(
            child: ContextualTooltip(
              tooltipKey: 'industryDropdown',
              child: DropdownButton<String>(
                key: const Key('industryDropdown'),
              value: state.industry,
              isExpanded: true,
              dropdownColor: const Color(0xFF1E293B),
              style: const TextStyle(color: Colors.white),
              hint: const Text('Industry', style: TextStyle(color: Colors.white70)),
              items: ['Retail', 'Service', 'Technology', 'Food & Beverage', 'Other']
                  .map((String value) {
                return DropdownMenuItem<String>(
                  value: value,
                  child: Text(value),
                );
              }).toList(),
                onChanged: (newValue) {
                  ref.read(wizardProvider.notifier).updateBusinessProfile(industry: newValue);
                },
              ),
            ),
          ),
        ),
        const Spacer(),
        _buildNavigationButtons(),
      ],
    );
  }

  Widget _buildGoalSelectionScreen(WizardState state) {
    final goals = ['Sell products online', 'Take bookings', 'Build a portfolio'];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'What\'s your primary goal right now?',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 20),
        Expanded(
          child: ListView.builder(
            itemCount: goals.length,
            itemBuilder: (context, index) {
              final goal = goals[index];
              final isSelected = state.primaryGoal == goal;
              return Padding(
                padding: const EdgeInsets.only(bottom: 10),
                child: GestureDetector(
                  onTap: () {
                    ref.read(wizardProvider.notifier).setPrimaryGoal(goal);
                  },
                  child: GlassContainer(
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Expanded(
                          child: Text(
                            goal,
                            style: const TextStyle(color: Colors.white, fontSize: 16),
                          ),
                        ),
                        Icon(
                          isSelected ? Icons.check_circle : Icons.radio_button_unchecked,
                          color: isSelected ? const Color(0xFF22C55E) : Colors.white54,
                        ),
                      ],
                    ),
                  ),
                ),
              );
            },
          ),
        ),
        _buildNavigationButtons(),
      ],
    );
  }

  Widget _buildAIGenerationReviewScreen(WizardState state) {
    final templates = [
      {'id': 'modern', 'name': 'Modern', 'icon': Icons.web, 'color': Colors.blue},
      {'id': 'cozy', 'name': 'Cozy', 'icon': Icons.coffee, 'color': Colors.orange},
      {'id': 'professional', 'name': 'Professional', 'icon': Icons.business, 'color': Colors.grey},
    ];

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Review AI-Generated Draft',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 10),
        const Text(
          'The Marketing & Advertising AI department has generated these storefronts for you based on your inputs. Choose one to start with or regenerate.',
          style: TextStyle(color: Colors.white70, fontSize: 16),
        ),
        const SizedBox(height: 20),
        Expanded(
          child: SingleChildScrollView(
            child: Column(
              children: templates.map((template) {
                final isSelected = state.templateSelection == template['id'];
                return Padding(
                  padding: const EdgeInsets.only(bottom: 15),
                  child: InkWell(
                    onTap: () {
                      ref.read(wizardProvider.notifier).setTemplateSelection(template['id'] as String);
                    },
                    child: Container(
                      decoration: BoxDecoration(
                        color: isSelected ? const Color(0xFF6B4EFF).withOpacity(0.3) : Colors.white.withOpacity(0.05),
                        border: Border.all(
                          color: isSelected ? const Color(0xFF6B4EFF) : Colors.white.withOpacity(0.1),
                          width: 2,
                        ),
                        borderRadius: BorderRadius.circular(15),
                      ),
                      padding: const EdgeInsets.all(20),
                      child: Row(
                        children: [
                          Icon(template['icon'] as IconData, size: 40, color: template['color'] as Color),
                          const SizedBox(width: 20),
                          Expanded(
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Text(
                                  template['name'] as String,
                                  style: const TextStyle(
                                    fontFamily: 'Outfit',
                                    fontSize: 20,
                                    fontWeight: FontWeight.bold,
                                    color: Colors.white,
                                  ),
                                ),
                                const SizedBox(height: 5),
                                const SizedBox(height: 10),
                                Container(
                                  height: 100,
                                  decoration: BoxDecoration(
                                    color: template['id'] == 'modern' ? Colors.blue.withOpacity(0.1) : template['id'] == 'cozy' ? Colors.orange.withOpacity(0.1) : Colors.grey.withOpacity(0.1),
                                    borderRadius: BorderRadius.circular(8),
                                    border: Border.all(color: Colors.white24),
                                  ),
                                  child: Center(
                                    child: Text(
                                      state.companyName ?? "Your Business",
                                      style: TextStyle(
                                        fontFamily: template['id'] == 'modern' ? 'Inter' : template['id'] == 'cozy' ? 'Outfit' : 'Times New Roman',
                                        fontSize: 24,
                                        fontWeight: FontWeight.bold,
                                        color: template['color'] as Color,
                                      ),
                                    ),
                                  ),
                                ),
                              ],
                            ),
                          ),
                          if (isSelected) const Icon(Icons.check_circle, color: Color(0xFF22C55E), size: 30),
                        ],
                      ),
                    ),
                  ),
                );
              }).toList(),
            ),
          ),
        ),
        const SizedBox(height: 20),
        _buildNavigationButtons(),
      ],
    );
  }

  Widget _buildPaymentSetupScreen(WizardState state) {
    final paymentModes = [
      {'id': 'stripe', 'name': 'Connect Stripe', 'icon': Icons.credit_card},
      {'id': 'ohc_link', 'name': 'Set up OHC Payment Link', 'icon': Icons.link},
      {'id': 'defer', 'name': 'Defer for later', 'icon': Icons.schedule},
    ];

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Payment Setup',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 10),
        const Text(
          'How would you like to get paid?',
          style: TextStyle(color: Colors.white70, fontSize: 16),
        ),
        const SizedBox(height: 20),
        Expanded(
          child: ListView.builder(
            itemCount: paymentModes.length,
            itemBuilder: (context, index) {
              final mode = paymentModes[index];
              final isSelected = state.paymentSetupMode == mode['id'];
              return Padding(
                padding: const EdgeInsets.only(bottom: 15),
                child: InkWell(
                  onTap: () {
                    ref.read(wizardProvider.notifier).setPaymentSetupMode(mode['id'] as String);
                  },
                  child: GlassContainer(
                    child: Row(
                      children: [
                        Icon(mode['icon'] as IconData, color: isSelected ? const Color(0xFF6B4EFF) : Colors.white54, size: 30),
                        const SizedBox(width: 20),
                        Expanded(
                          child: Text(
                            mode['name'] as String,
                            style: const TextStyle(
                              fontFamily: 'Inter',
                              fontSize: 18,
                              color: Colors.white,
                            ),
                          ),
                        ),
                        if (isSelected) const Icon(Icons.check_circle, color: Color(0xFF22C55E), size: 24),
                      ],
                    ),
                  ),
                ),
              );
            },
          ),
        ),
        _buildNavigationButtons(),
      ],
    );
  }

  Widget _buildReviewAndLaunchScreen(WizardState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Review & Launch',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 20),
        Expanded(
          child: SingleChildScrollView(
            child: GlassContainer(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  _buildSummaryItem('Company Name', state.companyName ?? 'Not set'),
                  const SizedBox(height: 10),
                  _buildSummaryItem('Industry', state.industry ?? 'Not set'),
                  const SizedBox(height: 10),
                  _buildSummaryItem('Primary Goal', state.primaryGoal ?? 'Not set'),
                  const SizedBox(height: 10),
                  _buildSummaryItem('Template', state.templateSelection ?? 'Not set'),
                  const SizedBox(height: 10),
                  _buildSummaryItem('Payment Setup', state.paymentSetupMode ?? 'Not set'),
                ],
              ),
            ),
          ),
        ),
        const SizedBox(height: 20),
        Row(
          children: [
            Expanded(
              flex: 1,
              child: ElevatedButton(
                onPressed: _prevStep,
                style: ElevatedButton.styleFrom(
                  backgroundColor: Colors.transparent,
                  side: const BorderSide(color: Colors.white54),
                  padding: const EdgeInsets.symmetric(vertical: 20),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(15),
                  ),
                ),
                child: const Text('Back', style: TextStyle(fontSize: 18, color: Colors.white)),
              ),
            ),
            const SizedBox(width: 15),
            Expanded(
              flex: 2,
              child: AnimatedBuilder(
                animation: _pulseAnimation,
                builder: (context, child) {
                  return Transform.scale(
                    scale: _pulseAnimation.value,
                    child: child,
                  );
                },
                child: ElevatedButton(
                  onPressed: () async {
                    await ref.read(wizardProvider.notifier).submitWizard();
                  },
                  style: ElevatedButton.styleFrom(
                    backgroundColor: const Color(0xFF22C55E),
                    padding: const EdgeInsets.symmetric(vertical: 20),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(15),
                    ),
                  ),
                  child: const Text('Launch My AI Team', style: TextStyle(fontSize: 18, color: Colors.white, fontWeight: FontWeight.bold)),
                ),
              ),
            ),
          ],
        ),
      ],
    );
  }

  Widget _buildSummaryItem(String label, String value) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          label,
          style: const TextStyle(color: Colors.white70, fontSize: 14),
        ),
        Text(
          value,
          style: const TextStyle(color: Colors.white, fontSize: 18, fontWeight: FontWeight.bold),
        ),
      ],
    );
  }

  Widget _buildNavigationButtons() {
    return Row(
      children: [
        if (ref.read(wizardProvider).currentStep > 0) ...[
          Expanded(
            child: ElevatedButton(
              onPressed: _prevStep,
              style: ElevatedButton.styleFrom(
                backgroundColor: Colors.transparent,
                side: const BorderSide(color: Colors.white54),
                padding: const EdgeInsets.symmetric(vertical: 20),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(15),
                ),
              ),
              child: const Text('Back', style: TextStyle(fontSize: 18, color: Colors.white)),
            ),
          ),
          const SizedBox(width: 15),
        ],
        Expanded(
          child: ElevatedButton(
            onPressed: _nextStep,
            style: ElevatedButton.styleFrom(
              backgroundColor: const Color(0xFF6B4EFF),
              padding: const EdgeInsets.symmetric(vertical: 20),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(15),
              ),
            ),
            child: const Text('Next', style: TextStyle(fontSize: 18, color: Colors.white, fontWeight: FontWeight.bold)),
          ),
        ),
      ],
    );
  }
}
