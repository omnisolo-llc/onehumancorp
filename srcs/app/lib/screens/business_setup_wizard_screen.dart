import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
import 'dart:ui';
import '../services/auth_service.dart';
import '../services/settings_service.dart';
import '../widgets/glass_card.dart';
import '../widgets/pulse_animation.dart';

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

  Widget _buildStepZero(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        PulseAnimation(child: Text('Welcome! Your AI team, ready in minutes.', style: Theme.of(context).textTheme.headlineSmall?.copyWith(fontFamily: 'Inter', color: Theme.of(context).colorScheme.onSurface))),
        SizedBox(height: 8),
        Text('Your business, live in minutes.', style: Theme.of(context).textTheme.bodyLarge?.copyWith(fontFamily: 'Inter', color: Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.7))),
      ],
    );
  }

  Widget _buildStepOne(BuildContext context, BusinessSetupState state, BusinessSetupNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Text('Business type', style: Theme.of(context).textTheme.titleLarge?.copyWith(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
        const SizedBox(height: 16),
        Wrap(
          spacing: 8.0,
          runSpacing: 8.0,
          children: ['Online Store', 'Service Business', 'Restaurant / Food', 'Creative / Portfolio', 'Local Business', 'Other'].map((type) {
            final isSelected = state.businessType == type;
            return GlassCard(
              color: isSelected ? Theme.of(context).colorScheme.primary.withValues(alpha: 0.2) : Theme.of(context).colorScheme.surface.withValues(alpha: 0.05),
              padding: EdgeInsets.zero,
              child: InkWell(
                onTap: () => notifier.updateBusinessType(type),
                borderRadius: BorderRadius.circular(16),
                child: Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Icon(Icons.business, color: isSelected ? Theme.of(context).colorScheme.primary : Theme.of(context).colorScheme.onSurface, size: 24),
                      const SizedBox(width: 8),
                      Text(
                        type,
                        style: TextStyle(
                          fontFamily: 'Inter',
                          color: isSelected ? Theme.of(context).colorScheme.primary : Theme.of(context).colorScheme.onSurface,
                          fontWeight: isSelected ? FontWeight.bold : FontWeight.normal,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            );
          }).toList(),
        ),
      ],
    );
  }

  Widget _buildStepTwo(BuildContext context, BusinessSetupState state, BusinessSetupNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        TextField(
          decoration: const InputDecoration(labelText: 'Business name', border: OutlineInputBorder()),
          onChanged: notifier.updateCompany,
          style: const TextStyle(fontFamily: 'Inter'),
        ),
        const SizedBox(height: 16),
        TextField(
          decoration: const InputDecoration(labelText: 'Description', border: OutlineInputBorder()),
          onChanged: notifier.updateDescription,
          style: const TextStyle(fontFamily: 'Inter'),
        ),
      ],
    );
  }

  Widget _buildStepThree(BuildContext context, BusinessSetupState state, BusinessSetupNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Text('What do you sell?', style: Theme.of(context).textTheme.titleLarge?.copyWith(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
        ...['Physical products', 'Digital downloads', 'Services / appointments', 'Food & beverages', 'Subscriptions'].map((item) {
          final isSelected = state.whatYouSell.contains(item);
          return Padding(
            padding: const EdgeInsets.only(bottom: 8.0),
            child: GlassCard(
              color: isSelected ? Theme.of(context).colorScheme.primary.withValues(alpha: 0.1) : Theme.of(context).colorScheme.surface.withValues(alpha: 0.05),
              padding: EdgeInsets.zero,
              child: InkWell(
                onTap: () => notifier.toggleWhatYouSell(item),
                borderRadius: BorderRadius.circular(16),
                child: Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 16.0, vertical: 12.0),
                  child: Row(
                    children: [
                      Checkbox(
                        value: isSelected,
                        onChanged: (bool? value) => notifier.toggleWhatYouSell(item),
                        activeColor: Theme.of(context).colorScheme.primary,
                      ),
                      const SizedBox(width: 12),
                      Expanded(
                        child: Text(item, style: Theme.of(context).textTheme.bodyLarge?.copyWith(fontFamily: 'Inter')),
                      ),
                    ],
                  ),
                ),
              ),
            ),
          );
        }),
      ],
    );
  }

  Widget _buildStepFour(BuildContext context, BusinessSetupState state, BusinessSetupNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Text('How do you want to receive payments?', style: Theme.of(context).textTheme.titleLarge?.copyWith(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
        ...[
          {'label': 'Online only', 'time': 'Est. time to first payment: Instant'},
          {'label': 'In-person (POS)', 'time': 'Est. time to first payment: Requires hardware setup'},
          {'label': 'Both', 'time': 'Est. time to first payment: Varies'},
          {'label': 'Skip for now', 'time': ''},
        ].map((opt) {
          final label = opt['label']!;
          final time = opt['time']!;
          final isSelected = state.payments == label;

          return Padding(
            padding: const EdgeInsets.only(bottom: 8.0),
            child: GlassCard(
              color: isSelected ? Theme.of(context).colorScheme.primary.withValues(alpha: 0.1) : Theme.of(context).colorScheme.surface.withValues(alpha: 0.05),
              padding: EdgeInsets.zero,
              child: InkWell(
                onTap: () => notifier.updatePayments(label),
                borderRadius: BorderRadius.circular(16),
                child: Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 16.0, vertical: 12.0),
                  child: Row(
                    children: [
                      Radio<String>(
                        value: label,
                        groupValue: state.payments,
                        onChanged: (String? value) {
                          if (value != null) notifier.updatePayments(value);
                        },
                        activeColor: Theme.of(context).colorScheme.primary,
                      ),
                      const SizedBox(width: 12),
                      Expanded(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text(label, style: Theme.of(context).textTheme.bodyLarge?.copyWith(fontFamily: 'Inter')),
                            if (time.isNotEmpty) ...[
                              const SizedBox(height: 4),
                              Text(time, style: Theme.of(context).textTheme.bodySmall?.copyWith(fontFamily: 'Inter', color: Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.7))),
                            ]
                          ],
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ),
          );
        }),
      ],
    );
  }

  Widget _buildStepFive(BuildContext context, BusinessSetupState state, BusinessSetupNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        TextField(
          decoration: const InputDecoration(labelText: 'Admin Name', border: OutlineInputBorder()),
          onChanged: notifier.updateAdminName,
          style: const TextStyle(fontFamily: 'Inter'),
        ),
        const SizedBox(height: 16),
        TextField(
          decoration: const InputDecoration(labelText: 'Admin Email', border: OutlineInputBorder()),
          onChanged: notifier.updateAdminEmail,
          style: const TextStyle(fontFamily: 'Inter'),
          keyboardType: TextInputType.emailAddress,
        ),
        const SizedBox(height: 16),
        TextField(
          obscureText: state.obscurePassword,
          onChanged: notifier.updateAdminPassword,
          style: const TextStyle(fontFamily: 'Inter'),
          decoration: InputDecoration(
            labelText: 'Admin Password',

            suffixIcon: IconButton(
              icon: Icon(state.obscurePassword ? Icons.visibility : Icons.visibility_off),
              onPressed: notifier.toggleObscurePassword,
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildStepSix(BuildContext context, BusinessSetupState state) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Text('Review & Launch', style: Theme.of(context).textTheme.titleLarge?.copyWith(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
        const SizedBox(height: 16),
        GlassCard(
          color: Theme.of(context).colorScheme.primary.withValues(alpha: 0.05),
          child: Padding(
            padding: const EdgeInsets.all(16.0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text('Business: ${state.companyName}', style: Theme.of(context).textTheme.bodyLarge),
                Text('Type: ${state.businessType}', style: Theme.of(context).textTheme.bodyMedium),
                Text('Selling: ${state.whatYouSell.join(", ")}', style: Theme.of(context).textTheme.bodyMedium),
                Text('Payments: ${state.payments}', style: Theme.of(context).textTheme.bodyMedium),
                Text('Admin: ${state.adminName} (${state.adminEmail})', style: Theme.of(context).textTheme.bodyMedium),
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
        color: Theme.of(context).colorScheme.surface,
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 600),
          child: GlassCard(
            child: Padding(
              padding: const EdgeInsets.all(24.0),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Text('Business Setup', style: Theme.of(context).textTheme.headlineSmall?.copyWith(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
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
                          case 0: return _buildStepZero(context);
                          case 1: return _buildStepOne(context, state, notifier);
                          case 2: return _buildStepTwo(context, state, notifier);
                          case 3: return _buildStepThree(context, state, notifier);
                          case 4: return _buildStepFour(context, state, notifier);
                          case 5: return _buildStepFive(context, state, notifier);
                          case 6: return _buildStepSix(context, state);
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
