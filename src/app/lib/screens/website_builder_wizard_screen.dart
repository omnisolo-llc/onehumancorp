import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class WebsiteBuilderState {
  final int step;
  final String template;
  final String primaryColor;
  final String logoUrl;
  final String productName;
  final double productPrice;
  final String productDescription;
  final String domainPreference;
  final bool isPublishing;

  const WebsiteBuilderState({
    this.step = 0,
    this.template = '',
    this.primaryColor = '#000000',
    this.logoUrl = '',
    this.productName = '',
    this.productPrice = 0.0,
    this.productDescription = '',
    this.domainPreference = 'ohc_subdomain',
    this.isPublishing = false,
  });

  WebsiteBuilderState copyWith({
    int? step,
    String? template,
    String? primaryColor,
    String? logoUrl,
    String? productName,
    double? productPrice,
    String? productDescription,
    String? domainPreference,
    bool? isPublishing,
  }) {
    return WebsiteBuilderState(
      step: step ?? this.step,
      template: template ?? this.template,
      primaryColor: primaryColor ?? this.primaryColor,
      logoUrl: logoUrl ?? this.logoUrl,
      productName: productName ?? this.productName,
      productPrice: productPrice ?? this.productPrice,
      productDescription: productDescription ?? this.productDescription,
      domainPreference: domainPreference ?? this.domainPreference,
      isPublishing: isPublishing ?? this.isPublishing,
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

  void selectTemplate(String t) => state = state.copyWith(template: t);
  void updateColor(String c) => state = state.copyWith(primaryColor: c);
  void updateLogo(String url) => state = state.copyWith(logoUrl: url);
  void updateProductName(String n) => state = state.copyWith(productName: n);
  void updateProductPrice(double p) => state = state.copyWith(productPrice: p);
  void updateProductDescription(String d) => state = state.copyWith(productDescription: d);
  void updateDomainPreference(String d) => state = state.copyWith(domainPreference: d);

  Future<void> publish(BuildContext context) async {
    state = state.copyWith(isPublishing: true);
    // Simulate network delay
    await Future.delayed(const Duration(seconds: 2));
    if (context.mounted) {
      state = state.copyWith(isPublishing: false);
      GoRouter.of(context).go('/dashboard');
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
  final _priceController = TextEditingController();

  @override
  void dispose() {
    _priceController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
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
                    const Text(
                      'Website Builder',
                      style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white),
                    ),
                    const SizedBox(height: 16),
                    AnimatedSwitcher(
                      duration: const Duration(milliseconds: 300),
                      transitionBuilder: (Widget child, Animation<double> animation) {
                        return FadeTransition(opacity: animation, child: child);
                      },
                      child: Container(
                        key: ValueKey<int>(state.step),
                        child: _buildStepContent(state, notifier),
                      ),
                    ),
                    const SizedBox(height: 24),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        if (state.step > 0)
                          TextButton(
                            onPressed: state.isPublishing ? null : notifier.prevStep,
                            child: const Text('Back', style: TextStyle(fontFamily: 'Inter', color: Colors.white70)),
                          )
                        else
                          const SizedBox(),
                        ElevatedButton(
                          onPressed: state.isPublishing ? null : () {
                            if (state.step < 4) {
                              notifier.nextStep();
                            } else {
                              notifier.publish(context);
                            }
                          },
                          child: state.isPublishing
                              ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white))
                              : Text(
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

  Widget _buildStepContent(WebsiteBuilderState state, WebsiteBuilderNotifier notifier) {
    if (state.step == 0) {
      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('Choose a Template', style: TextStyle(fontFamily: 'Inter', fontSize: 18, fontWeight: FontWeight.bold, color: Colors.white)),
          const SizedBox(height: 16),
          Wrap(
            spacing: 16,
            runSpacing: 16,
            children: ['Minimal', 'Modern', 'Playful', 'Corporate'].map((t) => GestureDetector(
              onTap: () => notifier.selectTemplate(t),
              child: Container(
                width: 150,
                height: 100,
                decoration: BoxDecoration(
                  color: state.template == t ? Colors.blue.withValues(alpha: 0.3) : Colors.white10,
                  border: Border.all(color: state.template == t ? Colors.blue : Colors.transparent),
                  borderRadius: BorderRadius.circular(12),
                ),
                child: Center(
                  child: Text(t, style: const TextStyle(color: Colors.white, fontFamily: 'Inter')),
                ),
              ),
            )).toList(),
          ),
        ],
      );
    } else if (state.step == 1) {
      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('Brand Colors & Logo', style: TextStyle(fontFamily: 'Inter', fontSize: 18, fontWeight: FontWeight.bold, color: Colors.white)),
          const SizedBox(height: 16),
          const Text('Primary Color', style: TextStyle(color: Colors.white70, fontFamily: 'Inter')),
          const SizedBox(height: 8),
          Wrap(
            spacing: 16,
            children: ['#FF5733', '#33FF57', '#3357FF', '#F333FF'].map((c) => GestureDetector(
              onTap: () => notifier.updateColor(c),
              child: Container(
                width: 40,
                height: 40,
                decoration: BoxDecoration(
                  color: Color(int.parse(c.substring(1, 7), radix: 16) + 0xFF000000),
                  shape: BoxShape.circle,
                  border: Border.all(color: state.primaryColor == c ? Colors.white : Colors.transparent, width: 2),
                ),
              ),
            )).toList(),
          ),
          const SizedBox(height: 24),
          ElevatedButton.icon(
            onPressed: () => notifier.updateLogo('uploaded_logo.png'),
            icon: const Icon(Icons.upload),
            label: const Text('Upload Logo'),
          ),
          if (state.logoUrl.isNotEmpty) ...[
            const SizedBox(height: 8),
            Text('Selected: ${state.logoUrl}', style: const TextStyle(color: Colors.white70)),
          ]
        ],
      );
    } else if (state.step == 2) {
      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('Add your first product or service', style: TextStyle(fontFamily: 'Inter', fontSize: 18, fontWeight: FontWeight.bold, color: Colors.white)),
          const SizedBox(height: 16),
          TextField(
            decoration: const InputDecoration(labelText: 'Name', labelStyle: TextStyle(color: Colors.white70)),
            onChanged: notifier.updateProductName,
            style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
          ),
          const SizedBox(height: 16),
          TextField(
            controller: _priceController,
            decoration: const InputDecoration(labelText: 'Price', labelStyle: TextStyle(color: Colors.white70)),
            keyboardType: const TextInputType.numberWithOptions(decimal: true),
            onChanged: (val) {
              final p = double.tryParse(val);
              if (p != null) notifier.updateProductPrice(p);
            },
            style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
          ),
          const SizedBox(height: 16),
          TextField(
            decoration: const InputDecoration(labelText: 'Description', labelStyle: TextStyle(color: Colors.white70)),
            onChanged: notifier.updateProductDescription,
            style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
            maxLines: 3,
          ),
        ],
      );
    } else if (state.step == 3) {
      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text('Connect a domain', style: TextStyle(fontFamily: 'Inter', fontSize: 18, fontWeight: FontWeight.bold, color: Colors.white)),
          const SizedBox(height: 16),
          RadioListTile<String>(
            title: const Text('Use a free OHC subdomain', style: TextStyle(color: Colors.white, fontFamily: 'Inter')),
            value: 'ohc_subdomain',
            groupValue: state.domainPreference,
            onChanged: (val) { if (val != null) notifier.updateDomainPreference(val); },
          ),
          RadioListTile<String>(
            title: const Text('Use my own domain', style: TextStyle(color: Colors.white, fontFamily: 'Inter')),
            value: 'custom_domain',
            groupValue: state.domainPreference,
            onChanged: (val) { if (val != null) notifier.updateDomainPreference(val); },
          ),
        ],
      );
    } else {
      return Column(
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          const Icon(Icons.public, size: 64, color: Colors.blueAccent),
          const SizedBox(height: 16),
          const Text('Ready to go live?', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white)),
          const SizedBox(height: 16),
          Text('Template: ${state.template}', style: const TextStyle(color: Colors.white70, fontFamily: 'Inter')),
          Text('Domain: ${state.domainPreference}', style: const TextStyle(color: Colors.white70, fontFamily: 'Inter')),
          const SizedBox(height: 24),
          const Text('Click Publish to launch your site.', style: TextStyle(color: Colors.white, fontFamily: 'Inter')),
        ],
      );
    }
  }
}
