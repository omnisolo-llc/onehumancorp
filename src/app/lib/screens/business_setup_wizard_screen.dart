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
  final String companyName;
  final String description;
  final List<String> whatDoYouSell;
  final String paymentMethod;
  final String industry;
  final String size;
  final List<String> goals;
  final String deployment;
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
    this.whatDoYouSell = const [],
    this.paymentMethod = '',
    this.industry = '',
    this.size = 'S',
    this.goals = const [],
    this.deployment = 'Cloud',
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
    String? description,
    List<String>? whatDoYouSell,
    String? paymentMethod,
    String? industry,
    String? size,
    List<String>? goals,
    String? deployment,
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
      description: description ?? this.description,
      whatDoYouSell: whatDoYouSell ?? this.whatDoYouSell,
      paymentMethod: paymentMethod ?? this.paymentMethod,
      industry: industry ?? this.industry,
      size: size ?? this.size,
      goals: goals ?? this.goals,
      deployment: deployment ?? this.deployment,
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
  void updateCompany(String name) => state = state.copyWith(companyName: name);
  void updateDescription(String desc) => state = state.copyWith(description: desc);
  void toggleWhatDoYouSell(String item) {
    final items = List<String>.from(state.whatDoYouSell);
    if (items.contains(item)) {
      items.remove(item);
    } else {
      items.add(item);
    }
    state = state.copyWith(whatDoYouSell: items);
  }
  void updatePaymentMethod(String method) => state = state.copyWith(paymentMethod: method);
  void updateIndustry(String val) => state = state.copyWith(industry: val);
  void updateSize(String val) => state = state.copyWith(size: val);
  void toggleGoal(String goal) {
    final goals = List<String>.from(state.goals);
    if (goals.contains(goal)) {
      goals.remove(goal);
    } else {
      goals.add(goal);
    }
    state = state.copyWith(goals: goals);
  }
  void updateDeployment(String val) => state = state.copyWith(deployment: val);
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
          'company_name': state.companyName,
          'business_type': state.businessType,
          'description': state.description,
          'what_do_you_sell': state.whatDoYouSell.join(','),
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
    final clientSettings = ref.watch(clientSettingsProvider).valueOrNull;
    final isStandalone = clientSettings?.standaloneMode ?? false;

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
                            const Text('Welcome! Your AI team, ready in minutes.', style: TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 16)),
                          ] else if (state.step == 1) ...[
                            const Text('What kind of business are you building?', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
                            const SizedBox(height: 16),
                            Wrap(
                              spacing: 16,
                              runSpacing: 16,
                              children: ['Online Store', 'Service Business', 'Restaurant / Food', 'Creative / Portfolio', 'Local Business', 'Other'].map((type) =>
                                InkWell(
                                  onTap: () => notifier.updateBusinessType(type),
                                  child: Container(
                                    width: 120,
                                    padding: const EdgeInsets.all(16),
                                    decoration: BoxDecoration(
                                      color: state.businessType == type ? Colors.blueAccent.withValues(alpha: 0.3) : Colors.transparent,
                                      border: Border.all(color: state.businessType == type ? Colors.blueAccent : Colors.white24),
                                      borderRadius: BorderRadius.circular(12),
                                    ),
                                    child: Column(
                                      children: [
                                        Icon(
                                          type == 'Online Store' ? Icons.storefront :
                                          type == 'Service Business' ? Icons.handyman :
                                          type == 'Restaurant / Food' ? Icons.restaurant :
                                          type == 'Creative / Portfolio' ? Icons.palette :
                                          type == 'Local Business' ? Icons.location_on : Icons.business,
                                          color: Colors.white,
                                          size: 32,
                                        ),
                                        const SizedBox(height: 8),
                                        Text(type, textAlign: TextAlign.center, style: const TextStyle(color: Colors.white, fontFamily: 'Inter', fontSize: 12)),
                                      ],
                                    ),
                                  ),
                                ),
                              ).toList(),
                            ),
                          ] else if (state.step == 2) ...[
                            TextField(
                              decoration: const InputDecoration(labelText: 'Company Name', labelStyle: TextStyle(color: Colors.white70)),
                              onChanged: notifier.updateCompany,
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                            ),
                            const SizedBox(height: 16),
                            TextField(
                              decoration: const InputDecoration(labelText: 'Short Description', labelStyle: TextStyle(color: Colors.white70)),
                              onChanged: notifier.updateDescription,
                              maxLines: 3,
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                            ),
                          ] else if (state.step == 3) ...[
                             const Text('What do you sell?', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
                             ...['Physical products', 'Digital downloads', 'Services / appointments', 'Food & beverages', 'Subscriptions'].map((item) => CheckboxListTile(
                              title: Text(item, style: const TextStyle(fontFamily: 'Inter', color: Colors.white)),
                              value: state.whatDoYouSell.contains(item),
                              checkColor: Colors.black,
                              activeColor: Colors.white,
                              onChanged: (bool? value) {
                                notifier.toggleWhatDoYouSell(item);
                              },
                            )),
                          ] else if (state.step == 4) ...[
                            const Text('How do you want to receive payments?', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
                            const SizedBox(height: 16),
                            ...['Online only', 'In-person (POS)', 'Both', 'Skip for now'].map((method) => RadioListTile<String>(
                              title: Text(method, style: const TextStyle(fontFamily: 'Inter', color: Colors.white)),
                              value: method,
                              groupValue: state.paymentMethod,
                              activeColor: Colors.blueAccent,
                              onChanged: (String? value) {
                                if (value != null) notifier.updatePaymentMethod(value);
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
                              obscureText: _obscurePassword,
                              onChanged: notifier.updateAdminPassword,
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                              decoration: InputDecoration(
                                labelText: 'Admin Password',
                                labelStyle: const TextStyle(color: Colors.white70),
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
                          ] else if (state.step == 6) ...[
                            const Text('Review & Launch', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white, fontSize: 20)),
                            const SizedBox(height: 16),
                            Text('Business: ${state.companyName} (${state.businessType})', style: const TextStyle(color: Colors.white70, fontFamily: 'Inter')),
                            Text('Selling: ${state.whatDoYouSell.join(", ")}', style: const TextStyle(color: Colors.white70, fontFamily: 'Inter')),
                            Text('Payments: ${state.paymentMethod}', style: const TextStyle(color: Colors.white70, fontFamily: 'Inter')),
                            Text('Admin: ${state.adminEmail}', style: const TextStyle(color: Colors.white70, fontFamily: 'Inter')),
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
