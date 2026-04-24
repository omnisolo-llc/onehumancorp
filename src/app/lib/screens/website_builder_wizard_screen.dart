import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class WebsiteBuilderState {
  final int step;
  final String selectedTemplate;
  final String brandColor;
  final String logoPath;
  final String productName;
  final String productPrice;
  final String productDescription;
  final String domainChoice;
  final String customDomain;
  final bool isPublishing;

  const WebsiteBuilderState({
    this.step = 0,
    this.selectedTemplate = '',
    this.brandColor = '',
    this.logoPath = '',
    this.productName = '',
    this.productPrice = '',
    this.productDescription = '',
    this.domainChoice = 'ohc_subdomain',
    this.customDomain = '',
    this.isPublishing = false,
  });

  WebsiteBuilderState copyWith({
    int? step,
    String? selectedTemplate,
    String? brandColor,
    String? logoPath,
    String? productName,
    String? productPrice,
    String? productDescription,
    String? domainChoice,
    String? customDomain,
    bool? isPublishing,
  }) {
    return WebsiteBuilderState(
      step: step ?? this.step,
      selectedTemplate: selectedTemplate ?? this.selectedTemplate,
      brandColor: brandColor ?? this.brandColor,
      logoPath: logoPath ?? this.logoPath,
      productName: productName ?? this.productName,
      productPrice: productPrice ?? this.productPrice,
      productDescription: productDescription ?? this.productDescription,
      domainChoice: domainChoice ?? this.domainChoice,
      customDomain: customDomain ?? this.customDomain,
      isPublishing: isPublishing ?? this.isPublishing,
    );
  }
}

class WebsiteBuilderNotifier extends StateNotifier<WebsiteBuilderState> {
  WebsiteBuilderNotifier() : super(const WebsiteBuilderState());

  void setStep(int step) => state = state.copyWith(step: step);
  void nextStep() => state = state.copyWith(step: state.step + 1);
  void prevStep() {
    if (state.step > 0) {
      state = state.copyWith(step: state.step - 1);
    }
  }

  void updateTemplate(String template) => state = state.copyWith(selectedTemplate: template);
  void updateBrandColor(String color) => state = state.copyWith(brandColor: color);
  void updateLogoPath(String path) => state = state.copyWith(logoPath: path);
  void updateProduct(String name, String price, String description) {
    state = state.copyWith(
      productName: name,
      productPrice: price,
      productDescription: description,
    );
  }
  void updateDomain(String choice, String custom) {
    state = state.copyWith(domainChoice: choice, customDomain: custom);
  }

  Future<void> publishSite() async {
    state = state.copyWith(isPublishing: true);
    await Future.delayed(const Duration(seconds: 1)); // Simulate network
    state = state.copyWith(isPublishing: false);
  }
}

final websiteBuilderProvider = StateNotifierProvider<WebsiteBuilderNotifier, WebsiteBuilderState>((ref) {
  return WebsiteBuilderNotifier();
});

class WebsiteBuilderWizardScreen extends ConsumerWidget {
  const WebsiteBuilderWizardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(websiteBuilderProvider);

    return Scaffold(
      appBar: AppBar(
        leading: state.step > 0
            ? IconButton(
                icon: const Icon(Icons.arrow_back),
                onPressed: () => ref.read(websiteBuilderProvider.notifier).prevStep(),
              )
            : IconButton(
                icon: const Icon(Icons.close),
                onPressed: () => context.go('/'),
              ),
        title: const Text('Build My Website', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: Center(
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(24),
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 800),
            child: GlassCard(
              child: Padding(
                padding: const EdgeInsets.all(32.0),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    _buildStepper(state.step),
                    const SizedBox(height: 32),
                    if (state.step == 0) _buildTemplateGallery(context, ref, state),
                    if (state.step == 1) _buildBrandColors(context, ref, state),
                    if (state.step == 2) _buildProductService(context, ref, state),
                    if (state.step == 3) _buildDomainSetup(context, ref, state),
                    if (state.step == 4) _buildPublishPreview(context, ref, state),
                    const SizedBox(height: 32),
                    _buildNavigation(context, ref, state),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildTemplateGallery(BuildContext context, WidgetRef ref, WebsiteBuilderState state) {
    final templates = ['Modern E-commerce', 'Creative Portfolio', 'Local Service', 'Restaurant Menu'];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text('Choose a Template', style: Theme.of(context).textTheme.headlineSmall?.copyWith(fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        GridView.builder(
          shrinkWrap: true,
          physics: const NeverScrollableScrollPhysics(),
          gridDelegate: const SliverGridDelegateWithMaxCrossAxisExtent(
            maxCrossAxisExtent: 250,
            crossAxisSpacing: 16,
            mainAxisSpacing: 16,
            childAspectRatio: 1.5,
          ),
          itemCount: templates.length,
          itemBuilder: (context, index) {
            final template = templates[index];
            final isSelected = state.selectedTemplate == template;
            return GestureDetector(
              onTap: () => ref.read(websiteBuilderProvider.notifier).updateTemplate(template),
              child: Container(
                decoration: BoxDecoration(
                  border: Border.all(color: isSelected ? Colors.blue : Colors.grey.shade800, width: 2),
                  borderRadius: BorderRadius.circular(12),
                  color: Colors.grey.shade900,
                ),
                child: Center(
                  child: Text(
                    template,
                    style: TextStyle(color: isSelected ? Colors.blue : Colors.white, fontFamily: 'Inter'),
                    textAlign: TextAlign.center,
                  ),
                ),
              ),
            );
          },
        ),
      ],
    );
  }

  Widget _buildBrandColors(BuildContext context, WidgetRef ref, WebsiteBuilderState state) {
    final colors = [Colors.red, Colors.blue, Colors.green, Colors.purple, Colors.orange];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text('Brand Colors & Logo', style: Theme.of(context).textTheme.headlineSmall?.copyWith(fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        const Text('Select a primary brand color:', style: TextStyle(fontFamily: 'Inter', color: Colors.grey)),
        const SizedBox(height: 16),
        Wrap(
          spacing: 16,
          runSpacing: 16,
          children: colors.map((color) {
            final colorString = color.value.toRadixString(16);
            final isSelected = state.brandColor == colorString;
            return GestureDetector(
              onTap: () => ref.read(websiteBuilderProvider.notifier).updateBrandColor(colorString),
              child: Container(
                width: 48,
                height: 48,
                decoration: BoxDecoration(
                  color: color,
                  shape: BoxShape.circle,
                  border: Border.all(color: isSelected ? Colors.white : Colors.transparent, width: 3),
                ),
              ),
            );
          }).toList(),
        ),
        const SizedBox(height: 32),
        ElevatedButton.icon(
          onPressed: () => ref.read(websiteBuilderProvider.notifier).updateLogoPath('generated_logo.png'),
          icon: const Icon(Icons.auto_awesome),
          label: const Text('Generate Logo with AI'),
        ),
        if (state.logoPath.isNotEmpty) ...[
          const SizedBox(height: 16),
          const Text('Logo selected (AI Generated)', style: TextStyle(color: Colors.green, fontFamily: 'Inter')),
        ]
      ],
    );
  }

  Widget _buildProductService(BuildContext context, WidgetRef ref, WebsiteBuilderState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text('Add Your First Product or Service', style: Theme.of(context).textTheme.headlineSmall?.copyWith(fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        TextFormField(
          decoration: const InputDecoration(labelText: 'Name', border: OutlineInputBorder()),
          initialValue: state.productName,
          onChanged: (val) => ref.read(websiteBuilderProvider.notifier).updateProduct(val, state.productPrice, state.productDescription),
        ),
        const SizedBox(height: 16),
        TextFormField(
          decoration: const InputDecoration(labelText: 'Price', border: OutlineInputBorder()),
          keyboardType: TextInputType.number,
          initialValue: state.productPrice,
          onChanged: (val) => ref.read(websiteBuilderProvider.notifier).updateProduct(state.productName, val, state.productDescription),
        ),
        const SizedBox(height: 16),
        TextFormField(
          decoration: const InputDecoration(labelText: 'Description (AI can rewrite this)', border: OutlineInputBorder()),
          maxLines: 3,
          initialValue: state.productDescription,
          onChanged: (val) => ref.read(websiteBuilderProvider.notifier).updateProduct(state.productName, state.productPrice, val),
        ),
        const SizedBox(height: 16),
        ElevatedButton.icon(
          onPressed: () {
            ref.read(websiteBuilderProvider.notifier).updateProduct(
              state.productName,
              state.productPrice,
              'A beautifully crafted ${state.productName} that brings joy and utility.',
            );
          },
          icon: const Icon(Icons.auto_awesome),
          label: const Text('Auto-generate Description'),
        ),
      ],
    );
  }

  Widget _buildDomainSetup(BuildContext context, WidgetRef ref, WebsiteBuilderState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text('Connect a Domain', style: Theme.of(context).textTheme.headlineSmall?.copyWith(fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        RadioListTile<String>(
          title: const Text('Use a free OHC subdomain', style: TextStyle(fontFamily: 'Inter')),
          subtitle: const Text('mybusiness.ohc.app', style: TextStyle(fontFamily: 'Inter', color: Colors.grey)),
          value: 'ohc_subdomain',
          groupValue: state.domainChoice,
          onChanged: (val) => ref.read(websiteBuilderProvider.notifier).updateDomain(val!, state.customDomain),
          activeColor: Colors.blue,
        ),
        RadioListTile<String>(
          title: const Text('Use my own domain', style: TextStyle(fontFamily: 'Inter')),
          value: 'own_domain',
          groupValue: state.domainChoice,
          onChanged: (val) => ref.read(websiteBuilderProvider.notifier).updateDomain(val!, state.customDomain),
          activeColor: Colors.blue,
        ),
        if (state.domainChoice == 'own_domain')
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16.0),
            child: TextFormField(
              decoration: const InputDecoration(labelText: 'e.g. mywebsite.com', border: OutlineInputBorder()),
              initialValue: state.customDomain,
              onChanged: (val) => ref.read(websiteBuilderProvider.notifier).updateDomain(state.domainChoice, val),
            ),
          ),
        RadioListTile<String>(
          title: const Text('Buy a new domain', style: TextStyle(fontFamily: 'Inter')),
          value: 'buy_domain',
          groupValue: state.domainChoice,
          onChanged: (val) => ref.read(websiteBuilderProvider.notifier).updateDomain(val!, state.customDomain),
          activeColor: Colors.blue,
        ),
      ],
    );
  }

  Widget _buildPublishPreview(BuildContext context, WidgetRef ref, WebsiteBuilderState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text('Ready to Go Live?', style: Theme.of(context).textTheme.headlineSmall?.copyWith(fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        Container(
          padding: const EdgeInsets.all(16),
          decoration: BoxDecoration(
            color: Colors.grey.shade900,
            borderRadius: BorderRadius.circular(12),
            border: Border.all(color: Colors.grey.shade800),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text('Template: ${state.selectedTemplate.isEmpty ? 'Not selected' : state.selectedTemplate}', style: const TextStyle(fontFamily: 'Inter')),
              const SizedBox(height: 8),
              Text('Product: ${state.productName.isEmpty ? 'Not specified' : state.productName}', style: const TextStyle(fontFamily: 'Inter')),
              const SizedBox(height: 8),
              Text('Domain: ${state.domainChoice == 'ohc_subdomain' ? 'mybusiness.ohc.app' : state.customDomain}', style: const TextStyle(fontFamily: 'Inter')),
            ],
          ),
        ),
        const SizedBox(height: 16),
        const Text('Click Publish below to make your site live instantly!', style: TextStyle(fontFamily: 'Inter', color: Colors.grey)),
      ],
    );
  }

  Widget _buildStepper(int currentStep) {
    final steps = ['Template', 'Brand', 'Product', 'Domain', 'Publish'];
    return LayoutBuilder(
      builder: (context, constraints) {
        return Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: List.generate(steps.length, (index) {
            final isActive = index <= currentStep;
            return Container(
              width: constraints.maxWidth / steps.length - 8,
              child: Column(
                children: [
                  CircleAvatar(
                    radius: 16,
                    backgroundColor: isActive ? Colors.blue : Colors.grey.shade800,
                    child: Text('${index + 1}', style: const TextStyle(color: Colors.white, fontSize: 12)),
                  ),
                  const SizedBox(height: 8),
                  Text(
                    steps[index],
                    style: TextStyle(
                      fontFamily: 'Inter',
                      fontSize: 10,
                      color: isActive ? Colors.white : Colors.grey,
                    ),
                    textAlign: TextAlign.center,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                  ),
                ],
              ),
            );
          }),
        );
      }
    );
  }

  Widget _buildNavigation(BuildContext context, WidgetRef ref, WebsiteBuilderState state) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.end,
      children: [
        if (state.step < 4)
          FilledButton(
            onPressed: () => ref.read(websiteBuilderProvider.notifier).nextStep(),
            child: const Text('Next', style: TextStyle(fontFamily: 'Inter')),
          )
        else
          FilledButton(
            style: FilledButton.styleFrom(backgroundColor: Colors.green),
            onPressed: state.isPublishing
                ? null
                : () async {
                    await ref.read(websiteBuilderProvider.notifier).publishSite();
                    if (context.mounted) {
                      ScaffoldMessenger.of(context).showSnackBar(
                        const SnackBar(content: Text('Website published successfully! Link copied to clipboard.')),
                      );
                      context.go('/');
                    }
                  },
            child: state.isPublishing
                ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2))
                : const Text('Publish', style: TextStyle(fontFamily: 'Inter')),
          ),
      ],
    );
  }
}
