import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';


import '../services/api_service.dart';

import '../services/settings_service.dart';
import '../widgets/glass_card.dart';

class BusinessSetupState {
  final int step;
  final String businessType;
  final String companyName;
  final String companyDescription;
  final List<String> sellItems;
  final String paymentType;
  final String adminName;
  final String adminEmail;
  final String adminPassword;
  final bool isLoading;
  final String? errorMessage;

  const BusinessSetupState({
    this.step = 0,
    this.businessType = '',
    this.companyName = '',
    this.companyDescription = '',
    this.sellItems = const [],
    this.paymentType = '',
    this.adminName = '',
    this.adminEmail = '',
    this.adminPassword = '',
    this.isLoading = false,
    this.errorMessage,
  });

  BusinessSetupState copyWith({
    int? step,
    String? businessType,
    String? companyName,
    String? companyDescription,
    List<String>? sellItems,
    String? paymentType,
    String? adminName,
    String? adminEmail,
    String? adminPassword,
    bool? isLoading,
    String? errorMessage,
  }) {
    return BusinessSetupState(
      step: step ?? this.step,
      businessType: businessType ?? this.businessType,
      companyName: companyName ?? this.companyName,
      companyDescription: companyDescription ?? this.companyDescription,
      sellItems: sellItems ?? this.sellItems,
      paymentType: paymentType ?? this.paymentType,
      adminName: adminName ?? this.adminName,
      adminEmail: adminEmail ?? this.adminEmail,
      adminPassword: adminPassword ?? this.adminPassword,
      isLoading: isLoading ?? this.isLoading,
      errorMessage: errorMessage,
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

  void updateBusinessType(String val) => state = state.copyWith(businessType: val);
  void updateCompany(String name) => state = state.copyWith(companyName: name);
  void updateCompanyDescription(String desc) => state = state.copyWith(companyDescription: desc);
  void toggleSellItem(String item) {
    if (state.sellItems.contains(item)) {
      state = state.copyWith(sellItems: state.sellItems.where((i) => i != item).toList());
    } else {
      state = state.copyWith(sellItems: [...state.sellItems, item]);
    }
  }
  void updatePaymentType(String val) => state = state.copyWith(paymentType: val);
  void updateAdminName(String val) => state = state.copyWith(adminName: val);
  void updateAdminEmail(String val) => state = state.copyWith(adminEmail: val);
  void updateAdminPassword(String val) => state = state.copyWith(adminPassword: val);

  Future<void> launch(BuildContext context, WidgetRef ref) async {
    final api = ref.read(apiServiceProvider);

    state = state.copyWith(isLoading: true, errorMessage: null);

    final isStandalone = ref.read(clientSettingsProvider).valueOrNull?.standaloneMode ?? false;

    if (!isStandalone && api != null) {
      final body = {
        'wizard_id': 'business_setup',
        'config': {
          'business_type': state.businessType,
          'company_name': state.companyName,
          'company_description': state.companyDescription,
          'sell_items': state.sellItems,
                    'payment_type': state.paymentType,
          'admin_name': state.adminName,
          'admin_email': state.adminEmail,
          'admin_password': state.adminPassword,
        }
      };

      try {
        await api.configureWizard(body);
      } catch (e) {
        state = state.copyWith(isLoading: false, errorMessage: 'Configuration failed: $e');
        return;
      }
    }

    state = state.copyWith(isLoading: false);

    if (context.mounted) {
      GoRouter.of(context).go('/welcome_checklist');
    }
  }
}

final businessSetupProvider = NotifierProvider<BusinessSetupNotifier, BusinessSetupState>(() {
  return BusinessSetupNotifier();
});

class BusinessSetupWizardScreen extends ConsumerStatefulWidget {
  const BusinessSetupWizardScreen({super.key});

  @override
  ConsumerState<BusinessSetupWizardScreen> createState() => _BusinessSetupWizardScreenState();
}

class _BusinessSetupWizardScreenState extends ConsumerState<BusinessSetupWizardScreen> {
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
                              const Center(
                                child: Text(
                                  'Your business, live in minutes.',
                                  style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 32, fontWeight: FontWeight.bold),
                                  textAlign: TextAlign.center,
                                ),
                              ),
                            ] else if (state.step == 1) ...[
                              const Text('Select your business type', style: TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 18, fontWeight: FontWeight.bold)),
                              const SizedBox(height: 16),
                              Wrap(
                                spacing: 16,
                                runSpacing: 16,
                                alignment: WrapAlignment.center,
                                children: [
                                  {'name': 'Online Store', 'icon': Icons.shopping_cart},
                                  {'name': 'Service Business', 'icon': Icons.handyman},
                                  {'name': 'Restaurant / Food', 'icon': Icons.restaurant},
                                  {'name': 'Creative / Portfolio', 'icon': Icons.brush},
                                  {'name': 'Local Business', 'icon': Icons.storefront},
                                  {'name': 'Other', 'icon': Icons.category},
                                ].map((typeInfo) {
                                  final name = typeInfo['name'] as String;
                                  final icon = typeInfo['icon'] as IconData;
                                  final isSelected = state.businessType == name;
                                  return InkWell(
                                    onTap: () => notifier.updateBusinessType(name),
                                    child: Container(
                                      width: 140,
                                      padding: const EdgeInsets.all(16),
                                      decoration: BoxDecoration(
                                        color: const Color(0xFF1A1A33),
                                        border: Border.all(color: isSelected ? Colors.blue : Colors.white24, width: 2),
                                        borderRadius: BorderRadius.circular(12),
                                      ),
                                      child: Column(
                                        mainAxisSize: MainAxisSize.min,
                                        children: [
                                          Icon(icon, size: 48, color: isSelected ? Colors.blue : Colors.white70),
                                          const SizedBox(height: 8),
                                          Text(name, textAlign: TextAlign.center, style: const TextStyle(fontFamily: 'Inter', color: Colors.white)),
                                        ],
                                      ),
                                    ),
                                  );
                                }).toList(),
                              ),
                            ] else if (state.step == 2) ...[
                              TextField(
                                decoration: const InputDecoration(labelText: 'Company Name', labelStyle: TextStyle(color: Colors.white70)),
                                onChanged: notifier.updateCompany,
                                style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                              ),
                              const SizedBox(height: 16),
                              TextField(
                                decoration: const InputDecoration(labelText: 'Tagline or Short Description', hintText: 'Leave blank to let AI auto-suggest', hintStyle: TextStyle(color: Colors.white38), labelStyle: TextStyle(color: Colors.white70)),
                                onChanged: notifier.updateCompanyDescription,
                                maxLines: 3,
                                style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                              ),
                            ] else if (state.step == 3) ...[
                              const Text('What do you sell?', style: TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 18, fontWeight: FontWeight.bold)),
                              const SizedBox(height: 16),
                              Wrap(
                                spacing: 16,
                                runSpacing: 16,
                                alignment: WrapAlignment.center,
                                children: [
                                  'Physical products',
                                  'Digital downloads',
                                  'Services / appointments',
                                  'Food & beverages',
                                  'Subscriptions',
                                ].map((item) {
                                  final isSelected = state.sellItems.contains(item);
                                  return InkWell(
                                    onTap: () => notifier.toggleSellItem(item),
                                    child: Container(
                                      width: 200,
                                      padding: const EdgeInsets.all(12),
                                      decoration: BoxDecoration(
                                        color: const Color(0xFF1A1A33),
                                        border: Border.all(color: isSelected ? Colors.green : Colors.white24, width: 2),
                                        borderRadius: BorderRadius.circular(12),
                                      ),
                                      child: Row(
                                        children: [
                                          Icon(isSelected ? Icons.check_circle : Icons.radio_button_unchecked, color: isSelected ? Colors.green : Colors.white70),
                                          const SizedBox(width: 8),
                                          Expanded(child: Text(item, style: const TextStyle(fontFamily: 'Inter', color: Colors.white))),
                                        ],
                                      ),
                                    ),
                                  );
                                }).toList(),
                              ),
                            ] else if (state.step == 4) ...[
                              const Text('How do you want to receive payments?', style: TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 18, fontWeight: FontWeight.bold)),
                              const SizedBox(height: 16),
                              Wrap(
                                spacing: 16,
                                runSpacing: 16,
                                alignment: WrapAlignment.center,
                                children: [
                                  {'type': 'Online only', 'desc': 'Get paid via Stripe', 'time': 'Fast'},
                                  {'type': 'In-person (POS)', 'desc': 'Tap-to-pay or card reader', 'time': 'Fast'},
                                  {'type': 'Both', 'desc': 'Online and in-person', 'time': 'Fast'},
                                  {'type': 'Skip for now', 'desc': 'Set up later', 'time': ''},
                                ].map((paymentInfo) {
                                  final item = paymentInfo['type']!;
                                  final desc = paymentInfo['desc']!;
                                  final isSelected = state.paymentType == item;
                                  return InkWell(
                                    onTap: () => notifier.updatePaymentType(item),
                                    child: Container(
                                      width: 200,
                                      padding: const EdgeInsets.all(16),
                                      decoration: BoxDecoration(
                                        color: const Color(0xFF1A1A33),
                                        border: Border.all(color: isSelected ? Colors.blue : Colors.white24, width: 2),
                                        borderRadius: BorderRadius.circular(12),
                                      ),
                                      child: Column(
                                        crossAxisAlignment: CrossAxisAlignment.start,
                                        children: [
                                          Row(
                                            mainAxisAlignment: MainAxisAlignment.spaceBetween,
                                            children: [
                                              Expanded(child: Text(item, style: const TextStyle(fontFamily: 'Inter', color: Colors.white, fontWeight: FontWeight.bold))),
                                              if (isSelected) const Icon(Icons.check_circle, color: Colors.blue, size: 20),
                                            ],
                                          ),
                                          const SizedBox(height: 8),
                                          Text(desc, style: const TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 12)),
                                        ],
                                      ),
                                    ),
                                  );
                                }).toList(),
                              ),
                                                        ] else if (state.step == 5) ...[
                              const Text('Administrator Account', style: TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 18, fontWeight: FontWeight.bold)),
                              const SizedBox(height: 16),
                              TextField(
                                decoration: const InputDecoration(labelText: 'Name', labelStyle: TextStyle(color: Colors.white70)),
                                onChanged: notifier.updateAdminName,
                                style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                              ),
                              const SizedBox(height: 16),
                              TextField(
                                decoration: const InputDecoration(labelText: 'Email', labelStyle: TextStyle(color: Colors.white70)),
                                onChanged: notifier.updateAdminEmail,
                                keyboardType: TextInputType.emailAddress,
                                style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                              ),
                              const SizedBox(height: 16),
                              TextField(
                                decoration: const InputDecoration(labelText: 'Password', labelStyle: TextStyle(color: Colors.white70)),
                                onChanged: notifier.updateAdminPassword,
                                obscureText: true,
                                style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                              ),
                              const SizedBox(height: 8),
                              LinearProgressIndicator(
                                value: state.adminPassword.length / 12 > 1.0 ? 1.0 : state.adminPassword.length / 12,
                                backgroundColor: Colors.white24,
                                color: state.adminPassword.length > 8 ? Colors.green : Colors.red,
                              ),
                              const SizedBox(height: 4),
                              Text(state.adminPassword.length > 8 ? 'Strong' : 'Weak', style: TextStyle(color: state.adminPassword.length > 8 ? Colors.green : Colors.red, fontSize: 12)),
                              const SizedBox(height: 24),
                              Row(
                                children: [
                                  Expanded(child: OutlinedButton.icon(onPressed: () {}, icon: const Icon(Icons.g_mobiledata), label: const Text('Google'))),
                                  const SizedBox(width: 16),
                                  Expanded(child: OutlinedButton.icon(onPressed: () {}, icon: const Icon(Icons.apple), label: const Text('Apple'))),
                                ],
                              ),
                            ] else if (state.step == 6) ...[
                              const Text('Ready to Launch', style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 24, fontWeight: FontWeight.bold)),
                              const SizedBox(height: 16),
                              Container(
                                padding: const EdgeInsets.all(16),
                                decoration: BoxDecoration(
                                  color: const Color(0xFF1A1A33),
                                  borderRadius: BorderRadius.circular(12),
                                ),
                                child: Column(
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  children: [
                                    Text('Business: ${state.companyName}', style: const TextStyle(color: Colors.white, fontWeight: FontWeight.bold, fontSize: 16)),
                                    Text('Type: ${state.businessType}', style: const TextStyle(color: Colors.white70)),
                                    const SizedBox(height: 8),
                                    const Text('You are about to deploy your business. AI agents will automatically design a starting website and begin working for you in the background.', style: TextStyle(color: Colors.white70)),
                                  ],
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
                            if (state.step < 6) {
                              notifier.nextStep();
                            } else {
                              notifier.launch(context, ref);
                            }
                          },
                          child: state.isLoading
                              ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2))
                              : Text(state.step == 6 ? 'Launch My Business →' : 'Next', style: const TextStyle(fontFamily: 'Inter')),
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