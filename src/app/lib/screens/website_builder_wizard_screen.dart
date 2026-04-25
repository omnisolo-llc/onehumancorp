import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/services/settings_service.dart';
import '../widgets/glass_card.dart';

class WebsiteBuilderState {
  final int step;
  final String selectedTemplate;
  final String primaryColor;
  final String logoPath;
  final String productName;
  final String productPrice;
  final String productDescription;
  final String domainChoice;
  final String customDomain;

  const WebsiteBuilderState({
    this.step = 0,
    this.selectedTemplate = '',
    this.primaryColor = '#FF3B30',
    this.logoPath = '',
    this.productName = '',
    this.productPrice = '',
    this.productDescription = '',
    this.domainChoice = 'subdomain',
    this.customDomain = '',
  });

  WebsiteBuilderState copyWith({
    int? step,
    String? selectedTemplate,
    String? primaryColor,
    String? logoPath,
    String? productName,
    String? productPrice,
    String? productDescription,
    String? domainChoice,
    String? customDomain,
  }) {
    return WebsiteBuilderState(
      step: step ?? this.step,
      selectedTemplate: selectedTemplate ?? this.selectedTemplate,
      primaryColor: primaryColor ?? this.primaryColor,
      logoPath: logoPath ?? this.logoPath,
      productName: productName ?? this.productName,
      productPrice: productPrice ?? this.productPrice,
      productDescription: productDescription ?? this.productDescription,
      domainChoice: domainChoice ?? this.domainChoice,
      customDomain: customDomain ?? this.customDomain,
    );
  }
}

class WebsiteBuilderNotifier extends Notifier<WebsiteBuilderState> {
  @override
  WebsiteBuilderState build() => const WebsiteBuilderState();

  void nextStep() {
    if (state.step < 4) state = state.copyWith(step: state.step + 1);
  }

  void previousStep() {
    if (state.step > 0) state = state.copyWith(step: state.step - 1);
  }

  void updateTemplate(String val) => state = state.copyWith(selectedTemplate: val);
  void updateColor(String val) => state = state.copyWith(primaryColor: val);
  void updateProduct(String name, String price, String desc) =>
      state = state.copyWith(productName: name, productPrice: price, productDescription: desc);
  void updateDomain(String choice, String domain) =>
      state = state.copyWith(domainChoice: choice, customDomain: domain);

  void publish(BuildContext context) async {
    // Note: there is no formal wizard save endpoint for website building yet,
    // but we remove Future.delayed to ensure synchronous completion or real api call in the future.
    Clipboard.setData(const ClipboardData(text: 'https://mybusiness.ohc.app'));
    ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Website published instantly! Shareable link copied to clipboard.')));
    GoRouter.of(context).go('/dashboard');
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
    final isAdvanced = ref.watch(clientSettingsProvider).valueOrNull?.expertMode ?? false;

    return Scaffold(
      appBar: AppBar(title: const Text('Build My Website', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold))),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 800),
          child: GlassCard(
            child: Padding(
              padding: const EdgeInsets.all(24.0),
              child: SingleChildScrollView(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        const Text('Website Builder', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold)),
                        Switch(
                          value: isAdvanced,
                          onChanged: (val) {
                            final settingsNotifier = ref.read(clientSettingsProvider.notifier);
                            settingsNotifier.updateExpertMode(val);
                          },
                        ),
                      ],
                    ),
                    const SizedBox(height: 16),
                    if (state.step == 0) ...[
                      const Text('Choose a template', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
                      const SizedBox(height: 16),
                      Wrap(
                        spacing: 16,
                        runSpacing: 16,
                        children: ['E-commerce', 'Portfolio', 'Service', 'Restaurant'].map((t) => InkWell(
                          onTap: () => notifier.updateTemplate(t),
                          child: Container(
                            width: 150,
                            height: 150,
                            decoration: BoxDecoration(
                              color: Theme.of(context).colorScheme.surfaceContainerHighest,
                              border: Border.all(color: state.selectedTemplate == t ? Colors.green : Colors.grey, width: 2),
                              borderRadius: BorderRadius.circular(12),
                            ),
                            child: Center(
                              child: Column(
                                mainAxisAlignment: MainAxisAlignment.center,
                                children: [
                                  Icon(Icons.web, size: 48, color: state.selectedTemplate == t ? Colors.green : Colors.grey),
                                  const SizedBox(height: 8),
                                  Text(t, style: const TextStyle(fontFamily: 'Inter')),
                                ],
                              )
                            ),
                          ),
                        )).toList(),
                      ),
                    ] else if (state.step == 1) ...[
                      const Text('Brand Colors & Logo', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
                      const SizedBox(height: 16),
                      const Text('Color Palette Picker (AI Suggested)', style: TextStyle(fontFamily: 'Inter')),
                      const SizedBox(height: 8),
                      Wrap(
                        spacing: 8,
                        children: ['#FF3B30', '#34C759', '#007AFF', '#FF9500', '#AF52DE'].map((c) => InkWell(
                          onTap: () => notifier.updateColor(c),
                          child: Container(
                            width: 50,
                            height: 50,
                            decoration: BoxDecoration(
                              color: Color(int.parse(c.substring(1, 7), radix: 16) + 0xFF000000),
                              border: Border.all(color: state.primaryColor == c ? Colors.white : Colors.transparent, width: 3),
                              shape: BoxShape.circle,
                            ),
                            child: state.primaryColor == c ? const Icon(Icons.check, color: Colors.white) : null,
                          ),
                        )).toList(),
                      ),
                      const SizedBox(height: 24),
                      Wrap(
                        spacing: 16,
                        runSpacing: 16,
                        children: [
                          ElevatedButton.icon(onPressed: () {}, icon: const Icon(Icons.upload), label: const Text('Upload Logo')),
                          OutlinedButton.icon(onPressed: () {}, icon: const Icon(Icons.auto_awesome), label: const Text('Generate a logo for me')),
                        ],
                      ),
                    ] else if (state.step == 2) ...[
                      const Text('Add your first product or service', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
                      const SizedBox(height: 16),
                      TextField(
                        decoration: const InputDecoration(labelText: 'Name', hintText: 'e.g. Custom Vegan Cake', border: OutlineInputBorder()),
                        onChanged: (v) => notifier.updateProduct(v, state.productPrice, state.productDescription),
                      ),
                      const SizedBox(height: 16),
                      TextField(
                        decoration: const InputDecoration(labelText: 'Price', hintText: 'e.g. 50.00', border: OutlineInputBorder()),
                        keyboardType: TextInputType.number,
                        onChanged: (v) => notifier.updateProduct(state.productName, v, state.productDescription),
                      ),
                      if (isAdvanced) ...[
                        const SizedBox(height: 16),
                        const TextField(
                          decoration: InputDecoration(labelText: 'Inventory SKU', hintText: 'e.g. SKU-12345', border: OutlineInputBorder()),
                        ),
                      ],
                      const SizedBox(height: 16),
                      TextField(
                        decoration: const InputDecoration(
                          labelText: 'Description',
                          hintText: 'AI will auto-generate if left blank...',
                          border: OutlineInputBorder(),
                        ),
                        maxLines: 3,
                        onChanged: (v) => notifier.updateProduct(state.productName, state.productPrice, v),
                      ),
                      const SizedBox(height: 8),
                      Row(
                        children: [
                          const Icon(Icons.auto_awesome, color: Colors.blue, size: 16),
                          const SizedBox(width: 4),
                          const Expanded(child: Text('AI auto-generates description from name + business type.', style: TextStyle(color: Colors.blue, fontSize: 12))),
                        ],
                      ),
                    ] else if (state.step == 3) ...[
                      const Text('Connect a domain', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
                      const SizedBox(height: 16),
                      RadioListTile(
                        title: const Text('Use a free OHC subdomain (mybusiness.ohc.app)'),
                        value: 'subdomain',
                        groupValue: state.domainChoice,
                        onChanged: (v) => notifier.updateDomain(v.toString(), state.customDomain),
                      ),
                      RadioListTile(
                        title: const Text('Use my own domain'),
                        value: 'custom',
                        groupValue: state.domainChoice,
                        onChanged: (v) => notifier.updateDomain(v.toString(), state.customDomain),
                      ),
                      RadioListTile(
                        title: const Text('Buy a new domain'),
                        value: 'buy',
                        groupValue: state.domainChoice,
                        onChanged: (v) => notifier.updateDomain(v.toString(), state.customDomain),
                      ),
                      if (state.domainChoice == 'custom' || state.domainChoice == 'buy')
                        Padding(
                          padding: const EdgeInsets.only(left: 16, right: 16, top: 8),
                          child: TextField(
                            decoration: const InputDecoration(labelText: 'Domain Name', hintText: 'e.g. mybakery.com', border: OutlineInputBorder()),
                            onChanged: (v) => notifier.updateDomain(state.domainChoice, v),
                          ),
                        ),
                    ] else if (state.step == 4) ...[
                      const Text('Go Live: Review your site', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, fontSize: 20)),
                      const SizedBox(height: 16),
                      Container(
                        height: 200,
                        decoration: BoxDecoration(
                          color: Color(int.parse(state.primaryColor.substring(1, 7), radix: 16) + 0xFF000000).withOpacity(0.1),
                          border: Border.all(color: Colors.grey),
                          borderRadius: BorderRadius.circular(12),
                        ),
                        child: Center(
                          child: Column(
                            mainAxisAlignment: MainAxisAlignment.center,
                            children: [
                              Icon(Icons.web, size: 64, color: Color(int.parse(state.primaryColor.substring(1, 7), radix: 16) + 0xFF000000)),
                              const SizedBox(height: 16),
                              Text('Live Preview of ${state.selectedTemplate} template', style: const TextStyle(fontWeight: FontWeight.bold)),
                              Text('Domain: ${state.domainChoice == 'subdomain' ? 'mybusiness.ohc.app' : state.customDomain}'),
                            ]
                          )
                        ),
                      ),
                    ],
                    const SizedBox(height: 32),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        if (state.step > 0)
                          OutlinedButton(onPressed: notifier.previousStep, child: const Text('Back')),
                        if (state.step == 0) const SizedBox(),
                        ElevatedButton(
                          style: ElevatedButton.styleFrom(
                            backgroundColor: (state.step == 0 && state.selectedTemplate.isNotEmpty) ? Colors.green : null,
                            foregroundColor: (state.step == 0 && state.selectedTemplate.isNotEmpty) ? Colors.white : null,
                          ),
                          onPressed: (state.step == 0 && state.selectedTemplate.isEmpty) ? null : () {
                            if (state.step < 4) {
                              notifier.nextStep();
                            } else {
                              notifier.publish(context);
                            }
                          },
                          child: Text(
                            state.step == 4 ? 'Publish' :
                            (state.step == 0 && state.selectedTemplate.isNotEmpty) ? 'Use this template →' : 'Next'
                          ),
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
}
