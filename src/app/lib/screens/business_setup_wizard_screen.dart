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
  final String businessType;
  final String businessName;
  final String businessDescription;
  final List<String> sellItems;
  final String paymentMethod;
  final String adminName;
  final String adminEmail;
  final String adminPassword;
  final bool isLoading;
  final String? errorMessage;

  const BusinessSetupState({
    this.step = 0,
    this.businessType = '',
    this.businessName = '',
    this.businessDescription = '',
    this.sellItems = const [],
    this.paymentMethod = '',
    this.adminName = '',
    this.adminEmail = '',
    this.adminPassword = '',
    this.isLoading = false,
    this.errorMessage,
  });

  BusinessSetupState copyWith({
    int? step,
    String? businessType,
    String? businessName,
    String? businessDescription,
    List<String>? sellItems,
    String? paymentMethod,
    String? adminName,
    String? adminEmail,
    String? adminPassword,
    bool? isLoading,
    String? errorMessage,
  }) {
    return BusinessSetupState(
      step: step ?? this.step,
      businessType: businessType ?? this.businessType,
      businessName: businessName ?? this.businessName,
      businessDescription: businessDescription ?? this.businessDescription,
      sellItems: sellItems ?? this.sellItems,
      paymentMethod: paymentMethod ?? this.paymentMethod,
      adminName: adminName ?? this.adminName,
      adminEmail: adminEmail ?? this.adminEmail,
      adminPassword: adminPassword ?? this.adminPassword,
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

  void updateBusinessType(String type) => state = state.copyWith(businessType: type);
  void updateBusinessName(String name) => state = state.copyWith(businessName: name);
  void updateBusinessDescription(String desc) => state = state.copyWith(businessDescription: desc);

  void toggleSellItem(String item) {
    final items = List<String>.from(state.sellItems);
    if (items.contains(item)) {
      items.remove(item);
    } else {
      items.add(item);
    }
    state = state.copyWith(sellItems: items);
  }

  void updatePaymentMethod(String val) => state = state.copyWith(paymentMethod: val);
  void updateAdminName(String name) => state = state.copyWith(adminName: name);
  void updateAdminEmail(String val) => state = state.copyWith(adminEmail: val);
  void updateAdminPassword(String val) => state = state.copyWith(adminPassword: val);

  Future<void> launch(BuildContext context, WidgetRef ref) async {
    final user = ref.read(authStateProvider).valueOrNull;
    final baseUrl = ref.read(backendUrlProvider);

    state = state.copyWith(isLoading: true, errorMessage: null);

    if (user != null && baseUrl.isNotEmpty) {
      final body = {
        'extras': {
          'business_type': state.businessType,
          'business_name': state.businessName,
          'business_description': state.businessDescription,
          'sell_items': state.sellItems.join(','),
          'payment_method': state.paymentMethod,
          'admin_name': state.adminName,
          'admin_email': state.adminEmail,
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

    if (context.mounted) {
      GoRouter.of(context).go('/dashboard');
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
  bool _obscurePassword = true;

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(businessSetupProvider);
    final notifier = ref.read(businessSetupProvider.notifier);

    // Check if simple strength meter is green. We just check length > 5
    Color passwordStrengthColor = state.adminPassword.length > 5 ? Colors.green : Colors.red;

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
                          crossAxisAlignment: CrossAxisAlignment.stretch,
                          children: [
                            if (state.step == 0) ...[
                              const Center(
                                child: Text('Your business, live in minutes.',
                                  style: TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 18, fontWeight: FontWeight.w600),
                                  textAlign: TextAlign.center,
                                ),
                              ),
                              const SizedBox(height: 24),
                              const Center(
                                child: Text('Welcome! Let\'s get your AI team and platform configured.',
                                  style: TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 14),
                                  textAlign: TextAlign.center,
                                ),
                              ),
                            ] else if (state.step == 1) ...[
                              const Text('What type of business are you starting?', style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold, color: Colors.white)),
                              const SizedBox(height: 16),
                              Wrap(
                                spacing: 12,
                                runSpacing: 12,
                                children: ['Online Store', 'Service Business', 'Restaurant / Food', 'Creative / Portfolio', 'Local Business', 'Other'].map((type) {
                                  bool isSelected = state.businessType == type;
                                  return InkWell(
                                    onTap: () => notifier.updateBusinessType(type),
                                    borderRadius: BorderRadius.circular(12),
                                    child: Container(
                                      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                                      decoration: BoxDecoration(
                                        color: isSelected ? Theme.of(context).colorScheme.primary.withOpacity(0.2) : Colors.white10,
                                        border: Border.all(color: isSelected ? Theme.of(context).colorScheme.primary : Colors.white24),
                                        borderRadius: BorderRadius.circular(12),
                                      ),
                                      child: Text(type, style: const TextStyle(fontFamily: 'Inter', color: Colors.white)),
                                    ),
                                  );
                                }).toList(),
                              ),
                            ] else if (state.step == 2) ...[
                              const Text('Tell us about your business', style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold, color: Colors.white)),
                              const SizedBox(height: 16),
                              TextField(
                                decoration: const InputDecoration(
                                  labelText: 'Business Name',
                                  labelStyle: TextStyle(color: Colors.white70),
                                  border: OutlineInputBorder(),
                                ),
                                onChanged: notifier.updateBusinessName,
                                style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                              ),
                              const SizedBox(height: 16),
                              TextField(
                                decoration: const InputDecoration(
                                  labelText: 'Short Description',
                                  labelStyle: TextStyle(color: Colors.white70),
                                  border: OutlineInputBorder(),
                                  helperText: 'AI will help you refine this later.',
                                  helperStyle: TextStyle(color: Colors.white54),
                                ),
                                maxLines: 3,
                                onChanged: notifier.updateBusinessDescription,
                                style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                              ),
                            ] else if (state.step == 3) ...[
                               const Text('What do you sell?', style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold, color: Colors.white)),
                               const SizedBox(height: 16),
                               ...['Physical products', 'Digital downloads', 'Services / appointments', 'Food & beverages', 'Subscriptions'].map((item) {
                                bool isSelected = state.sellItems.contains(item);
                                return Padding(
                                  padding: const EdgeInsets.only(bottom: 8.0),
                                  child: CheckboxListTile(
                                    title: Text(item, style: const TextStyle(fontFamily: 'Inter', color: Colors.white)),
                                    value: isSelected,
                                    checkColor: Colors.black,
                                    activeColor: Theme.of(context).colorScheme.primary,
                                    tileColor: isSelected ? Theme.of(context).colorScheme.primary.withOpacity(0.1) : Colors.white10,
                                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                                    onChanged: (bool? value) {
                                      notifier.toggleSellItem(item);
                                    },
                                  ),
                                );
                              }),
                            ] else if (state.step == 4) ...[
                               const Text('How do you want to receive payments?', style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold, color: Colors.white)),
                               const SizedBox(height: 16),
                               ...[
                                 {'label': 'Online only', 'eta': 'Ready immediately'},
                                 {'label': 'In-person (POS)', 'eta': 'Hardware ships in 2 days'},
                                 {'label': 'Both', 'eta': 'Online ready now, POS later'},
                                 {'label': 'Skip for now', 'eta': ''},
                               ].map((methodData) {
                                  String method = methodData['label']!;
                                  String eta = methodData['eta']!;
                                  bool isSelected = state.paymentMethod == method;
                                  return Padding(
                                    padding: const EdgeInsets.only(bottom: 8.0),
                                    child: InkWell(
                                      onTap: () => notifier.updatePaymentMethod(method),
                                      borderRadius: BorderRadius.circular(12),
                                      child: Container(
                                        padding: const EdgeInsets.all(16),
                                        decoration: BoxDecoration(
                                          color: isSelected ? Theme.of(context).colorScheme.primary.withOpacity(0.2) : Colors.white10,
                                          border: Border.all(color: isSelected ? Theme.of(context).colorScheme.primary : Colors.white24),
                                          borderRadius: BorderRadius.circular(12),
                                        ),
                                        child: Row(
                                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                                          children: [
                                            Text(method, style: const TextStyle(fontFamily: 'Inter', color: Colors.white, fontWeight: FontWeight.w500)),
                                            if (eta.isNotEmpty)
                                              Text(eta, style: const TextStyle(fontFamily: 'Inter', color: Colors.white54, fontSize: 12)),
                                          ],
                                        ),
                                      ),
                                    ),
                                  );
                                }),
                            ] else if (state.step == 5) ...[
                              const Text('Administrator Account', style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold, color: Colors.white)),
                              const SizedBox(height: 16),
                              TextField(
                                decoration: const InputDecoration(
                                  labelText: 'Name',
                                  labelStyle: TextStyle(color: Colors.white70),
                                  border: OutlineInputBorder(),
                                ),
                                onChanged: notifier.updateAdminName,
                                style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                              ),
                              const SizedBox(height: 16),
                              TextField(
                                decoration: const InputDecoration(
                                  labelText: 'Email',
                                  labelStyle: TextStyle(color: Colors.white70),
                                  border: OutlineInputBorder(),
                                ),
                                onChanged: notifier.updateAdminEmail,
                                style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                              ),
                              const SizedBox(height: 16),
                              TextField(
                                obscureText: _obscurePassword,
                                onChanged: notifier.updateAdminPassword,
                                style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                                decoration: InputDecoration(
                                  labelText: 'Password',
                                  labelStyle: const TextStyle(color: Colors.white70),
                                  border: const OutlineInputBorder(),
                                  suffixIcon: IconButton(
                                    icon: Icon(_obscurePassword ? Icons.visibility : Icons.visibility_off, color: Colors.white70),
                                    onPressed: () {
                                      setState(() {
                                        _obscurePassword = !_obscurePassword;
                                      });
                                    },
                                  ),
                                ),
                              ),
                              if (state.adminPassword.isNotEmpty)
                                Padding(
                                  padding: const EdgeInsets.only(top: 8.0),
                                  child: Row(
                                    children: [
                                      Container(
                                        width: 100,
                                        height: 4,
                                        decoration: BoxDecoration(
                                          color: passwordStrengthColor,
                                          borderRadius: BorderRadius.circular(2),
                                        ),
                                      ),
                                      const SizedBox(width: 8),
                                      Text(state.adminPassword.length > 5 ? 'Strong' : 'Weak', style: TextStyle(color: passwordStrengthColor, fontSize: 12)),
                                    ],
                                  ),
                                ),
                            ] else if (state.step == 6) ...[
                              const Text('Review & Launch', style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold, color: Colors.white)),
                              const SizedBox(height: 16),
                              Container(
                                padding: const EdgeInsets.all(16),
                                decoration: BoxDecoration(
                                  color: Colors.white.withOpacity(0.05),
                                  border: Border.all(color: Colors.white.withOpacity(0.1)),
                                  borderRadius: BorderRadius.circular(12),
                                ),
                                child: Column(
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  children: [
                                    _ReviewRow(label: 'Name', value: state.businessName),
                                    _ReviewRow(label: 'Type', value: state.businessType),
                                    _ReviewRow(label: 'Selling', value: state.sellItems.join(', ')),
                                    _ReviewRow(label: 'Payments', value: state.paymentMethod),
                                    _ReviewRow(label: 'Admin', value: state.adminEmail),
                                  ],
                                ),
                              ),
                            ],
                          ],
                        ),
                      ),
                    ),
                    const SizedBox(height: 32),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        if (state.step > 0)
                          TextButton(
                            onPressed: state.isLoading ? null : notifier.prevStep,
                            child: const Text('Back', style: TextStyle(fontFamily: 'Inter', color: Colors.white70)),
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
                          style: ElevatedButton.styleFrom(
                            backgroundColor: Theme.of(context).colorScheme.primary,
                            foregroundColor: Theme.of(context).colorScheme.onPrimary,
                            padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
                            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                          ),
                          child: state.isLoading
                              ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white))
                              : Text(state.step == 6 ? 'Launch My Business →' : 'Next', style: const TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
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

class _ReviewRow extends StatelessWidget {
  final String label;
  final String value;
  const _ReviewRow({required this.label, required this.value});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 8.0),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            width: 80,
            child: Text(label, style: const TextStyle(fontFamily: 'Inter', color: Colors.white54, fontSize: 14)),
          ),
          Expanded(
            child: Text(value.isEmpty ? 'Not specified' : value, style: const TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 14, fontWeight: FontWeight.w500)),
          ),
        ],
      ),
    );
  }
}
