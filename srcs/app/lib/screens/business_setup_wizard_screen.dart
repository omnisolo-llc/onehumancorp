import 'dart:convert';
import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;

import '../services/auth_service.dart';
import '../services/settings_service.dart';
import '../services/api_service.dart';
import '../widgets/glass_card.dart';

class BusinessSetupState {
  final bool obscurePassword;
  final int step;
  final String businessType;
  final String businessName;
  final String businessDescription;
  final List<String> whatYouSell;
  final String paymentPreference;
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
    this.whatYouSell = const [],
    this.paymentPreference = '',
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
    String? businessName,
    String? businessDescription,
    List<String>? whatYouSell,
    String? paymentPreference,
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
      businessName: businessName ?? this.businessName,
      businessDescription: businessDescription ?? this.businessDescription,
      whatYouSell: whatYouSell ?? this.whatYouSell,
      paymentPreference: paymentPreference ?? this.paymentPreference,
      adminName: adminName ?? this.adminName,
      adminEmail: adminEmail ?? this.adminEmail,
      adminPassword: adminPassword ?? this.adminPassword,
      isLoading: isLoading ?? this.isLoading,
      errorMessage: errorMessage,
      obscurePassword: obscurePassword ?? this.obscurePassword,
    );
  }
}

final businessSetupProvider =
    NotifierProvider<BusinessSetupNotifier, BusinessSetupState>(() {
      return BusinessSetupNotifier();
    });

class BusinessSetupNotifier extends Notifier<BusinessSetupState> {
  @override
  BusinessSetupState build() => const BusinessSetupState();

  Future<void> syncState(WidgetRef ref) async {
    try {
      final baseUrl =
          ref.read(clientSettingsProvider).valueOrNull?.backendUrl ??
          'http://localhost';
      final req = http.Request(
        'PATCH',
        Uri.parse('${baseUrl}/api/wizard/state/save'),
      );
      req.headers['Content-Type'] = 'application/json';
      req.body = jsonEncode({
        'step': state.step,
        'businessType': state.businessType,
        'businessName': state.businessName,
        'businessDescription': state.businessDescription,
        'whatYouSell': state.whatYouSell,
        'paymentPreference': state.paymentPreference,
        'adminName': state.adminName,
        'adminEmail': state.adminEmail,
      });

      final authState = ref.read(authStateProvider).valueOrNull;
      if (authState?.token != null) {
        req.headers['Authorization'] = 'Bearer ${authState!.token}';
      }

      final api = ref.read(apiServiceProvider);
    } catch (e) {
      // Best effort sync, ignore failure for now.
    }
  }

  void nextStep(WidgetRef ref) {
    if (state.step < 6) {
      state = state.copyWith(step: state.step + 1, errorMessage: null);
      syncState(ref);
    }
  }

  void prevStep(WidgetRef ref) {
    if (state.step > 0) {
      state = state.copyWith(step: state.step - 1, errorMessage: null);
      syncState(ref);
    }
  }

  void updateBusinessType(String type) {
    state = state.copyWith(businessType: type);
  }

  void updateBusinessName(String name) {
    state = state.copyWith(businessName: name);
    if (name.isNotEmpty && state.businessDescription.isEmpty) {
      state = state.copyWith(businessDescription: 'The best $name in town!');
    }
  }

  void updateBusinessDescription(String description) {
    state = state.copyWith(businessDescription: description);
  }

  void toggleWhatYouSell(String item) {
    final list = List<String>.from(state.whatYouSell);
    if (list.contains(item)) {
      list.remove(item);
    } else {
      list.add(item);
    }
    state = state.copyWith(whatYouSell: list);
  }

  void updatePaymentPreference(String pref) {
    state = state.copyWith(paymentPreference: pref);
  }

  void updateAdminName(String name) {
    state = state.copyWith(adminName: name);
  }

  void updateAdminEmail(String email) {
    state = state.copyWith(adminEmail: email);
  }

  void updateAdminPassword(String pwd) {
    state = state.copyWith(adminPassword: pwd);
  }

  void toggleObscurePassword() {
    state = state.copyWith(obscurePassword: !state.obscurePassword);
  }

  Future<void> launch(BuildContext context, WidgetRef ref) async {
    state = state.copyWith(isLoading: true, errorMessage: null);
    try {
      final baseUrl =
          ref.read(clientSettingsProvider).valueOrNull?.backendUrl ??
          'http://localhost';
      final req = http.Request(
        'POST',
        Uri.parse('${baseUrl}/api/wizard/configure'),
      );
      req.headers['Content-Type'] = 'application/json';
      req.body = jsonEncode({
        'businessType': state.businessType,
        'businessName': state.businessName,
        'businessDescription': state.businessDescription,
        'whatYouSell': state.whatYouSell,
        'paymentPreference': state.paymentPreference,
        'adminName': state.adminName,
        'adminEmail': state.adminEmail,
        'adminPassword': state.adminPassword,
      });

      final authState = ref.read(authStateProvider).valueOrNull;
      if (authState?.token != null) {
        req.headers['Authorization'] = 'Bearer ${authState!.token}';
      }

      final api = ref.read(apiServiceProvider);

      if (api != null) {
        final data = {
          'type': state.businessType,
          'name': state.businessName,
          'description': state.businessDescription,
          'sells_physical': state.whatYouSell.contains('Physical products'),
          'sells_digital': state.whatYouSell.contains('Digital downloads'),
          'sells_services': state.whatYouSell.contains('Services / appointments'),
          'sells_food': state.whatYouSell.contains('Food & beverages'),
          'sells_subscriptions': state.whatYouSell.contains('Subscriptions'),
          'payment_online': (state.paymentPreference == 'Online only' || state.paymentPreference == 'Both'),
          'payment_in_person': (state.paymentPreference == 'In-person (POS)' || state.paymentPreference == 'Both'),
          'admin_name': state.adminName,
          'admin_email': state.adminEmail,
        };
        await api.launchWizard(data);
        if (context.mounted) {
          context.go('/dashboard');
        }
      } else {
        state = state.copyWith(
          isLoading: false,
          errorMessage: 'Setup failed: API client not found',
        );
      }
    } catch (e) {
      state = state.copyWith(
        isLoading: false,
        errorMessage: 'Network error: $e',
      );
    }
  }
}

class BusinessSetupWizardScreen extends ConsumerWidget {
  const BusinessSetupWizardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(businessSetupProvider);
    final notifier = ref.read(businessSetupProvider.notifier);

    return Scaffold(
      body: Stack(
        children: [
          Container(
            decoration: const BoxDecoration(
              gradient: LinearGradient(
                begin: Alignment.topLeft,
                end: Alignment.bottomRight,
                colors: [Color(0xFF0D0D1A), Color(0xFF1A1A33)],
              ),
            ),
            child: Center(
              child: SingleChildScrollView(
                child: ConstrainedBox(
                  constraints: const BoxConstraints(maxWidth: 600),
                  child: GlassCard(
                    child: Padding(
                      padding: const EdgeInsets.all(32.0),
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          if (state.step > 0)
                            Text(
                              'Step ${state.step} of 6',
                              style: const TextStyle(
                                fontFamily: 'Inter',
                                color: Colors.white54,
                                fontSize: 14,
                              ),
                            ),
                          const SizedBox(height: 8),
                          if (state.errorMessage != null) ...[
                            Text(
                              state.errorMessage!,
                              style: const TextStyle(color: Colors.redAccent),
                            ),
                            const SizedBox(height: 16),
                          ],
                          AnimatedSwitcher(
                            duration: const Duration(milliseconds: 300),
                            transitionBuilder:
                                (Widget child, Animation<double> animation) {
                                  return FadeTransition(
                                    opacity: animation,
                                    child: child,
                                  );
                                },
                            child: Container(
                              key: ValueKey<int>(state.step),
                              child: Column(
                                mainAxisSize: MainAxisSize.min,
                                crossAxisAlignment: CrossAxisAlignment.stretch,
                                children: [
                                  if (state.step == 0) ...[
                                    TweenAnimationBuilder<double>(
                                      tween: Tween<double>(
                                        begin: 0.8,
                                        end: 1.0,
                                      ),
                                      duration: const Duration(
                                        milliseconds: 1500,
                                      ),
                                      builder: (context, scale, child) {
                                        return Transform.scale(
                                          scale: scale,
                                          child: const Icon(
                                            Icons.auto_awesome,
                                            size: 80,
                                            color: Colors.blueAccent,
                                          ),
                                        );
                                      },
                                    ),
                                    const SizedBox(height: 24),
                                    const Text(
                                      'Your business, live in minutes.',
                                      textAlign: TextAlign.center,
                                      style: TextStyle(
                                        fontFamily: 'Outfit',
                                        fontSize: 32,
                                        fontWeight: FontWeight.bold,
                                        color: Colors.white,
                                      ),
                                    ),
                                    const SizedBox(height: 16),
                                    const Text(
                                      'Our AI agents will set up your store, schedule, and website automatically. No tech skills needed.',
                                      textAlign: TextAlign.center,
                                      style: TextStyle(
                                        fontFamily: 'Inter',
                                        color: Colors.white70,
                                        fontSize: 16,
                                      ),
                                    ),
                                  ] else if (state.step == 1) ...[
                                    const Text(
                                      'What kind of business are you building?',
                                      style: TextStyle(
                                        fontFamily: 'Outfit',
                                        fontSize: 24,
                                        fontWeight: FontWeight.bold,
                                        color: Colors.white,
                                      ),
                                    ),
                                    const SizedBox(height: 24),
                                    Wrap(
                                      spacing: 16,
                                      runSpacing: 16,
                                      children: [
                                        _buildTypeCard(
                                          context,
                                          'Online Store',
                                          Icons.shopping_cart,
                                          state.businessType,
                                          notifier,
                                        ),
                                        _buildTypeCard(
                                          context,
                                          'Service Business',
                                          Icons.build,
                                          state.businessType,
                                          notifier,
                                        ),
                                        _buildTypeCard(
                                          context,
                                          'Restaurant / Food',
                                          Icons.restaurant,
                                          state.businessType,
                                          notifier,
                                        ),
                                        _buildTypeCard(
                                          context,
                                          'Creative / Portfolio',
                                          Icons.palette,
                                          state.businessType,
                                          notifier,
                                        ),
                                        _buildTypeCard(
                                          context,
                                          'Local Business',
                                          Icons.storefront,
                                          state.businessType,
                                          notifier,
                                        ),
                                        _buildTypeCard(
                                          context,
                                          'Other',
                                          Icons.more_horiz,
                                          state.businessType,
                                          notifier,
                                        ),
                                      ],
                                    ),
                                  ] else if (state.step == 2) ...[
                                    const Text(
                                      'What is your business called?',
                                      style: TextStyle(
                                        fontFamily: 'Outfit',
                                        fontSize: 24,
                                        fontWeight: FontWeight.bold,
                                        color: Colors.white,
                                      ),
                                    ),
                                    const SizedBox(height: 24),
                                    TextField(
                                      decoration: const InputDecoration(
                                        labelText: 'Business Name',
                                        labelStyle: TextStyle(
                                          color: Colors.white70,
                                        ),
                                        border: OutlineInputBorder(),
                                      ),
                                      onChanged: notifier.updateBusinessName,
                                      style: const TextStyle(
                                        fontFamily: 'Inter',
                                        color: Colors.white,
                                        fontSize: 20,
                                      ),
                                    ),
                                    const SizedBox(height: 24),
                                    TextField(
                                      controller:
                                          TextEditingController(
                                              text: state.businessDescription,
                                            )
                                            ..selection =
                                                TextSelection.collapsed(
                                                  offset: state
                                                      .businessDescription
                                                      .length,
                                                ),
                                      decoration: const InputDecoration(
                                        labelText: 'Short Description',
                                        labelStyle: TextStyle(
                                          color: Colors.white70,
                                        ),
                                        helperText:
                                            'AI suggested! Feel free to edit.',
                                        helperStyle: TextStyle(
                                          color: Colors.white54,
                                        ),
                                        border: OutlineInputBorder(),
                                      ),
                                      maxLines: 3,
                                      onChanged:
                                          notifier.updateBusinessDescription,
                                      style: const TextStyle(
                                        fontFamily: 'Inter',
                                        color: Colors.white,
                                      ),
                                    ),
                                  ] else if (state.step == 3) ...[
                                    const Text(
                                      'What do you sell?',
                                      style: TextStyle(
                                        fontFamily: 'Outfit',
                                        fontSize: 24,
                                        fontWeight: FontWeight.bold,
                                        color: Colors.white,
                                      ),
                                    ),
                                    const SizedBox(height: 24),
                                    _buildSellCheckbox(
                                      'Physical products',
                                      state.whatYouSell,
                                      notifier,
                                    ),
                                    _buildSellCheckbox(
                                      'Digital downloads',
                                      state.whatYouSell,
                                      notifier,
                                    ),
                                    _buildSellCheckbox(
                                      'Services / appointments',
                                      state.whatYouSell,
                                      notifier,
                                    ),
                                    _buildSellCheckbox(
                                      'Food & beverages',
                                      state.whatYouSell,
                                      notifier,
                                    ),
                                    _buildSellCheckbox(
                                      'Subscriptions',
                                      state.whatYouSell,
                                      notifier,
                                    ),
                                  ] else if (state.step == 4) ...[
                                    const Text(
                                      'How do you want to receive payments?',
                                      style: TextStyle(
                                        fontFamily: 'Outfit',
                                        fontSize: 24,
                                        fontWeight: FontWeight.bold,
                                        color: Colors.white,
                                      ),
                                    ),
                                    const SizedBox(height: 24),
                                    _buildPaymentCard(
                                      context,
                                      'Online only',
                                      'Accept cards, Apple Pay, Google Pay. ~5 mins to first payment.',
                                      state.paymentPreference,
                                      notifier,
                                    ),
                                    const SizedBox(height: 12),
                                    _buildPaymentCard(
                                      context,
                                      'In-person (POS)',
                                      'Use your phone to tap-to-pay. ~10 mins to first payment.',
                                      state.paymentPreference,
                                      notifier,
                                    ),
                                    const SizedBox(height: 12),
                                    _buildPaymentCard(
                                      context,
                                      'Both',
                                      'Online and in-person. ~10 mins to first payment.',
                                      state.paymentPreference,
                                      notifier,
                                    ),
                                    const SizedBox(height: 12),
                                    _buildPaymentCard(
                                      context,
                                      'Skip for now',
                                      'Set up payments later',
                                      state.paymentPreference,
                                      notifier,
                                    ),
                                  ] else if (state.step == 5) ...[
                                    const Text(
                                      'Create your admin account',
                                      style: TextStyle(
                                        fontFamily: 'Outfit',
                                        fontSize: 24,
                                        fontWeight: FontWeight.bold,
                                        color: Colors.white,
                                      ),
                                    ),
                                    const SizedBox(height: 24),
                                    TextField(
                                      decoration: const InputDecoration(
                                        labelText: 'Full Name',
                                        labelStyle: TextStyle(
                                          color: Colors.white70,
                                        ),
                                        border: OutlineInputBorder(),
                                      ),
                                      onChanged: notifier.updateAdminName,
                                      style: const TextStyle(
                                        fontFamily: 'Inter',
                                        color: Colors.white,
                                      ),
                                    ),
                                    const SizedBox(height: 16),
                                    TextField(
                                      decoration: const InputDecoration(
                                        labelText: 'Email Address',
                                        labelStyle: TextStyle(
                                          color: Colors.white70,
                                        ),
                                        border: OutlineInputBorder(),
                                      ),
                                      keyboardType: TextInputType.emailAddress,
                                      onChanged: notifier.updateAdminEmail,
                                      style: const TextStyle(
                                        fontFamily: 'Inter',
                                        color: Colors.white,
                                      ),
                                    ),
                                    const SizedBox(height: 16),
                                    TextField(
                                      obscureText: state.obscurePassword,
                                      onChanged: notifier.updateAdminPassword,
                                      style: const TextStyle(
                                        fontFamily: 'Inter',
                                        color: Colors.white,
                                      ),
                                      decoration: InputDecoration(
                                        labelText: 'Password',
                                        labelStyle: const TextStyle(
                                          color: Colors.white70,
                                        ),
                                        border: const OutlineInputBorder(),
                                        suffixIcon: IconButton(
                                          icon: Icon(
                                            state.obscurePassword
                                                ? Icons.visibility
                                                : Icons.visibility_off,
                                            color: Colors.white70,
                                          ),
                                          onPressed:
                                              notifier.toggleObscurePassword,
                                        ),
                                      ),
                                    ),
                                    const SizedBox(height: 8),
                                    _buildPasswordStrengthMeter(
                                      state.adminPassword,
                                    ),
                                    const SizedBox(height: 16),
                                    Row(
                                      mainAxisAlignment:
                                          MainAxisAlignment.center,
                                      children: [
                                        OutlinedButton.icon(
                                          onPressed: () {},
                                          icon: const Icon(Icons.g_mobiledata),
                                          label: const Text('Google SSO'),
                                        ),
                                        const SizedBox(width: 16),
                                        OutlinedButton.icon(
                                          onPressed: () {},
                                          icon: const Icon(Icons.apple),
                                          label: const Text('Apple SSO'),
                                        ),
                                      ],
                                    ),
                                  ] else if (state.step == 6) ...[
                                    const Text(
                                      'Ready to launch',
                                      style: TextStyle(
                                        fontFamily: 'Outfit',
                                        fontSize: 24,
                                        fontWeight: FontWeight.bold,
                                        color: Colors.white,
                                      ),
                                    ),
                                    const SizedBox(height: 24),
                                    Container(
                                      padding: const EdgeInsets.all(20),
                                      decoration: BoxDecoration(
                                        color: Colors.white.withOpacity(0.05),
                                        borderRadius: BorderRadius.circular(12),
                                        border: Border.all(
                                          color: Colors.white.withOpacity(0.1),
                                        ),
                                      ),
                                      child: Column(
                                        crossAxisAlignment:
                                            CrossAxisAlignment.start,
                                        children: [
                                          Text(
                                            'Business: ${state.businessName}',
                                            style: const TextStyle(
                                              fontFamily: 'Inter',
                                              color: Colors.white,
                                              fontSize: 18,
                                              fontWeight: FontWeight.bold,
                                            ),
                                          ),
                                          const SizedBox(height: 8),
                                          Text(
                                            'Type: ${state.businessType}',
                                            style: const TextStyle(
                                              fontFamily: 'Inter',
                                              color: Colors.white70,
                                            ),
                                          ),
                                          Text(
                                            'Selling: ${state.whatYouSell.join(', ')}',
                                            style: const TextStyle(
                                              fontFamily: 'Inter',
                                              color: Colors.white70,
                                            ),
                                          ),
                                          Text(
                                            'Payments: ${state.paymentPreference}',
                                            style: const TextStyle(
                                              fontFamily: 'Inter',
                                              color: Colors.white70,
                                            ),
                                          ),
                                        ],
                                      ),
                                    ),
                                  ],
                                ],
                              ),
                            ),
                          ),
                          const SizedBox(height: 32),
                          Wrap(
                            alignment: WrapAlignment.spaceBetween,
                            crossAxisAlignment: WrapCrossAlignment.center,
                            children: [
                              if (state.step > 0)
                                TextButton(
                                  onPressed: state.isLoading
                                      ? null
                                      : () => notifier.prevStep(ref),
                                  child: const Text(
                                    'Back',
                                    style: TextStyle(
                                      fontFamily: 'Inter',
                                      fontSize: 16,
                                    ),
                                  ),
                                )
                              else
                                const SizedBox(),
                              if (state.step == 0)
                                ElevatedButton(
                                  onPressed: () => notifier.nextStep(ref),
                                  style: ElevatedButton.styleFrom(
                                    padding: const EdgeInsets.symmetric(
                                      horizontal: 32,
                                      vertical: 16,
                                    ),
                                  ),
                                  child: const Text(
                                    'Get Started',
                                    style: TextStyle(
                                      fontFamily: 'Inter',
                                      fontSize: 16,
                                    ),
                                  ),
                                )
                              else if (state.step < 6)
                                ElevatedButton(
                                  onPressed: state.isLoading
                                      ? null
                                      : () => notifier.nextStep(ref),
                                  style: ElevatedButton.styleFrom(
                                    padding: const EdgeInsets.symmetric(
                                      horizontal: 32,
                                      vertical: 16,
                                    ),
                                  ),
                                  child: const Text(
                                    'Next',
                                    style: TextStyle(
                                      fontFamily: 'Inter',
                                      fontSize: 16,
                                    ),
                                  ),
                                )
                              else
                                TweenAnimationBuilder<double>(
                                  tween: Tween<double>(begin: 1.0, end: 1.05),
                                  duration: const Duration(seconds: 1),
                                  builder: (context, scale, child) {
                                    return Transform.scale(
                                      scale: scale,
                                      child: ElevatedButton(
                                        onPressed: state.isLoading
                                            ? null
                                            : () =>
                                                  notifier.launch(context, ref),
                                        style: ElevatedButton.styleFrom(
                                          backgroundColor: Colors.blueAccent,
                                          foregroundColor: Colors.white,
                                          padding: const EdgeInsets.symmetric(
                                            horizontal: 32,
                                            vertical: 16,
                                          ),
                                        ),
                                        child: state.isLoading
                                            ? const SizedBox(
                                                width: 24,
                                                height: 24,
                                                child:
                                                    CircularProgressIndicator(
                                                      strokeWidth: 2,
                                                      color: Colors.white,
                                                    ),
                                              )
                                            : const Text(
                                                'Launch My Business →',
                                                style: TextStyle(
                                                  fontFamily: 'Outfit',
                                                  fontSize: 18,
                                                  fontWeight: FontWeight.bold,
                                                ),
                                              ),
                                      ),
                                    );
                                  },
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
          ),
          if (state.isLoading)
            Container(
              color: Colors.black.withOpacity(0.8),
              child: const Center(
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    CircularProgressIndicator(),
                    SizedBox(height: 24),
                    Text(
                      'Your business is setting up...',
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 24,
                        color: Colors.white,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ],
                ),
              ),
            ),
        ],
      ),
    );
  }

  Widget _buildPasswordStrengthMeter(String password) {
    int strength = 0;
    if (password.length > 5) strength++;
    if (password.length > 8) strength++;
    if (password.contains(RegExp(r'[A-Z]'))) strength++;
    if (password.contains(RegExp(r'[0-9]'))) strength++;

    Color color = Colors.red;
    if (strength == 2) color = Colors.orange;
    if (strength > 2) color = Colors.green;

    return Row(
      children: [
        Expanded(
          child: LinearProgressIndicator(
            value: strength / 4,
            backgroundColor: Colors.white10,
            color: color,
          ),
        ),
        const SizedBox(width: 8),
        Text(
          strength == 0
              ? ''
              : strength < 3
              ? 'Weak'
              : 'Strong',
          style: TextStyle(color: color, fontSize: 12),
        ),
      ],
    );
  }

  Widget _buildTypeCard(
    BuildContext context,
    String label,
    IconData icon,
    String selected,
    BusinessSetupNotifier notifier,
  ) {
    final isSelected = label == selected;
    return GestureDetector(
      onTap: () => notifier.updateBusinessType(label),
      child: Container(
        width: 140,
        height: 120,
        decoration: BoxDecoration(
          color: isSelected
              ? Colors.blueAccent.withOpacity(0.2)
              : Colors.white.withOpacity(0.05),
          border: Border.all(
            color: isSelected
                ? Colors.blueAccent
                : Colors.white.withOpacity(0.1),
          ),
          borderRadius: BorderRadius.circular(16),
        ),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              icon,
              size: 40,
              color: isSelected ? Colors.blueAccent : Colors.white70,
            ),
            const SizedBox(height: 12),
            Text(
              label,
              textAlign: TextAlign.center,
              style: TextStyle(
                fontFamily: 'Inter',
                color: isSelected ? Colors.white : Colors.white70,
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildSellCheckbox(
    String label,
    List<String> selected,
    BusinessSetupNotifier notifier,
  ) {
    final isSelected = selected.contains(label);
    return CheckboxListTile(
      title: Text(
        label,
        style: const TextStyle(
          fontFamily: 'Inter',
          color: Colors.white,
          fontSize: 18,
        ),
      ),
      value: isSelected,
      checkColor: Colors.black,
      activeColor: Colors.blueAccent,
      onChanged: (bool? value) => notifier.toggleWhatYouSell(label),
      controlAffinity: ListTileControlAffinity.leading,
    );
  }

  Widget _buildPaymentCard(
    BuildContext context,
    String title,
    String subtitle,
    String selected,
    BusinessSetupNotifier notifier,
  ) {
    final isSelected = title == selected;
    return GestureDetector(
      onTap: () => notifier.updatePaymentPreference(title),
      child: Container(
        padding: const EdgeInsets.all(16),
        decoration: BoxDecoration(
          color: isSelected
              ? Colors.blueAccent.withOpacity(0.2)
              : Colors.white.withOpacity(0.05),
          border: Border.all(
            color: isSelected
                ? Colors.blueAccent
                : Colors.white.withOpacity(0.1),
          ),
          borderRadius: BorderRadius.circular(12),
        ),
        child: Row(
          children: [
            Icon(
              isSelected
                  ? Icons.radio_button_checked
                  : Icons.radio_button_unchecked,
              color: isSelected ? Colors.blueAccent : Colors.white54,
            ),
            const SizedBox(width: 16),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    title,
                    style: const TextStyle(
                      fontFamily: 'Inter',
                      color: Colors.white,
                      fontSize: 18,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  const SizedBox(height: 4),
                  Text(
                    subtitle,
                    style: const TextStyle(
                      fontFamily: 'Inter',
                      color: Colors.white70,
                      fontSize: 14,
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
