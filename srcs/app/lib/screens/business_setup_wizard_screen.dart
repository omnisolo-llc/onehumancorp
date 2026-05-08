import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../providers/wizard_provider.dart';
import '../main.dart'; // For GlassContainer
import 'package:confetti/confetti.dart';
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
  final _productNameController = TextEditingController();
  final _productPriceController = TextEditingController();
  final _productDescController = TextEditingController();

  late ConfettiController _confettiController;
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
    _confettiController = ConfettiController(duration: const Duration(seconds: 3));
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
    _productNameController.dispose();
    _productPriceController.dispose();
    _productDescController.dispose();
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

    if (state.isLoading) {
      return const Scaffold(
        backgroundColor: Color(0xFF0F172A),
        body: Center(child: CircularProgressIndicator()),
      );
    }

    if (state.currentStep == 10) {
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
        return _buildTemplateSelectionScreen(state);
      case 4:
        return _buildFirstProductScreen(state);
      case 5:
        return _buildExternalIntegrationsScreen();
      case 6:
        return _buildDeploymentPreferenceScreen(state);
      case 7:
        return _buildAdministratorAccountScreen();
      case 8:
        return _buildDomainGoLiveScreen(state);
      case 9:
        return _buildReviewAndLaunchScreen(state);
      default:
        return const SizedBox.shrink();
    }
  }

  Widget _buildTemplateSelectionScreen(WizardState state) {
    final templates = [
      {'id': 'modern', 'name': 'Modern Minimal'},
      {'id': 'bold', 'name': 'Bold & Playful'},
      {'id': 'classic', 'name': 'Classic Elegant'}
    ];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Choose a Vibe',
          style: TextStyle(fontFamily: 'Outfit', fontSize: 28, fontWeight: FontWeight.bold, color: Colors.white),
        ),
        const SizedBox(height: 10),
        const Text(
          'Select a starting template for your storefront.',
          style: TextStyle(color: Colors.white70),
        ),
        const SizedBox(height: 20),
        Expanded(
          child: ListView.builder(
            itemCount: templates.length,
            itemBuilder: (context, index) {
              final template = templates[index];
              final isSelected = state.selectedTemplate == template['id'];
              return Padding(
                padding: const EdgeInsets.only(bottom: 10),
                child: GestureDetector(
                  onTap: () {
                    ref.read(wizardProvider.notifier).setTemplate(template['id']!);
                  },
                  child: GlassContainer(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: [
                            Text(template['name']!, style: const TextStyle(color: Colors.white, fontSize: 16)),
                            Icon(isSelected ? Icons.check_circle : Icons.radio_button_unchecked, color: isSelected ? const Color(0xFF22C55E) : Colors.white54),
                          ],
                        ),
                        const SizedBox(height: 10),
                        Container(
                          height: 80,
                          decoration: BoxDecoration(
                            color: Colors.white10,
                            borderRadius: BorderRadius.circular(8),
                          ),
                          child: Center(child: Text('Mini-preview for ${state.companyName ?? 'your business'}', style: const TextStyle(color: Colors.white54, fontSize: 12))),
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

  Widget _buildFirstProductScreen(WizardState state) {
    if (_productNameController.text.isEmpty && state.productName != null) {
      _productNameController.text = state.productName!;
    }
    if (_productPriceController.text.isEmpty && state.productPrice != null) {
      _productPriceController.text = state.productPrice!;
    }
    if (_productDescController.text.isEmpty && state.productDescription != null && state.productDescription != "Generating...") {
      _productDescController.text = state.productDescription!;
    } else if (state.productDescription == "Generating...") {
      _productDescController.text = state.productDescription!;
    }

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Add First Product',
          style: TextStyle(fontFamily: 'Outfit', fontSize: 28, fontWeight: FontWeight.bold, color: Colors.white),
        ),
        const SizedBox(height: 10),
        Expanded(
          child: SingleChildScrollView(
            child: Column(
              children: [
                GlassContainer(
                  child: TextField(
                    key: const Key('productNameField'),
                    controller: _productNameController,
                    style: const TextStyle(color: Colors.white),
                    decoration: const InputDecoration(labelText: 'Product/Service Name', labelStyle: TextStyle(color: Colors.white70), border: InputBorder.none),
                    onChanged: (value) {
                      ref.read(wizardProvider.notifier).updateProduct(name: value);
                    },
                  ),
                ),
                const SizedBox(height: 10),
                GlassContainer(
                  child: TextField(
                    key: const Key('productPriceField'),
                    controller: _productPriceController,
                    keyboardType: TextInputType.number,
                    style: const TextStyle(color: Colors.white),
                    decoration: const InputDecoration(labelText: 'Price (\$)', labelStyle: TextStyle(color: Colors.white70), border: InputBorder.none),
                    onChanged: (value) {
                      ref.read(wizardProvider.notifier).updateProduct(price: value);
                    },
                  ),
                ),
                const SizedBox(height: 10),
                Row(
                  children: [
                    Expanded(
                      child: ElevatedButton.icon(
                        onPressed: () {
                          ref.read(wizardProvider.notifier).generateAiDescription();
                        },
                        icon: const Icon(Icons.auto_awesome, color: Colors.white, size: 16),
                        label: const Text('AI Write', style: TextStyle(color: Colors.white)),
                        style: ElevatedButton.styleFrom(backgroundColor: const Color(0xFF6B4EFF).withOpacity(0.5)),
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 10),
                GlassContainer(
                  child: TextField(
                    key: const Key('productDescField'),
                    controller: _productDescController,
                    maxLines: 2,
                    style: const TextStyle(color: Colors.white),
                    decoration: const InputDecoration(labelText: 'Description', labelStyle: TextStyle(color: Colors.white70), border: InputBorder.none),
                    onChanged: (value) {
                      ref.read(wizardProvider.notifier).updateProduct(description: value);
                    },
                  ),
                ),
              ],
            ),
          ),
        ),
        _buildNavigationButtons(),
      ],
    );
  }

  Widget _buildDomainGoLiveScreen(WizardState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Go Live!',
          style: TextStyle(fontFamily: 'Outfit', fontSize: 28, fontWeight: FontWeight.bold, color: Colors.white),
        ),
        const SizedBox(height: 20),
        GlassContainer(
          child: Column(
            children: [
              const Text('Your store is ready to publish at:', style: TextStyle(color: Colors.white70)),
              const SizedBox(height: 10),
              Text('https://${state.subdomain ?? 'mybusiness.ohc.app'}', style: const TextStyle(color: Colors.white, fontSize: 18, fontWeight: FontWeight.bold)),
              const SizedBox(height: 20),
              Stack(
                alignment: Alignment.center,
                children: [
                  ElevatedButton(
                    onPressed: () {
                      _confettiController.play();
                    },
                    style: ElevatedButton.styleFrom(backgroundColor: const Color(0xFF22C55E), padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 15)),
                    child: const Text('Publish Store', style: TextStyle(color: Colors.white, fontSize: 16, fontWeight: FontWeight.bold)),
                  ),
                  ConfettiWidget(
                    confettiController: _confettiController,
                    blastDirectionality: BlastDirectionality.explosive,
                    shouldLoop: false,
                    colors: const [Colors.green, Colors.blue, Colors.pink, Colors.orange, Colors.purple],
                  ),
                ],
              ),
            ],
          ),
        ),
        const Spacer(),
        _buildNavigationButtons(),
      ],
    );
  }

  Widget _buildWelcomeScreen() {
    return Column(
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
          'Launch and run your small business from anywhere. Let\'s get your AI team configured.',
          style: TextStyle(
            fontFamily: 'Inter',
            fontSize: 16,
            color: Colors.white70,
          ),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 40),
        WalkthroughHighlight(
          showHighlight: _showWalkthrough,
          speechBubbleText: "Start setting up your store here!",
          onDismiss: () => setState(() => _showWalkthrough = false),
          child: ElevatedButton(
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
            child: const Text('Get Started', style: TextStyle(fontSize: 18, color: Colors.white, fontWeight: FontWeight.bold)),
          ),
        ),
      ],
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
