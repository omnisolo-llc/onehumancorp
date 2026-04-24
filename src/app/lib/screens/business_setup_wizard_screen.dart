import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
import 'dart:ui';
import '../services/auth_service.dart';
import '../services/settings_service.dart';
import '../services/api_service.dart';
import '../widgets/glass_card.dart';
import '../widgets/pulse_animation.dart';

class BusinessSetupState {
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
      errorMessage: errorMessage ?? this.errorMessage,
    );
  }
}

class BusinessSetupNotifier extends Notifier<BusinessSetupState> {
  @override
  BusinessSetupState build() => const BusinessSetupState();

  void nextStep(WidgetRef ref) {
    if (state.step < 6) {
      state = state.copyWith(step: state.step + 1);
      _autoSave(ref);
    }
  }

  void prevStep(WidgetRef ref) {
    if (state.step > 0) {
      state = state.copyWith(step: state.step - 1);
      _autoSave(ref);
    }
  }

  Future<void> _autoSave(WidgetRef ref) async {
    final user = ref.read(authStateProvider).valueOrNull;
    final baseUrl = ref.read(backendUrlProvider);
    if (user != null && baseUrl.isNotEmpty) {
      final body = {
        'extras': {
          'company_name': state.businessName,
          'business_type': state.businessType,
          'business_description': state.businessDescription,
          'what_you_sell': state.whatYouSell.join(','),
          'payment_preference': state.paymentPreference,
          'admin_name': state.adminName,
          'admin_email': state.adminEmail,
          'wizard_step': state.step.toString(),
        },
      };
      try {
        await http.post(
          Uri.parse('$baseUrl/api/wizard/configure'),
          headers: {
            'Authorization': 'Bearer ${user.token}',
            'Content-Type': 'application/json',
          },
          body: jsonEncode(body),
        );
      } catch (_) {}
    }
  }

  void updateBusinessType(String val) =>
      state = state.copyWith(businessType: val);
  void updateBusinessName(String name) =>
      state = state.copyWith(businessName: name);
  void updateBusinessDescription(String desc) =>
      state = state.copyWith(businessDescription: desc);

  void toggleWhatYouSell(String item) {
    final list = List<String>.from(state.whatYouSell);
    if (list.contains(item)) {
      list.remove(item);
    } else {
      list.add(item);
    }
    state = state.copyWith(whatYouSell: list);
  }

  void updatePaymentPreference(String val) =>
      state = state.copyWith(paymentPreference: val);
  void updateAdminName(String name) => state = state.copyWith(adminName: name);
  void updateAdminEmail(String val) => state = state.copyWith(adminEmail: val);
  void updateAdminPassword(String val) =>
      state = state.copyWith(adminPassword: val);

  void loadFromDraft(Map<String, dynamic> extras) {
    state = state.copyWith(
      businessName: extras['company_name'] as String? ?? '',
      businessType: extras['business_type'] as String? ?? '',
      businessDescription: extras['business_description'] as String? ?? '',
      whatYouSell: (extras['what_you_sell'] as String? ?? '')
          .split(',')
          .where((e) => e.isNotEmpty)
          .toList(),
      paymentPreference: extras['payment_preference'] as String? ?? '',
      adminName: extras['admin_name'] as String? ?? '',
      adminEmail: extras['admin_email'] as String? ?? '',
      step: int.tryParse(extras['wizard_step'] as String? ?? '0') ?? 0,
    );
  }

  Future<void> launch(BuildContext context, WidgetRef ref) async {
    final user = ref.read(authStateProvider).valueOrNull;
    final baseUrl = ref.read(backendUrlProvider);

    state = state.copyWith(isLoading: true, errorMessage: null);

    if (user != null && baseUrl.isNotEmpty) {
      final body = {
        'extras': {
          'company_name': state.businessName,
          'business_type': state.businessType,
          'business_description': state.businessDescription,
          'what_you_sell': state.whatYouSell.join(','),
          'payment_preference': state.paymentPreference,
          'admin_name': state.adminName,
          'admin_email': state.adminEmail,
        },
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
          state = state.copyWith(
            isLoading: false,
            errorMessage: 'Configuration failed: ${res.statusCode}',
          );
          return;
        }
      } catch (e) {
        state = state.copyWith(
          isLoading: false,
          errorMessage: 'Network error: $e',
        );
        return;
      }
    }

    state = state.copyWith(isLoading: false);

    if (context.mounted) {
      GoRouter.of(context).go('/dashboard');
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
  bool _obscurePassword = true;
  final _nameCtrl = TextEditingController();
  final _descCtrl = TextEditingController();

  @override
  void initState() {
    super.initState();
    _loadDraft();
  }

  @override
  void dispose() {
    _nameCtrl.dispose();
    _descCtrl.dispose();
    super.dispose();
  }

  Future<void> _loadDraft() async {
    final user = ref.read(authStateProvider).valueOrNull;
    final baseUrl = ref.read(backendUrlProvider);
    if (user != null && baseUrl.isNotEmpty) {
      try {
        final resp = await http.get(
          Uri.parse('$baseUrl/api/wizard/status'),
          headers: {'Authorization': 'Bearer ${user.token}'},
        );
        if (resp.statusCode == 200) {
          final data = jsonDecode(resp.body);
          if (data['extras'] != null) {
            ref
                .read(businessSetupProvider.notifier)
                .loadFromDraft(data['extras']);
            final state = ref.read(businessSetupProvider);
            _nameCtrl.text = state.businessName;
            _descCtrl.text = state.businessDescription;
          }
        }
      } catch (_) {}
    }
  }

  void _generateSuggestions(BusinessSetupNotifier notifier, String type) async {
    final api = ref.read(apiServiceProvider);
    if (api == null || _nameCtrl.text.isEmpty) return;

    final result = await api.generateBusinessSuggestions(_nameCtrl.text, type);
    _descCtrl.text = result['description'] ?? '';
    notifier.updateBusinessDescription(_descCtrl.text);
  }

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
                          children: [
                            if (state.step == 0) ...[
                              const Icon(
                                Icons.rocket_launch,
                                size: 64,
                                color: Colors.blueAccent,
                              ),
                              const SizedBox(height: 16),
                              const Text(
                                'Your business, live in minutes.',
                                style: TextStyle(
                                  fontFamily: 'Outfit',
                                  fontSize: 28,
                                  fontWeight: FontWeight.bold,
                                  color: Colors.white,
                                ),
                                textAlign: TextAlign.center,
                              ),
                              const SizedBox(height: 16),
                              const Text(
                                'Let AI do the heavy lifting while you focus on your vision.',
                                style: TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white70,
                                  fontSize: 16,
                                ),
                                textAlign: TextAlign.center,
                              ),
                            ] else if (state.step == 1) ...[
                              const Text(
                                'What type of business are you starting?',
                                style: TextStyle(
                                  fontFamily: 'Inter',
                                  fontWeight: FontWeight.bold,
                                  color: Colors.white,
                                  fontSize: 18,
                                ),
                              ),
                              const SizedBox(height: 16),
                              Wrap(
                                spacing: 16,
                                runSpacing: 16,
                                alignment: WrapAlignment.center,
                                children:
                                    [
                                      {
                                        'label': 'Online Store',
                                        'icon': Icons.shopping_cart,
                                      },
                                      {
                                        'label': 'Service Business',
                                        'icon': Icons.build,
                                      },
                                      {
                                        'label': 'Restaurant / Food',
                                        'icon': Icons.restaurant,
                                      },
                                      {
                                        'label': 'Creative / Portfolio',
                                        'icon': Icons.palette,
                                      },
                                      {
                                        'label': 'Local Business',
                                        'icon': Icons.storefront,
                                      },
                                      {
                                        'label': 'Other',
                                        'icon': Icons.category,
                                      },
                                    ].map((item) {
                                      final isSelected =
                                          state.businessType == item['label'];
                                      return GestureDetector(
                                        onTap: () {
                                          notifier.updateBusinessType(
                                            item['label'] as String,
                                          );
                                        },
                                        child: Container(
                                          width: 140,
                                          padding: const EdgeInsets.all(16),
                                          decoration: BoxDecoration(
                                            color: isSelected
                                                ? Colors.blueAccent.withOpacity(
                                                    0.2,
                                                  )
                                                : Colors.white.withOpacity(
                                                    0.05,
                                                  ),
                                            border: Border.all(
                                              color: isSelected
                                                  ? Colors.blueAccent
                                                  : Colors.white.withOpacity(
                                                      0.1,
                                                    ),
                                              width: 2,
                                            ),
                                            borderRadius: BorderRadius.circular(
                                              16,
                                            ),
                                          ),
                                          child: Column(
                                            children: [
                                              Icon(
                                                item['icon'] as IconData,
                                                size: 48,
                                                color: isSelected
                                                    ? Colors.blueAccent
                                                    : Colors.white70,
                                              ),
                                              const SizedBox(height: 12),
                                              Text(
                                                item['label'] as String,
                                                style: TextStyle(
                                                  fontFamily: 'Inter',
                                                  color: isSelected
                                                      ? Colors.white
                                                      : Colors.white70,
                                                ),
                                                textAlign: TextAlign.center,
                                              ),
                                            ],
                                          ),
                                        ),
                                      );
                                    }).toList(),
                              ),
                            ] else if (state.step == 2) ...[
                              const Text(
                                'Tell us about your business',
                                style: TextStyle(
                                  fontFamily: 'Inter',
                                  fontWeight: FontWeight.bold,
                                  color: Colors.white,
                                  fontSize: 18,
                                ),
                              ),
                              const SizedBox(height: 16),
                              TextField(
                                controller: _nameCtrl,
                                decoration: const InputDecoration(
                                  labelText: 'Business Name',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updateBusinessName,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                              ),
                              const SizedBox(height: 16),
                              TextField(
                                controller: _descCtrl,
                                decoration: const InputDecoration(
                                  labelText: 'Short Description',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updateBusinessDescription,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                                maxLines: 3,
                              ),
                              const SizedBox(height: 16),
                              ElevatedButton.icon(
                                onPressed: () => _generateSuggestions(
                                  notifier,
                                  state.businessType,
                                ),
                                icon: const Icon(Icons.auto_awesome),
                                label: const Text(
                                  'AI Auto-suggest Description',
                                ),
                                style: ElevatedButton.styleFrom(
                                  backgroundColor: Colors.blueAccent
                                      .withOpacity(0.2),
                                  foregroundColor: Colors.blueAccent,
                                ),
                              ),
                            ] else if (state.step == 3) ...[
                              const Text(
                                'What do you sell?',
                                style: TextStyle(
                                  fontFamily: 'Inter',
                                  fontWeight: FontWeight.bold,
                                  color: Colors.white,
                                  fontSize: 18,
                                ),
                              ),
                              const SizedBox(height: 16),
                              ...[
                                'Physical products',
                                'Digital downloads',
                                'Services / appointments',
                                'Food & beverages',
                                'Subscriptions',
                              ].map(
                                (item) => CheckboxListTile(
                                  title: Text(
                                    item,
                                    style: const TextStyle(
                                      fontFamily: 'Inter',
                                      color: Colors.white,
                                    ),
                                  ),
                                  value: state.whatYouSell.contains(item),
                                  checkColor: Colors.black,
                                  activeColor: Colors.white,
                                  onChanged: (bool? value) {
                                    notifier.toggleWhatYouSell(item);
                                  },
                                ),
                              ),
                            ] else if (state.step == 4) ...[
                              const Text(
                                'How do you want to receive payments?',
                                style: TextStyle(
                                  fontFamily: 'Inter',
                                  fontWeight: FontWeight.bold,
                                  color: Colors.white,
                                  fontSize: 18,
                                ),
                              ),
                              const SizedBox(height: 16),
                              ...[
                                'Online only',
                                'In-person (POS)',
                                'Both',
                                'Skip for now',
                              ].map(
                                (dep) => RadioListTile<String>(
                                  title: Text(
                                    dep,
                                    style: const TextStyle(
                                      fontFamily: 'Inter',
                                      color: Colors.white,
                                    ),
                                  ),
                                  value: dep,
                                  groupValue: state.paymentPreference,
                                  activeColor: Colors.blueAccent,
                                  onChanged: (String? value) {
                                    if (value != null)
                                      notifier.updatePaymentPreference(value);
                                  },
                                ),
                              ),
                            ] else if (state.step == 5) ...[
                              const Text(
                                'Create Administrator Account',
                                style: TextStyle(
                                  fontFamily: 'Inter',
                                  fontWeight: FontWeight.bold,
                                  color: Colors.white,
                                  fontSize: 18,
                                ),
                              ),
                              const SizedBox(height: 16),
                              Row(
                                children: [
                                  Expanded(
                                    child: OutlinedButton.icon(
                                      onPressed: () {},
                                      icon: const Icon(Icons.g_mobiledata),
                                      label: const Text('Google SSO'),
                                    ),
                                  ),
                                  const SizedBox(width: 8),
                                  Expanded(
                                    child: OutlinedButton.icon(
                                      onPressed: () {},
                                      icon: const Icon(Icons.apple),
                                      label: const Text('Apple SSO'),
                                    ),
                                  ),
                                ],
                              ),
                              const SizedBox(height: 16),
                              const Row(
                                children: [
                                  Expanded(
                                    child: Divider(color: Colors.white24),
                                  ),
                                  Padding(
                                    padding: EdgeInsets.symmetric(
                                      horizontal: 8.0,
                                    ),
                                    child: Text(
                                      'OR',
                                      style: TextStyle(color: Colors.white54),
                                    ),
                                  ),
                                  Expanded(
                                    child: Divider(color: Colors.white24),
                                  ),
                                ],
                              ),
                              const SizedBox(height: 16),
                              TextField(
                                decoration: const InputDecoration(
                                  labelText: 'Full Name',
                                  labelStyle: TextStyle(color: Colors.white70),
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
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updateAdminEmail,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                              ),
                              const SizedBox(height: 16),
                              TextField(
                                obscureText: _obscurePassword,
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
                                  suffixIcon: IconButton(
                                    icon: Icon(
                                      _obscurePassword
                                          ? Icons.visibility
                                          : Icons.visibility_off,
                                      color: Colors.white70,
                                    ),
                                    onPressed: () {
                                      setState(() {
                                        _obscurePassword = !_obscurePassword;
                                      });
                                    },
                                  ),
                                ),
                              ),
                              const SizedBox(height: 8),
                              LinearProgressIndicator(
                                value: state.adminPassword.length / 12,
                                color: state.adminPassword.length > 8
                                    ? Colors.green
                                    : Colors.orange,
                                backgroundColor: Colors.white10,
                              ),
                              const SizedBox(height: 4),
                              const Text(
                                'Password strength',
                                style: TextStyle(
                                  fontSize: 10,
                                  color: Colors.white54,
                                ),
                              ),
                            ] else if (state.step == 6) ...[
                              const Text(
                                'Review & Launch',
                                style: TextStyle(
                                  fontFamily: 'Inter',
                                  fontWeight: FontWeight.bold,
                                  color: Colors.white,
                                  fontSize: 18,
                                ),
                              ),
                              const SizedBox(height: 16),
                              Container(
                                padding: const EdgeInsets.all(16),
                                decoration: BoxDecoration(
                                  color: Colors.white.withOpacity(0.05),
                                  borderRadius: BorderRadius.circular(12),
                                  border: Border.all(
                                    color: Colors.white.withOpacity(0.1),
                                  ),
                                ),
                                child: Column(
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  children: [
                                    Text(
                                      'Business: ${state.businessName}',
                                      style: const TextStyle(
                                        color: Colors.white,
                                        fontFamily: 'Inter',
                                      ),
                                    ),
                                    const SizedBox(height: 8),
                                    Text(
                                      'Type: ${state.businessType}',
                                      style: const TextStyle(
                                        color: Colors.white,
                                        fontFamily: 'Inter',
                                      ),
                                    ),
                                    const SizedBox(height: 8),
                                    Text(
                                      'Admin: ${state.adminEmail}',
                                      style: const TextStyle(
                                        color: Colors.white,
                                        fontFamily: 'Inter',
                                      ),
                                    ),
                                  ],
                                ),
                              ),
                              const SizedBox(height: 24),
                              if (state.isLoading)
                                const Column(
                                  children: [
                                    CircularProgressIndicator(),
                                    SizedBox(height: 16),
                                    Text(
                                      'Your business is setting up...',
                                      style: TextStyle(
                                        color: Colors.white,
                                        fontFamily: 'Inter',
                                      ),
                                    ),
                                  ],
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
                            onPressed: state.isLoading
                                ? null
                                : () => notifier.prevStep(ref),
                            child: const Text(
                              'Back',
                              style: TextStyle(fontFamily: 'Inter'),
                            ),
                          )
                        else
                          const SizedBox(),
                        ElevatedButton(
                          onPressed: state.isLoading
                              ? null
                              : () {
                                  if (state.step < 4) {
                                    notifier.nextStep(ref);
                                  } else {
                                    notifier.launch(context, ref);
                                  }
                                },
                          child: state.isLoading
                              ? const SizedBox(
                                  width: 20,
                                  height: 20,
                                  child: CircularProgressIndicator(
                                    strokeWidth: 2,
                                  ),
                                )
                              : Text(
                                  state.step == 4
                                      ? 'Launch My AI Team →'
                                      : 'Next',
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
