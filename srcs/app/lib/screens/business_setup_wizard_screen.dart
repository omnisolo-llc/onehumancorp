import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:flutter/services.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:confetti/confetti.dart';

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


  late ConfettiController _confettiController;
  bool _showConfetti = false;

  late AnimationController _heroAnimationController;

  late Animation<double> _heroAnimation;

  late AnimationController _pulseAnimationController;
  late Animation<double> _pulseAnimation;

  bool _showWalkthrough = true;
  bool _isGenerating = false;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      // Load saved state if any, in real app this would use the real logged in tenant ID
      ref.read(wizardProvider.notifier).setTenantId('tenant-mock-local');
      ref.read(wizardProvider.notifier).loadState('tenant-mock-local');
    });

    _heroAnimationController = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 2),
    )..repeat(reverse: true);

    _confettiController = ConfettiController(duration: const Duration(seconds: 3));


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
    _confettiController.dispose();
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

    if (state.currentStep == 4) {
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
        return _buildProductScreen(state);
      case 2:
        return _buildReviewAndLaunchScreen(state);
      case 3:
        return _buildWelcomeChecklistScreen(state);
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
        const SizedBox(height: 15),
        GlassContainer(
          child: DropdownButtonHideUnderline(
            child: ContextualTooltip(
              tooltipKey: 'sizeDropdown',
              child: DropdownButton<String>(
                key: const Key('sizeDropdown'),
              value: state.size,
              isExpanded: true,
              dropdownColor: const Color(0xFF1E293B),
              style: const TextStyle(color: Colors.white),
              hint: const Text('Size', style: TextStyle(color: Colors.white70)),
              items: ['1-10', '11-50', '51-200', '201+']
                  .map((String value) {
                return DropdownMenuItem<String>(
                  value: value,
                  child: Text(value),
                );
              }).toList(),
                onChanged: (newValue) {
                  ref.read(wizardProvider.notifier).updateBusinessProfile(size: newValue);
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
    final goals = ['Support', 'Build software', 'Marketing', 'Data', 'Custom'];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'What are your goals?',
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
              final isSelected = state.goals.contains(goal);
              return Padding(
                padding: const EdgeInsets.only(bottom: 10),
                child: GestureDetector(
                  onTap: () {
                    ref.read(wizardProvider.notifier).toggleGoal(goal);
                  },
                  child: GlassContainer(
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Text(
                          goal,
                          style: const TextStyle(color: Colors.white, fontSize: 16),
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

  Widget _buildExternalIntegrationsScreen() {
    if (widget.environmentMode == EnvironmentMode.standaloneDesktop) {
      return Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Text(
            'Local Environment Optimization',
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
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: const [
                    Icon(Icons.speed, size: 60, color: Color(0xFF22C55E)),
                    SizedBox(height: 20),
                    Text(
                      'Bypassing Cloud Dependencies',
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 20,
                        fontWeight: FontWeight.bold,
                        color: Colors.white,
                      ),
                    ),
                    SizedBox(height: 10),
                    Text(
                      'Running in Standalone Desktop mode. Heavy cloud-specific dependencies like Redis and multi-tenant external databases are safely bypassed for local host-machine efficiency.',
                      textAlign: TextAlign.center,
                      style: TextStyle(color: Colors.white70, fontSize: 16),
                    ),
                  ],
                ),
              ),
            ),
          ),
          const SizedBox(height: 20),
          _buildNavigationButtons(),
        ],
      );
    }

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'External Integrations',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 20),
        GlassContainer(
          child: const TextField(
            key: Key('redisUrlField'),
            style: TextStyle(color: Colors.white),
            decoration: InputDecoration(
              labelText: 'Redis URL',
              labelStyle: TextStyle(color: Colors.white70),
              border: InputBorder.none,
            ),
          ),
        ),
        const SizedBox(height: 15),
        GlassContainer(
          child: const TextField(
            key: Key('dbUrlField'),
            style: TextStyle(color: Colors.white),
            decoration: InputDecoration(
              labelText: 'Multi-tenant DB URL',
              labelStyle: TextStyle(color: Colors.white70),
              border: InputBorder.none,
            ),
          ),
        ),
        const Spacer(),
        _buildNavigationButtons(),
      ],
    );
  }

  Widget _buildDeploymentPreferenceScreen(WizardState state) {
    final options = ['Cloud', 'Desktop', 'Mobile-only'];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Deployment Preference',
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
            itemCount: options.length,
            itemBuilder: (context, index) {
              final option = options[index];
              final isSelected = state.deploymentPreference == option;
              return Padding(
                padding: const EdgeInsets.only(bottom: 10),
                child: GestureDetector(
                  onTap: () {
                    ref.read(wizardProvider.notifier).setDeploymentPreference(option);
                  },
                  child: GlassContainer(
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Text(
                          option,
                          style: const TextStyle(color: Colors.white, fontSize: 16),
                        ),
                        Icon(
                          isSelected ? Icons.radio_button_checked : Icons.radio_button_unchecked,
                          color: isSelected ? const Color(0xFF6B4EFF) : Colors.white54,
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

  Widget _buildAdministratorAccountScreen() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Administrator Account',
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
            key: const Key('adminNameField'),
            controller: _adminNameController,
            style: const TextStyle(color: Colors.white),
            decoration: const InputDecoration(
              labelText: 'Name',
              labelStyle: TextStyle(color: Colors.white70),
              border: InputBorder.none,
            ),
            onChanged: (value) {
              ref.read(wizardProvider.notifier).updateAdminAccount(name: value);
            },
          ),
        ),
        const SizedBox(height: 15),
        GlassContainer(
          child: TextField(
            key: const Key('adminEmailField'),
            controller: _adminEmailController,
            style: const TextStyle(color: Colors.white),
            decoration: const InputDecoration(
              labelText: 'Email',
              labelStyle: TextStyle(color: Colors.white70),
              border: InputBorder.none,
            ),
            onChanged: (value) {
              ref.read(wizardProvider.notifier).updateAdminAccount(email: value);
            },
          ),
        ),
        const SizedBox(height: 15),
        GlassContainer(
          child: TextField(
            key: const Key('adminPasswordField'),
            controller: _adminPasswordController,
            obscureText: true,
            style: const TextStyle(color: Colors.white),
            decoration: const InputDecoration(
              labelText: 'Password',
              labelStyle: TextStyle(color: Colors.white70),
              border: InputBorder.none,
            ),
            onChanged: (value) {
              ref.read(wizardProvider.notifier).updateAdminAccount(password: value);
            },
          ),
        ),
        const Spacer(),
        _buildNavigationButtons(),
      ],
    );
  }

  Widget _buildTemplateSelectionScreen(WizardState state) {
    final templates = [
      {'id': 'modern', 'name': 'Modern', 'icon': Icons.web, 'color': Colors.blue},
      {'id': 'cozy', 'name': 'Cozy', 'icon': Icons.coffee, 'color': Colors.orange},
      {'id': 'professional', 'name': 'Professional', 'icon': Icons.business, 'color': Colors.grey},
    ];

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Select a Template',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 10),
        const Text(
          'Choose a starting vibe for your storefront.',
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

  Widget _buildProductScreen(WizardState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Add your first product or service',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 10),
        const Text(
          'You can add more later.',
          style: TextStyle(color: Colors.white70, fontSize: 16),
        ),
        const SizedBox(height: 20),
        Expanded(
          child: SingleChildScrollView(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text('Name', style: TextStyle(color: Colors.white70, fontSize: 14)),
                const SizedBox(height: 5),
                GlassContainer(
                  child: TextFormField(
                    key: const Key('productNameField'),
                    initialValue: state.productName,
                    style: const TextStyle(color: Colors.white),
                    decoration: const InputDecoration(
                      hintText: 'e.g. Custom Birthday Cake',
                      hintStyle: TextStyle(color: Colors.white24),
                      border: InputBorder.none,
                      contentPadding: EdgeInsets.all(15),
                    ),
                    onChanged: (value) {
                      ref.read(wizardProvider.notifier).updateProductDetails(name: value);
                    },
                  ),
                ),
                const SizedBox(height: 15),
                const Text('Description (AI will help!)', style: TextStyle(color: Colors.white70, fontSize: 14)),
                const SizedBox(height: 5),
                GlassContainer(
                  child: TextFormField(
                    key: const Key('productDescField'),
                    initialValue: state.productDescription,
                    style: const TextStyle(color: Colors.white),
                    maxLines: 3,
                    decoration: const InputDecoration(
                      hintText: 'Describe what you are selling...',
                      hintStyle: TextStyle(color: Colors.white24),
                      border: InputBorder.none,
                      contentPadding: EdgeInsets.all(15),
                    ),
                    onChanged: (value) {
                      ref.read(wizardProvider.notifier).updateProductDetails(description: value);
                    },
                  ),
                ),
                const SizedBox(height: 10),
                ElevatedButton(
                  onPressed: _isGenerating ? null : () async {
                    setState(() { _isGenerating = true; });
                    await ref.read(wizardProvider.notifier).autoGenerateProductDescription();
                    if (mounted) {
                      setState(() { _isGenerating = false; });
                    }
                  },
                  style: ElevatedButton.styleFrom(
                    backgroundColor: Colors.white.withOpacity(0.1),
                    padding: const EdgeInsets.symmetric(vertical: 15),
                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
                  ),
                  child: Center(
                    child: _isGenerating
                      ? const SizedBox(height: 20, width: 20, child: CircularProgressIndicator(color: Colors.white, strokeWidth: 2))
                      : const Text('✨ Auto-generate description', style: TextStyle(color: Colors.white))
                  ),
                ),
                const SizedBox(height: 15),
                const Text('Price', style: TextStyle(color: Colors.white70, fontSize: 14)),
                const SizedBox(height: 5),
                GlassContainer(
                  child: TextFormField(
                    key: const Key('productPriceField'),
                    initialValue: state.productPrice,
                    style: const TextStyle(color: Colors.white),
                    keyboardType: TextInputType.number,
                    decoration: InputDecoration(
                      hintText: '0.00',
                      prefixText: '${NumberFormat.simpleCurrency(locale: Localizations.localeOf(context).toString()).currencySymbol} ',
                      prefixStyle: const TextStyle(color: Colors.white),
                      hintStyle: TextStyle(color: Colors.white24),
                      border: InputBorder.none,
                      contentPadding: EdgeInsets.all(15),
                    ),
                    onChanged: (value) {
                      ref.read(wizardProvider.notifier).updateProductDetails(price: value);
                    },
                  ),
                ),
                const SizedBox(height: 20),
                Container(
                  height: 120,
                  decoration: BoxDecoration(
                    color: Colors.white.withOpacity(0.05),
                    borderRadius: BorderRadius.circular(15),
                    border: Border.all(color: Colors.white.withOpacity(0.1)),
                  ),
                  child: const Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(Icons.camera_alt, color: Colors.white54, size: 40),
                        SizedBox(height: 10),
                        Text('Tap to select an image', style: TextStyle(color: Colors.white54)),
                      ],
                    ),
                  ),
                ),
              ],
            ),
          ),
        ),
        const SizedBox(height: 20),
        _buildNavigationButtons(),
      ],
    );
  }

  Widget _buildDomainScreen(WizardState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Choose a Domain',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 10),
        const Text(
          'This is how customers will find you online.',
          style: TextStyle(color: Colors.white70, fontSize: 16),
        ),
        const SizedBox(height: 20),
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text('Your free OHC domain', style: TextStyle(color: Colors.white70, fontSize: 14)),
              const SizedBox(height: 5),
              GlassContainer(
                child: TextFormField(
                  key: const Key('domainField'),
                  initialValue: state.domainChoice ?? '\${(state.companyName ?? "mybusiness").replaceAll(" ", "").toLowerCase()}.ohc.app',
                  style: const TextStyle(color: Colors.white),
                  decoration: const InputDecoration(
                    border: InputBorder.none,
                    contentPadding: EdgeInsets.all(15),
                  ),
                  onChanged: (value) {
                    ref.read(wizardProvider.notifier).setDomainChoice(value);
                  },
                ),
              ),
            ],
          ),
        ),
        const SizedBox(height: 20),
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
                  _buildSummaryItem('Size', state.size ?? 'Not set'),
                  const SizedBox(height: 10),
                  _buildSummaryItem('Goals', state.goals.isEmpty ? 'None' : state.goals.join(', ')),
                  const SizedBox(height: 10),
                  _buildSummaryItem('Deployment', state.deploymentPreference ?? 'Not set'),
                  const SizedBox(height: 10),
                  _buildSummaryItem('Admin', state.adminName ?? 'Not set'),
                  const SizedBox(height: 10),
                  _buildSummaryItem('Template', state.templateSelection ?? 'Not set'),
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
                    setState(() {
                      _showConfetti = true;
                    });
                    _confettiController.play();

                    if (state.domainChoice != null) {
                      final url = 'https://${state.domainChoice}';
                      await Clipboard.setData(ClipboardData(text: url));
                      if (mounted) {
                        ScaffoldMessenger.of(context).showSnackBar(
                          const SnackBar(content: Text('Shareable link copied to clipboard!')),
                        );
                      }
                    }

                    await Future.delayed(const Duration(seconds: 2));
                    await ref.read(wizardProvider.notifier).submitWizard();
                  },
                  style: ElevatedButton.styleFrom(

                    backgroundColor: const Color(0xFF22C55E),
                    padding: const EdgeInsets.symmetric(vertical: 20),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(15),
                    ),
                  ),
                  key: const Key('launchAIBtn'), child: const Text('Launch My Business →', style: TextStyle(fontSize: 18, color: Colors.white, fontWeight: FontWeight.bold)),
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
        if (_showConfetti)
          const Text(
            'CONFETTI Success',
            style: TextStyle(
              fontSize: 20,
              color: Colors.green,
              fontWeight: FontWeight.bold,
            ),
            textAlign: TextAlign.center,
          ),
        const SizedBox(height: 10),
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
            ref.read(wizardProvider.notifier).nextStep();
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
// Validated feature
