import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../providers/wizard_provider.dart';
import '../main.dart'; // For GlassContainer
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
  final _intentController = TextEditingController();
  final _adminEmailController = TextEditingController();
  final _adminPasswordController = TextEditingController();

  late AnimationController _pulseAnimationController;
  late Animation<double> _pulseAnimation;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      ref.read(wizardProvider.notifier).setTenantId('tenant-mock-local');
      ref.read(wizardProvider.notifier).loadState('tenant-mock-local');
    });

    _pulseAnimationController = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 1500),
    )..repeat(reverse: true);

    _pulseAnimation = Tween<double>(begin: 1.0, end: 1.05).animate(
      CurvedAnimation(parent: _pulseAnimationController, curve: Curves.easeInOut),
    );
  }

  @override
  void dispose() {
    _intentController.dispose();
    _adminEmailController.dispose();
    _adminPasswordController.dispose();
    _pulseAnimationController.dispose();
    super.dispose();
  }

  void _nextStep() {
    ref.read(wizardProvider.notifier).nextStep();
  }

  void _prevStep() {
    ref.read(wizardProvider.notifier).prevStep();
  }

  void _generateBusiness() async {
    // Save intent
    ref.read(wizardProvider.notifier).setIntent(_intentController.text);

    // Move to generating step
    _nextStep();

    // Simulate AI generation delay
    await Future.delayed(const Duration(seconds: 3));

    // Mock the generated data
    ref.read(wizardProvider.notifier).updateBusinessProfile(
      companyName: "Generated Business",
      industry: "Retail",
      size: "1-10",
    );
    ref.read(wizardProvider.notifier).updateProductDetails(
      name: "Flagship Product",
      description: "AI Generated Description",
      price: "19.99",
    );
    ref.read(wizardProvider.notifier).setDomainChoice("generatedbusiness.ohc.app");

    // Move to review step
    if (mounted) {
       _nextStep();
    }
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(wizardProvider);

    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
        actions: [
          IconButton(
            icon: const Icon(Icons.help_outline, color: Colors.white),
            onPressed: () {
              Navigator.push(context, MaterialPageRoute(builder: (context) => const HelpCenterScreen()));
            },
          ),
        ],
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 600),
          child: Padding(
            padding: const EdgeInsets.all(20),
            child: _buildCurrentStep(state),
          ),
        ),
      ),
    );
  }

  Widget _buildCurrentStep(WizardState state) {
    switch (state.currentStep) {
      case 0:
        return _buildWelcomeScreen();
      case 1:
        return _buildIntakeScreen(state);
      case 2:
        return _buildGeneratingScreen();
      case 3:
        return _buildReviewAndLaunchScreen(state);
      case 4:
        return _buildWelcomeChecklistScreen(state);
      default:
        return _buildWelcomeScreen();
    }
  }

  Widget _buildWelcomeScreen() {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Welcome to OHC',
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
          'Log in or sign up to continue.',
          style: TextStyle(color: Colors.white70, fontSize: 16),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 40),
        GlassContainer(
          child: Column(
            children: [
              TextFormField(
                controller: _adminEmailController,
                key: const Key('signupEmailField'),
                style: const TextStyle(color: Colors.white),
                decoration: const InputDecoration(
                  labelText: 'Email',
                  labelStyle: TextStyle(color: Colors.white54),
                  border: InputBorder.none,
                ),
              ),
              const Divider(color: Colors.white24),
              TextFormField(
                controller: _adminPasswordController,
                key: const Key('signupPasswordField'),
                obscureText: true,
                style: const TextStyle(color: Colors.white),
                decoration: const InputDecoration(
                  labelText: 'Password',
                  labelStyle: TextStyle(color: Colors.white54),
                  border: InputBorder.none,
                ),
              ),
            ],
          ),
        ),
        const SizedBox(height: 30),
        ElevatedButton(
          key: const Key('signupBtn'),
          onPressed: () {
            ref.read(wizardProvider.notifier).updateAdminAccount(
                  email: _adminEmailController.text,
                  password: _adminPasswordController.text,
                  name: 'Admin',
                );
            _nextStep();
          },
          style: ElevatedButton.styleFrom(
            backgroundColor: const Color(0xFF6B4EFF),
            padding: const EdgeInsets.symmetric(vertical: 20),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(15),
            ),
          ),
          child: const Text('Continue', style: TextStyle(fontSize: 18, color: Colors.white, fontWeight: FontWeight.bold)),
        ),
      ],
    );
  }

  Widget _buildIntakeScreen(WizardState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'What do you want to build today?',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 10),
        const Text(
          'Describe your business in a few words, and our AI will build it for you.',
          style: TextStyle(color: Colors.white70, fontSize: 16),
        ),
        const SizedBox(height: 20),
        Expanded(
          child: GlassContainer(
            child: TextFormField(
              controller: _intentController,
              key: const Key('intentField'),
              maxLines: null,
              expands: true,
              style: const TextStyle(color: Colors.white, fontSize: 18),
              decoration: const InputDecoration(
                hintText: 'e.g., I sell vegan cakes in Portland...',
                hintStyle: TextStyle(color: Colors.white38),
                border: InputBorder.none,
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
              child: ElevatedButton(
                key: const Key('generateBtn'),
                onPressed: _generateBusiness,
                style: ElevatedButton.styleFrom(
                  backgroundColor: const Color(0xFF6B4EFF),
                  padding: const EdgeInsets.symmetric(vertical: 20),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(15),
                  ),
                ),
                child: const Text('Generate Business', style: TextStyle(fontSize: 18, color: Colors.white, fontWeight: FontWeight.bold)),
              ),
            ),
          ],
        ),
      ],
    );
  }

  Widget _buildGeneratingScreen() {
    return Center(
      child: GlassContainer(
        child: Padding(
          padding: const EdgeInsets.all(40.0),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: const [
              CircularProgressIndicator(color: Color(0xFF6B4EFF)),
              SizedBox(height: 20),
              Text(
                'Designing storefront...',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 20,
                  color: Colors.white,
                ),
              ),
              SizedBox(height: 10),
              Text(
                'Writing policies...',
                style: TextStyle(color: Colors.white70),
              ),
            ],
          ),
        ),
      ),
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
                  _buildSummaryItem('Intent', state.intent ?? 'Not set'),
                  const SizedBox(height: 10),
                  _buildSummaryItem('Company Name', state.companyName ?? 'Not set'),
                  const SizedBox(height: 10),
                  _buildSummaryItem('Industry', state.industry ?? 'Not set'),
                  const SizedBox(height: 10),
                  _buildSummaryItem('Product', state.productName ?? 'Not set'),
                  const SizedBox(height: 10),
                  _buildSummaryItem('Domain', state.domainChoice ?? 'Not set'),
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
                onPressed: () {
                    // Go back to intake screen
                    ref.read(wizardProvider.notifier).prevStep();
                    ref.read(wizardProvider.notifier).prevStep();
                },
                style: ElevatedButton.styleFrom(
                  backgroundColor: Colors.transparent,
                  side: const BorderSide(color: Colors.white54),
                  padding: const EdgeInsets.symmetric(vertical: 20),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(15),
                  ),
                ),
                child: const Text('Edit', style: TextStyle(fontSize: 18, color: Colors.white)),
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
                  key: const Key('launchAIBtn'),
                  child: const Text('Launch Business', style: TextStyle(fontSize: 18, color: Colors.white, fontWeight: FontWeight.bold)),
                ),
              ),
            ),
          ],
        ),
      ],
    );
  }

  Widget _buildWelcomeChecklistScreen(WizardState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'You\'re set up!',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 10),
        const Text(
          'Here\'s what to do next:',
          style: TextStyle(color: Colors.white70, fontSize: 16),
        ),
        const SizedBox(height: 20),
        Expanded(
          child: SingleChildScrollView(
            child: Column(
              children: [
                _buildChecklistItem('✅ Business live', true),
                const SizedBox(height: 10),
                _buildChecklistItem('⬜ Add 3 more products', false),
                const SizedBox(height: 10),
                _buildChecklistItem('⬜ Connect Instagram', false),
                const SizedBox(height: 10),
                _buildChecklistItem('⬜ Share your link with a friend', false),
              ],
            ),
          ),
        ),
        const SizedBox(height: 20),
        ElevatedButton(
          onPressed: () {
            // Replace with dashboard navigation
            Navigator.pushReplacement(
              context,
              MaterialPageRoute(builder: (context) => const DashboardScreen()),
            );
          },
          style: ElevatedButton.styleFrom(
            backgroundColor: const Color(0xFF6B4EFF),
            padding: const EdgeInsets.symmetric(vertical: 20),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(15),
            ),
          ),
          child: const Text('Go to Dashboard', style: TextStyle(fontSize: 18, color: Colors.white, fontWeight: FontWeight.bold)),
        ),
      ],
    );
  }

  Widget _buildChecklistItem(String text, bool isCompleted) {
    return Container(
      padding: const EdgeInsets.all(15),
      decoration: BoxDecoration(
        color: isCompleted ? Colors.green.withOpacity(0.1) : Colors.white.withOpacity(0.05),
        border: Border.all(color: isCompleted ? Colors.green : Colors.white24),
        borderRadius: BorderRadius.circular(10),
      ),
      child: Row(
        children: [
          Expanded(
            child: Text(
              text,
              style: TextStyle(
                color: isCompleted ? Colors.greenAccent : Colors.white,
                fontSize: 16,
              ),
            ),
          ),
          const Icon(Icons.chevron_right, color: Colors.white54),
        ],
      ),
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
}
