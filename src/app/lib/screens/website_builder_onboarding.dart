import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
import '../services/auth_service.dart';
import '../services/settings_service.dart';
import '../widgets/glass_card.dart';

class WebsiteBuilderState {
  final int step;
  final String? template;
  final String? colorPalette;
  final String? logoPath;
  final String? productName;
  final String? productPhotoPath;
  final String? productPrice;
  final String? productDescription;
  final String domainPreference;

  const WebsiteBuilderState({
    this.step = 0,
    this.template,
    this.colorPalette,
    this.logoPath,
    this.productName,
    this.productPhotoPath,
    this.productPrice,
    this.productDescription,
    this.domainPreference = 'free',
  });

  WebsiteBuilderState copyWith({
    int? step,
    String? template,
    String? colorPalette,
    String? logoPath,
    String? productName,
    String? productPhotoPath,
    String? productPrice,
    String? productDescription,
    String? domainPreference,
  }) {
    return WebsiteBuilderState(
      step: step ?? this.step,
      template: template ?? this.template,
      colorPalette: colorPalette ?? this.colorPalette,
      logoPath: logoPath ?? this.logoPath,
      productName: productName ?? this.productName,
      productPhotoPath: productPhotoPath ?? this.productPhotoPath,
      productPrice: productPrice ?? this.productPrice,
      productDescription: productDescription ?? this.productDescription,
      domainPreference: domainPreference ?? this.domainPreference,
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

  void selectTemplate(String t) => state = state.copyWith(template: t);
  void selectPalette(String p) => state = state.copyWith(colorPalette: p);
  void setLogo(String l) => state = state.copyWith(logoPath: l);
  void updateProduct(String name, String price, String desc) => state = state.copyWith(productName: name, productPrice: price, productDescription: desc);
  void selectDomainPref(String d) => state = state.copyWith(domainPreference: d);

  Future<void> publish(BuildContext context, WidgetRef ref) async {
    final user = ref.read(authStateProvider).valueOrNull;
    final baseUrl = ref.read(backendUrlProvider);

    if (user != null && baseUrl.isNotEmpty) {
      final body = {
        'extras': {
          'website_template': state.template ?? '',
          'website_color_palette': state.colorPalette ?? '',
          'website_logo_path': state.logoPath ?? '',
          'website_product_name': state.productName ?? '',
          'website_product_price': state.productPrice ?? '',
          'website_product_description': state.productDescription ?? '',
          'website_domain_preference': state.domainPreference,
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
          if (context.mounted) {
            ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text("Failed to save website config: ${res.statusCode}")));
          }
          return;
        }
      } catch (e) {
        if (context.mounted) {
          ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text("Network error: $e")));
        }
        return;
      }
    }

    if (context.mounted) {
      ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text("Website published successfully!")));
      GoRouter.of(context).go('/dashboard');
    }
  }
}

final websiteBuilderProvider = NotifierProvider<WebsiteBuilderNotifier, WebsiteBuilderState>(() {
  return WebsiteBuilderNotifier();
});

class WebsiteBuilderOnboardingScreen extends ConsumerWidget {
  const WebsiteBuilderOnboardingScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(websiteBuilderProvider);
    final notifier = ref.read(websiteBuilderProvider.notifier);

    return Scaffold(
      appBar: AppBar(title: const Text('Website Builder', style: TextStyle(fontFamily: 'Outfit'))),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 800),
          child: GlassCard(
            child: Padding(
              padding: const EdgeInsets.all(24.0),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Text('Step ${state.step + 1} of 5', style: const TextStyle(fontFamily: 'Inter', fontSize: 14, color: Colors.grey)),
                  const SizedBox(height: 16),
                  if (state.step == 0) ...[
                    const Text('Select a Template', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold)),
                    const SizedBox(height: 16),
                    Wrap(
                      spacing: 16,
                      runSpacing: 16,
                      children: ['Minimal', 'Bold', 'Classic'].map((t) => InkWell(
                        onTap: () => notifier.selectTemplate(t),
                        child: Container(
                          width: 150,
                          height: 100,
                          decoration: BoxDecoration(
                            border: Border.all(color: state.template == t ? Colors.blue : Colors.grey),
                            borderRadius: BorderRadius.circular(8),
                          ),
                          child: Center(child: Text(t, style: const TextStyle(fontFamily: 'Inter'))),
                        ),
                      )).toList(),
                    ),
                  ] else if (state.step == 1) ...[
                    const Text('Brand Colors & Logo', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold)),
                    const SizedBox(height: 16),
                    Wrap(
                      spacing: 16,
                      children: ['Ocean', 'Sunset', 'Forest'].map((p) => ChoiceChip(
                        label: Text(p, style: const TextStyle(fontFamily: 'Inter')),
                        selected: state.colorPalette == p,
                        onSelected: (val) { if (val) notifier.selectPalette(p); },
                      )).toList(),
                    ),
                    const SizedBox(height: 16),
                    ElevatedButton(
                      onPressed: () => notifier.setLogo('generated_logo.png'),
                      child: const Text('Generate a logo for me', style: TextStyle(fontFamily: 'Inter')),
                    ),
                  ] else if (state.step == 2) ...[
                    const Text('Add Product or Service', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold)),
                    const SizedBox(height: 16),
                    TextField(
                      decoration: const InputDecoration(labelText: 'Name'),
                      onChanged: (v) => notifier.updateProduct(v, state.productPrice ?? '', state.productDescription ?? ''),
                    ),
                    TextField(
                      decoration: const InputDecoration(labelText: 'Price'),
                      onChanged: (v) => notifier.updateProduct(state.productName ?? '', v, state.productDescription ?? ''),
                    ),
                    TextField(
                      decoration: const InputDecoration(labelText: 'Description'),
                      onChanged: (v) => notifier.updateProduct(state.productName ?? '', state.productPrice ?? '', v),
                    ),
                  ] else if (state.step == 3) ...[
                    const Text('Connect a Domain', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold)),
                    const SizedBox(height: 16),
                    ...['Free Subdomain', 'Own Domain', 'Buy Domain'].map((d) => RadioListTile(
                      title: Text(d, style: const TextStyle(fontFamily: 'Inter')),
                      value: d,
                      groupValue: state.domainPreference,
                      onChanged: (val) { if (val != null) notifier.selectDomainPref(val.toString()); },
                    )),
                  ] else if (state.step == 4) ...[
                    const Text('Go Live', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold)),
                    const SizedBox(height: 16),
                    const Text('Your website is ready to be published.', style: TextStyle(fontFamily: 'Inter')),
                  ],
                  const SizedBox(height: 24),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      if (state.step > 0)
                        TextButton(onPressed: notifier.previousStep, child: const Text('Back')),
                      if (state.step == 0)
                        const SizedBox(),
                      ElevatedButton(
                        onPressed: () {
                          if (state.step < 4) {
                            notifier.nextStep();
                          } else {
                            notifier.publish(context, ref);
                          }
                        },
                        child: Text(state.step == 4 ? 'Publish' : 'Next', style: const TextStyle(fontFamily: 'Inter')),
                      ),
                    ],
                  )
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
