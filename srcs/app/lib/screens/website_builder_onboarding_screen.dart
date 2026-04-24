import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:flutter/services.dart';
import '../services/auth_service.dart';
import '../services/settings_service.dart';
import '../widgets/glass_card.dart';

class WebsiteBuilderState {
  final int step;
  final bool isLoading;
  final String selectedTemplate;
  final Color selectedColor;
  final String? logoUrl;
  final String productName;
  final String productPrice;
  final String productDesc;
  final String domainChoice;
  final String? errorMessage;
  final bool advancedMode;

  const WebsiteBuilderState({
    this.step = 0,
    this.isLoading = false,
    this.selectedTemplate = '',
    this.selectedColor = Colors.blue,
    this.logoUrl,
    this.productName = '',
    this.productPrice = '',
    this.productDesc = '',
    this.domainChoice = 'subdomain',
    this.errorMessage,
    this.advancedMode = false,
  });

  WebsiteBuilderState copyWith({
    int? step,
    bool? isLoading,
    String? selectedTemplate,
    Color? selectedColor,
    String? logoUrl,
    String? productName,
    String? productPrice,
    String? productDesc,
    String? domainChoice,
    String? errorMessage,
    bool? advancedMode,
  }) {
    return WebsiteBuilderState(
      step: step ?? this.step,
      isLoading: isLoading ?? this.isLoading,
      selectedTemplate: selectedTemplate ?? this.selectedTemplate,
      selectedColor: selectedColor ?? this.selectedColor,
      logoUrl: logoUrl != null ? (logoUrl == 'null' ? null : logoUrl) : this.logoUrl,
      productName: productName ?? this.productName,
      productPrice: productPrice ?? this.productPrice,
      productDesc: productDesc ?? this.productDesc,
      domainChoice: domainChoice ?? this.domainChoice,
      errorMessage: errorMessage != null ? (errorMessage == 'null' ? null : errorMessage) : this.errorMessage,
      advancedMode: advancedMode ?? this.advancedMode,
    );
  }
}

class WebsiteBuilderNotifier extends Notifier<WebsiteBuilderState> {
  @override
  WebsiteBuilderState build() => const WebsiteBuilderState();

  void nextStep() => state = state.copyWith(step: state.step + 1);
  void prevStep() => state = state.copyWith(step: state.step > 0 ? state.step - 1 : 0);
  void setTemplate(String t) => state = state.copyWith(selectedTemplate: t);
  void setColor(Color c) => state = state.copyWith(selectedColor: c);
  void setLogo(String l) => state = state.copyWith(logoUrl: l);
  void setProductName(String n) => state = state.copyWith(productName: n);
  void setProductPrice(String p) => state = state.copyWith(productPrice: p);
  void setProductDesc(String d) => state = state.copyWith(productDesc: d);
  void setDomainChoice(String d) => state = state.copyWith(domainChoice: d);
  void toggleAdvancedMode() => state = state.copyWith(advancedMode: !state.advancedMode);

  Future<void> publish(BuildContext context, WidgetRef ref) async {
    final user = ref.read(authStateProvider).valueOrNull;
    final baseUrl = ref.read(backendUrlProvider);

    state = state.copyWith(isLoading: true, errorMessage: 'null');

    if (user == null || baseUrl.isEmpty) {
      state = state.copyWith(isLoading: false, errorMessage: 'Not authenticated or backend URL missing');
      return;
    }

    final body = {
      'template': state.selectedTemplate,
      'color': state.selectedColor.value.toString(),
      'logoUrl': state.logoUrl ?? '',
      'productName': state.productName,
      'productPrice': state.productPrice,
      'productDesc': state.productDesc,
      'domainChoice': state.domainChoice,
    };

    try {
      final res = await http.post(
        Uri.parse('$baseUrl/api/wizard/website'),
        headers: {
          'Authorization': 'Bearer ${user.token}',
          'Content-Type': 'application/json',
        },
        body: jsonEncode(body),
      );

      if (res.statusCode != 200) {
        state = state.copyWith(isLoading: false, errorMessage: 'Publish failed: ${res.statusCode}');
        return;
      }
    } catch (e) {
      state = state.copyWith(isLoading: false, errorMessage: 'Network error: $e');
      return;
    }

    state = state.copyWith(isLoading: false);

    // Auto-copy shareable link to clipboard per requirement
    const shareableLink = 'https://mybusiness.ohc.app';
    Clipboard.setData(const ClipboardData(text: shareableLink));

    if (context.mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Website published successfully! Link copied to clipboard.')),
      );
      context.go('/dashboard');
    }
  }
}

final websiteBuilderProvider = NotifierProvider<WebsiteBuilderNotifier, WebsiteBuilderState>(() {
  return WebsiteBuilderNotifier();
});

class WebsiteBuilderOnboardingScreen extends ConsumerStatefulWidget {
  const WebsiteBuilderOnboardingScreen({super.key});

  @override
  ConsumerState<WebsiteBuilderOnboardingScreen> createState() => _WebsiteBuilderOnboardingScreenState();
}

class _WebsiteBuilderOnboardingScreenState extends ConsumerState<WebsiteBuilderOnboardingScreen> {
  final _productNameCtrl = TextEditingController();
  final _productPriceCtrl = TextEditingController();
  final _productDescCtrl = TextEditingController();

  @override
  void dispose() {
    _productNameCtrl.dispose();
    _productPriceCtrl.dispose();
    _productDescCtrl.dispose();
    super.dispose();
  }

  Widget _buildStep0_Template(WebsiteBuilderState state, WebsiteBuilderNotifier notifier) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Choose a Template', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold, fontSize: 20, color: Colors.white)),
        const SizedBox(height: 8),
        const Text('Select a starting point. AI will customize it for you.', style: TextStyle(fontFamily: 'Inter', color: Colors.white70)),
        const SizedBox(height: 16),
        Wrap(
          spacing: 16,
          runSpacing: 16,
          children: [
            _TemplateCard(title: 'Modern Retail', industry: 'Retail', selected: state.selectedTemplate == 'modern_retail', onTap: () => notifier.setTemplate('modern_retail')),
            _TemplateCard(title: 'Service Booking', industry: 'Services', selected: state.selectedTemplate == 'service_booking', onTap: () => notifier.setTemplate('service_booking')),
            _TemplateCard(title: 'Food & Menu', industry: 'Restaurant', selected: state.selectedTemplate == 'food_menu', onTap: () => notifier.setTemplate('food_menu')),
          ],
        ),
      ],
    );
  }

  Widget _buildStep1_Brand(WebsiteBuilderState state, WebsiteBuilderNotifier notifier) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Brand Colors & Logo', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold, fontSize: 20, color: Colors.white)),
        const SizedBox(height: 16),
        const Text('Color Palette', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
        const SizedBox(height: 8),
        Row(
          children: [
            _ColorPickerItem(color: Colors.blue, selected: state.selectedColor == Colors.blue, onTap: () => notifier.setColor(Colors.blue)),
            _ColorPickerItem(color: Colors.green, selected: state.selectedColor == Colors.green, onTap: () => notifier.setColor(Colors.green)),
            _ColorPickerItem(color: Colors.orange, selected: state.selectedColor == Colors.orange, onTap: () => notifier.setColor(Colors.orange)),
          ],
        ),
        const SizedBox(height: 24),
        const Text('Logo', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
        const SizedBox(height: 8),
        Row(
          children: [
            OutlinedButton.icon(
              onPressed: () => notifier.setLogo('uploaded_logo'),
              icon: const Icon(Icons.upload),
              label: const Text('Upload'),
            ),
            const SizedBox(width: 8),
            OutlinedButton.icon(
              onPressed: () => notifier.setLogo('ai_generated'),
              icon: const Icon(Icons.auto_awesome),
              label: const Text('Generate'),
            ),
          ],
        ),
        if (state.logoUrl != null) ...[
          const SizedBox(height: 8),
          Text('Logo ready!', style: TextStyle(color: Colors.green.shade300, fontFamily: 'Inter')),
        ]
      ],
    );
  }

  Widget _buildStep2_Product(WebsiteBuilderState state, WebsiteBuilderNotifier notifier) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Add your first offering', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold, fontSize: 20, color: Colors.white)),
        const SizedBox(height: 16),
        TextField(
          controller: _productNameCtrl,
          onChanged: notifier.setProductName,
          style: const TextStyle(color: Colors.white),
          decoration: const InputDecoration(labelText: 'Name', labelStyle: TextStyle(color: Colors.white70)),
        ),
        const SizedBox(height: 16),
        OutlinedButton.icon(
          onPressed: () {},
          icon: const Icon(Icons.add_a_photo),
          label: const Text('Upload Photo'),
        ),
        const SizedBox(height: 16),
        TextField(
          controller: _productPriceCtrl,
          onChanged: notifier.setProductPrice,
          style: const TextStyle(color: Colors.white),
          decoration: const InputDecoration(labelText: 'Price', labelStyle: TextStyle(color: Colors.white70)),
          keyboardType: TextInputType.number,
        ),
        const SizedBox(height: 16),
        TextField(
          controller: _productDescCtrl,
          onChanged: notifier.setProductDesc,
          style: const TextStyle(color: Colors.white),
          decoration: InputDecoration(
            labelText: 'Short Description',
            labelStyle: const TextStyle(color: Colors.white70),
            suffixIcon: IconButton(
              icon: const Icon(Icons.auto_awesome, color: Colors.white70),
              onPressed: () {
                _productDescCtrl.text = "AI Generated: Premium quality offering for your needs.";
                notifier.setProductDesc(_productDescCtrl.text);
              },
            ),
          ),
          maxLines: 2,
        ),
        if (state.advancedMode) ...[
          const SizedBox(height: 24),
          const Text('Advanced Inventory Options', style: TextStyle(color: Colors.white54, fontWeight: FontWeight.bold)),
          const SizedBox(height: 8),
          const TextField(
            style: TextStyle(color: Colors.white),
            decoration: InputDecoration(labelText: 'SKU (Optional)', labelStyle: TextStyle(color: Colors.white70)),
          ),
        ]
      ],
    );
  }

  Widget _buildStep3_Domain(WebsiteBuilderState state, WebsiteBuilderNotifier notifier) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Connect a domain', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold, fontSize: 20, color: Colors.white)),
        const SizedBox(height: 16),
        RadioListTile<String>(
          title: const Text('Use a free OHC subdomain', style: TextStyle(color: Colors.white)),
          subtitle: const Text('mybusiness.ohc.app', style: TextStyle(color: Colors.white54)),
          value: 'subdomain',
          groupValue: state.domainChoice,
          onChanged: (val) => notifier.setDomainChoice(val!),
          activeColor: Colors.blueAccent,
        ),
        RadioListTile<String>(
          title: const Text('Use my own domain', style: TextStyle(color: Colors.white)),
          value: 'own',
          groupValue: state.domainChoice,
          onChanged: (val) => notifier.setDomainChoice(val!),
          activeColor: Colors.blueAccent,
        ),
        RadioListTile<String>(
          title: const Text('Buy a new domain', style: TextStyle(color: Colors.white)),
          value: 'buy',
          groupValue: state.domainChoice,
          onChanged: (val) => notifier.setDomainChoice(val!),
          activeColor: Colors.blueAccent,
        ),
        if (state.advancedMode) ...[
          const SizedBox(height: 24),
          const Text('Advanced DNS Configuration', style: TextStyle(color: Colors.white54, fontWeight: FontWeight.bold)),
          const SizedBox(height: 8),
          const TextField(
            style: TextStyle(color: Colors.white),
            decoration: InputDecoration(labelText: 'Custom CNAME Record', labelStyle: TextStyle(color: Colors.white70)),
          ),
        ]
      ],
    );
  }

  Widget _buildStep4_Publish(WebsiteBuilderState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.center,
      children: [
        const Text('Ready to go live!', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold, fontSize: 24, color: Colors.white)),
        const SizedBox(height: 16),
        const Text('Your store is fully generated and ready.', style: TextStyle(fontFamily: 'Inter', color: Colors.white70)),
        const SizedBox(height: 32),
        Container(
          height: 300,
          width: double.infinity,
          decoration: BoxDecoration(
            color: Colors.white10,
            borderRadius: BorderRadius.circular(16),
            border: Border.all(color: Colors.white24),
          ),
          child: const Center(
            child: Text('Live Preview', style: TextStyle(color: Colors.white54)),
          ),
        ),
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(websiteBuilderProvider);
    final notifier = ref.read(websiteBuilderProvider.notifier);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Website Builder'),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () {
            if (state.step > 0) {
              notifier.prevStep();
            } else {
              context.pop();
            }
          },
        ),
        actions: [
          Row(
            children: [
              const Text('Advanced', style: TextStyle(fontSize: 12)),
              Switch(
                value: state.advancedMode,
                onChanged: (_) => notifier.toggleAdvancedMode(),
                activeColor: Colors.blueAccent,
              ),
            ],
          )
        ],
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
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(24),
              child: GlassCard(
                child: Padding(
                  padding: const EdgeInsets.all(32),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      if (state.errorMessage != null) ...[
                        Text(state.errorMessage!, style: const TextStyle(color: Colors.red)),
                        const SizedBox(height: 16),
                      ],
                      LinearProgressIndicator(value: (state.step + 1) / 5),
                      const SizedBox(height: 24),
                      AnimatedSwitcher(
                        duration: const Duration(milliseconds: 300),
                        child: Container(
                          key: ValueKey(state.step),
                          child: () {
                            switch (state.step) {
                              case 0: return _buildStep0_Template(state, notifier);
                              case 1: return _buildStep1_Brand(state, notifier);
                              case 2: return _buildStep2_Product(state, notifier);
                              case 3: return _buildStep3_Domain(state, notifier);
                              case 4: return _buildStep4_Publish(state);
                              default: return const SizedBox();
                            }
                          }(),
                        ),
                      ),
                      const SizedBox(height: 32),
                      Row(
                        mainAxisAlignment: MainAxisAlignment.end,
                        children: [
                          if (state.step == 4)
                            ElevatedButton(
                              onPressed: state.isLoading ? null : () => notifier.publish(context, ref),
                              child: state.isLoading
                                  ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2))
                                  : const Text('Publish'),
                            )
                          else
                            ElevatedButton(
                              onPressed: (state.step == 0 && state.selectedTemplate.isEmpty) ? null : notifier.nextStep,
                              child: const Text('Next'),
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
      ),
    );
  }
}

class _TemplateCard extends StatelessWidget {
  final String title;
  final String industry;
  final bool selected;
  final VoidCallback onTap;

  const _TemplateCard({required this.title, required this.industry, required this.selected, required this.onTap});

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: onTap,
      child: Container(
        width: 150,
        height: 250,
        decoration: BoxDecoration(
          color: selected ? Colors.blue.withOpacity(0.2) : Colors.white.withOpacity(0.05),
          borderRadius: BorderRadius.circular(16),
          border: Border.all(color: selected ? Colors.blue : Colors.white24, width: selected ? 2 : 1),
        ),
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          children: [
            Container(
              height: 100,
              width: double.infinity,
              decoration: BoxDecoration(
                color: Colors.white10,
                borderRadius: BorderRadius.circular(8),
              ),
              child: const Center(child: Icon(Icons.image, color: Colors.white54)),
            ),
            const SizedBox(height: 16),
            Text(title, style: const TextStyle(color: Colors.white, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
            Text(industry, style: const TextStyle(color: Colors.white70, fontSize: 12, fontFamily: 'Inter')),
            if (selected)
              const Padding(
                padding: EdgeInsets.only(top: 8.0),
                child: Text('Use this template →', style: TextStyle(color: Colors.greenAccent, fontSize: 10)),
              )
          ],
        ),
      ),
    );
  }
}

class _ColorPickerItem extends StatelessWidget {
  final Color color;
  final bool selected;
  final VoidCallback onTap;

  const _ColorPickerItem({required this.color, required this.selected, required this.onTap});

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: onTap,
      child: Container(
        margin: const EdgeInsets.only(right: 12),
        width: 40,
        height: 40,
        decoration: BoxDecoration(
          color: color,
          shape: BoxShape.circle,
          border: selected ? Border.all(color: Colors.white, width: 3) : null,
        ),
      ),
    );
  }
}
