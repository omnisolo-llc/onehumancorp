import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class WebsiteBuilderState {
  final int step;
  final String selectedTemplate;
  final String selectedPalette;
  final String logoUrl;
  final String productName;
  final String productPrice;
  final String productDesc;
  final String domainChoice;
  final bool isLoading;
  final String? errorMessage;

  const WebsiteBuilderState({
    this.step = 0,
    this.selectedTemplate = '',
    this.selectedPalette = '',
    this.logoUrl = '',
    this.productName = '',
    this.productPrice = '',
    this.productDesc = '',
    this.domainChoice = 'subdomain',
    this.isLoading = false,
    this.errorMessage,
  });

  WebsiteBuilderState copyWith({
    int? step,
    String? selectedTemplate,
    String? selectedPalette,
    String? logoUrl,
    String? productName,
    String? productPrice,
    String? productDesc,
    String? domainChoice,
    bool? isLoading,
    String? errorMessage,
  }) {
    return WebsiteBuilderState(
      step: step ?? this.step,
      selectedTemplate: selectedTemplate ?? this.selectedTemplate,
      selectedPalette: selectedPalette ?? this.selectedPalette,
      logoUrl: logoUrl ?? this.logoUrl,
      productName: productName ?? this.productName,
      productPrice: productPrice ?? this.productPrice,
      productDesc: productDesc ?? this.productDesc,
      domainChoice: domainChoice ?? this.domainChoice,
      isLoading: isLoading ?? this.isLoading,
      errorMessage: errorMessage ?? this.errorMessage,
    );
  }
}

class WebsiteBuilderNotifier extends Notifier<WebsiteBuilderState> {
  @override
  WebsiteBuilderState build() => const WebsiteBuilderState();

  void nextStep() {
    if (state.step < 4) {
      state = state.copyWith(step: state.step + 1);
    }
  }

  void prevStep() {
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

  void updateLogo(String url) {
    state = state.copyWith(logoUrl: url);
  }

  void updateProductName(String name) {
    state = state.copyWith(productName: name);
    // Simple mock AI auto-generation
    state = state.copyWith(productDesc: 'A wonderful $name for your needs.');
  }

  void updateProductPrice(String price) {
    state = state.copyWith(productPrice: price);
  }

  void updateProductDesc(String desc) {
    state = state.copyWith(productDesc: desc);
  }

  void selectDomain(String choice) {
    state = state.copyWith(domainChoice: choice);
  }

  Future<void> publish(BuildContext context) async {
    state = state.copyWith(isLoading: true, errorMessage: null);
    // Simulate network delay
    await Future.delayed(const Duration(seconds: 1));
    state = state.copyWith(isLoading: false);

    if (context.mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Website published to mybusiness.ohc.app')),
      );
      context.go('/dashboard');
    }
  }
}

final websiteBuilderProvider = NotifierProvider<WebsiteBuilderNotifier, WebsiteBuilderState>(
  () => WebsiteBuilderNotifier(),
);

class WebsiteBuilderWizardScreen extends ConsumerWidget {
  const WebsiteBuilderWizardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(websiteBuilderProvider);
    final notifier = ref.read(websiteBuilderProvider.notifier);

    return Scaffold(
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Color(0xFF0D0D1A), Color(0xFF1A1A33)],
          ),
        ),
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 800),
            child: GlassCard(
              child: Padding(
                padding: const EdgeInsets.all(24.0),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    const Text('Build My Website', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white)),
                    const SizedBox(height: 16),
                    if (state.errorMessage != null) ...[
                      Text(state.errorMessage!, style: const TextStyle(color: Colors.red)),
                      const SizedBox(height: 16),
                    ],
                    AnimatedSwitcher(
                      duration: const Duration(milliseconds: 300),
                      child: Container(
                        key: ValueKey<int>(state.step),
                        child: _buildStepContent(context, state, notifier),
                      ),
                    ),
                    const SizedBox(height: 24),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        if (state.step > 0)
                          TextButton(
                            onPressed: state.isLoading ? null : notifier.prevStep,
                            child: const Text('Back', style: TextStyle(fontFamily: 'Inter')),
                          )
                        else
                          const SizedBox(),
                        ElevatedButton(
                          onPressed: state.isLoading ? null : () {
                            if (state.step < 4) {
                              notifier.nextStep();
                            } else {
                              notifier.publish(context);
                            }
                          },
                          child: state.isLoading
                              ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2))
                              : Text(state.step == 4 ? 'Publish' : 'Next', style: const TextStyle(fontFamily: 'Inter')),
                        ),
                      ],
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildStepContent(BuildContext context, WebsiteBuilderState state, WebsiteBuilderNotifier notifier) {
    switch (state.step) {
      case 0:
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text('Choose a Template', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
            const SizedBox(height: 16),
            Wrap(
              spacing: 16,
              runSpacing: 16,
              children: ['Modern Minimal', 'Bold Storefront', 'Creative Portfolio'].map((t) => InkWell(
                onTap: () => notifier.selectTemplate(t),
                child: Container(
                  width: 200,
                  height: 150,
                  decoration: BoxDecoration(
                    color: state.selectedTemplate == t ? Colors.blue.withOpacity(0.3) : Colors.white.withOpacity(0.1),
                    border: Border.all(color: state.selectedTemplate == t ? Colors.blue : Colors.transparent),
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: Center(child: Text(t, style: const TextStyle(color: Colors.white), textAlign: TextAlign.center)),
                ),
              )).toList(),
            ),
            if (state.selectedTemplate.isNotEmpty) ...[
              const SizedBox(height: 16),
              ElevatedButton(
                onPressed: () => notifier.nextStep(),
                style: ElevatedButton.styleFrom(backgroundColor: Colors.green),
                child: const Text('Use this template →'),
              ),
            ]
          ],
        );
      case 1:
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text('Brand Colors & Logo', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
            const SizedBox(height: 16),
            const Text('Palette:', style: TextStyle(color: Colors.white)),
            Wrap(
              spacing: 8,
              children: ['Ocean', 'Sunset', 'Forest'].map((p) => ChoiceChip(
                label: Text(p),
                selected: state.selectedPalette == p,
                onSelected: (val) {
                  if (val) notifier.selectPalette(p);
                },
              )).toList(),
            ),
            const SizedBox(height: 16),
            ElevatedButton.icon(
              onPressed: () => notifier.updateLogo('auto-generated-logo.png'),
              icon: const Icon(Icons.auto_awesome),
              label: const Text('Generate a logo for me'),
            ),
            if (state.logoUrl.isNotEmpty)
               Padding(
                 padding: const EdgeInsets.only(top: 8.0),
                 child: Text('Logo selected: ${state.logoUrl}', style: const TextStyle(color: Colors.white70)),
               ),
          ],
        );
      case 2:
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text('Add your first product or service', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
            const SizedBox(height: 16),
            TextField(
              decoration: const InputDecoration(labelText: 'Name', labelStyle: TextStyle(color: Colors.white70)),
              onChanged: notifier.updateProductName,
              style: const TextStyle(color: Colors.white),
            ),
            const SizedBox(height: 16),
            TextField(
              decoration: const InputDecoration(labelText: 'Price', labelStyle: TextStyle(color: Colors.white70)),
              keyboardType: TextInputType.number,
              onChanged: notifier.updateProductPrice,
              style: const TextStyle(color: Colors.white),
            ),
            const SizedBox(height: 16),
            TextField(
              controller: TextEditingController(text: state.productDesc)..selection = TextSelection.collapsed(offset: state.productDesc.length),
              decoration: const InputDecoration(labelText: 'Description (AI Auto-generated)', labelStyle: TextStyle(color: Colors.white70)),
              onChanged: notifier.updateProductDesc,
              style: const TextStyle(color: Colors.white),
              maxLines: 3,
            ),
          ],
        );
      case 3:
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text('Connect a domain', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
            const SizedBox(height: 16),
            RadioListTile<String>(
              title: const Text('Use a free OHC subdomain (mybusiness.ohc.app)', style: TextStyle(color: Colors.white)),
              value: 'subdomain',
              groupValue: state.domainChoice,
              onChanged: (val) => notifier.selectDomain(val!),
            ),
            RadioListTile<String>(
              title: const Text('Use my own domain', style: TextStyle(color: Colors.white)),
              value: 'custom',
              groupValue: state.domainChoice,
              onChanged: (val) => notifier.selectDomain(val!),
            ),
            RadioListTile<String>(
              title: const Text('Buy a domain', style: TextStyle(color: Colors.white)),
              value: 'buy',
              groupValue: state.domainChoice,
              onChanged: (val) => notifier.selectDomain(val!),
            ),
          ],
        );
      case 4:
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text('Preview & Go Live', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
            const SizedBox(height: 16),
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Colors.white.withOpacity(0.05),
                borderRadius: BorderRadius.circular(8),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text('Template: ${state.selectedTemplate}', style: const TextStyle(color: Colors.white)),
                  Text('Palette: ${state.selectedPalette}', style: const TextStyle(color: Colors.white)),
                  Text('Product: ${state.productName} - \$${state.productPrice}', style: const TextStyle(color: Colors.white)),
                  Text('Domain: ${state.domainChoice == 'subdomain' ? 'Free Subdomain' : 'Custom'}', style: const TextStyle(color: Colors.white)),
                ],
              ),
            ),
            const SizedBox(height: 16),
            const Text('Ready to show the world?', style: TextStyle(color: Colors.white)),
          ],
        );
      default:
        return const SizedBox();
    }
  }
}
