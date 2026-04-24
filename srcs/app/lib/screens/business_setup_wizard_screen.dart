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
  final bool obscurePassword;
  final int step;
  final String businessType;
  final String companyName;
  final String description;
  final List<String> products;
  final String payments;
  final String adminName;
  final String adminEmail;
  final String adminPassword;
  final bool isLoading;
  final String? errorMessage;

  const BusinessSetupState({
    this.step = 0,
    this.businessType = '',
    this.companyName = '',
    this.description = '',
    this.products = const [],
    this.payments = '',
    this.adminName = '',
    this.adminEmail = '',
    this.adminPassword = '',
    this.isLoading = false,
    this.errorMessage,
    this.obscurePassword = true,
  });

  BusinessSetupState copyWith({
    int? step,
    String? businessType,
    String? companyName,
    String? description,
    List<String>? products,
    String? payments,
    String? adminName,
    String? adminEmail,
    String? adminPassword,
    bool? isLoading,
    String? errorMessage,
    bool? obscurePassword,
  }) {
    return BusinessSetupState(
      step: step ?? this.step,
      businessType: businessType ?? this.businessType,
      companyName: companyName ?? this.companyName,
      description: description ?? this.description,
      products: products ?? this.products,
      payments: payments ?? this.payments,
      adminName: adminName ?? this.adminName,
      adminEmail: adminEmail ?? this.adminEmail,
      adminPassword: adminPassword ?? this.adminPassword,
      isLoading: isLoading ?? this.isLoading,
      errorMessage: errorMessage ?? this.errorMessage,
      obscurePassword: obscurePassword ?? this.obscurePassword,
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
  void updateCompany(String name) => state = state.copyWith(companyName: name);
  void updateDescription(String desc) => state = state.copyWith(description: desc);
  void toggleProduct(String product) {
    final products = List<String>.from(state.products);
    if (products.contains(product)) {
      products.remove(product);
    } else {
      products.add(product);
    }
    state = state.copyWith(products: products);
  }
  void updatePayments(String val) => state = state.copyWith(payments: val);
  void updateAdminName(String name) => state = state.copyWith(adminName: name);
  void updateAdminEmail(String val) => state = state.copyWith(adminEmail: val);
  void updateAdminPassword(String val) => state = state.copyWith(adminPassword: val);
  void toggleObscurePassword() => state = state.copyWith(obscurePassword: !state.obscurePassword);

  Future<void> launch(BuildContext context, WidgetRef ref) async {
    final user = ref.read(authStateProvider).valueOrNull;
    final baseUrl = ref.read(backendUrlProvider);

    state = state.copyWith(isLoading: true, errorMessage: null);

    if (user != null && baseUrl.isNotEmpty) {
      final body = {
        'extras': {
          'business_type': state.businessType,
          'company_name': state.companyName,
          'description': state.description,
          'products': state.products.join(','),
          'payments': state.payments,
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
                            const Text('Your business, live in minutes.', style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 28, fontWeight: FontWeight.bold)),
                          ] else if (state.step == 1) ...[
                            const Text('Business Type', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
                            const SizedBox(height: 16),
                            Wrap(
                              spacing: 8.0,
                              runSpacing: 8.0,
                              children: [
                                'Online Store', 'Service Business', 'Restaurant / Food', 'Creative / Portfolio', 'Local Business', 'Other'
                              ].map((type) => GestureDetector(
                                onTap: () => notifier.updateBusinessType(type),
                                child: Container(
                                  padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                                  decoration: BoxDecoration(
                                    color: state.businessType == type ? Colors.blueAccent : Colors.white.withOpacity(0.1),
                                    borderRadius: BorderRadius.circular(8),
                                    border: Border.all(color: state.businessType == type ? Colors.blue : Colors.white24),
                                  ),
                                  child: Text(type, style: const TextStyle(color: Colors.white)),
                                ),
                              )).toList(),
                            ),
                          ] else if (state.step == 2) ...[
                            TextField(
                              decoration: const InputDecoration(labelText: 'Company Name', labelStyle: TextStyle(color: Colors.white70)),
                              onChanged: notifier.updateCompany,
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                            ),
                            const SizedBox(height: 16),
                            TextField(
                              decoration: const InputDecoration(labelText: 'Description / Tagline', labelStyle: TextStyle(color: Colors.white70)),
                              onChanged: notifier.updateDescription,
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                            ),
                          ] else if (state.step == 3) ...[
                             const Text('What do you sell?', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
                             const SizedBox(height: 16),
                             ...['Physical products', 'Digital downloads', 'Services / appointments', 'Food & beverages', 'Subscriptions'].map((product) => CheckboxListTile(
                              title: Text(product, style: const TextStyle(fontFamily: 'Inter', color: Colors.white)),
                              value: state.products.contains(product),
                              checkColor: Colors.black,
                              activeColor: Colors.white,
                              onChanged: (bool? value) {
                                notifier.toggleProduct(product);
                              },
                            )),
                          ] else if (state.step == 4) ...[
                             const Text('How do you want to receive payments?', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
                             const SizedBox(height: 16),
                             ...['Online only', 'In-person (POS)', 'Both', 'Skip for now'].map((payment) => RadioListTile<String>(
                              title: Text(payment, style: const TextStyle(fontFamily: 'Inter', color: Colors.white)),
                              value: payment,
                              groupValue: state.payments,
                              activeColor: Colors.blueAccent,
                              onChanged: (String? value) {
                                if (value != null) notifier.updatePayments(value);
                              },
                            )),
                          ] else if (state.step == 5) ...[
                            TextField(
                              decoration: const InputDecoration(labelText: 'Admin Name', labelStyle: TextStyle(color: Colors.white70)),
                              onChanged: notifier.updateAdminName,
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                            ),
                            const SizedBox(height: 16),
                            TextField(
                              decoration: const InputDecoration(labelText: 'Admin Email', labelStyle: TextStyle(color: Colors.white70)),
                              onChanged: notifier.updateAdminEmail,
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                            ),
                            const SizedBox(height: 16),
                            TextField(
                              obscureText: state.obscurePassword,
                              onChanged: notifier.updateAdminPassword,
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                              decoration: InputDecoration(
                                labelText: 'Admin Password',
                                labelStyle: const TextStyle(color: Colors.white70),
                                suffixIcon: IconButton(
                                  icon: Icon(state.obscurePassword ? Icons.visibility : Icons.visibility_off, color: Colors.white70),
                                  onPressed: () {
                                    notifier.toggleObscurePassword();
                                  },
                                ),
                              ),
                            ),
                          ] else if (state.step == 6) ...[
                             const Text('Review & Launch', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
                             const SizedBox(height: 16),
                             Container(
                               padding: const EdgeInsets.all(16),
                               decoration: BoxDecoration(
                                 color: Colors.white.withOpacity(0.05),
                                 border: Border.all(color: Colors.white.withOpacity(0.1)),
                                 borderRadius: BorderRadius.circular(8),
                               ),
                               child: Column(
                                 crossAxisAlignment: CrossAxisAlignment.start,
                                 children: [
                                   Text('Name: ${state.companyName}', style: const TextStyle(color: Colors.white)),
                                   Text('Type: ${state.businessType}', style: const TextStyle(color: Colors.white)),
                                   Text('Products: ${state.products.join(', ')}', style: const TextStyle(color: Colors.white)),
                                   Text('Payments: ${state.payments}', style: const TextStyle(color: Colors.white)),
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
