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
  final List<String> whatDoYouSell;
  final String payments;
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
    this.whatDoYouSell = const [],
    this.payments = '',
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
    List<String>? whatDoYouSell,
    String? payments,
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
      whatDoYouSell: whatDoYouSell ?? this.whatDoYouSell,
      payments: payments ?? this.payments,
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
  void toggleWhatDoYouSell(String item) {
    final list = List<String>.from(state.whatDoYouSell);
    if (list.contains(item)) {
      list.remove(item);
    } else {
      list.add(item);
    }
    state = state.copyWith(whatDoYouSell: list);
  }
  void updatePayments(String val) => state = state.copyWith(payments: val);
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
          'company_name': state.businessName,
          'business_description': state.businessDescription,
          'what_do_you_sell': state.whatDoYouSell.join(','),
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

class BusinessSetupWizardScreen extends ConsumerStatefulWidget {
  const BusinessSetupWizardScreen({super.key});

  @override
  ConsumerState<BusinessSetupWizardScreen> createState() => _BusinessSetupWizardScreenState();
}

class _BusinessSetupWizardScreenState extends ConsumerState<BusinessSetupWizardScreen> {
  bool _obscurePassword = true;

  Widget _buildStep0() {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: const [
        Text(
          'Your business, live in minutes.',
          style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 28, fontWeight: FontWeight.bold),
          textAlign: TextAlign.center,
        ),
        SizedBox(height: 16),
        Text(
          'Our AI agents will set up your website, configure your inventory, and handle your customers. Let’s get started.',
          style: TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 16),
          textAlign: TextAlign.center,
        ),
      ],
    );
  }

  Widget _buildStep1(BusinessSetupState state, BusinessSetupNotifier notifier) {
    final types = ['Online Store', 'Service Business', 'Restaurant / Food', 'Creative / Portfolio', 'Local Business', 'Other'];
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        const Text(
          'What type of business are you building?',
          style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 20, fontWeight: FontWeight.bold),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 16),
        Wrap(
          spacing: 12,
          runSpacing: 12,
          alignment: WrapAlignment.center,
          children: types.map((type) => GestureDetector(
            onTap: () => notifier.updateBusinessType(type),
            child: AnimatedContainer(
              duration: const Duration(milliseconds: 200),
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
              decoration: BoxDecoration(
                color: state.businessType == type ? Colors.blueAccent.withOpacity(0.3) : Colors.white.withOpacity(0.05),
                border: Border.all(color: state.businessType == type ? Colors.blueAccent : Colors.white.withOpacity(0.1)),
                borderRadius: BorderRadius.circular(12),
              ),
              child: Text(
                type,
                style: TextStyle(
                  fontFamily: 'Inter',
                  color: state.businessType == type ? Colors.white : Colors.white70,
                ),
              ),
            ),
          )).toList(),
        ),
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(businessSetupProvider);
    final notifier = ref.read(businessSetupProvider.notifier);

    return Scaffold(
      backgroundColor: Colors.transparent,
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
                    if (state.step > 0)
                      const Text(
                        'Business Setup',
                        style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white70),
                      ),
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
                            if (state.step == 0) _buildStep0(),
                            if (state.step == 1) _buildStep1(state, notifier),
                            if (state.step == 2) _buildStep2(state, notifier),
                            if (state.step == 3) _buildStep3(state, notifier),
                            if (state.step == 4) _buildStep4(state, notifier),
                            if (state.step == 5) _buildStep5(state, notifier),
                            if (state.step == 6) _buildStep6(state),
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
                          style: ElevatedButton.styleFrom(
                            backgroundColor: Colors.blueAccent,
                            padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
                            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                          ),
                          onPressed: state.isLoading ? null : () {
                            if (state.step < 6) {
                              notifier.nextStep();
                            } else {
                              notifier.launch(context, ref);
                            }
                          },
                          child: state.isLoading
                              ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white))
                              : Text(
                                  state.step == 6 ? 'Launch My Business →' : (state.step == 0 ? 'Get Started' : 'Next'),
                                  style: const TextStyle(fontFamily: 'Inter', color: Colors.white, fontWeight: FontWeight.bold),
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

  Widget _buildStep2(BusinessSetupState state, BusinessSetupNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text(
          'What is your business called?',
          style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 20, fontWeight: FontWeight.bold),
        ),
        const SizedBox(height: 16),
        TextField(
          decoration: const InputDecoration(
            labelText: 'Business Name',
            labelStyle: TextStyle(color: Colors.white70),
            filled: true,
            fillColor: Colors.black26,
            border: OutlineInputBorder(borderRadius: BorderRadius.all(Radius.circular(12))),
          ),
          onChanged: notifier.updateBusinessName,
          style: const TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 18),
        ),
        const SizedBox(height: 24),
        const Text(
          'Describe what you do (optional)',
          style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 20, fontWeight: FontWeight.bold),
        ),
        const SizedBox(height: 16),
        TextField(
          maxLines: 3,
          decoration: const InputDecoration(
            labelText: 'Description',
            labelStyle: TextStyle(color: Colors.white70),
            filled: true,
            fillColor: Colors.black26,
            border: OutlineInputBorder(borderRadius: BorderRadius.all(Radius.circular(12))),
            hintText: 'e.g. I bake custom cakes from my kitchen...',
            hintStyle: TextStyle(color: Colors.white38),
          ),
          onChanged: notifier.updateBusinessDescription,
          style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
        ),
      ],
    );
  }

  Widget _buildStep3(BusinessSetupState state, BusinessSetupNotifier notifier) {
    final sellItems = ['Physical products', 'Digital downloads', 'Services / appointments', 'Food & beverages', 'Subscriptions'];
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        const Text(
          'What do you sell?',
          style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 20, fontWeight: FontWeight.bold),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 8),
        const Text(
          'Select all that apply.',
          style: TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 14),
        ),
        const SizedBox(height: 16),
        Wrap(
          spacing: 12,
          runSpacing: 12,
          alignment: WrapAlignment.center,
          children: sellItems.map((item) {
            final isSelected = state.whatDoYouSell.contains(item);
            return GestureDetector(
              onTap: () => notifier.toggleWhatDoYouSell(item),
              child: AnimatedContainer(
                duration: const Duration(milliseconds: 200),
                padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                decoration: BoxDecoration(
                  color: isSelected ? Colors.green.withOpacity(0.3) : Colors.white.withOpacity(0.05),
                  border: Border.all(color: isSelected ? Colors.green : Colors.white.withOpacity(0.1)),
                  borderRadius: BorderRadius.circular(12),
                ),
                child: Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Icon(isSelected ? Icons.check_circle : Icons.circle_outlined, color: isSelected ? Colors.green : Colors.white54, size: 20),
                    const SizedBox(width: 8),
                    Text(
                      item,
                      style: TextStyle(
                        fontFamily: 'Inter',
                        color: isSelected ? Colors.white : Colors.white70,
                      ),
                    ),
                  ],
                ),
              ),
            );
          }).toList(),
        ),
      ],
    );
  }

  Widget _buildStep4(BusinessSetupState state, BusinessSetupNotifier notifier) {
    final payOptions = ['Online only', 'In-person (POS)', 'Both', 'Skip for now'];
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        const Text(
          'How do you want to receive payments?',
          style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 20, fontWeight: FontWeight.bold),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 16),
        Column(
          children: payOptions.map((opt) => Padding(
            padding: const EdgeInsets.only(bottom: 12.0),
            child: GestureDetector(
              onTap: () => notifier.updatePayments(opt),
              child: AnimatedContainer(
                duration: const Duration(milliseconds: 200),
                width: double.infinity,
                padding: const EdgeInsets.all(16),
                decoration: BoxDecoration(
                  color: state.payments == opt ? Colors.blueAccent.withOpacity(0.3) : Colors.white.withOpacity(0.05),
                  border: Border.all(color: state.payments == opt ? Colors.blueAccent : Colors.white.withOpacity(0.1)),
                  borderRadius: BorderRadius.circular(12),
                ),
                child: Text(
                  opt,
                  style: TextStyle(
                    fontFamily: 'Inter',
                    fontSize: 16,
                    color: state.payments == opt ? Colors.white : Colors.white70,
                  ),
                  textAlign: TextAlign.center,
                ),
              ),
            ),
          )).toList(),
        ),
      ],
    );
  }

  Widget _buildStep5(BusinessSetupState state, BusinessSetupNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text(
          'Create your admin account',
          style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 20, fontWeight: FontWeight.bold),
        ),
        const SizedBox(height: 16),
        TextField(
          decoration: const InputDecoration(
            labelText: 'Your Name',
            labelStyle: TextStyle(color: Colors.white70),
            filled: true,
            fillColor: Colors.black26,
            border: OutlineInputBorder(borderRadius: BorderRadius.all(Radius.circular(12))),
          ),
          onChanged: notifier.updateAdminName,
          style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
        ),
        const SizedBox(height: 16),
        TextField(
          decoration: const InputDecoration(
            labelText: 'Email Address',
            labelStyle: TextStyle(color: Colors.white70),
            filled: true,
            fillColor: Colors.black26,
            border: OutlineInputBorder(borderRadius: BorderRadius.all(Radius.circular(12))),
          ),
          keyboardType: TextInputType.emailAddress,
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
            filled: true,
            fillColor: Colors.black26,
            border: const OutlineInputBorder(borderRadius: BorderRadius.all(Radius.circular(12))),
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
      ],
    );
  }

  Widget _buildStep6(BusinessSetupState state) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        const Icon(Icons.rocket_launch, size: 64, color: Colors.blueAccent),
        const SizedBox(height: 16),
        const Text(
          'You are ready to launch!',
          style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 24, fontWeight: FontWeight.bold),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 24),
        Container(
          padding: const EdgeInsets.all(16),
          decoration: BoxDecoration(
            color: Colors.white.withOpacity(0.05),
            borderRadius: BorderRadius.circular(12),
            border: Border.all(color: Colors.white.withOpacity(0.1)),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              _buildSummaryRow('Business', state.businessName.isEmpty ? 'Not set' : state.businessName),
              _buildSummaryRow('Type', state.businessType.isEmpty ? 'Not set' : state.businessType),
              _buildSummaryRow('Selling', state.whatDoYouSell.isEmpty ? 'Not set' : state.whatDoYouSell.join(', ')),
              _buildSummaryRow('Payments', state.payments.isEmpty ? 'Not set' : state.payments),
              _buildSummaryRow('Admin', state.adminEmail.isEmpty ? 'Not set' : state.adminEmail),
            ],
          ),
        ),
      ],
    );
  }

  Widget _buildSummaryRow(String label, String value) {
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
            child: Text(value, style: const TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 14, fontWeight: FontWeight.bold)),
          ),
        ],
      ),
    );
  }

}
