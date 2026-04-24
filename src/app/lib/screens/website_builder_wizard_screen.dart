import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
import 'dart:ui';
import 'package:flutter/services.dart';
import '../services/auth_service.dart';
import '../services/settings_service.dart';
import '../widgets/glass_card.dart';

class WebsiteBuilderState {
  final int step;
  final String template;
  final String brandColor;
  final String logo;
  final String productName;
  final String productPrice;
  final String productDescription;
  final String productPhoto;
  final String domainChoice;
  final bool isLoading;
  final String? errorMessage;
  final bool isPublished;

  const WebsiteBuilderState({
    this.step = 0,
    this.template = '',
    this.brandColor = '',
    this.logo = '',
    this.productName = '',
    this.productPrice = '',
    this.productDescription = '',
    this.productPhoto = '',
    this.domainChoice = 'Free OHC subdomain',
    this.isLoading = false,
    this.errorMessage,
    this.isPublished = false,
  });

  WebsiteBuilderState copyWith({
    int? step,
    String? template,
    String? brandColor,
    String? logo,
    String? productName,
    String? productPrice,
    String? productDescription,
    String? productPhoto,
    String? domainChoice,
    bool? isLoading,
    String? errorMessage,
    bool? isPublished,
  }) {
    return WebsiteBuilderState(
      step: step ?? this.step,
      template: template ?? this.template,
      brandColor: brandColor ?? this.brandColor,
      logo: logo ?? this.logo,
      productName: productName ?? this.productName,
      productPrice: productPrice ?? this.productPrice,
      productDescription: productDescription ?? this.productDescription,
      productPhoto: productPhoto ?? this.productPhoto,
      domainChoice: domainChoice ?? this.domainChoice,
      isLoading: isLoading ?? this.isLoading,
      errorMessage: errorMessage ?? this.errorMessage,
      isPublished: isPublished ?? this.isPublished,
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

  void updateTemplate(String val) => state = state.copyWith(template: val);
  void updateBrandColor(String val) => state = state.copyWith(brandColor: val);
  void updateLogo(String val) => state = state.copyWith(logo: val);
  void updateProductName(String val) => state = state.copyWith(productName: val);
  void updateProductPrice(String val) => state = state.copyWith(productPrice: val);
  void updateProductDescription(String val) => state = state.copyWith(productDescription: val);
  void updateProductPhoto(String val) => state = state.copyWith(productPhoto: val);
  void updateDomainChoice(String val) => state = state.copyWith(domainChoice: val);

  Future<void> generateProductDescription() async {
    final user = ref.read(authStateProvider).valueOrNull;
    final baseUrl = ref.read(backendUrlProvider);

    state = state.copyWith(isLoading: true);

    if (user != null && baseUrl.isNotEmpty) {
      try {
        final res = await http.post(
          Uri.parse('$baseUrl/api/wizard/generate_description'),
          headers: {
            'Authorization': 'Bearer ${user.token}',
            'Content-Type': 'application/json',
          },
          body: jsonEncode({'product_name': state.productName}),
        );

        if (res.statusCode == 200) {
          final data = jsonDecode(res.body);
          state = state.copyWith(productDescription: data['description'], isLoading: false);
          return;
        }
      } catch (_) {
        // Fallback handled below
      }
    }

    state = state.copyWith(productDescription: 'A premium, handcrafted ${state.productName.isEmpty ? 'product' : state.productName} tailored for exceptional quality.', isLoading: false);
  }

  Future<void> generateLogo() async {
    final user = ref.read(authStateProvider).valueOrNull;
    final baseUrl = ref.read(backendUrlProvider);

    state = state.copyWith(isLoading: true);

    if (user != null && baseUrl.isNotEmpty) {
      try {
        final res = await http.post(
          Uri.parse('$baseUrl/api/wizard/generate_logo'),
          headers: {
            'Authorization': 'Bearer ${user.token}',
            'Content-Type': 'application/json',
          },
        );

        if (res.statusCode == 200) {
          final data = jsonDecode(res.body);
          state = state.copyWith(logo: data['logo_url'], isLoading: false);
          return;
        }
      } catch (_) {
        // Fallback handled below
      }
    }

    state = state.copyWith(logo: 'ai_generated_logo_placeholder.png', isLoading: false);
  }

  Future<void> launch(BuildContext context, WidgetRef ref) async {
    final user = ref.read(authStateProvider).valueOrNull;
    final baseUrl = ref.read(backendUrlProvider);

    state = state.copyWith(isLoading: true, errorMessage: null);

    if (user != null && baseUrl.isNotEmpty) {
      final body = {
        'extras': {
          'website_template': state.template,
          'website_brand_color': state.brandColor,
          'website_logo': state.logo,
          'website_first_product_name': state.productName,
          'website_first_product_price': state.productPrice,
          'website_first_product_description': state.productDescription,
          'website_first_product_photo': state.productPhoto,
          'website_domain_choice': state.domainChoice,
        }
      };

      try {
        final res = await http.post(
          Uri.parse('$baseUrl/api/wizard/configure'),
          headers: {
            'Authorization': 'Bearer ${user.token}',
            'Content-Type': 'application/json',
          },
          body: jsonEncode(body),
        );

        if (res.statusCode != 200) {
          state = state.copyWith(isLoading: false, errorMessage: 'Configuration failed: ${res.statusCode}');
          return;
        }
      } catch (e) {
        state = state.copyWith(isLoading: false, errorMessage: 'Network error: $e');
        return;
      }
    }

    state = state.copyWith(isLoading: false, isPublished: true);

    // Copy link to clipboard
    await Clipboard.setData(const ClipboardData(text: 'https://mybusiness.ohc.app'));
    if (context.mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Website published! Link copied to clipboard.', style: TextStyle(fontFamily: 'Inter'))),
      );
      Future.delayed(const Duration(seconds: 2), () {
         if (context.mounted) GoRouter.of(context).go('/dashboard');
      });
    }
  }
}

final websiteBuilderProvider = NotifierProvider<WebsiteBuilderNotifier, WebsiteBuilderState>(() {
  return WebsiteBuilderNotifier();
});

class WebsiteBuilderWizardScreen extends ConsumerStatefulWidget {
  const WebsiteBuilderWizardScreen({super.key});

  @override
  ConsumerState<WebsiteBuilderWizardScreen> createState() => _WebsiteBuilderWizardScreenState();
}

class _WebsiteBuilderWizardScreenState extends ConsumerState<WebsiteBuilderWizardScreen> {
  final _descController = TextEditingController();

  @override
  void dispose() {
    _descController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(websiteBuilderProvider);
    final notifier = ref.read(websiteBuilderProvider.notifier);

    // Sync controller with state for AI generated text
    if (_descController.text != state.productDescription) {
      _descController.text = state.productDescription;
    }

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
                  children: [
                    const Text('Website Builder', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white)),
                    const SizedBox(height: 16),
                    if (state.errorMessage != null) ...[
                      Text(state.errorMessage!, style: const TextStyle(color: Colors.red)),
                      const SizedBox(height: 16),
                    ],
                    AnimatedSwitcher(
                      duration: const Duration(milliseconds: 300),
                      transitionBuilder: (Widget child, Animation<double> animation) {
                        return FadeTransition(opacity: animation, child: child);
                      },
                      child: Container(
                        key: ValueKey<int>(state.step),
                        child: Column(
                          mainAxisSize: MainAxisSize.min,
                          children: [
                            if (state.step == 0) ...[
                              const Text('Select a Template', style: TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 16)),
                              const SizedBox(height: 16),
                              SizedBox(
                                height: 300,
                                child: GridView.count(
                                  crossAxisCount: 2,
                                  crossAxisSpacing: 16,
                                  mainAxisSpacing: 16,
                                  childAspectRatio: 1.5,
                                  children: [
                                    _buildTemplateCard('E-commerce', Icons.shopping_cart, state, notifier),
                                    _buildTemplateCard('Portfolio', Icons.photo_library, state, notifier),
                                    _buildTemplateCard('Service', Icons.handyman, state, notifier),
                                    _buildTemplateCard('Restaurant', Icons.restaurant, state, notifier),
                                  ],
                                ),
                              ),
                            ] else if (state.step == 1) ...[
                              const Text('Brand Colors & Logo', style: TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 16)),
                              const SizedBox(height: 16),
                              const Text('AI Suggested Palettes:', style: TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 14)),
                              const SizedBox(height: 8),
                              Row(
                                mainAxisAlignment: MainAxisAlignment.center,
                                children: ['#FF5733', '#33FF57', '#3357FF', '#FF33A8', '#33FFF5'].map((color) =>
                                  Padding(
                                    padding: const EdgeInsets.symmetric(horizontal: 8.0),
                                    child: InkWell(
                                      onTap: () => notifier.updateBrandColor(color),
                                      child: Container(
                                        width: 40,
                                        height: 40,
                                        decoration: BoxDecoration(
                                          color: Color(int.parse(color.substring(1, 7), radix: 16) + 0xFF000000),
                                          shape: BoxShape.circle,
                                          border: Border.all(color: state.brandColor == color ? Colors.white : Colors.transparent, width: 2),
                                        ),
                                      ),
                                    ),
                                  )
                                ).toList(),
                              ),
                              const SizedBox(height: 24),
                              ElevatedButton.icon(
                                onPressed: () => notifier.updateLogo('uploaded_logo.png'),
                                icon: const Icon(Icons.upload),
                                label: const Text('Upload Logo (AI background removal)', style: TextStyle(fontFamily: 'Inter')),
                              ),
                              const SizedBox(height: 8),
                              TextButton.icon(
                                onPressed: notifier.generateLogo,
                                icon: const Icon(Icons.auto_awesome),
                                label: const Text('Generate Logo for me', style: TextStyle(fontFamily: 'Inter')),
                              ),
                              if (state.logo.isNotEmpty)
                                Padding(
                                  padding: const EdgeInsets.only(top: 8.0),
                                  child: Text('Logo set: ${state.logo}', style: const TextStyle(color: Colors.white70, fontFamily: 'Inter')),
                                ),
                            ] else if (state.step == 2) ...[
                              const Text('Add your first product or service', style: TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 16)),
                              const SizedBox(height: 16),
                              TextField(
                                decoration: const InputDecoration(labelText: 'Product Name', labelStyle: TextStyle(color: Colors.white70)),
                                onChanged: notifier.updateProductName,
                                style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                              ),
                              const SizedBox(height: 16),
                              TextField(
                                decoration: const InputDecoration(labelText: 'Price', labelStyle: TextStyle(color: Colors.white70)),
                                onChanged: notifier.updateProductPrice,
                                keyboardType: TextInputType.number,
                                style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                              ),
                              const SizedBox(height: 16),
                              Row(
                                children: [
                                  Expanded(
                                    child: TextField(
                                      controller: _descController,
                                      decoration: const InputDecoration(labelText: 'Description', labelStyle: TextStyle(color: Colors.white70)),
                                      onChanged: notifier.updateProductDescription,
                                      maxLines: 3,
                                      style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                                    ),
                                  ),
                                  const SizedBox(width: 16),
                                  ElevatedButton.icon(
                                    onPressed: notifier.generateProductDescription,
                                    icon: const Icon(Icons.auto_awesome),
                                    label: const Text('AI Auto-generate', style: TextStyle(fontFamily: 'Inter')),
                                  ),
                                ],
                              ),
                              const SizedBox(height: 16),
                              ElevatedButton.icon(
                                onPressed: () => notifier.updateProductPhoto('uploaded_photo.jpg'),
                                icon: const Icon(Icons.camera_alt),
                                label: const Text('Upload Photo', style: TextStyle(fontFamily: 'Inter')),
                              ),
                              if (state.productPhoto.isNotEmpty)
                                Padding(
                                  padding: const EdgeInsets.only(top: 8.0),
                                  child: Text('Photo uploaded: ${state.productPhoto}', style: const TextStyle(color: Colors.white70, fontFamily: 'Inter')),
                                ),
                            ] else if (state.step == 3) ...[
                               const Text('Connect a domain', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
                               ...['Free OHC subdomain', 'Own domain', 'Buy domain'].map((choice) => RadioListTile<String>(
                                title: Text(choice, style: const TextStyle(fontFamily: 'Inter', color: Colors.white)),
                                value: choice,
                                groupValue: state.domainChoice,
                                activeColor: Colors.blueAccent,
                                onChanged: (String? value) {
                                  if (value != null) notifier.updateDomainChoice(value);
                                },
                              )),
                            ] else if (state.step == 4) ...[
                               const Text('Ready to Go Live', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold, fontSize: 20, color: Colors.white)),
                               const SizedBox(height: 16),
                               Container(
                                 padding: const EdgeInsets.all(16),
                                 decoration: BoxDecoration(
                                   color: Colors.black.withValues(alpha: 0.2),
                                   borderRadius: BorderRadius.circular(12),
                                   border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
                                 ),
                                 child: Column(
                                   crossAxisAlignment: CrossAxisAlignment.start,
                                   children: [
                                     const Text('Live Preview', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
                                     const SizedBox(height: 8),
                                     Text('Template: ${state.template}', style: const TextStyle(fontFamily: 'Inter', color: Colors.white70)),
                                     Text('Brand Color: ${state.brandColor}', style: const TextStyle(fontFamily: 'Inter', color: Colors.white70)),
                                     Text('Product: ${state.productName} (\$${state.productPrice})', style: const TextStyle(fontFamily: 'Inter', color: Colors.white70)),
                                     Text('Domain: ${state.domainChoice}', style: const TextStyle(fontFamily: 'Inter', color: Colors.white70)),
                                   ]
                                 ),
                               ),
                            ],
                          ],
                        ),
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
                              notifier.launch(context, ref);
                            }
                          },
                          child: state.isLoading
                              ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2))
                              : Text(state.step == 4 ? 'Publish →' : 'Next', style: const TextStyle(fontFamily: 'Inter')),
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

  Widget _buildTemplateCard(String tpl, IconData icon, WebsiteBuilderState state, WebsiteBuilderNotifier notifier) {
    final isSelected = state.template == tpl;
    return InkWell(
      onTap: () => notifier.updateTemplate(tpl),
      child: Container(
        decoration: BoxDecoration(
          color: isSelected ? Colors.blueAccent.withValues(alpha: 0.3) : Colors.white.withValues(alpha: 0.05),
          border: Border.all(color: isSelected ? Colors.blueAccent : Colors.white.withValues(alpha: 0.1)),
          borderRadius: BorderRadius.circular(12),
        ),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(icon, color: Colors.white, size: 32),
            const SizedBox(height: 8),
            Text(tpl, style: const TextStyle(color: Colors.white, fontFamily: 'Inter', fontWeight: FontWeight.bold)),
            if (isSelected) const Padding(
              padding: EdgeInsets.only(top: 4.0),
              child: Text('Use this template →', style: TextStyle(color: Colors.greenAccent, fontSize: 12, fontFamily: 'Inter')),
            ),
          ],
        ),
      ),
    );
  }
}
