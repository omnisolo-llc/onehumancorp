import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'dart:ui';
import '../widgets/glass_card.dart';

class WebsiteBuilderState {
  final int step;
  final String? selectedTemplate;
  final String? brandColor;
  final String? logoPath;
  final String productName;
  final String productPrice;
  final String productDescription;
  final String domainChoice;
  final bool isExpertMode;

  const WebsiteBuilderState({
    this.step = 0,
    this.selectedTemplate,
    this.brandColor,
    this.logoPath,
    this.productName = '',
    this.productPrice = '',
    this.productDescription = '',
    this.domainChoice = 'free',
    this.isExpertMode = false,
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
    bool? isExpertMode,
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
      isExpertMode: isExpertMode ?? this.isExpertMode,
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

  void updateBrandColor(String color) {
    state = state.copyWith(brandColor: color);
  }

  void updateLogo(String path) {
    state = state.copyWith(logoPath: path);
  }

  void updateProductName(String name) {
    state = state.copyWith(productName: name);
    if (name.isNotEmpty && state.productDescription.isEmpty) {
      state = state.copyWith(productDescription: 'Beautiful $name crafted with care.');
    }
  }

  void updateProductPrice(String price) {
    state = state.copyWith(productPrice: price);
  }

  void updateProductDescription(String desc) {
    state = state.copyWith(productDescription: desc);
  }

  void updateDomainChoice(String choice) {
    state = state.copyWith(domainChoice: choice);
  }

  void toggleExpertMode(bool val) {
    state = state.copyWith(isExpertMode: val);
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
            child: GlassCard(
              child: Padding(
                padding: const EdgeInsets.all(24.0),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        const Text(
                          'Website Builder',
                          style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white),
                        ),
                        Row(
                          children: [
                            const Text('Expert Mode', style: TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 12)),
                            Switch(
                              value: state.isExpertMode,
                              onChanged: notifier.toggleExpertMode,
                              activeColor: Colors.blueAccent,
                            ),
                          ],
                        ),
                      ],
                    ),
                    const SizedBox(height: 24),
                    AnimatedSwitcher(
                      duration: const Duration(milliseconds: 300),
                      transitionBuilder: (Widget child, Animation<double> animation) {
                        return FadeTransition(opacity: animation, child: child);
                      },
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
                            onPressed: notifier.prevStep,
                            child: const Text('Back', style: TextStyle(fontFamily: 'Inter', color: Colors.white70)),
                          )
                        else
                          const SizedBox(),
                        ElevatedButton(
                          onPressed: () {
                            if (state.step < 4) {
                              notifier.nextStep();
                            } else {
                              ScaffoldMessenger.of(context).showSnackBar(
                                const SnackBar(content: Text('Website Published!')),
                              );
                              context.go('/dashboard');
                            }
                          },
                          child: Text(
                            state.step == 4 ? 'Publish' : 'Next',
                            style: const TextStyle(fontFamily: 'Inter'),
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

  Widget _buildStepContent(BuildContext context, WebsiteBuilderState state, WebsiteBuilderNotifier notifier) {
    switch (state.step) {
      case 0:
        return _buildTemplateGallery(state, notifier);
      case 1:
        return _buildBrandColors(state, notifier);
      case 2:
        return _buildFirstProduct(state, notifier);
      case 3:
        return _buildDomainConnect(state, notifier);
      case 4:
        return _buildGoLive(state);
      default:
        return const SizedBox();
    }
  }

  Widget _buildTemplateGallery(WebsiteBuilderState state, WebsiteBuilderNotifier notifier) {
    final templates = ['E-commerce', 'Portfolio', 'Service', 'Blog'];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      children: [
        const Text('Choose a Template', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white)),
        const SizedBox(height: 16),
        Wrap(
          spacing: 16,
          runSpacing: 16,
          children: templates.map((template) {
            final isSelected = state.selectedTemplate == template;
            return GestureDetector(
              onTap: () => notifier.selectTemplate(template),
              child: Container(
                width: 120,
                height: 160,
                decoration: BoxDecoration(
                  color: isSelected ? Colors.blueAccent.withOpacity(0.2) : Colors.white.withOpacity(0.05),
                  border: Border.all(color: isSelected ? Colors.blueAccent : Colors.white24),
                  borderRadius: BorderRadius.circular(12),
                ),
                child: Center(
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Icon(Icons.web, color: isSelected ? Colors.blueAccent : Colors.white70, size: 40),
                      const SizedBox(height: 8),
                      Text(template, style: TextStyle(color: isSelected ? Colors.blueAccent : Colors.white70, fontFamily: 'Inter')),
                      if (isSelected)
                         const Padding(
                           padding: EdgeInsets.only(top: 8.0),
                           child: Text('Use this template →', style: TextStyle(color: Colors.green, fontFamily: 'Inter', fontSize: 12)),
                         ),
                    ],
                  ),
                ),
              ),
            );
          }).toList(),
        ),
      ],
    );
  }

  Widget _buildBrandColors(WebsiteBuilderState state, WebsiteBuilderNotifier notifier) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      children: [
        const Text('Brand Colors & Logo', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white)),
        const SizedBox(height: 16),
        const Text('Pick a color palette:', style: TextStyle(fontFamily: 'Inter', color: Colors.white70)),
        const SizedBox(height: 8),
        Row(
          children: ['#FF5733', '#33FF57', '#3357FF'].map((color) {
            final isSelected = state.brandColor == color;
            return GestureDetector(
              onTap: () => notifier.updateBrandColor(color),
              child: Container(
                margin: const EdgeInsets.only(right: 12),
                width: 40,
                height: 40,
                decoration: BoxDecoration(
                  color: Color(int.parse(color.replaceAll('#', '0xFF'))),
                  shape: BoxShape.circle,
                  border: Border.all(color: isSelected ? Colors.white : Colors.transparent, width: 2),
                ),
              ),
            );
          }).toList(),
        ),
        const SizedBox(height: 24),
        const Text('Upload Logo:', style: TextStyle(fontFamily: 'Inter', color: Colors.white70)),
        const SizedBox(height: 8),
        ElevatedButton.icon(
          onPressed: () => notifier.updateLogo('uploaded_logo.png'),
          icon: const Icon(Icons.upload),
          label: Text(state.logoPath ?? 'Choose File', style: const TextStyle(fontFamily: 'Inter')),
        ),
        if (state.isExpertMode) ...[
          const SizedBox(height: 16),
          const TextField(
            decoration: InputDecoration(
              labelText: 'Custom CSS (Expert)',
              labelStyle: TextStyle(color: Colors.white70),
              enabledBorder: OutlineInputBorder(borderSide: BorderSide(color: Colors.white24)),
              focusedBorder: OutlineInputBorder(borderSide: BorderSide(color: Colors.blueAccent)),
            ),
            style: TextStyle(color: Colors.white, fontFamily: 'monospace'),
            maxLines: 3,
          )
        ],
      ],
    );
  }

  Widget _buildFirstProduct(WebsiteBuilderState state, WebsiteBuilderNotifier notifier) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      children: [
        const Text('Add your first product or service', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white)),
        const SizedBox(height: 16),
        TextField(
          decoration: const InputDecoration(labelText: 'Name', labelStyle: TextStyle(color: Colors.white70)),
          onChanged: notifier.updateProductName,
          style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
          controller: TextEditingController.fromValue(TextEditingValue(text: state.productName, selection: TextSelection.collapsed(offset: state.productName.length))),
        ),
        const SizedBox(height: 16),
        TextField(
          decoration: const InputDecoration(labelText: 'Price', labelStyle: TextStyle(color: Colors.white70)),
          keyboardType: TextInputType.number,
          onChanged: notifier.updateProductPrice,
          style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
          controller: TextEditingController.fromValue(TextEditingValue(text: state.productPrice, selection: TextSelection.collapsed(offset: state.productPrice.length))),
        ),
        const SizedBox(height: 16),
        TextField(
          decoration: const InputDecoration(labelText: 'Description (AI Suggested)', labelStyle: TextStyle(color: Colors.white70)),
          maxLines: 3,
          onChanged: notifier.updateProductDescription,
          style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
          controller: TextEditingController.fromValue(TextEditingValue(text: state.productDescription, selection: TextSelection.collapsed(offset: state.productDescription.length))),
        ),
        if (state.isExpertMode) ...[
          const SizedBox(height: 16),
          const TextField(
             decoration: InputDecoration(labelText: 'SKU / Inventory ID', labelStyle: TextStyle(color: Colors.white70)),
             style: TextStyle(color: Colors.white, fontFamily: 'Inter'),
          )
        ]
      ],
    );
  }

  Widget _buildDomainConnect(WebsiteBuilderState state, WebsiteBuilderNotifier notifier) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      children: [
        const Text('Connect a domain', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white)),
        const SizedBox(height: 16),
        RadioListTile<String>(
          title: const Text('Use a free OHC subdomain (mybusiness.ohc.app)', style: TextStyle(color: Colors.white, fontFamily: 'Inter')),
          value: 'free',
          groupValue: state.domainChoice,
          onChanged: (val) => notifier.updateDomainChoice(val!),
          activeColor: Colors.blueAccent,
        ),
        RadioListTile<String>(
          title: const Text('Use my own domain', style: TextStyle(color: Colors.white, fontFamily: 'Inter')),
          value: 'own',
          groupValue: state.domainChoice,
          onChanged: (val) => notifier.updateDomainChoice(val!),
          activeColor: Colors.blueAccent,
        ),
        RadioListTile<String>(
          title: const Text('Buy a domain', style: TextStyle(color: Colors.white, fontFamily: 'Inter')),
          value: 'buy',
          groupValue: state.domainChoice,
          onChanged: (val) => notifier.updateDomainChoice(val!),
          activeColor: Colors.blueAccent,
        ),
        if (state.isExpertMode && state.domainChoice == 'own') ...[
           const SizedBox(height: 16),
           const Text('Configure DNS A Records to point to 192.168.1.100', style: TextStyle(color: Colors.white70, fontFamily: 'monospace')),
        ]
      ],
    );
  }

  Widget _buildGoLive(WebsiteBuilderState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      children: [
        const Text('Ready to Go Live', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white)),
        const SizedBox(height: 16),
        const Text('Your website is ready to be published to the world.', style: TextStyle(color: Colors.white70, fontFamily: 'Inter')),
        const SizedBox(height: 24),
        Container(
          padding: const EdgeInsets.all(16),
          decoration: BoxDecoration(
            color: Colors.white.withOpacity(0.05),
            borderRadius: BorderRadius.circular(12),
            border: Border.all(color: Colors.white24),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
               Text('Template: ${state.selectedTemplate ?? 'None'}', style: const TextStyle(color: Colors.white, fontFamily: 'Inter')),
               const SizedBox(height: 8),
               Text('Product: ${state.productName.isNotEmpty ? state.productName : 'None'}', style: const TextStyle(color: Colors.white, fontFamily: 'Inter')),
               const SizedBox(height: 8),
               Text('Domain: ${state.domainChoice == 'free' ? 'mybusiness.ohc.app' : 'Custom'}', style: const TextStyle(color: Colors.white, fontFamily: 'Inter')),
            ],
          ),
        ),
      ],
    );
  }
}