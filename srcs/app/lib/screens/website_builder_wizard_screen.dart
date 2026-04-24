import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';

// State definition for the wizard
class WebsiteBuilderState {
  final int currentStep;
  final String? selectedTemplate;
  final String? selectedColorPalette;
  final bool aiLogoRequested;
  final String? productName;
  final String? productPrice;
  final String? productDescription;
  final String? domainType; // 'subdomain', 'own', 'buy'
  final bool isPublishing;

  const WebsiteBuilderState({
    this.currentStep = 0,
    this.selectedTemplate,
    this.selectedColorPalette,
    this.aiLogoRequested = false,
    this.productName,
    this.productPrice,
    this.productDescription,
    this.domainType,
    this.isPublishing = false,
  });

  WebsiteBuilderState copyWith({
    int? currentStep,
    String? selectedTemplate,
    String? selectedColorPalette,
    bool? aiLogoRequested,
    String? productName,
    String? productPrice,
    String? productDescription,
    String? domainType,
    bool? isPublishing,
  }) {
    return WebsiteBuilderState(
      currentStep: currentStep ?? this.currentStep,
      selectedTemplate: selectedTemplate ?? this.selectedTemplate,
      selectedColorPalette: selectedColorPalette ?? this.selectedColorPalette,
      aiLogoRequested: aiLogoRequested ?? this.aiLogoRequested,
      productName: productName ?? this.productName,
      productPrice: productPrice ?? this.productPrice,
      productDescription: productDescription ?? this.productDescription,
      domainType: domainType ?? this.domainType,
      isPublishing: isPublishing ?? this.isPublishing,
    );
  }
}

class WebsiteBuilderNotifier extends StateNotifier<WebsiteBuilderState> {
  WebsiteBuilderNotifier() : super(const WebsiteBuilderState());

  void nextStep() => state = state.copyWith(currentStep: state.currentStep + 1);
  void previousStep() {
    if (state.currentStep > 0) {
      state = state.copyWith(currentStep: state.currentStep - 1);
    }
  }

  void selectTemplate(String templateId) =>
      state = state.copyWith(selectedTemplate: templateId);
  void selectColorPalette(String paletteId) =>
      state = state.copyWith(selectedColorPalette: paletteId);
  void toggleAiLogo(bool requested) =>
      state = state.copyWith(aiLogoRequested: requested);
  void updateProduct(String name, String price, String description) =>
      state = state.copyWith(
          productName: name, productPrice: price, productDescription: description);
  void selectDomainType(String type) =>
      state = state.copyWith(domainType: type);

  Future<void> publish(BuildContext context) async {
    state = state.copyWith(isPublishing: true);
    // Simulate API call to publish
    await Future.delayed(const Duration(seconds: 2));
    if (!mounted) return;
    state = state.copyWith(isPublishing: false);
    // Show success toast and go to dashboard
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Your website is now live! Link copied to clipboard.', style: TextStyle(fontFamily: 'Inter'))),
    );
    context.go('/dashboard');
  }
}

final websiteBuilderProvider =
    StateNotifierProvider<WebsiteBuilderNotifier, WebsiteBuilderState>(
        (ref) => WebsiteBuilderNotifier());

class WebsiteBuilderWizardScreen extends ConsumerWidget {
  const WebsiteBuilderWizardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(websiteBuilderProvider);
    final notifier = ref.read(websiteBuilderProvider.notifier);
    final theme = Theme.of(context);

    // OHC Premium Design Standards:
    // Background: Dark gradient, subtle glowing blobs or just a nice soft background
    // Container: Glassmorphism

    return Scaffold(
      backgroundColor: theme.colorScheme.surface,
      appBar: AppBar(
        title: const Text('Website Builder', style: TextStyle(fontFamily: 'Outfit')),
        leading: state.currentStep > 0
            ? IconButton(
                icon: const Icon(Icons.arrow_back),
                onPressed: notifier.previousStep,
              )
            : IconButton(
                icon: const Icon(Icons.close),
                onPressed: () => context.go('/dashboard'),
              ),
      ),
      body: SafeArea(
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 414), // Mobile-first sizing
            child: Padding(
              padding: const EdgeInsets.all(24.0),
              child: GlassCard(
                child: Padding(
                  padding: const EdgeInsets.all(24.0),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      // Progress Indicator
                      LinearProgressIndicator(
                        value: (state.currentStep + 1) / 5,
                        backgroundColor: theme.colorScheme.surfaceContainerHighest,
                        color: theme.colorScheme.primary,
                      ),
                      const SizedBox(height: 24),
                      Expanded(
                        child: AnimatedSwitcher(
                          duration: const Duration(milliseconds: 300),
                          switchInCurve: Curves.easeOutQuart,
                          switchOutCurve: Curves.easeInQuart,
                          child: _buildStepContent(state, notifier, context),
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildStepContent(WebsiteBuilderState state, WebsiteBuilderNotifier notifier, BuildContext context) {
    switch (state.currentStep) {
      case 0:
        return _TemplateGalleryStep(state: state, notifier: notifier);
      case 1:
        return _BrandColorsLogoStep(state: state, notifier: notifier);
      case 2:
        return _AddProductStep(state: state, notifier: notifier);
      case 3:
        return _ConnectDomainStep(state: state, notifier: notifier);
      case 4:
        return _GoLiveStep(state: state, notifier: notifier);
      default:
        return const SizedBox.shrink();
    }
  }
}

// Step 0: Template Gallery
class _TemplateGalleryStep extends StatelessWidget {
  final WebsiteBuilderState state;
  final WebsiteBuilderNotifier notifier;

  const _TemplateGalleryStep({required this.state, required this.notifier});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Column(
      key: const ValueKey('step0'),
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text('Choose a Template', style: theme.textTheme.headlineMedium?.copyWith(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        const SizedBox(height: 16),
        Text('Select a starting point for your storefront.', style: theme.textTheme.bodyLarge?.copyWith(fontFamily: 'Inter')),
        const SizedBox(height: 24),
        Expanded(
          child: GridView.count(
            crossAxisCount: 2,
            crossAxisSpacing: 16,
            mainAxisSpacing: 16,
            childAspectRatio: 0.7,
            children: [
              _TemplateCard(
                id: 'minimal',
                name: 'Minimal',
                isSelected: state.selectedTemplate == 'minimal',
                onTap: () => notifier.selectTemplate('minimal'),
              ),
              _TemplateCard(
                id: 'bold',
                name: 'Bold',
                isSelected: state.selectedTemplate == 'bold',
                onTap: () => notifier.selectTemplate('bold'),
              ),
              _TemplateCard(
                id: 'elegant',
                name: 'Elegant',
                isSelected: state.selectedTemplate == 'elegant',
                onTap: () => notifier.selectTemplate('elegant'),
              ),
              _TemplateCard(
                id: 'playful',
                name: 'Playful',
                isSelected: state.selectedTemplate == 'playful',
                onTap: () => notifier.selectTemplate('playful'),
              ),
            ],
          ),
        ),
        const SizedBox(height: 24),
        ElevatedButton(
          onPressed: state.selectedTemplate != null ? notifier.nextStep : null,
          style: ElevatedButton.styleFrom(
            minimumSize: const Size.fromHeight(56),
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
          ),
          child: const Text('Use this template →', style: TextStyle(fontFamily: 'Outfit', fontSize: 18)),
        ),
      ],
    );
  }
}

class _TemplateCard extends StatelessWidget {
  final String id;
  final String name;
  final bool isSelected;
  final VoidCallback onTap;

  const _TemplateCard({
    required this.id,
    required this.name,
    required this.isSelected,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return GestureDetector(
      onTap: onTap,
      child: Container(
        decoration: BoxDecoration(
          border: Border.all(
            color: isSelected ? theme.colorScheme.primary : theme.colorScheme.outline.withValues(alpha: 0.3),
            width: isSelected ? 2 : 1,
          ),
          borderRadius: BorderRadius.circular(12),
          color: isSelected ? theme.colorScheme.primary.withValues(alpha: 0.1) : Colors.transparent,
        ),
        child: Column(
          children: [
            Expanded(
              child: Container(
                margin: const EdgeInsets.all(8),
                decoration: BoxDecoration(
                  color: theme.colorScheme.surfaceContainerHighest,
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Center(child: Icon(Icons.web, size: 48, color: theme.colorScheme.onSurfaceVariant)),
              ),
            ),
            Padding(
              padding: const EdgeInsets.all(8.0),
              child: Text(name, style: const TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
            ),
          ],
        ),
      ),
    );
  }
}

// Step 1: Brand Colors & Logo
class _BrandColorsLogoStep extends StatelessWidget {
  final WebsiteBuilderState state;
  final WebsiteBuilderNotifier notifier;

  const _BrandColorsLogoStep({required this.state, required this.notifier});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Column(
      key: const ValueKey('step1'),
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text('Brand Identity', style: theme.textTheme.headlineMedium?.copyWith(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        const SizedBox(height: 16),
        Text('Pick a color palette and logo.', style: theme.textTheme.bodyLarge?.copyWith(fontFamily: 'Inter')),
        const SizedBox(height: 24),
        Text('Color Palette', style: theme.textTheme.titleMedium?.copyWith(fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceAround,
          children: [
            _PaletteSelector(
              id: 'palette1',
              colors: const [Colors.blue, Colors.lightBlueAccent],
              isSelected: state.selectedColorPalette == 'palette1',
              onTap: () => notifier.selectColorPalette('palette1'),
            ),
            _PaletteSelector(
              id: 'palette2',
              colors: const [Colors.green, Colors.lightGreen],
              isSelected: state.selectedColorPalette == 'palette2',
              onTap: () => notifier.selectColorPalette('palette2'),
            ),
            _PaletteSelector(
              id: 'palette3',
              colors: const [Colors.purple, Colors.deepPurpleAccent],
              isSelected: state.selectedColorPalette == 'palette3',
              onTap: () => notifier.selectColorPalette('palette3'),
            ),
          ],
        ),
        const SizedBox(height: 32),
        Text('Logo', style: theme.textTheme.titleMedium?.copyWith(fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        SwitchListTile(
          title: const Text('Generate a logo for me (AI)', style: TextStyle(fontFamily: 'Inter')),
          value: state.aiLogoRequested,
          onChanged: (val) {
             notifier.toggleAiLogo(val);
             if (val) {
                // Display 3 mock AI options logic would exist here
                ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Generating 3 logo options...')));
             }
          },
          activeColor: theme.colorScheme.primary,
        ),
        if (!state.aiLogoRequested)
          Padding(
            padding: const EdgeInsets.only(top: 16.0),
            child: InkWell(
              onTap: () {
                 // Logo upload logic would go here
                 ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Logo uploaded.')));
              },
              child: Container(
                height: 100,
                decoration: BoxDecoration(
                  border: Border.all(color: theme.colorScheme.outline),
                  borderRadius: BorderRadius.circular(12),
                  color: theme.colorScheme.surfaceContainerHighest,
                ),
                child: Center(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(Icons.upload_file, color: theme.colorScheme.primary),
                      const SizedBox(height: 8),
                      const Text('Upload Logo', style: TextStyle(fontFamily: 'Inter')),
                    ],
                  ),
                ),
              ),
            ),
          ),
        const Spacer(),
        ElevatedButton(
          onPressed: state.selectedColorPalette != null ? notifier.nextStep : null,
          style: ElevatedButton.styleFrom(
            minimumSize: const Size.fromHeight(56),
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
          ),
          child: const Text('Next Step →', style: TextStyle(fontFamily: 'Outfit', fontSize: 18)),
        ),
      ],
    );
  }
}

class _PaletteSelector extends StatelessWidget {
  final String id;
  final List<Color> colors;
  final bool isSelected;
  final VoidCallback onTap;

  const _PaletteSelector({
    required this.id,
    required this.colors,
    required this.isSelected,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return GestureDetector(
      onTap: onTap,
      child: Container(
        width: 60,
        height: 60,
        decoration: BoxDecoration(
          shape: BoxShape.circle,
          border: Border.all(
            color: isSelected ? theme.colorScheme.primary : Colors.transparent,
            width: 3,
          ),
          gradient: LinearGradient(colors: colors),
        ),
        child: isSelected
            ? const Icon(Icons.check, color: Colors.white)
            : null,
      ),
    );
  }
}

// Step 2: Add Product/Service
class _AddProductStep extends StatefulWidget {
  final WebsiteBuilderState state;
  final WebsiteBuilderNotifier notifier;

  const _AddProductStep({required this.state, required this.notifier});

  @override
  State<_AddProductStep> createState() => _AddProductStepState();
}

class _AddProductStepState extends State<_AddProductStep> {
  late TextEditingController _nameController;
  late TextEditingController _priceController;
  late TextEditingController _descController;
  bool _isGeneratingDesc = false;

  @override
  void initState() {
    super.initState();
    _nameController = TextEditingController(text: widget.state.productName);
    _priceController = TextEditingController(text: widget.state.productPrice);
    _descController = TextEditingController(text: widget.state.productDescription);
  }

  @override
  void dispose() {
    _nameController.dispose();
    _priceController.dispose();
    _descController.dispose();
    super.dispose();
  }

  void _updateState() {
    widget.notifier.updateProduct(_nameController.text, _priceController.text, _descController.text);
  }

  Future<void> _generateAiDescription() async {
    if (_nameController.text.isEmpty) return;
    setState(() => _isGeneratingDesc = true);
    // Simulate AI generation
    await Future.delayed(const Duration(seconds: 1));
    if (!mounted) return;
    setState(() {
      _isGeneratingDesc = false;
      _descController.text = 'Premium ${_nameController.text} crafted with care and designed to delight. Perfect for any occasion.';
    });
    _updateState();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final isFormValid = _nameController.text.isNotEmpty && _priceController.text.isNotEmpty;

    return SingleChildScrollView(
      key: const ValueKey('step2'),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('First Product', style: theme.textTheme.headlineMedium?.copyWith(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
          const SizedBox(height: 16),
          Text('Add something to sell on your new site.', style: theme.textTheme.bodyLarge?.copyWith(fontFamily: 'Inter')),
          const SizedBox(height: 24),
          TextField(
            controller: _nameController,
            decoration: const InputDecoration(labelText: 'Product Name', border: OutlineInputBorder()),
            onChanged: (_) => _updateState(),
          ),
          const SizedBox(height: 16),
          // Placeholder for photo upload
          InkWell(
            onTap: () {
               // Photo upload logic would go here
               ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Photo uploaded.')));
            },
            child: Container(
              height: 100,
              decoration: BoxDecoration(
                border: Border.all(color: theme.colorScheme.outline),
                borderRadius: BorderRadius.circular(12),
                color: theme.colorScheme.surfaceContainerHighest,
              ),
              child: Center(
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Icon(Icons.add_a_photo, color: theme.colorScheme.primary),
                    const SizedBox(height: 8),
                    const Text('Add Photo (Camera/Upload)', style: TextStyle(fontFamily: 'Inter')),
                  ],
                ),
              ),
            ),
          ),
          const SizedBox(height: 16),
          TextField(
            controller: _priceController,
            keyboardType: const TextInputType.numberWithOptions(decimal: true),
            decoration: const InputDecoration(labelText: 'Price', prefixText: '\$ ', border: OutlineInputBorder()),
            onChanged: (_) => _updateState(),
          ),
          const SizedBox(height: 16),
          Row(
            children: [
              const Text('Description', style: TextStyle(fontFamily: 'Outfit')),
              const Spacer(),
              TextButton.icon(
                icon: _isGeneratingDesc ? const SizedBox(width: 16, height: 16, child: CircularProgressIndicator(strokeWidth: 2)) : const Icon(Icons.auto_awesome, size: 16),
                label: const Text('AI Write', style: TextStyle(fontFamily: 'Inter')),
                onPressed: _nameController.text.isNotEmpty && !_isGeneratingDesc ? _generateAiDescription : null,
              ),
            ],
          ),
          TextField(
            controller: _descController,
            maxLines: 3,
            decoration: const InputDecoration(border: OutlineInputBorder()),
            onChanged: (_) => _updateState(),
          ),
          const SizedBox(height: 32),
          ElevatedButton(
            onPressed: isFormValid ? widget.notifier.nextStep : null,
            style: ElevatedButton.styleFrom(
              minimumSize: const Size.fromHeight(56),
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
            ),
            child: const Text('Next Step →', style: TextStyle(fontFamily: 'Outfit', fontSize: 18)),
          ),
        ],
      ),
    );
  }
}

// Step 3: Connect Domain
class _ConnectDomainStep extends StatelessWidget {
  final WebsiteBuilderState state;
  final WebsiteBuilderNotifier notifier;

  const _ConnectDomainStep({required this.state, required this.notifier});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Column(
      key: const ValueKey('step3'),
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text('Connect a Domain', style: theme.textTheme.headlineMedium?.copyWith(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        const SizedBox(height: 16),
        Text('How should customers find you?', style: theme.textTheme.bodyLarge?.copyWith(fontFamily: 'Inter')),
        const SizedBox(height: 24),
        _DomainOption(
          title: 'Use a free OHC subdomain',
          subtitle: 'mybusiness.ohc.app',
          isSelected: state.domainType == 'subdomain',
          onTap: () => notifier.selectDomainType('subdomain'),
        ),
        const SizedBox(height: 16),
        _DomainOption(
          title: 'Use my own domain',
          subtitle: 'I already own a domain name',
          isSelected: state.domainType == 'own',
          onTap: () => notifier.selectDomainType('own'),
        ),
        const SizedBox(height: 16),
        _DomainOption(
          title: 'Buy a new domain',
          subtitle: 'Get a custom .com or .store',
          isSelected: state.domainType == 'buy',
          onTap: () => notifier.selectDomainType('buy'),
        ),
        const Spacer(),
        ElevatedButton(
          onPressed: state.domainType != null ? notifier.nextStep : null,
          style: ElevatedButton.styleFrom(
            minimumSize: const Size.fromHeight(56),
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
          ),
          child: const Text('Review & Publish →', style: TextStyle(fontFamily: 'Outfit', fontSize: 18)),
        ),
      ],
    );
  }
}

class _DomainOption extends StatelessWidget {
  final String title;
  final String subtitle;
  final bool isSelected;
  final VoidCallback onTap;

  const _DomainOption({
    required this.title,
    required this.subtitle,
    required this.isSelected,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return ListTile(
      onTap: onTap,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
        side: BorderSide(
          color: isSelected ? theme.colorScheme.primary : theme.colorScheme.outline.withValues(alpha: 0.3),
          width: isSelected ? 2 : 1,
        ),
      ),
      tileColor: isSelected ? theme.colorScheme.primary.withValues(alpha: 0.1) : null,
      title: Text(title, style: const TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
      subtitle: Text(subtitle, style: const TextStyle(fontFamily: 'Inter')),
      trailing: isSelected ? Icon(Icons.check_circle, color: theme.colorScheme.primary) : const Icon(Icons.circle_outlined),
    );
  }
}

// Step 4: Go Live Preview
class _GoLiveStep extends StatelessWidget {
  final WebsiteBuilderState state;
  final WebsiteBuilderNotifier notifier;

  const _GoLiveStep({required this.state, required this.notifier});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Column(
      key: const ValueKey('step4'),
      crossAxisAlignment: CrossAxisAlignment.center,
      children: [
        Text('Ready to Launch', style: theme.textTheme.headlineMedium?.copyWith(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        const SizedBox(height: 16),
        Text('Your storefront is fully configured.', style: theme.textTheme.bodyLarge?.copyWith(fontFamily: 'Inter'), textAlign: TextAlign.center),
        const SizedBox(height: 24),
        Expanded(
          child: Container(
            width: 250,
            decoration: BoxDecoration(
              border: Border.all(color: theme.colorScheme.outline),
              borderRadius: BorderRadius.circular(24),
              color: theme.colorScheme.surfaceContainerHighest,
            ),
            clipBehavior: Clip.antiAlias,
            child: Column(
              children: [
                Container(
                  height: 120,
                  color: state.selectedColorPalette == 'palette1' ? Colors.blue : (state.selectedColorPalette == 'palette2' ? Colors.green : Colors.purple),
                  child: Center(
                    child: Text('My Store', style: theme.textTheme.headlineSmall?.copyWith(color: Colors.white, fontFamily: 'Outfit')),
                  ),
                ),
                Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Container(height: 100, color: theme.colorScheme.surfaceContainer),
                      const SizedBox(height: 8),
                      Text(state.productName ?? 'Product', style: const TextStyle(fontWeight: FontWeight.bold)),
                      Text('\$${state.productPrice ?? '0.00'}'),
                      const SizedBox(height: 8),
                      ElevatedButton(onPressed: () {}, child: const Text('Buy Now')),
                    ],
                  ),
                ),
              ],
            ),
          ),
        ),
        const SizedBox(height: 24),
        ElevatedButton(
          onPressed: state.isPublishing ? null : () => notifier.publish(context),
          style: ElevatedButton.styleFrom(
            minimumSize: const Size.fromHeight(56),
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
            backgroundColor: theme.colorScheme.primary,
            foregroundColor: theme.colorScheme.onPrimary,
          ),
          child: state.isPublishing
              ? const SizedBox(width: 24, height: 24, child: CircularProgressIndicator(color: Colors.white, strokeWidth: 2))
              : const Text('Publish Now 🚀', style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold)),
        ),
      ],
    );
  }
}
