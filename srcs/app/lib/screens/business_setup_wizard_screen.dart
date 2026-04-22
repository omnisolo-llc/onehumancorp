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
  final List<String> whatYouSell;
  final String payments;
  final String adminName;
  final String adminEmail;
  final String adminPassword;
  final bool isLoading;
  final String? errorMessage;
  final bool obscurePassword;

  const BusinessSetupState({
    this.step = 0,
    this.businessType = '',
    this.companyName = '',
    this.description = '',
    this.whatYouSell = const [],
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
    List<String>? whatYouSell,
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
      whatYouSell: whatYouSell ?? this.whatYouSell,
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

  void updateBusinessType(String val) => state = state.copyWith(businessType: val);
  void updateCompany(String name) => state = state.copyWith(companyName: name);
  void updateDescription(String val) => state = state.copyWith(description: val);
  void toggleWhatYouSell(String val) {
    final list = List<String>.from(state.whatYouSell);
    if (list.contains(val)) {
      list.remove(val);
    } else {
      list.add(val);
    }
    state = state.copyWith(whatYouSell: list);
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
          'what_you_sell': state.whatYouSell.join(','),
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
      context.go('/dashboard');
    }
  }
}

final businessSetupProvider = NotifierProvider<BusinessSetupNotifier, BusinessSetupState>(() {
  return BusinessSetupNotifier();
});

class BusinessSetupWizardScreen extends ConsumerWidget {
  const BusinessSetupWizardScreen({super.key});

  Widget _buildStepZero() {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: const [
        Text('Welcome! Your AI team, ready in minutes.', style: TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 16)),
        SizedBox(height: 8),
        Text('Your business, live in minutes.', style: TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 14)),
      ],
    );
  }

  Widget _buildStepOne(BusinessSetupState state, BusinessSetupNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        const Text('Business type', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
        const SizedBox(height: 16),
        Wrap(
          spacing: 8.0,
          runSpacing: 8.0,
          children: ['Online Store', 'Service Business', 'Restaurant / Food', 'Creative / Portfolio', 'Local Business', 'Other'].map((type) {
            final isSelected = state.businessType == type;
            return ChoiceChip(
              label: Text(type, style: const TextStyle(fontFamily: 'Inter')),
              selected: isSelected,
              onSelected: (selected) {
                if (selected) notifier.updateBusinessType(type);
              },
              selectedColor: Colors.blueAccent,
              backgroundColor: const Color(0xFF1A1A33),
              labelStyle: TextStyle(color: isSelected ? Colors.white : Colors.white70),
            );
          }).toList(),
        ),
      ],
    );
  }

  Widget _buildStepTwo(BusinessSetupState state, BusinessSetupNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        TextField(
          decoration: const InputDecoration(labelText: 'Business name', labelStyle: TextStyle(color: Colors.white70)),
          onChanged: notifier.updateCompany,
          style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
        ),
        const SizedBox(height: 16),
        TextField(
          decoration: const InputDecoration(labelText: 'Description', labelStyle: TextStyle(color: Colors.white70)),
          onChanged: notifier.updateDescription,
          style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
        ),
      ],
    );
  }

  Widget _buildStepThree(BusinessSetupState state, BusinessSetupNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        const Text('What do you sell?', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
        ...['Physical products', 'Digital downloads', 'Services / appointments', 'Food & beverages', 'Subscriptions'].map((item) => CheckboxListTile(
          title: Text(item, style: const TextStyle(fontFamily: 'Inter', color: Colors.white)),
          value: state.whatYouSell.contains(item),
          checkColor: Colors.black,
          activeColor: Colors.white,
          onChanged: (bool? value) {
            notifier.toggleWhatYouSell(item);
          },
        )),
      ],
    );
  }

  Widget _buildStepFour(BusinessSetupState state, BusinessSetupNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        const Text('How do you want to receive payments?', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
        ...['Online only', 'In-person (POS)', 'Both', 'Skip for now'].map((dep) => RadioListTile<String>(
          title: Text(dep, style: const TextStyle(fontFamily: 'Inter', color: Colors.white)),
          value: dep,
          groupValue: state.payments,
          activeColor: Colors.blueAccent,
          onChanged: (String? value) {
            if (value != null) notifier.updatePayments(value);
          },
        )),
      ],
    );
  }

  Widget _buildStepFive(BusinessSetupState state, BusinessSetupNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
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
          keyboardType: TextInputType.emailAddress,
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
              onPressed: notifier.toggleObscurePassword,
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildStepSix(BusinessSetupState state) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        const Text('Review & Launch', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white, fontSize: 20)),
        const SizedBox(height: 16),
        GlassCard(
          color: Colors.white.withOpacity(0.05),
          child: Padding(
            padding: const EdgeInsets.all(16.0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text('Business: ${state.companyName}', style: const TextStyle(color: Colors.white)),
                Text('Type: ${state.businessType}', style: const TextStyle(color: Colors.white70)),
                Text('Selling: ${state.whatYouSell.join(", ")}', style: const TextStyle(color: Colors.white70)),
                Text('Payments: ${state.payments}', style: const TextStyle(color: Colors.white70)),
                Text('Admin: ${state.adminName} (${state.adminEmail})', style: const TextStyle(color: Colors.white70)),
              ],
            ),
          ),
        ),
      ],
    );
  }

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
                      child: () {
                        switch (state.step) {
                          case 0: return _buildStepZero();
                          case 1: return _buildStepOne(state, notifier);
                          case 2: return _buildStepTwo(state, notifier);
                          case 3: return _buildStepThree(state, notifier);
                          case 4: return _buildStepFour(state, notifier);
                          case 5: return _buildStepFive(state, notifier);
                          case 6: return _buildStepSix(state);
                          default: return const SizedBox();
                        }
                      }(),
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
