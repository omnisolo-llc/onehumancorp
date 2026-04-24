import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'dart:ui';
import '../widgets/glass_card.dart';
import 'package:flutter/services.dart';

class WebsiteBuilderState {
  final int step;
  final String selectedTemplate;
  final String brandColor;
  final String logoUrl;
  final String productName;
  final String productDescription;
  final String productPrice;
  final String domainChoice;
  final bool expertMode;

  const WebsiteBuilderState({
    this.step = 0,
    this.selectedTemplate = '',
    this.brandColor = '',
    this.logoUrl = '',
    this.productName = '',
    this.productDescription = '',
    this.productPrice = '',
    this.domainChoice = 'free',
    this.expertMode = false,
  });

  WebsiteBuilderState copyWith({
    int? step,
    String? selectedTemplate,
    String? brandColor,
    String? logoUrl,
    String? productName,
    String? productDescription,
    String? productPrice,
    String? domainChoice,
    bool? expertMode,
  }) {
    return WebsiteBuilderState(
      step: step ?? this.step,
      selectedTemplate: selectedTemplate ?? this.selectedTemplate,
      brandColor: brandColor ?? this.brandColor,
      logoUrl: logoUrl ?? this.logoUrl,
      productName: productName ?? this.productName,
      productDescription: productDescription ?? this.productDescription,
      productPrice: productPrice ?? this.productPrice,
      domainChoice: domainChoice ?? this.domainChoice,
      expertMode: expertMode ?? this.expertMode,
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

  void updateTemplate(String val) =>
      state = state.copyWith(selectedTemplate: val);
  void updateBrandColor(String val) => state = state.copyWith(brandColor: val);
  void updateLogoUrl(String val) => state = state.copyWith(logoUrl: val);
  void updateProductName(String val) =>
      state = state.copyWith(productName: val);
  void updateProductDescription(String val) =>
      state = state.copyWith(productDescription: val);
  void updateProductPrice(String val) =>
      state = state.copyWith(productPrice: val);
  void updateDomainChoice(String val) =>
      state = state.copyWith(domainChoice: val);
  void updateExpertMode(bool val) => state = state.copyWith(expertMode: val);
  void generateAIDescription() {
    state = state.copyWith(
      productDescription: "AI generated description for ${state.productName}",
    );
  }
}

final websiteBuilderProvider =
    NotifierProvider<WebsiteBuilderNotifier, WebsiteBuilderState>(() {
  return WebsiteBuilderNotifier();
});

class WebsiteBuilderWizardScreen extends ConsumerStatefulWidget {
  const WebsiteBuilderWizardScreen({super.key});

  @override
  ConsumerState<WebsiteBuilderWizardScreen> createState() =>
      _WebsiteBuilderWizardScreenState();
}

class _WebsiteBuilderWizardScreenState
    extends ConsumerState<WebsiteBuilderWizardScreen> {
  Widget _buildGlassmorphism({required Widget child}) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: BackdropFilter(
        filter: ImageFilter.compose(
          outer: const ColorFilter.matrix(<double>[
            1.168,
            -0.153,
            -0.015,
            0,
            0,
            -0.046,
            1.061,
            -0.015,
            0,
            0,
            -0.046,
            -0.152,
            1.198,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
          ]),
          inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        ),
        child: Container(
          padding: const EdgeInsets.all(16),
          decoration: BoxDecoration(
            color: Colors.white.withOpacity(0.05),
            borderRadius: BorderRadius.circular(12),
            border: Border.all(color: Colors.white.withOpacity(0.1)),
          ),
          child: child,
        ),
      ),
    );
  }

  Widget _buildTemplateGallery(
    WebsiteBuilderState state,
    WebsiteBuilderNotifier notifier,
  ) {
    final templates = [
      'Modern E-commerce',
      'Creative Portfolio',
      'Local Restaurant',
      'Service Booking',
    ];

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Select a template to start with.',
          style: TextStyle(fontFamily: 'Inter', fontSize: 16),
        ),
        const SizedBox(height: 16),
        GridView.builder(
          shrinkWrap: true,
          physics: const NeverScrollableScrollPhysics(),
          gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
            crossAxisCount: 2,
            crossAxisSpacing: 16,
            mainAxisSpacing: 16,
            childAspectRatio: 0.8,
          ),
          itemCount: templates.length,
          itemBuilder: (context, index) {
            final template = templates[index];
            final isSelected = state.selectedTemplate == template;
            return GestureDetector(
              onTap: () => notifier.updateTemplate(template),
              child: _buildGlassmorphism(
                child: Column(
                  children: [
                    Expanded(
                      child: Container(
                        decoration: BoxDecoration(
                          color: Colors.grey.withOpacity(0.3),
                          borderRadius: BorderRadius.circular(8),
                        ),
                        child: Center(
                          child: Icon(
                            Icons.web,
                            size: 48,
                            color: Colors.white.withOpacity(0.5),
                          ),
                        ),
                      ),
                    ),
                    const SizedBox(height: 8),
                    Text(
                      template,
                      style: const TextStyle(
                        fontFamily: 'Inter',
                        fontWeight: FontWeight.bold,
                      ),
                      textAlign: TextAlign.center,
                    ),
                    const SizedBox(height: 8),
                    ElevatedButton(
                      onPressed: () => notifier.updateTemplate(template),
                      style: ElevatedButton.styleFrom(
                        backgroundColor:
                            isSelected
                                ? Colors.green
                                : Theme.of(context).colorScheme.primary,
                      ),
                      child: Text(
                        isSelected ? 'Selected ✓' : 'Use this template →',
                      ),
                    ),
                  ],
                ),
              ),
            );
          },
        ),
      ],
    );
  }

  Widget _buildBrandAndLogo(
    WebsiteBuilderState state,
    WebsiteBuilderNotifier notifier,
  ) {
    return _buildGlassmorphism(
      child: Column(
        children: [
          const Text(
            'Pick your brand color palette.',
            style: TextStyle(fontFamily: 'Inter', fontSize: 16),
          ),
          const SizedBox(height: 16),
          Wrap(
            alignment: WrapAlignment.spaceEvenly,
            spacing: 8,
            runSpacing: 8,
            children:
                ['Blue/Gold', 'Green/Cream', 'Purple/Pink'].map((color) {
                  return ChoiceChip(
                    label: Text(color),
                    selected: state.brandColor == color,
                    onSelected: (selected) {
                      if (selected) notifier.updateBrandColor(color);
                    },
                  );
                }).toList(),
          ),
          const SizedBox(height: 32),
          const Text(
            'Upload a logo or generate one with AI.',
            style: TextStyle(fontFamily: 'Inter', fontSize: 16),
          ),
          const SizedBox(height: 16),
          ElevatedButton.icon(
            onPressed: () => notifier.updateLogoUrl('uploaded_logo.png'),
            icon: const Icon(Icons.upload_file),
            label: const Text('Upload Logo'),
          ),
          const SizedBox(height: 8),
          OutlinedButton.icon(
            onPressed: () => notifier.updateLogoUrl('ai_generated_logo.png'),
            icon: const Icon(Icons.auto_awesome),
            label: const Text('Generate a logo for me'),
          ),
          if (state.logoUrl.isNotEmpty) ...[
            const SizedBox(height: 16),
            Text(
              'Selected Logo: ${state.logoUrl}',
              style: const TextStyle(color: Colors.green, fontFamily: 'Inter'),
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildAddProduct(
    WebsiteBuilderState state,
    WebsiteBuilderNotifier notifier,
  ) {
    return _buildGlassmorphism(
      child: Column(
        children: [
          TextField(
            decoration: const InputDecoration(
              labelText: 'Product / Service Name',
            ),
            onChanged: notifier.updateProductName,
            controller:
                TextEditingController(text: state.productName)
                  ..selection = TextSelection.collapsed(
                    offset: state.productName.length,
                  ),
          ),
          const SizedBox(height: 16),
          TextField(
            decoration: const InputDecoration(labelText: 'Price'),
            keyboardType: TextInputType.number,
            onChanged: notifier.updateProductPrice,
            controller:
                TextEditingController(text: state.productPrice)
                  ..selection = TextSelection.collapsed(
                    offset: state.productPrice.length,
                  ),
          ),
          const SizedBox(height: 16),
          Row(
            children: [
              Expanded(
                child: TextField(
                  decoration: const InputDecoration(
                    labelText: 'Short Description',
                  ),
                  onChanged: notifier.updateProductDescription,
                  controller:
                      TextEditingController(text: state.productDescription)
                        ..selection = TextSelection.collapsed(
                          offset: state.productDescription.length,
                        ),
                  maxLines: 3,
                ),
              ),
              IconButton(
                icon: const Icon(Icons.auto_awesome, color: Colors.blueAccent),
                tooltip: 'AI Auto-generate',
                onPressed: () => notifier.generateAIDescription(),
              ),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildDomain(
    WebsiteBuilderState state,
    WebsiteBuilderNotifier notifier,
  ) {
    return _buildGlassmorphism(
      child: Column(
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text(
                'Progressive Disclosure Mode',
                style: TextStyle(
                  fontFamily: 'Inter',
                  fontWeight: FontWeight.bold,
                ),
              ),
              Switch(
                value: state.expertMode,
                onChanged: notifier.updateExpertMode,
              ),
            ],
          ),
          const SizedBox(height: 16),
          RadioListTile<String>(
            title: const Text(
              'Use a free OHC subdomain (mybusiness.ohc.app)',
              style: TextStyle(fontFamily: 'Inter'),
            ),
            value: 'free',
            groupValue: state.domainChoice,
            onChanged: (v) => notifier.updateDomainChoice(v!),
          ),
          RadioListTile<String>(
            title: const Text(
              'Use my own domain',
              style: TextStyle(fontFamily: 'Inter'),
            ),
            value: 'own',
            groupValue: state.domainChoice,
            onChanged: (v) => notifier.updateDomainChoice(v!),
          ),
          RadioListTile<String>(
            title: const Text(
              'Buy a domain',
              style: TextStyle(fontFamily: 'Inter'),
            ),
            value: 'buy',
            groupValue: state.domainChoice,
            onChanged: (v) => notifier.updateDomainChoice(v!),
          ),
          if (state.expertMode && state.domainChoice == 'own') ...[
            const SizedBox(height: 16),
            const TextField(
              decoration: InputDecoration(
                labelText: 'Custom Domain Name (e.g. example.com)',
              ),
            ),
            const SizedBox(height: 16),
            const TextField(
              decoration: InputDecoration(labelText: 'DNS A Record IP'),
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildGoLive(WebsiteBuilderState state) {
    return _buildGlassmorphism(
      child: Column(
        children: [
          const Text(
            'Preview your live site!',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 20,
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 16),
          Container(
            height: 200,
            decoration: BoxDecoration(
              color: Colors.grey.withOpacity(0.2),
              borderRadius: BorderRadius.circular(12),
            ),
            child: const Center(
              child: Icon(Icons.preview, size: 64, color: Colors.white54),
            ),
          ),
          const SizedBox(height: 24),
          ElevatedButton(
            onPressed: () {
              Clipboard.setData(
                const ClipboardData(text: 'https://mybusiness.ohc.app'),
              );
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(content: Text('Link copied to clipboard!')),
              );
              context.go('/dashboard');
            },
            style: ElevatedButton.styleFrom(
              padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 16),
              backgroundColor: Colors.blueAccent,
            ),
            child: const Text(
              'Publish',
              style: TextStyle(fontFamily: 'Inter', fontSize: 18),
            ),
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(websiteBuilderProvider);
    final notifier = ref.read(websiteBuilderProvider.notifier);

    return Scaffold(
      appBar: AppBar(
        title: const Text(
          'Website Builder',
          style: TextStyle(fontFamily: 'Outfit'),
        ),
        leading: IconButton(
          icon: const Icon(Icons.close),
          onPressed: () => context.go('/dashboard'),
        ),
      ),
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
            constraints: const BoxConstraints(maxWidth: 600),
            child: Theme(
              data: Theme.of(context).copyWith(canvasColor: Colors.transparent),
              child: Stepper(
                currentStep: state.step,
                onStepContinue: () {
                  if (state.step < 4) {
                    notifier.nextStep();
                  }
                },
                onStepCancel: notifier.prevStep,
                controlsBuilder: (context, details) {
                  if (state.step == 4) {
                    return const SizedBox.shrink();
                  } // Hide controls on Go Live step
                  return Padding(
                    padding: const EdgeInsets.only(top: 16.0),
                    child: Row(
                      children: [
                        ElevatedButton(
                          onPressed: details.onStepContinue,
                          child: const Text('Next'),
                        ),
                        const SizedBox(width: 8),
                        if (state.step > 0)
                          TextButton(
                            onPressed: details.onStepCancel,
                            child: const Text('Back'),
                          ),
                      ],
                    ),
                  );
                },
                steps: [
                  Step(
                    title: const Text(
                      'Template',
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        color: Colors.white,
                      ),
                    ),
                    content: _buildTemplateGallery(state, notifier),
                    isActive: state.step >= 0,
                  ),
                  Step(
                    title: const Text(
                      'Brand',
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        color: Colors.white,
                      ),
                    ),
                    content: _buildBrandAndLogo(state, notifier),
                    isActive: state.step >= 1,
                  ),
                  Step(
                    title: const Text(
                      'Product',
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        color: Colors.white,
                      ),
                    ),
                    content: _buildAddProduct(state, notifier),
                    isActive: state.step >= 2,
                  ),
                  Step(
                    title: const Text(
                      'Domain',
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        color: Colors.white,
                      ),
                    ),
                    content: _buildDomain(state, notifier),
                    isActive: state.step >= 3,
                  ),
                  Step(
                    title: const Text(
                      'Go Live',
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        color: Colors.white,
                      ),
                    ),
                    content: _buildGoLive(state),
                    isActive: state.step >= 4,
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
