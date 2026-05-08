import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../providers/wizard_provider.dart';
import '../main.dart'; // For GlassContainer

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
  final _domainChoiceController = TextEditingController();

  late AnimationController _heroAnimationController;
  late Animation<double> _heroAnimation;

  late AnimationController _pulseAnimationController;
  late Animation<double> _pulseAnimation;

  bool _isLaunching = false;

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
    _domainChoiceController.dispose();
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

    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 600),
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
        return _buildAdministratorAccountScreen(state);
      case 2:
        return _buildBusinessProfileScreen(state);
      case 3:
        return _buildSellingCategoriesScreen(state);
      case 4:
        return _buildPaymentPreferenceScreen(state);
      case 5:
        return _buildTemplateSelectionScreen(state);
      case 6:
        return _buildFirstProductScreen(state);
      case 7:
        return _buildDomainSelectionScreen(state);
      case 8:
        return _buildReviewAndLaunchScreen(state);
      case 9:
        return _buildWelcomeChecklistScreen(state);
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
          textAlign: TextAlign.center,
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 32,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 10),
        const Text(
          'Your business, live in minutes.\nZero tech skills needed.',
          textAlign: TextAlign.center,
          style: TextStyle(
            fontSize: 16,
            color: Colors.white70,
          ),
        ),
        const SizedBox(height: 50),
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

  Widget _buildAdministratorAccountScreen(WizardState state) {
    if (_adminNameController.text.isEmpty && state.adminName != null) _adminNameController.text = state.adminName!;
    if (_adminEmailController.text.isEmpty && state.adminEmail != null) _adminEmailController.text = state.adminEmail!;
    if (_adminPasswordController.text.isEmpty && state.adminPassword != null) _adminPasswordController.text = state.adminPassword!;

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
        const SizedBox(height: 10),
        const Text(
          'Create your account to save progress across devices.',
          style: TextStyle(color: Colors.white70, fontSize: 16),
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

  Widget _buildBusinessProfileScreen(WizardState state) {
    if (_companyNameController.text.isEmpty && state.companyName != null) _companyNameController.text = state.companyName!;

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
              onChanged: (value) {
                ref.read(wizardProvider.notifier).updateBusinessProfile(industry: value);
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
              hint: const Text('Company Size', style: TextStyle(color: Colors.white70)),
              items: ['1-10', '11-50', '51-200', '201+'].map((String value) {
                return DropdownMenuItem<String>(
                  value: value,
                  child: Text(value),
                );
              }).toList(),
              onChanged: (value) {
                ref.read(wizardProvider.notifier).updateBusinessProfile(size: value);
              },
            ),
          ),
        ),
        const SizedBox(height: 15),
        const Text(
          'What are your goals?',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 18,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 10),
        Expanded(
          child: ListView(
            children: ['Support', 'Build software', 'Marketing', 'Data', 'Custom'].map((goal) {
              final isSelected = state.goals.contains(goal);
              return Padding(
                padding: const EdgeInsets.only(bottom: 8.0),
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
            }).toList(),
          ),
        ),
        _buildNavigationButtons(),
      ],
    );
  }

  Widget _buildSellingCategoriesScreen(WizardState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'What are you selling?',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 10),
        const Text(
          'Select all that apply.',
          style: TextStyle(color: Colors.white70, fontSize: 16),
        ),
        const SizedBox(height: 20),
        Expanded(
          child: ListView(
            children: [
              _buildCategoryCheckbox('Physical Products', state.sellPhysical, () {
                ref.read(wizardProvider.notifier).toggleSellingCategory(physical: !state.sellPhysical);
              }),
              _buildCategoryCheckbox('Digital Products', state.sellDigital, () {
                ref.read(wizardProvider.notifier).toggleSellingCategory(digital: !state.sellDigital);
              }),
              _buildCategoryCheckbox('Services', state.sellServices, () {
                ref.read(wizardProvider.notifier).toggleSellingCategory(services: !state.sellServices);
              }),
              _buildCategoryCheckbox('Food & Beverage', state.sellFood, () {
                ref.read(wizardProvider.notifier).toggleSellingCategory(food: !state.sellFood);
              }),
              _buildCategoryCheckbox('Subscriptions', state.sellSubscriptions, () {
                ref.read(wizardProvider.notifier).toggleSellingCategory(subscriptions: !state.sellSubscriptions);
              }),
            ],
          ),
        ),
        _buildNavigationButtons(),
      ],
    );
  }

  Widget _buildCategoryCheckbox(String label, bool isSelected, VoidCallback onTap) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 10),
      child: GestureDetector(
        onTap: onTap,
        child: GlassContainer(
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text(
                label,
                style: const TextStyle(color: Colors.white, fontSize: 16),
              ),
              Icon(
                isSelected ? Icons.check_box : Icons.check_box_outline_blank,
                color: isSelected ? const Color(0xFF6B4EFF) : Colors.white54,
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildPaymentPreferenceScreen(WizardState state) {
    final options = ['Cloud', 'Desktop', 'Mobile-only', 'online', 'in_person', 'skip'];
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
              final isSelected = state.deploymentPreference == option || state.paymentPreference == option;
              return Padding(
                padding: const EdgeInsets.only(bottom: 10),
                child: GestureDetector(
                  onTap: () {
                    if (['Cloud', 'Desktop', 'Mobile-only'].contains(option)) {
                       ref.read(wizardProvider.notifier).setDeploymentPreference(option);
                    } else {
                       ref.read(wizardProvider.notifier).setPaymentPreference(option);
                    }
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
    final templates = ['Dark Mode', 'Classic', 'Modern', 'Playful'];

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Choose a Template',
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
            itemCount: templates.length,
            itemBuilder: (context, index) {
              final template = templates[index];
              final isSelected = state.websiteTemplate == template;

              return Padding(
                padding: const EdgeInsets.only(bottom: 15),
                child: GestureDetector(
                  onTap: () {
                    ref.read(wizardProvider.notifier).setWebsiteTemplate(template);
                  },
                  child: GlassContainer(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: [
                            Text(
                              template,
                              style: const TextStyle(color: Colors.white, fontSize: 18, fontWeight: FontWeight.bold),
                            ),
                            Icon(
                              isSelected ? Icons.check_circle : Icons.radio_button_unchecked,
                              color: isSelected ? const Color(0xFF22C55E) : Colors.white54,
                            ),
                          ],
                        ),
                        if (isSelected) ...[
                           const SizedBox(height: 15),
                           Container(
                             height: 100,
                             width: double.infinity,
                             decoration: BoxDecoration(
                               color: Colors.white.withOpacity(0.1),
                               borderRadius: BorderRadius.circular(10),
                             ),
                             child: Center(
                               child: Text(
                                 state.companyName ?? 'Your Storefront',
                                 style: const TextStyle(color: Colors.white, fontSize: 24, fontFamily: 'Outfit'),
                               )
                             ),
                           ),
                        ]
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
    if (_productNameController.text.isEmpty && state.productName != null) _productNameController.text = state.productName!;
    if (_productPriceController.text.isEmpty && state.productPrice != null) _productPriceController.text = state.productPrice!;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Add your first product',
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
              ref.read(wizardProvider.notifier).setProductDetails(name: value);
            },
          ),
        ),
        const SizedBox(height: 15),
        GlassContainer(
          child: Row(
            children: [
              const Padding(
                padding: EdgeInsets.only(left: 15.0),
                child: Text('\$', style: TextStyle(color: Colors.white, fontSize: 18)),
              ),
              Expanded(
                child: TextField(
                  key: const Key('productPriceField'),
                  controller: _productPriceController,
                  keyboardType: TextInputType.number,
                  style: const TextStyle(color: Colors.white),
                  decoration: const InputDecoration(
                    labelText: 'Price',
                    labelStyle: TextStyle(color: Colors.white70),
                    border: InputBorder.none,
                    contentPadding: EdgeInsets.only(left: 10),
                  ),
                  onChanged: (value) {
                    ref.read(wizardProvider.notifier).setProductDetails(price: value);
                  },
                ),
              ),
            ],
          ),
        ),
        const SizedBox(height: 15),
        GlassContainer(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text('Photo', style: TextStyle(color: Colors.white70)),
              const SizedBox(height: 10),
              Center(
                child: ElevatedButton.icon(
                  onPressed: () {
                    // Mock photo upload
                  },
                  icon: const Icon(Icons.upload_file),
                  label: const Text('Upload Photo'),
                  style: ElevatedButton.styleFrom(
                    backgroundColor: Colors.white.withOpacity(0.1),
                    foregroundColor: Colors.white,
                  ),
                ),
              ),
            ],
          ),
        ),
        const Spacer(),
        _buildNavigationButtons(),
      ],
    );
  }

  Widget _buildDomainSelectionScreen(WizardState state) {
    if (_domainChoiceController.text.isEmpty && state.domainChoice != null) _domainChoiceController.text = state.domainChoice!;

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
          'Your free ohc.app subdomain',
          style: TextStyle(color: Colors.white70, fontSize: 16),
        ),
        const SizedBox(height: 20),
        GlassContainer(
          child: Row(
            children: [
              Expanded(
                child: TextField(
                  key: const Key('domainField'),
                  controller: _domainChoiceController,
                  style: const TextStyle(color: Colors.white),
                  decoration: const InputDecoration(
                    labelText: 'Subdomain',
                    labelStyle: TextStyle(color: Colors.white70),
                    border: InputBorder.none,
                  ),
                  onChanged: (value) {
                    ref.read(wizardProvider.notifier).setDomainChoice(value);
                  },
                ),
              ),
              const Padding(
                padding: EdgeInsets.only(right: 15.0),
                child: Text('.ohc.app', style: TextStyle(color: Colors.white54, fontSize: 16)),
              ),
            ],
          ),
        ),
        const Spacer(),
        _buildNavigationButtons(),
      ],
    );
  }

  Widget _buildReviewAndLaunchScreen(WizardState state) {
    if (state.launchSuccess) {
       return Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const Text('🎉', style: TextStyle(fontSize: 80)),
          const SizedBox(height: 20),
          const Text(
            'Your business is live!',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 32,
              fontWeight: FontWeight.bold,
              color: Colors.white,
            ),
          ),
          const SizedBox(height: 10),
          SelectableText(
            'https://${state.domainChoice ?? 'mybusiness'}.ohc.app',
            style: const TextStyle(fontSize: 18, color: Color(0xFF6B4EFF), decoration: TextDecoration.underline),
          ),
          const SizedBox(height: 30),
          ElevatedButton(
            onPressed: _nextStep,
            style: ElevatedButton.styleFrom(
              backgroundColor: const Color(0xFF22C55E),
              padding: const EdgeInsets.symmetric(vertical: 20, horizontal: 40),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(15),
              ),
            ),
            child: const Text('View Welcome Checklist', style: TextStyle(fontSize: 18, color: Colors.white, fontWeight: FontWeight.bold)),
          ),
        ],
      );
    }

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
                  _buildSummaryItem('Deployment', state.deploymentPreference ?? state.paymentPreference ?? 'Not set'),
                  const SizedBox(height: 10),
                  _buildSummaryItem('Admin', state.adminName ?? state.adminEmail ?? 'Not set'),
                  const SizedBox(height: 10),
                  _buildSummaryItem('Template', state.websiteTemplate ?? 'Not set'),
                  const SizedBox(height: 10),
                  _buildSummaryItem('Domain', '${state.domainChoice ?? 'mybusiness'}.ohc.app'),
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
                onPressed: _isLaunching ? null : _prevStep,
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
                    scale: _isLaunching ? 1.0 : _pulseAnimation.value,
                    child: child,
                  );
                },
                child: ElevatedButton(
                  onPressed: _isLaunching ? null : () async {
                    setState(() { _isLaunching = true; });
                    await ref.read(wizardProvider.notifier).submitWizard();

                    // Mock copying to clipboard
                    await Clipboard.setData(ClipboardData(text: 'https://${state.domainChoice ?? 'mybusiness'}.ohc.app'));

                    setState(() { _isLaunching = false; });
                  },
                  style: ElevatedButton.styleFrom(
                    backgroundColor: const Color(0xFF6B4EFF),
                    padding: const EdgeInsets.symmetric(vertical: 20),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(15),
                    ),
                  ),
                  child: _isLaunching
                      ? const CircularProgressIndicator(color: Colors.white)
                      : const Text('Launch My AI Team', style: TextStyle(fontSize: 18, color: Colors.white, fontWeight: FontWeight.bold)),
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
          'Welcome Checklist',
          textAlign: TextAlign.center,
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 28,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 10),
        const Text(
          "You're set up! Here's what to do next:",
          textAlign: TextAlign.center,
          style: TextStyle(color: Colors.white70, fontSize: 16),
        ),
        const SizedBox(height: 30),
        Expanded(
          child: SingleChildScrollView(
            child: GlassContainer(
              child: Column(
                children: [
                  _buildChecklistItem('Business live', true),
                  const Divider(color: Colors.white24),
                  _buildChecklistItem('Add 3 more products', false),
                  const Divider(color: Colors.white24),
                  _buildChecklistItem('Connect Instagram', false),
                  const Divider(color: Colors.white24),
                  _buildChecklistItem('Share your link with a friend', false),
                ],
              ),
            ),
          ),
        ),
        const SizedBox(height: 20),
        ElevatedButton(
          onPressed: () {
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

  Widget _buildChecklistItem(String title, bool isDone) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 10),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Icon(
            isDone ? Icons.check_circle : Icons.radio_button_unchecked,
            color: isDone ? const Color(0xFF22C55E) : Colors.white54,
            size: 28,
          ),
          const SizedBox(width: 15),
          Expanded(
            child: Text(
              title,
              style: TextStyle(
                fontSize: 18,
                color: isDone ? Colors.white : Colors.white70,
                decoration: isDone ? TextDecoration.lineThrough : null,
              ),
            ),
          ),
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
