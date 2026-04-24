import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class WebsiteBuilderState {
  final int step;
  final String selectedTemplate;
  final String selectedPalette;
  final String logoUrl;
  final String productName;
  final String productPrice;
  final String productDescription;
  final String domainType;
  final String customDomain;
  final bool isLoading;
  final String? errorMessage;
  final bool isPublished;

  const WebsiteBuilderState({
    this.step = 0,
    this.selectedTemplate = '',
    this.selectedPalette = '',
    this.logoUrl = '',
    this.productName = '',
    this.productPrice = '',
    this.productDescription = '',
    this.domainType = '',
    this.customDomain = '',
    this.isLoading = false,
    this.errorMessage,
    this.isPublished = false,
  });

  WebsiteBuilderState copyWith({
    int? step,
    String? selectedTemplate,
    String? selectedPalette,
    String? logoUrl,
    String? productName,
    String? productPrice,
    String? productDescription,
    String? domainType,
    String? customDomain,
    bool? isLoading,
    String? errorMessage,
    bool? isPublished,
  }) {
    return WebsiteBuilderState(
      step: step ?? this.step,
      selectedTemplate: selectedTemplate ?? this.selectedTemplate,
      selectedPalette: selectedPalette ?? this.selectedPalette,
      logoUrl: logoUrl ?? this.logoUrl,
      productName: productName ?? this.productName,
      productPrice: productPrice ?? this.productPrice,
      productDescription: productDescription ?? this.productDescription,
      domainType: domainType ?? this.domainType,
      customDomain: customDomain ?? this.customDomain,
      isLoading: isLoading ?? this.isLoading,
      errorMessage: errorMessage,
      isPublished: isPublished ?? this.isPublished,
    );
  }
}

class WebsiteBuilderNotifier extends Notifier<WebsiteBuilderState> {
  @override
  WebsiteBuilderState build() => const WebsiteBuilderState();

  void nextStep() {
    state = state.copyWith(step: state.step + 1);
  }

  void previousStep() {
    if (state.step > 0) {
      state = state.copyWith(step: state.step - 1);
    }
  }

  void selectTemplate(String template) {
    state = state.copyWith(selectedTemplate: template);
  }

  void selectPalette(String palette) {
    state = state.copyWith(selectedPalette: palette);
  }

  Future<void> generateLogo() async {
    state = state.copyWith(isLoading: true, errorMessage: null);
    try {
      final api = ref.read(apiServiceProvider);
      if (api != null) {
        final logo = await api.generateLogo();
        state = state.copyWith(logoUrl: logo, isLoading: false);
      } else {
         state = state.copyWith(logoUrl: 'generated_logo.png', isLoading: false);
      }
    } catch (e) {
      state = state.copyWith(isLoading: false, errorMessage: e.toString());
    }
  }

  void updateProduct(String name, String price, String description) {
    state = state.copyWith(
      productName: name,
      productPrice: price,
      productDescription: description,
    );
  }

  Future<void> generateProductDescription(String name) async {
    state = state.copyWith(isLoading: true, errorMessage: null);
    try {
      final api = ref.read(apiServiceProvider);
      if (api != null) {
        final desc = await api.generateProductDescription(name);
        state = state.copyWith(productDescription: desc, isLoading: false);
      } else {
        state = state.copyWith(productDescription: 'Generated description for $name', isLoading: false);
      }
    } catch (e) {
      state = state.copyWith(isLoading: false, errorMessage: e.toString());
    }
  }

  void selectDomainType(String type) {
    state = state.copyWith(domainType: type);
  }

  Future<void> publish() async {
    state = state.copyWith(isLoading: true, errorMessage: null);
    try {
      final api = ref.read(apiServiceProvider);
      if (api != null) {
        await api.publishWebsite(state);
      }
      state = state.copyWith(isLoading: false, isPublished: true);
    } catch (e) {
      state = state.copyWith(isLoading: false, errorMessage: e.toString());
    }
  }
}

final websiteBuilderProvider = NotifierProvider<WebsiteBuilderNotifier, WebsiteBuilderState>(() {
  return WebsiteBuilderNotifier();
});

class WebsiteBuilderWizardScreen extends ConsumerWidget {
  const WebsiteBuilderWizardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(websiteBuilderProvider);
    final notifier = ref.read(websiteBuilderProvider.notifier);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Build Your Website', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () {
            if (state.step > 0 && !state.isPublished) {
              notifier.previousStep();
            } else {
              context.go('/dashboard');
            }
          },
        ),
      ),
      body: SafeArea(
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 600),
            child: Padding(
              padding: const EdgeInsets.all(24.0),
              child: AnimatedSwitcher(
                duration: const Duration(milliseconds: 300),
                child: _buildStep(context, ref, state),
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildStep(BuildContext context, WidgetRef ref, WebsiteBuilderState state) {
    if (state.isPublished) {
      return _buildSuccessStep(context, ref, state);
    }
    switch (state.step) {
      case 0:
        return _buildTemplateGallery(context, ref, state);
      case 1:
        return _buildBrandColorsLogo(context, ref, state);
      case 2:
        return _buildAddProduct(context, ref, state);
      case 3:
        return _buildDomainSelection(context, ref, state);
      case 4:
        return _buildReviewPublish(context, ref, state);
      default:
        return const SizedBox.shrink();
    }
  }

  Widget _buildTemplateGallery(BuildContext context, WidgetRef ref, WebsiteBuilderState state) {
    final notifier = ref.read(websiteBuilderProvider.notifier);
    final templates = ['Modern minimal', 'Bold storefront', 'Elegant portfolio'];

    return Column(
      key: const ValueKey('step0'),
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text('Choose a Template', style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 8),
        const Text('Select a starting point for your website.', style: TextStyle(fontFamily: 'Inter')),
        const SizedBox(height: 24),
        Expanded(
          child: ListView.builder(
            itemCount: templates.length,
            itemBuilder: (context, index) {
              final isSelected = state.selectedTemplate == templates[index];
              return Padding(
                padding: const EdgeInsets.only(bottom: 16.0),
                child: Semantics(
                  button: true,
                  label: templates[index],
                  child: InkWell(
                    onTap: () => notifier.selectTemplate(templates[index]),
                    borderRadius: BorderRadius.circular(16),
                    child: GlassCard(
                      child: Container(
                        padding: const EdgeInsets.all(16),
                        decoration: BoxDecoration(
                          border: isSelected ? Border.all(color: Colors.green, width: 2) : null,
                          borderRadius: BorderRadius.circular(16),
                        ),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Container(
                              height: 120,
                              decoration: BoxDecoration(
                                color: Theme.of(context).colorScheme.surfaceContainerHighest,
                                borderRadius: BorderRadius.circular(8),
                              ),
                              child: const Center(child: Icon(Icons.web, size: 48)),
                            ),
                            const SizedBox(height: 16),
                            Text(templates[index], style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 18, fontFamily: 'Outfit')),
                            const SizedBox(height: 8),
                            if (isSelected)
                              const Text('Use this template →', style: TextStyle(color: Colors.green, fontWeight: FontWeight.bold, fontFamily: 'Inter')),
                          ],
                        ),
                      ),
                    ),
                  ),
                ),
              );
            },
          ),
        ),
        FilledButton(
          onPressed: state.selectedTemplate.isNotEmpty ? () => notifier.nextStep() : null,
          child: const Text('Next'),
        ),
      ],
    );
  }

  Widget _buildBrandColorsLogo(BuildContext context, WidgetRef ref, WebsiteBuilderState state) {
    final notifier = ref.read(websiteBuilderProvider.notifier);
    final palettes = ['Ocean Blue', 'Earthy Greens', 'Sunset Warm'];

    return Column(
      key: const ValueKey('step1'),
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text('Brand & Logo', style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 24),
        const Text('Select a color palette:', style: TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Inter')),
        const SizedBox(height: 8),
        Wrap(
          spacing: 8,
          children: palettes.map((p) => ChoiceChip(
            label: Text(p),
            selected: state.selectedPalette == p,
            onSelected: (val) => val ? notifier.selectPalette(p) : null,
          )).toList(),
        ),
        const SizedBox(height: 32),
        const Text('Logo:', style: TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Inter')),
        const SizedBox(height: 16),
        if (state.logoUrl.isNotEmpty) ...[
          Container(
            padding: const EdgeInsets.all(16),
            decoration: BoxDecoration(color: Colors.grey.withValues(alpha: 0.1), borderRadius: BorderRadius.circular(8)),
            child: const Center(child: Icon(Icons.image, size: 64)),
          ),
          const SizedBox(height: 16),
        ],
        OutlinedButton.icon(
          onPressed: state.isLoading ? null : () => notifier.generateLogo(),
          icon: state.isLoading ? const SizedBox(width: 16, height: 16, child: CircularProgressIndicator(strokeWidth: 2)) : const Icon(Icons.auto_awesome),
          label: const Text('Generate a logo for me'),
        ),
        const Spacer(),
        FilledButton(
          onPressed: state.selectedPalette.isNotEmpty ? () => notifier.nextStep() : null,
          child: const Text('Next'),
        ),
      ],
    );
  }

  Widget _buildAddProduct(BuildContext context, WidgetRef ref, WebsiteBuilderState state) {
    final notifier = ref.read(websiteBuilderProvider.notifier);
    final nameCtrl = TextEditingController(text: state.productName);
    final priceCtrl = TextEditingController(text: state.productPrice);
    final descCtrl = TextEditingController(text: state.productDescription);

    return SingleChildScrollView(
      key: const ValueKey('step2'),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Text('Add Your First Product', style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
          const SizedBox(height: 24),
          TextField(
            controller: nameCtrl,
            decoration: const InputDecoration(labelText: 'Product Name', border: OutlineInputBorder()),
            onChanged: (v) => notifier.updateProduct(v, state.productPrice, state.productDescription),
          ),
          const SizedBox(height: 16),
          TextField(
            controller: priceCtrl,
            keyboardType: TextInputType.number,
            decoration: const InputDecoration(labelText: 'Price', border: OutlineInputBorder()),
            onChanged: (v) => notifier.updateProduct(state.productName, v, state.productDescription),
          ),
          const SizedBox(height: 16),
          TextField(
            controller: descCtrl,
            maxLines: 3,
            decoration: const InputDecoration(labelText: 'Description', border: OutlineInputBorder()),
            onChanged: (v) => notifier.updateProduct(state.productName, state.productPrice, v),
          ),
          const SizedBox(height: 8),
          Align(
            alignment: Alignment.centerRight,
            child: TextButton.icon(
              onPressed: state.isLoading || state.productName.isEmpty ? null : () => notifier.generateProductDescription(state.productName),
              icon: state.isLoading ? const SizedBox(width: 16, height: 16, child: CircularProgressIndicator(strokeWidth: 2)) : const Icon(Icons.auto_awesome),
              label: const Text('Auto-generate description'),
            ),
          ),
          const SizedBox(height: 32),
          FilledButton(
            onPressed: state.productName.isNotEmpty ? () {
              notifier.updateProduct(nameCtrl.text, priceCtrl.text, descCtrl.text);
              notifier.nextStep();
            } : null,
            child: const Text('Next'),
          ),
        ],
      ),
    );
  }

  Widget _buildDomainSelection(BuildContext context, WidgetRef ref, WebsiteBuilderState state) {
    final notifier = ref.read(websiteBuilderProvider.notifier);
    final options = ['Use a free OHC subdomain', 'Use my own domain', 'Buy a domain'];

    return Column(
      key: const ValueKey('step3'),
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text('Connect a Domain', style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 24),
        ...options.map((opt) => Padding(
          padding: const EdgeInsets.only(bottom: 12.0),
          child: RadioListTile<String>(
            title: Text(opt, style: const TextStyle(fontFamily: 'Inter')),
            value: opt,
            groupValue: state.domainType,
            onChanged: (v) {
              if (v != null) notifier.selectDomainType(v);
            },
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8), side: BorderSide(color: Colors.grey.withValues(alpha: 0.3))),
          ),
        )),
        const Spacer(),
        FilledButton(
          onPressed: state.domainType.isNotEmpty ? () => notifier.nextStep() : null,
          child: const Text('Next'),
        ),
      ],
    );
  }

  Widget _buildReviewPublish(BuildContext context, WidgetRef ref, WebsiteBuilderState state) {
    final notifier = ref.read(websiteBuilderProvider.notifier);

    return Column(
      key: const ValueKey('step4'),
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text('Ready to go live!', style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 24),
        GlassCard(
          child: Padding(
            padding: const EdgeInsets.all(24.0),
            child: Column(
              children: [
                const Icon(Icons.web, size: 64, color: Colors.blue),
                const SizedBox(height: 16),
                Text('Template: ${state.selectedTemplate}', style: const TextStyle(fontFamily: 'Inter')),
                Text('Domain: ${state.domainType}', style: const TextStyle(fontFamily: 'Inter')),
              ],
            ),
          ),
        ),
        const Spacer(),
        if (state.errorMessage != null)
          Padding(
            padding: const EdgeInsets.only(bottom: 16.0),
            child: Text(state.errorMessage!, style: const TextStyle(color: Colors.red)),
          ),
        FilledButton(
          onPressed: state.isLoading ? null : () => notifier.publish(),
          child: state.isLoading
            ? const SizedBox(width: 24, height: 24, child: CircularProgressIndicator(color: Colors.white, strokeWidth: 2))
            : const Text('Publish'),
        ),
      ],
    );
  }

  Widget _buildSuccessStep(BuildContext context, WidgetRef ref, WebsiteBuilderState state) {
    return Column(
      key: const ValueKey('success'),
      mainAxisAlignment: MainAxisAlignment.center,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Icon(Icons.check_circle, size: 80, color: Colors.green),
        const SizedBox(height: 24),
        const Text('Your website is live!', textAlign: TextAlign.center, style: TextStyle(fontSize: 28, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
        const SizedBox(height: 16),
        GlassCard(
          child: Padding(
            padding: const EdgeInsets.all(16.0),
            child: Row(
              children: [
                const Expanded(child: Text('https://mybusiness.ohc.app', style: TextStyle(fontFamily: 'Inter', fontSize: 16))),
                IconButton(
                  icon: const Icon(Icons.copy),
                  onPressed: () {
                    Clipboard.setData(const ClipboardData(text: 'https://mybusiness.ohc.app'));
                    ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Copied to clipboard')));
                  },
                ),
              ],
            ),
          ),
        ),
        const SizedBox(height: 48),
        FilledButton(
          onPressed: () => context.go('/dashboard'),
          child: const Text('Go to Dashboard'),
        ),
      ],
    );
  }
}
