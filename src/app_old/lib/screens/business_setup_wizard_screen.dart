import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'dart:convert';
import 'dart:ui';
import '../services/api_service.dart';
import '../services/auth_service.dart';
import '../services/settings_service.dart';
import '../widgets/glass_card.dart';

class BusinessSetupState {
  final int step;
  final String businessType;
  final String companyName;
  final String productsServices;
  final String paymentPref;
  final String templateId;
  final String firstProductName;
  final String firstProductDesc;
  final String firstProductPrice;
  final String domainName;
  final bool isLoading;
  final String? errorMessage;

  const BusinessSetupState({
    this.step = 0,
    this.businessType = '',
    this.companyName = '',
    this.productsServices = '',
    this.paymentPref = 'stripe',
    this.templateId = 'modern',
    this.firstProductName = '',
    this.firstProductDesc = '',
    this.firstProductPrice = '',
    this.domainName = '',
    this.isLoading = false,
    this.errorMessage,
  });

  BusinessSetupState copyWith({
    int? step,
    String? businessType,
    String? companyName,
    String? productsServices,
    String? paymentPref,
    String? templateId,
    String? firstProductName,
    String? firstProductDesc,
    String? firstProductPrice,
    String? domainName,
    bool? isLoading,
    String? errorMessage,
  }) {
    return BusinessSetupState(
      step: step ?? this.step,
      businessType: businessType ?? this.businessType,
      companyName: companyName ?? this.companyName,
      productsServices: productsServices ?? this.productsServices,
      paymentPref: paymentPref ?? this.paymentPref,
      templateId: templateId ?? this.templateId,
      firstProductName: firstProductName ?? this.firstProductName,
      firstProductDesc: firstProductDesc ?? this.firstProductDesc,
      firstProductPrice: firstProductPrice ?? this.firstProductPrice,
      domainName: domainName ?? this.domainName,
      isLoading: isLoading ?? this.isLoading,
      errorMessage:
          errorMessage, // Notice this is not coalesced so it clears correctly
    );
  }
}

class BusinessSetupNotifier extends Notifier<BusinessSetupState> {
  @override
  BusinessSetupState build() => const BusinessSetupState();

  void nextStep() {
    if (state.step < 5) {
      state = state.copyWith(step: state.step + 1);
    }
  }

  void prevStep() {
    if (state.step > 0) {
      state = state.copyWith(step: state.step - 1);
    }
  }

  void updateBusinessType(String val) =>
      state = state.copyWith(businessType: val);
  void updateCompany(String name) => state = state.copyWith(companyName: name);
  void updateProductsServices(String val) =>
      state = state.copyWith(productsServices: val);
  void updatePaymentPref(String val) =>
      state = state.copyWith(paymentPref: val);
  void updateTemplateId(String val) => state = state.copyWith(templateId: val);
  void updateFirstProductName(String val) =>
      state = state.copyWith(firstProductName: val);
  void updateFirstProductDesc(String val) =>
      state = state.copyWith(firstProductDesc: val);
  void updateFirstProductPrice(String val) =>
      state = state.copyWith(firstProductPrice: val);
  void updateDomainName(String val) => state = state.copyWith(domainName: val);

  Future<void> launch(BuildContext context, WidgetRef ref) async {
    final api = ref.read(apiServiceProvider);

    state = state.copyWith(isLoading: true, errorMessage: null);

    if (api != null) {
      final body = {
        'extras': {
          'business_type': state.businessType,
          'company_name': state.companyName,
          'products_services': state.productsServices,
          'payment_pref': state.paymentPref,
          'template_id': state.templateId,
          'first_product_name': state.firstProductName,
          'first_product_desc': state.firstProductDesc,
          'first_product_price': state.firstProductPrice,
          'domain_name': state.domainName,
        },
      };

      try {
        await api.configureWizard(body);
      } catch (e) {
        state = state.copyWith(
          isLoading: false,
          errorMessage: 'Configuration failed: $e',
        );
        return;
      }
    }

    final url =
        'https://${state.domainName.isEmpty ? 'yourbusiness' : state.domainName}.ohc.app';
    await Clipboard.setData(ClipboardData(text: url));

    state = state.copyWith(isLoading: false);

    if (context.mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('🎉 Published! Link copied to clipboard: $url')),
      );
      GoRouter.of(context).go('/welcome_checklist');
    }
  }
}

final businessSetupProvider =
    NotifierProvider<BusinessSetupNotifier, BusinessSetupState>(() {
      return BusinessSetupNotifier();
    });

class BusinessSetupWizardScreen extends ConsumerStatefulWidget {
  const BusinessSetupWizardScreen({super.key});

  @override
  ConsumerState<BusinessSetupWizardScreen> createState() =>
      _BusinessSetupWizardScreenState();
}

class _BusinessSetupWizardScreenState
    extends ConsumerState<BusinessSetupWizardScreen> {
  @override
  Widget build(BuildContext context) {
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
                    const Text(
                      'Business Setup',
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 24,
                        fontWeight: FontWeight.bold,
                        color: Colors.white,
                      ),
                    ),
                    const SizedBox(height: 16),
                    if (state.errorMessage != null) ...[
                      Text(
                        state.errorMessage!,
                        style: const TextStyle(color: Colors.red),
                      ),
                      const SizedBox(height: 16),
                    ],
                    AnimatedSwitcher(
                      duration: const Duration(milliseconds: 300),
                      transitionBuilder: (
                        Widget child,
                        Animation<double> animation,
                      ) {
                        return FadeTransition(opacity: animation, child: child);
                      },
                      child: Container(
                        key: ValueKey<int>(state.step),
                        child: Column(
                          mainAxisSize: MainAxisSize.min,
                          children: [
                            if (state.step == 0) ...[
                              const Text(
                                'Welcome! Your AI team, ready in minutes.',
                                style: TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                  fontSize: 16,
                                ),
                              ),
                            ] else if (state.step == 1) ...[
                              TextField(
                                decoration: const InputDecoration(
                                  labelText:
                                      'Business Type (e.g. Baker, Handyman)',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updateBusinessType,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                              ),
                              const SizedBox(height: 16),
                              TextField(
                                decoration: const InputDecoration(
                                  labelText: 'Company Name',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updateCompany,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                              ),
                            ] else if (state.step == 2) ...[
                              TextField(
                                decoration: const InputDecoration(
                                  labelText: 'What do you sell?',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updateProductsServices,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                              ),
                              const SizedBox(height: 16),
                              DropdownButtonFormField<String>(
                                value: state.paymentPref,
                                decoration: const InputDecoration(
                                  labelText: 'Payment Preference',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                dropdownColor: const Color(0xFF1A1A33),
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                                items: const [
                                  DropdownMenuItem(
                                    value: 'stripe',
                                    child: Text('Stripe'),
                                  ),
                                  DropdownMenuItem(
                                    value: 'paypal',
                                    child: Text('PayPal'),
                                  ),
                                ],
                                onChanged: (val) {
                                  if (val != null)
                                    notifier.updatePaymentPref(val);
                                },
                              ),
                            ] else if (state.step == 3) ...[
                              DropdownButtonFormField<String>(
                                value: state.templateId,
                                decoration: const InputDecoration(
                                  labelText: 'Template Selection',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                dropdownColor: const Color(0xFF1A1A33),
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                                items: const [
                                  DropdownMenuItem(
                                    value: 'modern',
                                    child: Text('Modern'),
                                  ),
                                  DropdownMenuItem(
                                    value: 'classic',
                                    child: Text('Classic'),
                                  ),
                                  DropdownMenuItem(
                                    value: 'bold',
                                    child: Text('Bold'),
                                  ),
                                ],
                                onChanged: (val) {
                                  if (val != null)
                                    notifier.updateTemplateId(val);
                                },
                              ),
                            ] else if (state.step == 4) ...[
                              TextField(
                                decoration: const InputDecoration(
                                  labelText: 'First Product Name',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updateFirstProductName,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                              ),
                              const SizedBox(height: 16),
                              TextField(
                                decoration: const InputDecoration(
                                  labelText:
                                      'Product Description (AI will expand)',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updateFirstProductDesc,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                              ),
                              const SizedBox(height: 16),
                              TextField(
                                decoration: const InputDecoration(
                                  labelText: 'Price',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updateFirstProductPrice,
                                keyboardType: TextInputType.number,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                              ),
                            ] else if (state.step == 5) ...[
                              TextField(
                                decoration: const InputDecoration(
                                  labelText: 'Domain Name',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updateDomainName,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                              ),
                              const SizedBox(height: 8),
                              const Text(
                                '.ohc.app',
                                style: TextStyle(
                                  color: Colors.white70,
                                  fontFamily: 'Inter',
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
                            onPressed:
                                state.isLoading ? null : notifier.prevStep,
                            child: const Text(
                              'Back',
                              style: TextStyle(fontFamily: 'Inter'),
                            ),
                          )
                        else
                          const SizedBox(),
                        ElevatedButton(
                          onPressed:
                              state.isLoading
                                  ? null
                                  : () {
                                    if (state.step < 5) {
                                      notifier.nextStep();
                                    } else {
                                      notifier.launch(context, ref);
                                    }
                                  },
                          child:
                              state.isLoading
                                  ? const SizedBox(
                                    width: 20,
                                    height: 20,
                                    child: CircularProgressIndicator(
                                      strokeWidth: 2,
                                    ),
                                  )
                                  : Text(
                                    state.step == 5 ? 'Publish 🎉' : 'Next',
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
}
