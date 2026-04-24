import 'package:shared_preferences/shared_preferences.dart';
import 'package:ohc_app/router.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
import '../services/auth_service.dart';
import '../services/settings_service.dart';
import '../widgets/glass_card.dart';

class BusinessSetupState {
  final bool obscurePassword;
  final int step;
  final String businessType;
  final String companyName;
  final String description;
  final List<String> whatYouSell;
  final String paymentMethod;
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
    this.whatYouSell = const [],
    this.paymentMethod = '',
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
    String? paymentMethod,
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
      paymentMethod: paymentMethod ?? this.paymentMethod,
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

  Future<void> _saveDraftState(WidgetRef ref) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setInt('wizard_step', state.step);
    await prefs.setString('wizard_company', state.companyName);
    await prefs.setString('wizard_desc', state.description);
    await prefs.setString('wizard_type', state.businessType);
    await prefs.setString('wizard_payment', state.paymentMethod);
    await prefs.setStringList('wizard_sell', state.whatYouSell);

    final settings = ref.read(clientSettingsProvider).value;
    if (settings != null) {
      final httpClient = http.Client();
      try {
        final authState = ref.read(authStateProvider);
        final token = authState.valueOrNull?.token;
        await httpClient.post(
          Uri.parse('${settings.backendUrl}/api/v1/wizard/setup'),
          headers: {
            'Content-Type': 'application/json',
            'Authorization': 'Bearer $token',
          },
          body: jsonEncode({
            'step': state.step,
            'businessType': state.businessType,
            'companyName': state.companyName,
            'description': state.description,
            'whatYouSell': state.whatYouSell,
            'paymentMethod': state.paymentMethod,
          }),
        );
      } catch (_) {}
    }
  }

  Future<void> loadDraft(WidgetRef ref) async {
    final settings = ref.read(clientSettingsProvider).value;
    bool loadedFromBackend = false;

    if (settings != null) {
      final httpClient = http.Client();
      try {
        final authState = ref.read(authStateProvider);
        final token = authState.valueOrNull?.token;
        final response = await httpClient.get(
          Uri.parse('${settings.backendUrl}/api/v1/wizard/draft'),
          headers: {'Authorization': 'Bearer $token'},
        );
        if (response.statusCode == 200) {
          final data = jsonDecode(response.body);
          state = state.copyWith(
            step: data['step'] ?? 0,
            businessType: data['businessType'] ?? '',
            companyName: data['companyName'] ?? '',
            description: data['description'] ?? '',
            whatYouSell: List<String>.from(data['whatYouSell'] ?? []),
            paymentMethod: data['paymentMethod'] ?? '',
          );
          loadedFromBackend = true;
        }
      } catch (_) {}
    }

    if (!loadedFromBackend) {
      final prefs = await SharedPreferences.getInstance();
      if (prefs.containsKey('wizard_step')) {
        state = state.copyWith(
          step: prefs.getInt('wizard_step') ?? 0,
          companyName: prefs.getString('wizard_company') ?? '',
          description: prefs.getString('wizard_desc') ?? '',
          businessType: prefs.getString('wizard_type') ?? '',
          paymentMethod: prefs.getString('wizard_payment') ?? '',
          whatYouSell: prefs.getStringList('wizard_sell') ?? const [],
        );
      }
    }
  }

  void nextStep([WidgetRef? ref]) {
    if (state.step < 6) {
      state = state.copyWith(step: state.step + 1);
      if (ref != null) _saveDraftState(ref);
    }
  }

  void prevStep() {
    if (state.step > 0) {
      state = state.copyWith(step: state.step - 1);
    }
  }

  void updateBusinessType(String type) =>
      state = state.copyWith(businessType: type);
  void updateCompany(String name) {
    String desc = state.description;
    if (desc.isEmpty || desc.endsWith('services to the community.')) {
      if (name.isNotEmpty && state.businessType.isNotEmpty) {
        desc =
            '$name provides amazing ${state.businessType.toLowerCase()} services to the community.';
      } else if (name.isEmpty) {
        desc = '';
      }
    }
    state = state.copyWith(companyName: name, description: desc);
  }

  void updateDescription(String desc) =>
      state = state.copyWith(description: desc);

  void toggleWhatYouSell(String item) {
    final list = List<String>.from(state.whatYouSell);
    if (list.contains(item)) {
      list.remove(item);
    } else {
      list.add(item);
    }
    state = state.copyWith(whatYouSell: list);
  }

  void updatePaymentMethod(String method) =>
      state = state.copyWith(paymentMethod: method);

  void updateAdminName(String name) => state = state.copyWith(adminName: name);
  void updateAdminEmail(String val) => state = state.copyWith(adminEmail: val);
  void updateAdminPassword(String val) =>
      state = state.copyWith(adminPassword: val);
  void toggleObscurePassword() =>
      state = state.copyWith(obscurePassword: !state.obscurePassword);

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
          'payment_method': state.paymentMethod,
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
      context.go('/dashboard');
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
    extends ConsumerState<BusinessSetupWizardScreen>
    with SingleTickerProviderStateMixin {
  late final TextEditingController _companyNameController;
  late final TextEditingController _descriptionController;
  late final AnimationController _pulseController;

  @override
  void initState() {
    super.initState();
    _companyNameController = TextEditingController(
      text: ref.read(businessSetupProvider).companyName,
    );
    _descriptionController = TextEditingController(
      text: ref.read(businessSetupProvider).description,
    );

    _pulseController = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 1000),
    )..repeat(reverse: true);

    WidgetsBinding.instance.addPostFrameCallback((_) {
      ref.read(businessSetupProvider.notifier).loadDraft(ref);
    });
  }

  @override
  void dispose() {
    _companyNameController.dispose();
    _descriptionController.dispose();
    _pulseController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(businessSetupProvider);
    final notifier = ref.read(businessSetupProvider.notifier);

    if (_companyNameController.text != state.companyName) {
      _companyNameController.text = state.companyName;
    }
    if (_descriptionController.text != state.description) {
      _descriptionController.text = state.description;
    }

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
                                'Your business, live in minutes.',
                                style: TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                  fontSize: 16,
                                ),
                              ),
                            ] else if (state.step == 1) ...[
                              const Text(
                                'Business Type',
                                style: TextStyle(
                                  fontFamily: 'Inter',
                                  fontWeight: FontWeight.bold,
                                  color: Colors.white,
                                  fontSize: 18,
                                ),
                              ),
                              const SizedBox(height: 16),
                              Wrap(
                                spacing: 12,
                                runSpacing: 12,
                                alignment: WrapAlignment.center,
                                children:
                                    [
                                      {
                                        'title': 'Online Store',
                                        'icon': Icons.shopping_cart,
                                      },
                                      {
                                        'title': 'Service Business',
                                        'icon': Icons.build,
                                      },
                                      {
                                        'title': 'Restaurant / Food',
                                        'icon': Icons.restaurant,
                                      },
                                      {
                                        'title': 'Creative / Portfolio',
                                        'icon': Icons.brush,
                                      },
                                      {
                                        'title': 'Local Business',
                                        'icon': Icons.storefront,
                                      },
                                      {
                                        'title': 'Other',
                                        'icon': Icons.business,
                                      },
                                    ].map((typeInfo) {
                                      final type = typeInfo['title'] as String;
                                      final icon = typeInfo['icon'] as IconData;
                                      final isSelected =
                                          state.businessType == type;
                                      return GestureDetector(
                                        onTap:
                                            () => notifier.updateBusinessType(
                                              type,
                                            ),
                                        child: Container(
                                          width: 140,
                                          height: 120,
                                          decoration: BoxDecoration(
                                            color:
                                                isSelected
                                                    ? Colors.blueAccent
                                                        .withOpacity(0.2)
                                                    : Colors.white.withOpacity(
                                                      0.05,
                                                    ),
                                            border: Border.all(
                                              color:
                                                  isSelected
                                                      ? Colors.blueAccent
                                                      : Colors.white
                                                          .withOpacity(0.1),
                                            ),
                                            borderRadius: BorderRadius.circular(
                                              12,
                                            ),
                                          ),
                                          child: Column(
                                            mainAxisAlignment:
                                                MainAxisAlignment.center,
                                            children: [
                                              Icon(
                                                icon,
                                                size: 40,
                                                color:
                                                    isSelected
                                                        ? Colors.blueAccent
                                                        : Colors.white70,
                                              ),
                                              const SizedBox(height: 12),
                                              Text(
                                                type,
                                                textAlign: TextAlign.center,
                                                style: TextStyle(
                                                  fontFamily: 'Inter',
                                                  color:
                                                      isSelected
                                                          ? Colors.white
                                                          : Colors.white70,
                                                ),
                                              ),
                                            ],
                                          ),
                                        ),
                                      );
                                    }).toList(),
                              ),
                            ] else if (state.step == 2) ...[
                              TextFormField(
                                controller: _companyNameController,
                                decoration: const InputDecoration(
                                  labelText: 'Company name',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updateCompany,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                              ),
                              const SizedBox(height: 16),
                              TextFormField(
                                controller: _descriptionController,
                                decoration: const InputDecoration(
                                  labelText: 'Description',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updateDescription,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                                maxLines: 3,
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
                              Wrap(
                                spacing: 12,
                                runSpacing: 12,
                                alignment: WrapAlignment.center,
                                children:
                                    [
                                      {
                                        'title': 'Physical products',
                                        'icon': Icons.inventory,
                                      },
                                      {
                                        'title': 'Digital downloads',
                                        'icon': Icons.download,
                                      },
                                      {
                                        'title': 'Services / appointments',
                                        'icon': Icons.calendar_today,
                                      },
                                      {
                                        'title': 'Food & beverages',
                                        'icon': Icons.fastfood,
                                      },
                                      {
                                        'title': 'Subscriptions',
                                        'icon': Icons.repeat,
                                      },
                                    ].map((itemInfo) {
                                      final item = itemInfo['title'] as String;
                                      final icon = itemInfo['icon'] as IconData;
                                      final isSelected = state.whatYouSell
                                          .contains(item);
                                      return GestureDetector(
                                        onTap:
                                            () => notifier.toggleWhatYouSell(
                                              item,
                                            ),
                                        child: Container(
                                          width: 140,
                                          height: 120,
                                          decoration: BoxDecoration(
                                            color:
                                                isSelected
                                                    ? Colors.blueAccent
                                                        .withOpacity(0.2)
                                                    : Colors.white.withOpacity(
                                                      0.05,
                                                    ),
                                            border: Border.all(
                                              color:
                                                  isSelected
                                                      ? Colors.blueAccent
                                                      : Colors.white
                                                          .withOpacity(0.1),
                                            ),
                                            borderRadius: BorderRadius.circular(
                                              12,
                                            ),
                                          ),
                                          child: Column(
                                            mainAxisAlignment:
                                                MainAxisAlignment.center,
                                            children: [
                                              Icon(
                                                icon,
                                                size: 40,
                                                color:
                                                    isSelected
                                                        ? Colors.blueAccent
                                                        : Colors.white70,
                                              ),
                                              const SizedBox(height: 12),
                                              Text(
                                                item,
                                                textAlign: TextAlign.center,
                                                style: TextStyle(
                                                  fontFamily: 'Inter',
                                                  color:
                                                      isSelected
                                                          ? Colors.white
                                                          : Colors.white70,
                                                  fontSize: 12,
                                                ),
                                              ),
                                            ],
                                          ),
                                        ),
                                      );
                                    }).toList(),
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
                                {
                                  'title': 'Online only',
                                  'eta': 'Instant setup',
                                },
                                {
                                  'title': 'In-person (POS)',
                                  'eta': 'Ships in 3-5 days',
                                },
                                {'title': 'Both', 'eta': 'Start online today'},
                                {
                                  'title': 'Skip for now',
                                  'eta': 'Set up later',
                                },
                              ].map((depInfo) {
                                final dep = depInfo['title']!;
                                final eta = depInfo['eta']!;
                                final isSelected = state.paymentMethod == dep;
                                return Padding(
                                  padding: const EdgeInsets.only(bottom: 8.0),
                                  child: GestureDetector(
                                    onTap:
                                        () => notifier.updatePaymentMethod(dep),
                                    child: Container(
                                      decoration: BoxDecoration(
                                        color:
                                            isSelected
                                                ? Colors.blueAccent.withOpacity(
                                                  0.2,
                                                )
                                                : Colors.white.withOpacity(
                                                  0.05,
                                                ),
                                        border: Border.all(
                                          color:
                                              isSelected
                                                  ? Colors.blueAccent
                                                  : Colors.white.withOpacity(
                                                    0.1,
                                                  ),
                                        ),
                                        borderRadius: BorderRadius.circular(12),
                                      ),
                                      padding: const EdgeInsets.all(16),
                                      child: Row(
                                        children: [
                                          Icon(
                                            isSelected
                                                ? Icons.radio_button_checked
                                                : Icons.radio_button_unchecked,
                                            color:
                                                isSelected
                                                    ? Colors.blueAccent
                                                    : Colors.white70,
                                          ),
                                          const SizedBox(width: 16),
                                          Expanded(
                                            child: Column(
                                              crossAxisAlignment:
                                                  CrossAxisAlignment.start,
                                              children: [
                                                Text(
                                                  dep,
                                                  style: const TextStyle(
                                                    fontFamily: 'Inter',
                                                    color: Colors.white,
                                                    fontWeight: FontWeight.bold,
                                                  ),
                                                ),
                                                Text(
                                                  eta,
                                                  style: const TextStyle(
                                                    fontFamily: 'Inter',
                                                    color: Colors.white54,
                                                    fontSize: 12,
                                                  ),
                                                ),
                                              ],
                                            ),
                                          ),
                                        ],
                                      ),
                                    ),
                                  ),
                                );
                              }),
                            ] else if (state.step == 5) ...[
                              const Text(
                                'Admin Account',
                                style: TextStyle(
                                  fontFamily: 'Inter',
                                  fontWeight: FontWeight.bold,
                                  color: Colors.white,
                                ),
                              ),
                              TextFormField(
                                initialValue: state.adminName,
                                decoration: const InputDecoration(
                                  labelText: 'Admin Name',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updateAdminName,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                              ),
                              const SizedBox(height: 16),
                              TextFormField(
                                initialValue: state.adminEmail,
                                decoration: const InputDecoration(
                                  labelText: 'Admin Email',
                                  labelStyle: TextStyle(color: Colors.white70),
                                ),
                                onChanged: notifier.updateAdminEmail,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                              ),
                              const SizedBox(height: 16),
                              TextFormField(
                                initialValue: state.adminPassword,
                                obscureText: state.obscurePassword,
                                onChanged: notifier.updateAdminPassword,
                                style: const TextStyle(
                                  fontFamily: 'Inter',
                                  color: Colors.white,
                                ),
                                decoration: InputDecoration(
                                  labelText: 'Admin Password',
                                  labelStyle: const TextStyle(
                                    color: Colors.white70,
                                  ),
                                  suffixIcon: IconButton(
                                    icon: Icon(
                                      state.obscurePassword
                                          ? Icons.visibility
                                          : Icons.visibility_off,
                                      color: Colors.white70,
                                    ),
                                    onPressed: () {
                                      notifier.toggleObscurePassword();
                                    },
                                  ),
                                ),
                              ),
                              const SizedBox(height: 8),
                              Row(
                                children: [
                                  Expanded(
                                    child: Container(
                                      height: 4,
                                      color:
                                          state.adminPassword.length > 2
                                              ? Colors.red
                                              : Colors.white24,
                                    ),
                                  ),
                                  const SizedBox(width: 4),
                                  Expanded(
                                    child: Container(
                                      height: 4,
                                      color:
                                          state.adminPassword.length > 5
                                              ? Colors.orange
                                              : Colors.white24,
                                    ),
                                  ),
                                  const SizedBox(width: 4),
                                  Expanded(
                                    child: Container(
                                      height: 4,
                                      color:
                                          state.adminPassword.length > 8
                                              ? Colors.green
                                              : Colors.white24,
                                    ),
                                  ),
                                ],
                              ),
                              const SizedBox(height: 24),
                              const Text(
                                'Or sign up with',
                                style: TextStyle(
                                  color: Colors.white54,
                                  fontSize: 12,
                                ),
                              ),
                              const SizedBox(height: 8),
                              Row(
                                mainAxisAlignment: MainAxisAlignment.center,
                                children: [
                                  OutlinedButton.icon(
                                    onPressed: () {},
                                    icon: const Icon(
                                      Icons.g_mobiledata,
                                      color: Colors.white,
                                    ),
                                    label: const Text(
                                      'Google',
                                      style: TextStyle(color: Colors.white),
                                    ),
                                  ),
                                  const SizedBox(width: 16),
                                  OutlinedButton.icon(
                                    onPressed: () {},
                                    icon: const Icon(
                                      Icons.apple,
                                      color: Colors.white,
                                    ),
                                    label: const Text(
                                      'Apple',
                                      style: TextStyle(color: Colors.white),
                                    ),
                                  ),
                                ],
                              ),
                            ] else if (state.step == 6) ...[
                              const Text(
                                'Review & Launch',
                                style: TextStyle(
                                  fontFamily: 'Inter',
                                  fontWeight: FontWeight.bold,
                                  color: Colors.white,
                                  fontSize: 20,
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
                                      'Business Type: ${state.businessType}',
                                      style: const TextStyle(
                                        color: Colors.white,
                                        fontFamily: 'Inter',
                                      ),
                                    ),
                                    const SizedBox(height: 8),
                                    Text(
                                      'Name: ${state.companyName}',
                                      style: const TextStyle(
                                        color: Colors.white,
                                        fontFamily: 'Inter',
                                        fontWeight: FontWeight.bold,
                                        fontSize: 16,
                                      ),
                                    ),
                                    const SizedBox(height: 8),
                                    Text(
                                      'Selling: ${state.whatYouSell.join(', ')}',
                                      style: const TextStyle(
                                        color: Colors.white70,
                                        fontFamily: 'Inter',
                                      ),
                                    ),
                                    const SizedBox(height: 8),
                                    Text(
                                      'Payments: ${state.paymentMethod}',
                                      style: const TextStyle(
                                        color: Colors.white70,
                                        fontFamily: 'Inter',
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
                        state.step == 6
                            ? AnimatedBuilder(
                              animation: _pulseController,
                              builder: (context, child) {
                                return Transform.scale(
                                  scale:
                                      state.isLoading
                                          ? 1.0
                                          : 1.0 +
                                              (_pulseController.value * 0.05),
                                  child: ElevatedButton(
                                    style: ElevatedButton.styleFrom(
                                      backgroundColor: Colors.blueAccent,
                                      padding: const EdgeInsets.symmetric(
                                        horizontal: 24,
                                        vertical: 12,
                                      ),
                                    ),
                                    onPressed:
                                        state.isLoading
                                            ? null
                                            : () {
                                              notifier.launch(context, ref);
                                            },
                                    child:
                                        state.isLoading
                                            ? const SizedBox(
                                              width: 20,
                                              height: 20,
                                              child: CircularProgressIndicator(
                                                strokeWidth: 2,
                                                color: Colors.white,
                                              ),
                                            )
                                            : const Text(
                                              'Launch My Business →',
                                              style: TextStyle(
                                                fontFamily: 'Inter',
                                                fontWeight: FontWeight.bold,
                                                color: Colors.white,
                                              ),
                                            ),
                                  ),
                                );
                              },
                            )
                            : ElevatedButton(
                              onPressed:
                                  state.isLoading
                                      ? null
                                      : () {
                                        notifier.nextStep(ref);
                                      },
                              child: const Text(
                                'Continue',
                                style: TextStyle(fontFamily: 'Inter'),
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
