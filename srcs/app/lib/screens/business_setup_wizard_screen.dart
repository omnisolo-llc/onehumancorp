import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
import 'dart:ui';
import '../services/auth_service.dart';
import '../services/settings_service.dart';
import '../widgets/glass_card.dart';

class BusinessSetupState {
  final int step;
  final String companyName;
  final String industry;
  final String paymentPreference;
  final String templateSelection;
  final String productName;
  final String productDescription;
  final String productPrice;
  final String domain;
  final bool isLoading;
  final String? errorMessage;

  const BusinessSetupState({
    this.step = 0,
    this.companyName = '',
    this.industry = '',
    this.paymentPreference = 'Stripe',
    this.templateSelection = 'Modern',
    this.productName = '',
    this.productDescription = '',
    this.productPrice = '',
    this.domain = '',
    this.isLoading = false,
    this.errorMessage,
  });

  BusinessSetupState copyWith({
    int? step,
    String? companyName,
    String? industry,
    String? paymentPreference,
    String? templateSelection,
    String? productName,
    String? productDescription,
    String? productPrice,
    String? domain,
    bool? isLoading,
    String? errorMessage,
  }) {
    return BusinessSetupState(
      step: step ?? this.step,
      companyName: companyName ?? this.companyName,
      industry: industry ?? this.industry,
      paymentPreference: paymentPreference ?? this.paymentPreference,
      templateSelection: templateSelection ?? this.templateSelection,
      productName: productName ?? this.productName,
      productDescription: productDescription ?? this.productDescription,
      productPrice: productPrice ?? this.productPrice,
      domain: domain ?? this.domain,
      isLoading: isLoading ?? this.isLoading,
      errorMessage: errorMessage ?? this.errorMessage,
    );
  }
}

class BusinessSetupNotifier extends Notifier<BusinessSetupState> {
  @override
  BusinessSetupState build() => const BusinessSetupState();

  void nextStep() {
    if (state.step < 6) {
      state = state.copyWith(step: state.step + 1);
    }
  }

  void prevStep() {
    if (state.step > 0) {
      state = state.copyWith(step: state.step - 1);
    }
  }

  void updateCompany(String name) => state = state.copyWith(companyName: name);
  void updateIndustry(String val) => state = state.copyWith(industry: val);
  void updatePaymentPreference(String val) => state = state.copyWith(paymentPreference: val);
  void updateTemplateSelection(String val) => state = state.copyWith(templateSelection: val);
  void updateProductName(String val) => state = state.copyWith(productName: val);
  void updateProductDescription(String val) => state = state.copyWith(productDescription: val);
  void updateProductPrice(String val) => state = state.copyWith(productPrice: val);
  void updateDomain(String val) => state = state.copyWith(domain: val);

  Future<void> launch(BuildContext context, WidgetRef ref, {bool publish = false, bool dashboard = false}) async {
    final user = ref.read(authStateProvider).valueOrNull;
    final baseUrl = ref.read(backendUrlProvider);

    state = state.copyWith(isLoading: true, errorMessage: null);

    if (user != null && baseUrl.isNotEmpty) {
      final body = {
        'extras': {
          'company_name': state.companyName,
          'industry': state.industry,
          'payment_preference': state.paymentPreference,
          'template_selection': state.templateSelection,
          'product_name': state.productName,
          'product_description': state.productDescription,
          'product_price': state.productPrice,
          'domain': state.domain,
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

    state = state.copyWith(isLoading: false);

    if (publish) {
      // Simulate API call for publishing and confetti
      state = state.copyWith(step: 6);
    } else if (dashboard && context.mounted) {
      GoRouter.of(context).go('/dashboard');
    }
  }
}

final businessSetupProvider = NotifierProvider<BusinessSetupNotifier, BusinessSetupState>(() {
  return BusinessSetupNotifier();
});

class BusinessSetupWizardScreen extends ConsumerWidget {
  const BusinessSetupWizardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(businessSetupProvider);
    final notifier = ref.read(businessSetupProvider.notifier);

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
                  const Text('Business Setup', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white)),
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
                            const Text('Welcome! Let\'s get your business online in under 10 minutes.', style: TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 16)),
                          ] else if (state.step == 1) ...[
                            TextField(
                              decoration: const InputDecoration(labelText: 'Business Name', labelStyle: TextStyle(color: Colors.white70)),
                              onChanged: notifier.updateCompany,
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                            ),
                            const SizedBox(height: 16),
                            TextField(
                              decoration: const InputDecoration(labelText: 'What do you sell?', hintText: 'e.g. Handmade jewelry', hintStyle: TextStyle(color: Colors.white30), labelStyle: TextStyle(color: Colors.white70)),
                              onChanged: notifier.updateIndustry,
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                            ),
                          ] else if (state.step == 2) ...[
                             const Text('Payment Preferences', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
                             ...['Stripe (Credit Cards/Apple Pay)', 'Cash / In-Person', 'Bank Transfer'].map((dep) => RadioListTile<String>(
                                title: Text(dep, style: const TextStyle(fontFamily: 'Inter', color: Colors.white)),
                                value: dep,
                                groupValue: state.paymentPreference,
                                activeColor: Colors.blueAccent,
                                onChanged: (String? value) {
                                  if (value != null) notifier.updatePaymentPreference(value);
                                },
                              )),
                          ] else if (state.step == 3) ...[
                             const Text('Template Selection', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
                             const SizedBox(height: 8),
                             const Text('Select a template to preview it below', style: TextStyle(fontFamily: 'Inter', color: Colors.white70)),
                             const SizedBox(height: 16),
                             DropdownButtonFormField<String>(
                              value: state.templateSelection,
                              decoration: const InputDecoration(labelText: 'Template', labelStyle: TextStyle(color: Colors.white70)),
                              dropdownColor: const Color(0xFF1A1A33),
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                              items: const [
                                DropdownMenuItem(value: 'Modern', child: Text('Modern')),
                                DropdownMenuItem(value: 'Classic', child: Text('Classic')),
                                DropdownMenuItem(value: 'Bold', child: Text('Bold')),
                              ],
                              onChanged: (val) {
                                if (val != null) notifier.updateTemplateSelection(val);
                              },
                            ),
                            const SizedBox(height: 16),
                            Container(
                              height: 100,
                              width: double.infinity,
                              decoration: BoxDecoration(
                                color: Colors.white.withValues(alpha: 0.1),
                                borderRadius: BorderRadius.circular(8),
                              ),
                              child: Center(
                                child: Text('${state.templateSelection} Template Preview for ${state.companyName.isEmpty ? "Your Business" : state.companyName}', style: const TextStyle(color: Colors.white, fontFamily: 'Outfit')),
                              ),
                            )
                          ] else if (state.step == 4) ...[
                            const Text('First Product / Service Add', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
                            const SizedBox(height: 8),
                            TextField(
                              decoration: const InputDecoration(labelText: 'Name', labelStyle: TextStyle(color: Colors.white70)),
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
                            TextField(
                              decoration: const InputDecoration(labelText: 'Description (AI will auto-generate if empty)', labelStyle: TextStyle(color: Colors.white70)),
                              onChanged: notifier.updateProductDescription,
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                            ),
                            const SizedBox(height: 16),
                            OutlinedButton.icon(
                              onPressed: () {},
                              icon: const Icon(Icons.camera_alt),
                              label: const Text('Upload Photo'),
                            )
                          ] else if (state.step == 5) ...[
                            const Text('Domain & Go-Live', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
                            const SizedBox(height: 8),
                            TextField(
                              decoration: const InputDecoration(labelText: 'Free Subdomain', hintText: 'mybusiness', suffixText: '.ohc.app', hintStyle: TextStyle(color: Colors.white30), labelStyle: TextStyle(color: Colors.white70)),
                              onChanged: notifier.updateDomain,
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                            ),
                            const SizedBox(height: 16),
                            const Text('When ready, click "Publish" below to go live!', style: TextStyle(fontFamily: 'Inter', color: Colors.white)),
                          ] else if (state.step == 6) ...[
                            const Text('Welcome Checklist', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white)),
                            const SizedBox(height: 16),
                            const ListTile(leading: Icon(Icons.check_circle, color: Colors.green), title: Text('Business live', style: TextStyle(color: Colors.white))),
                            const ListTile(leading: Icon(Icons.circle_outlined, color: Colors.white54), title: Text('Add 3 more products', style: TextStyle(color: Colors.white))),
                            const ListTile(leading: Icon(Icons.circle_outlined, color: Colors.white54), title: Text('Connect Instagram', style: TextStyle(color: Colors.white))),
                            const ListTile(leading: Icon(Icons.circle_outlined, color: Colors.white54), title: Text('Share your link with a friend', style: TextStyle(color: Colors.white))),
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
                          if (state.step == 5) {
                            notifier.launch(context, ref, publish: true);
                          } else if (state.step < 6) {
                            notifier.nextStep();
                          } else {
                            notifier.launch(context, ref, dashboard: true);
                          }
                        },
                        child: state.isLoading
                            ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2))
                            : Text(state.step == 5 ? 'Publish' : state.step == 6 ? 'Go to Dashboard →' : 'Next', style: const TextStyle(fontFamily: 'Inter')),
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
