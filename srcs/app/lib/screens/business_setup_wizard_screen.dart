import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../providers/wizard_provider.dart';
import '../main.dart';

class BusinessSetupWizardScreen extends ConsumerStatefulWidget {
  const BusinessSetupWizardScreen({super.key});

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

  late AnimationController _heroAnimationController;
  late Animation<double> _heroAnimation;

  late AnimationController _pulseAnimationController;
  late Animation<double> _pulseAnimation;

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
    _productNameController.dispose();
    _productPriceController.dispose();
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

    if (state.currentStep == 9) {
      return const DashboardScreen();
    }

    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 400),
          child: Padding(
            padding: const EdgeInsets.all(20),
            child: _buildCurrentStep(state.currentStep, state),
          ),
        ),
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
        return _buildDeploymentPreferenceScreen(state);
      case 4:
        return _buildAdministratorAccountScreen();
      case 5:
        return _buildTemplateSelectionScreen(state);
      case 6:
        return _buildAddProductScreen(state);
      case 7:
        return _buildDomainSelectionScreen(state);
      case 8:
        return _buildReviewAndLaunchScreen(state);
      default:
        return const SizedBox.shrink();
    }
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
        ElevatedButton(
          onPressed: _nextStep,
          style: ElevatedButton.styleFrom(
            backgroundColor: const Color(0xFF6B4EFF),
            padding: const EdgeInsets.symmetric(vertical: 20),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(15),
            ),
          ),
          child: const Text('Get Started', style: TextStyle(fontSize: 18, color: Colors.white, fontWeight: FontWeight.bold)),
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
        const SizedBox(height: 15),
        GlassContainer(
          child: DropdownButtonHideUnderline(
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


  Widget _buildTemplateSelectionScreen(WizardState state) {
    final templates = ['Modern', 'Classic', 'Playful'];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Expanded(
          child: SingleChildScrollView(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                const Text(
                  'Template Selection',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 28,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                ),
                const SizedBox(height: 20),
                ...templates.map((template) => Padding(
                  padding: const EdgeInsets.only(bottom: 10),
                  child: GlassContainer(
                    child: InkWell(
                      onTap: () {
                        ref.read(wizardProvider.notifier).updateWebsiteTemplate(template);
                      },
                      child: Padding(
                        padding: const EdgeInsets.all(15),
                        child: Row(
                          children: [
                            Icon(
                              state.websiteTemplate == template ? Icons.radio_button_checked : Icons.radio_button_unchecked,
                              color: state.websiteTemplate == template ? const Color(0xFF6B4EFF) : Colors.white54,
                            ),
                            const SizedBox(width: 15),
                            Text(template, style: const TextStyle(color: Colors.white, fontSize: 16)),
                          ],
                        ),
                      ),
                    ),
                  ),
                )).toList(),
              ],
            ),
          ),
        ),
        const SizedBox(height: 20),
        _buildNavigationButtons(),
      ],
    );
  }

  Widget _buildAddProductScreen(WizardState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Expanded(
          child: SingleChildScrollView(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                const Text(
                  'Add a Product',
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
                    key: const Key('productNameField'),
                    controller: _productNameController,
                    style: const TextStyle(color: Colors.white),
                    decoration: const InputDecoration(
                      labelText: 'Product Name',
                      labelStyle: TextStyle(color: Colors.white70),
                      border: InputBorder.none,
                    ),
                    onChanged: (value) {
                      ref.read(wizardProvider.notifier).updateProduct(name: value);
                    },
                  ),
                ),
                const SizedBox(height: 15),
                GlassContainer(
                  child: TextField(
                    key: const Key('productPriceField'),
                    controller: _productPriceController,
                    style: const TextStyle(color: Colors.white),
                    decoration: const InputDecoration(
                      labelText: 'Price',
                      labelStyle: TextStyle(color: Colors.white70),
                      border: InputBorder.none,
                    ),
                    onChanged: (value) {
                      ref.read(wizardProvider.notifier).updateProduct(price: value);
                    },
                  ),
                ),
                const SizedBox(height: 15),
                ElevatedButton.icon(
                  onPressed: () {
                    ref.read(wizardProvider.notifier).updateProduct(
                      description: 'An AI-generated description for ${state.productName}.'
                    );
                  },
                  icon: const Icon(Icons.auto_awesome),
                  label: const Text('Auto-generate description'),
                  style: ElevatedButton.styleFrom(
                    backgroundColor: const Color(0xFF6B4EFF),
                  ),
                ),
                if (state.productDescription.isNotEmpty) ...[
                  const SizedBox(height: 15),
                  GlassContainer(
                    child: Padding(
                      padding: const EdgeInsets.all(15.0),
                      child: Text(
                        state.productDescription,
                        style: const TextStyle(color: Colors.white70),
                      ),
                    ),
                  ),
                ]
              ],
            ),
          ),
        ),
        const SizedBox(height: 20),
        _buildNavigationButtons(),
      ],
    );
  }

  Widget _buildDomainSelectionScreen(WizardState state) {
    final domains = ['🌐 Free OHC Domain', '🔗 Connect Custom Domain'];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Expanded(
          child: SingleChildScrollView(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                const Text(
                  'Domain Auto-assignment',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 28,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                ),
                const SizedBox(height: 20),
                ...domains.map((domain) => Padding(
                  padding: const EdgeInsets.only(bottom: 10),
                  child: GlassContainer(
                    child: InkWell(
                      onTap: () {
                        ref.read(wizardProvider.notifier).updateDomainChoice(domain.contains('Free') ? 'subdomain' : 'custom');
                      },
                      child: Padding(
                        padding: const EdgeInsets.all(15),
                        child: Row(
                          children: [
                            Icon(
                              (state.domainChoice == 'subdomain' && domain.contains('Free')) || (state.domainChoice == 'custom' && domain.contains('Custom'))
                                  ? Icons.radio_button_checked : Icons.radio_button_unchecked,
                              color: (state.domainChoice == 'subdomain' && domain.contains('Free')) || (state.domainChoice == 'custom' && domain.contains('Custom'))
                                  ? const Color(0xFF6B4EFF) : Colors.white54,
                            ),
                            const SizedBox(width: 15),
                            Expanded(child: Text(domain, style: const TextStyle(color: Colors.white, fontSize: 16))),
                          ],
                        ),
                      ),
                    ),
                  ),
                )).toList(),
              ],
            ),
          ),
        ),
        const SizedBox(height: 20),
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
                    // Show confetti
                    ScaffoldMessenger.of(context).showSnackBar(
                      const SnackBar(content: Text('🎉 Your business is live! Link copied to clipboard.')),
                    );
                    await ref.read(wizardProvider.notifier).submitWizard();
                    if (context.mounted) {
                      Navigator.of(context).pushReplacement(
                        MaterialPageRoute(builder: (context) => const DashboardScreen()),
                      );
                    }
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
