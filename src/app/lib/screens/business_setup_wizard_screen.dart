import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
import '../services/auth_service.dart';
import '../services/api_service.dart';
import '../widgets/glass_card.dart';

class BusinessSetupState {
  final int step;
  final String businessType;
  final String companyName;
  final String businessDescription;
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
    this.businessDescription = '',
    this.whatYouSell = const [],
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
    String? companyName,
    String? businessDescription,
    List<String>? whatYouSell,
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
      companyName: companyName ?? this.companyName,
      businessDescription: businessDescription ?? this.businessDescription,
      whatYouSell: whatYouSell ?? this.whatYouSell,
      paymentMethod: paymentMethod ?? this.paymentMethod,
      adminName: adminName ?? this.adminName,
      adminEmail: adminEmail ?? this.adminEmail,
      adminPassword: adminPassword ?? this.adminPassword,
      isLoading: isLoading ?? this.isLoading,
      errorMessage: errorMessage ?? this.errorMessage, // null is handled intentionally
    );
  }

  Map<String, dynamic> toJson() => {
        'step': step,
        'businessType': businessType,
        'companyName': companyName,
        'businessDescription': businessDescription,
        'whatYouSell': whatYouSell,
        'paymentMethod': paymentMethod,
        'adminName': adminName,
        'adminEmail': adminEmail,
        // Don't save password in plain text state
      };

  factory BusinessSetupState.fromJson(Map<String, dynamic> json) {
    return BusinessSetupState(
      step: json['step'] as int? ?? 0,
      businessType: json['businessType'] as String? ?? '',
      companyName: json['companyName'] as String? ?? '',
      businessDescription: json['businessDescription'] as String? ?? '',
      whatYouSell: (json['whatYouSell'] as List<dynamic>?)?.cast<String>() ?? [],
      paymentMethod: json['paymentMethod'] as String? ?? '',
      adminName: json['adminName'] as String? ?? '',
      adminEmail: json['adminEmail'] as String? ?? '',
    );
  }
}

class BusinessSetupNotifier extends StateNotifier<BusinessSetupState> {
  final Ref ref;

  BusinessSetupNotifier(this.ref) : super(const BusinessSetupState()) {
    _loadState();
  }

  Future<void> _loadState() async {
    try {
      final user = await ref.read(authStateProvider.future);
      final token = user?.token;
      final res = await http.get(
        Uri.parse('/api/wizard/state'),
        headers: token != null ? {'Authorization': 'Bearer $token'} : {},
      );
      if (res.statusCode == 200) {
        final Map<String, dynamic> data = jsonDecode(res.body);
        if (data.isNotEmpty) {
          state = BusinessSetupState.fromJson(data);
        }
      }
    } catch (e) {
      // Ignore load errors, start fresh
    }
  }

  Future<void> _saveState() async {
    try {
      final user = await ref.read(authStateProvider.future);
      final token = user?.token;
      await http.post(
        Uri.parse('/api/wizard/state/save'),
        headers: {
          'Content-Type': 'application/json',
          if (token != null) 'Authorization': 'Bearer $token',
        },
        body: jsonEncode(state.toJson()),
      );
    } catch (e) {
      // Best effort save
    }
  }

  void updateField({
    String? businessType,
    String? companyName,
    String? businessDescription,
    List<String>? whatYouSell,
    String? paymentMethod,
    String? adminName,
    String? adminEmail,
    String? adminPassword,
  }) {
    state = state.copyWith(
      businessType: businessType,
      companyName: companyName,
      businessDescription: businessDescription,
      whatYouSell: whatYouSell,
      paymentMethod: paymentMethod,
      adminName: adminName,
      adminEmail: adminEmail,
      adminPassword: adminPassword,
      errorMessage: null,
    );
    _saveState();
  }

  void nextStep() {
    if (state.step < 6) {
      state = state.copyWith(step: state.step + 1, errorMessage: null);
      _saveState();
    }
  }

  void prevStep() {
    if (state.step > 0) {
      state = state.copyWith(step: state.step - 1, errorMessage: null);
      _saveState();
    }
  }

  void setErrorMessage(String? msg) {
    state = state.copyWith(errorMessage: msg);
  }

  Future<void> launch(BuildContext context) async {
    state = state.copyWith(isLoading: true, errorMessage: null);
    try {
      final api = ref.read(apiServiceProvider);
      if (api != null) {
        await http.post(
          Uri.parse("/api/wizard/configure"),
          headers: {"Content-Type": "application/json"},
          body: jsonEncode({
            'extras': {
               'business_wizard_completed': 'true',
               'company_name': state.companyName,
            }
          }),
        );

        // Wait a bit to simulate provisioning
        await Future.delayed(const Duration(seconds: 2));
      }
      if (context.mounted) {
         context.go('/dashboard');
      }
    } catch (e) {
      state = state.copyWith(isLoading: false, errorMessage: 'Launch failed: $e');
    }
  }
}

final businessSetupProvider = StateNotifierProvider<BusinessSetupNotifier, BusinessSetupState>((ref) {
  return BusinessSetupNotifier(ref);
});

class BusinessSetupWizardScreen extends ConsumerWidget {
  const BusinessSetupWizardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(businessSetupProvider);
    final notifier = ref.read(businessSetupProvider.notifier);

    return Scaffold(
      body: Stack(
        children: [
          // Background Gradient matching OHC premium tokens
          Container(
            decoration: BoxDecoration(
              gradient: LinearGradient(
                begin: Alignment.topLeft,
                end: Alignment.bottomRight,
                colors: [
                  Theme.of(context).colorScheme.surface,
                  Theme.of(context).colorScheme.primaryContainer.withValues(alpha: 0.1),
                ],
              ),
            ),
          ),

          SafeArea(
            child: Column(
              children: [
                if (state.step > 0 && !state.isLoading)
                  Padding(
                    padding: const EdgeInsets.all(16.0),
                    child: Row(
                      children: [
                        IconButton(
                          icon: const Icon(Icons.arrow_back),
                          onPressed: () => notifier.prevStep(),
                        ),
                        Expanded(
                          child: LinearProgressIndicator(
                            value: (state.step) / 6.0,
                            backgroundColor: Theme.of(context).colorScheme.surfaceContainerHighest,
                            color: Theme.of(context).colorScheme.primary,
                            borderRadius: BorderRadius.circular(4),
                          ),
                        ),
                        const SizedBox(width: 48), // Balance back button
                      ],
                    ),
                  ),

                Expanded(
                  child: Center(
                    child: SingleChildScrollView(
                      padding: const EdgeInsets.symmetric(horizontal: 24.0, vertical: 16.0),
                      child: AnimatedSwitcher(
                        duration: const Duration(milliseconds: 300),
                        transitionBuilder: (Widget child, Animation<double> animation) {
                          return FadeTransition(
                            opacity: animation,
                            child: SlideTransition(
                              position: Tween<Offset>(
                                begin: const Offset(0.05, 0),
                                end: Offset.zero,
                              ).animate(animation),
                              child: child,
                            ),
                          );
                        },
                        child: _buildStep(state.step, context, ref),
                      ),
                    ),
                  ),
                ),
              ],
            ),
          ),

          if (state.isLoading)
             Container(
               color: Colors.black54,
               child: const Center(
                 child: Column(
                   mainAxisSize: MainAxisSize.min,
                   children: [
                     CircularProgressIndicator(),
                     SizedBox(height: 16),
                     Text(
                       'Your business is setting up…',
                       style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white),
                     ),
                   ],
                 ),
               ),
             ),
        ],
      ),
    );
  }

  Widget _buildStep(int step, BuildContext context, WidgetRef ref) {
    switch (step) {
      case 0:
        return _WelcomeStep(key: const ValueKey(0));
      case 1:
        return _BusinessTypeStep(key: const ValueKey(1));
      case 2:
        return _BusinessDetailsStep(key: const ValueKey(2));
      case 3:
        return _WhatYouSellStep(key: const ValueKey(3));
      case 4:
        return _PaymentPreferenceStep(key: const ValueKey(4));
      case 5:
        return _AdminAccountStep(key: const ValueKey(5));
      case 6:
        return _ReviewLaunchStep(key: const ValueKey(6));
      default:
        return const SizedBox.shrink(key: ValueKey(-1));
    }
  }
}

// ---------------------------------------------------------
// STEP 0: Welcome Screen
// ---------------------------------------------------------
class _WelcomeStep extends ConsumerWidget {
  const _WelcomeStep({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        const Icon(Icons.rocket_launch, size: 80, color: Colors.blueAccent),
        const SizedBox(height: 32),
        const Text(
          'Your business, live in minutes.',
          textAlign: TextAlign.center,
          style: TextStyle(fontFamily: 'Outfit', fontSize: 36, fontWeight: FontWeight.bold),
        ),
        const SizedBox(height: 16),
        const Text(
          'One Human Corp gives you a beautiful storefront, AI agents, and everything you need to succeed online.',
          textAlign: TextAlign.center,
          style: TextStyle(fontFamily: 'Inter', fontSize: 18, color: Colors.grey),
        ),
        const SizedBox(height: 48),
        FilledButton(
          onPressed: () => ref.read(businessSetupProvider.notifier).nextStep(),
          style: FilledButton.styleFrom(
            padding: const EdgeInsets.symmetric(horizontal: 48, vertical: 20),
            textStyle: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Inter'),
          ),
          child: const Text('Get Started'),
        ),
      ],
    );
  }
}

// ---------------------------------------------------------
// STEP 1: Business Type
// ---------------------------------------------------------
class _BusinessTypeStep extends ConsumerWidget {
  const _BusinessTypeStep({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(businessSetupProvider);
    final notifier = ref.read(businessSetupProvider.notifier);

    final types = [
      {'icon': Icons.storefront, 'label': 'Online Store'},
      {'icon': Icons.handyman, 'label': 'Service Business'},
      {'icon': Icons.restaurant, 'label': 'Restaurant / Food'},
      {'icon': Icons.brush, 'label': 'Creative / Portfolio'},
      {'icon': Icons.location_city, 'label': 'Local Business'},
      {'icon': Icons.category, 'label': 'Other'},
    ];

    return Container(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Text(
            'What kind of business are you building?',
            style: TextStyle(fontFamily: 'Outfit', fontSize: 28, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 32),
          Wrap(
            spacing: 16,
            runSpacing: 16,
            alignment: WrapAlignment.center,
            children: types.map((t) {
              final isSelected = state.businessType == t['label'];
              return InkWell(
                onTap: () {
                  notifier.updateField(businessType: t['label'] as String);
                  notifier.nextStep();
                },
                borderRadius: BorderRadius.circular(16),
                child: Container(
                  width: 160,
                  padding: const EdgeInsets.all(24),
                  decoration: BoxDecoration(
                    color: isSelected ? Theme.of(context).colorScheme.primaryContainer : Theme.of(context).colorScheme.surface,
                    border: Border.all(
                      color: isSelected ? Theme.of(context).colorScheme.primary : Theme.of(context).dividerColor,
                      width: 2,
                    ),
                    borderRadius: BorderRadius.circular(16),
                  ),
                  child: Column(
                    children: [
                      Icon(t['icon'] as IconData, size: 48, color: isSelected ? Theme.of(context).colorScheme.primary : Colors.grey),
                      const SizedBox(height: 16),
                      Text(
                        t['label'] as String,
                        textAlign: TextAlign.center,
                        style: TextStyle(
                          fontFamily: 'Inter',
                          fontWeight: FontWeight.bold,
                          color: isSelected ? Theme.of(context).colorScheme.primary : Theme.of(context).colorScheme.onSurface,
                        ),
                      ),
                    ],
                  ),
                ),
              );
            }).toList(),
          ),
          if (state.errorMessage != null) ...[
             const SizedBox(height: 16),
             Text(state.errorMessage!, style: TextStyle(color: Theme.of(context).colorScheme.error)),
          ],
        ],
      ),
    );
  }
}

// ---------------------------------------------------------
// STEP 2: Business Details
// ---------------------------------------------------------
class _BusinessDetailsStep extends ConsumerStatefulWidget {
  const _BusinessDetailsStep({super.key});

  @override
  ConsumerState<_BusinessDetailsStep> createState() => _BusinessDetailsStepState();
}

class _BusinessDetailsStepState extends ConsumerState<_BusinessDetailsStep> {
  late TextEditingController _nameController;
  late TextEditingController _descController;
  bool _generating = false;

  @override
  void initState() {
    super.initState();
    final state = ref.read(businessSetupProvider);
    _nameController = TextEditingController(text: state.companyName);
    _descController = TextEditingController(text: state.businessDescription);
  }

  @override
  void dispose() {
    _nameController.dispose();
    _descController.dispose();
    super.dispose();
  }

  Future<void> _autoSuggest() async {
    if (_nameController.text.trim().isEmpty) return;
    setState(() => _generating = true);

    try {
      final user = await ref.read(authStateProvider.future);
      final token = user?.token;
      final res = await http.post(
        Uri.parse('/api/wizard/generate_description'),
        headers: {
          'Content-Type': 'application/json',
          if (token != null) 'Authorization': 'Bearer $token',
        },
        body: jsonEncode({'product_name': _nameController.text.trim()}),
      );

      if (res.statusCode == 200) {
        final data = jsonDecode(res.body);
        setState(() {
          _descController.text = data['description'] ?? '';
        });
      }
    } catch (e) {
      // Ignore
    } finally {
      if (mounted) {
        setState(() => _generating = false);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Text(
            'Name & Description',
            style: TextStyle(fontFamily: 'Outfit', fontSize: 28, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 32),
          TextField(
            controller: _nameController,
            style: const TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
            decoration: const InputDecoration(
              labelText: 'Business Name',
              border: OutlineInputBorder(),
            ),
          ),
          const SizedBox(height: 24),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text('Short Description', style: TextStyle(fontWeight: FontWeight.bold)),
              TextButton.icon(
                icon: _generating ? const SizedBox(width: 16, height: 16, child: CircularProgressIndicator(strokeWidth: 2)) : const Icon(Icons.auto_awesome),
                label: const Text('AI Auto-suggest'),
                onPressed: _generating ? null : _autoSuggest,
              ),
            ],
          ),
          const SizedBox(height: 8),
          TextField(
            controller: _descController,
            maxLines: 4,
            decoration: const InputDecoration(
              hintText: 'What do you do?',
              border: OutlineInputBorder(),
            ),
          ),
          const SizedBox(height: 32),
          FilledButton(
            onPressed: () {
               if (_nameController.text.trim().isEmpty) {
                 ref.read(businessSetupProvider.notifier).setErrorMessage("Name is required");
                 return;
               }
               ref.read(businessSetupProvider.notifier).updateField(
                 companyName: _nameController.text.trim(),
                 businessDescription: _descController.text.trim(),
               );
               ref.read(businessSetupProvider.notifier).nextStep();
            },
            child: const Padding(
              padding: EdgeInsets.all(16.0),
              child: Text('Next'),
            ),
          ),
          if (ref.watch(businessSetupProvider).errorMessage != null) ...[
             const SizedBox(height: 16),
             Text(ref.watch(businessSetupProvider).errorMessage!, style: TextStyle(color: Theme.of(context).colorScheme.error)),
          ],
        ],
      ),
    );
  }
}

// ---------------------------------------------------------
// STEP 3: What do you sell?
// ---------------------------------------------------------
class _WhatYouSellStep extends ConsumerWidget {
  const _WhatYouSellStep({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(businessSetupProvider);
    final notifier = ref.read(businessSetupProvider.notifier);

    final options = [
      "Physical products",
      "Digital downloads",
      "Services / appointments",
      "Food & beverages",
      "Subscriptions"
    ];

    return Container(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Text(
            'What do you sell?',
            style: TextStyle(fontFamily: 'Outfit', fontSize: 28, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 16),
          const Text('Select all that apply.', style: TextStyle(color: Colors.grey)),
          const SizedBox(height: 32),
          ...options.map((opt) {
            final isSelected = state.whatYouSell.contains(opt);
            return Padding(
              padding: const EdgeInsets.only(bottom: 12.0),
              child: CheckboxListTile(
                title: Text(opt, style: const TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Inter')),
                value: isSelected,
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(12),
                  side: BorderSide(color: isSelected ? Theme.of(context).colorScheme.primary : Theme.of(context).dividerColor),
                ),
                tileColor: isSelected ? Theme.of(context).colorScheme.primaryContainer.withValues(alpha: 0.3) : Theme.of(context).colorScheme.surface,
                onChanged: (val) {
                  final list = List<String>.from(state.whatYouSell);
                  if (val == true) {
                    list.add(opt);
                  } else {
                    list.remove(opt);
                  }
                  notifier.updateField(whatYouSell: list);
                },
              ),
            );
          }),
          const SizedBox(height: 24),
          FilledButton(
            onPressed: () {
               if (state.whatYouSell.isEmpty) {
                 notifier.setErrorMessage("Please select at least one option.");
                 return;
               }
               notifier.nextStep();
            },
            child: const Padding(
              padding: EdgeInsets.all(16.0),
              child: Text('Next'),
            ),
          ),
          if (state.errorMessage != null) ...[
             const SizedBox(height: 16),
             Text(state.errorMessage!, style: TextStyle(color: Theme.of(context).colorScheme.error)),
          ],
        ],
      ),
    );
  }
}

// ---------------------------------------------------------
// STEP 4: Payment Preference
// ---------------------------------------------------------
class _PaymentPreferenceStep extends ConsumerWidget {
  const _PaymentPreferenceStep({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(businessSetupProvider);
    final notifier = ref.read(businessSetupProvider.notifier);

    final options = [
      {'id': 'online', 'title': 'Online only', 'eta': 'Instant setup'},
      {'id': 'pos', 'title': 'In-person (POS)', 'eta': 'Ships in 3 days'},
      {'id': 'both', 'title': 'Both', 'eta': 'Setup online now'},
      {'id': 'skip', 'title': 'Skip for now', 'eta': ''},
    ];

    return Container(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Text(
            'How do you want to receive payments?',
            style: TextStyle(fontFamily: 'Outfit', fontSize: 28, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 32),
          ...options.map((opt) {
            final isSelected = state.paymentMethod == opt['id'];
            return Padding(
              padding: const EdgeInsets.only(bottom: 12.0),
              child: InkWell(
                onTap: () {
                  notifier.updateField(paymentMethod: opt['id'] as String);
                  notifier.nextStep();
                },
                borderRadius: BorderRadius.circular(12),
                child: Container(
                  padding: const EdgeInsets.all(20),
                  decoration: BoxDecoration(
                    color: isSelected ? Theme.of(context).colorScheme.primaryContainer.withValues(alpha: 0.3) : Theme.of(context).colorScheme.surface,
                    border: Border.all(
                      color: isSelected ? Theme.of(context).colorScheme.primary : Theme.of(context).dividerColor,
                      width: 2,
                    ),
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: Row(
                    children: [
                      Icon(
                        isSelected ? Icons.radio_button_checked : Icons.radio_button_unchecked,
                        color: isSelected ? Theme.of(context).colorScheme.primary : Colors.grey,
                      ),
                      const SizedBox(width: 16),
                      Expanded(
                        child: Text(opt['title'] as String, style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 16)),
                      ),
                      if ((opt['eta'] as String).isNotEmpty)
                        Text(opt['eta'] as String, style: const TextStyle(color: Colors.grey, fontSize: 13)),
                    ],
                  ),
                ),
              ),
            );
          }),
          if (state.errorMessage != null) ...[
             const SizedBox(height: 16),
             Text(state.errorMessage!, style: TextStyle(color: Theme.of(context).colorScheme.error)),
          ],
        ],
      ),
    );
  }
}

// ---------------------------------------------------------
// STEP 5: Administrator Account
// ---------------------------------------------------------
class _AdminAccountStep extends ConsumerStatefulWidget {
  const _AdminAccountStep({super.key});

  @override
  ConsumerState<_AdminAccountStep> createState() => _AdminAccountStepState();
}

class _AdminAccountStepState extends ConsumerState<_AdminAccountStep> {
  late TextEditingController _nameController;
  late TextEditingController _emailController;
  late TextEditingController _passController;
  bool _obscure = true;

  @override
  void initState() {
    super.initState();
    final state = ref.read(businessSetupProvider);
    _nameController = TextEditingController(text: state.adminName);
    _emailController = TextEditingController(text: state.adminEmail);
    _passController = TextEditingController(text: state.adminPassword);
  }

  @override
  void dispose() {
    _nameController.dispose();
    _emailController.dispose();
    _passController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final notifier = ref.read(businessSetupProvider.notifier);

    // Very basic strength calculation for visual feedback
    final passLength = _passController.text.length;
    double strength = 0;
    if (passLength > 0) strength = 0.25;
    if (passLength >= 6) strength = 0.5;
    if (passLength >= 8) strength = 0.75;
    if (passLength >= 10 && _passController.text.contains(RegExp(r'[0-9]'))) strength = 1.0;

    Color strengthColor = Colors.red;
    if (strength >= 0.5) strengthColor = Colors.orange;
    if (strength >= 0.75) strengthColor = Colors.green;

    return Container(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Text(
            'Create your Administrator account',
            style: TextStyle(fontFamily: 'Outfit', fontSize: 28, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 32),
          TextField(
            controller: _nameController,
            decoration: const InputDecoration(labelText: 'Full Name', border: OutlineInputBorder()),
          ),
          const SizedBox(height: 16),
          TextField(
            controller: _emailController,
            keyboardType: TextInputType.emailAddress,
            decoration: const InputDecoration(labelText: 'Email Address', border: OutlineInputBorder()),
          ),
          const SizedBox(height: 16),
          TextField(
            controller: _passController,
            obscureText: _obscure,
            onChanged: (v) => setState((){}),
            decoration: InputDecoration(
              labelText: 'Password',
              border: const OutlineInputBorder(),
              suffixIcon: IconButton(
                icon: Icon(_obscure ? Icons.visibility_off : Icons.visibility),
                onPressed: () => setState(() => _obscure = !_obscure),
              ),
            ),
          ),
          const SizedBox(height: 8),
          if (passLength > 0)
            LinearProgressIndicator(
              value: strength,
              backgroundColor: Colors.grey.shade300,
              color: strengthColor,
            ),
          const SizedBox(height: 32),

          const Center(child: Text('OR', style: TextStyle(color: Colors.grey, fontWeight: FontWeight.bold))),
          const SizedBox(height: 16),

          OutlinedButton.icon(
            onPressed: () {},
            icon: const Icon(Icons.g_mobiledata, size: 24),
            label: const Text('Continue with Google'),
            style: OutlinedButton.styleFrom(padding: const EdgeInsets.all(16)),
          ),
          const SizedBox(height: 8),
          OutlinedButton.icon(
            onPressed: () {},
            icon: const Icon(Icons.apple, size: 24),
            label: const Text('Continue with Apple'),
            style: OutlinedButton.styleFrom(padding: const EdgeInsets.all(16)),
          ),

          const SizedBox(height: 32),
          FilledButton(
            onPressed: () {
               if (_nameController.text.trim().isEmpty || _emailController.text.trim().isEmpty || _passController.text.trim().isEmpty) {
                 notifier.setErrorMessage("All fields are required.");
                 return;
               }
               notifier.updateField(
                 adminName: _nameController.text.trim(),
                 adminEmail: _emailController.text.trim(),
                 adminPassword: _passController.text.trim(),
               );
               notifier.nextStep();
            },
            child: const Padding(
              padding: EdgeInsets.all(16.0),
              child: Text('Next'),
            ),
          ),
          if (ref.watch(businessSetupProvider).errorMessage != null) ...[
             const SizedBox(height: 16),
             Text(ref.watch(businessSetupProvider).errorMessage!, style: TextStyle(color: Theme.of(context).colorScheme.error)),
          ],
        ],
      ),
    );
  }
}

// ---------------------------------------------------------
// STEP 6: Review & Launch
// ---------------------------------------------------------
class _ReviewLaunchStep extends ConsumerStatefulWidget {
  const _ReviewLaunchStep({super.key});

  @override
  ConsumerState<_ReviewLaunchStep> createState() => _ReviewLaunchStepState();
}

class _ReviewLaunchStepState extends ConsumerState<_ReviewLaunchStep> with SingleTickerProviderStateMixin {
  late AnimationController _pulseController;

  @override
  void initState() {
    super.initState();
    _pulseController = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 1500),
    )..repeat(reverse: true);
  }

  @override
  void dispose() {
    _pulseController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(businessSetupProvider);

    return Container(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Text(
            'Review & Launch',
            textAlign: TextAlign.center,
            style: TextStyle(fontFamily: 'Outfit', fontSize: 32, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 32),

          GlassCard(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                _SummaryRow('Business Name', state.companyName),
                const Divider(),
                _SummaryRow('Type', state.businessType),
                const Divider(),
                _SummaryRow('Selling', state.whatYouSell.join(', ')),
                const Divider(),
                _SummaryRow('Payments', state.paymentMethod),
                const Divider(),
                _SummaryRow('Admin', state.adminEmail),
              ],
            ),
          ),

          const SizedBox(height: 48),

          AnimatedBuilder(
            animation: _pulseController,
            builder: (context, child) {
              return Transform.scale(
                scale: 1.0 + (_pulseController.value * 0.05),
                child: FilledButton(
                  onPressed: () => ref.read(businessSetupProvider.notifier).launch(context),
                  style: FilledButton.styleFrom(
                    padding: const EdgeInsets.symmetric(vertical: 24),
                    backgroundColor: Theme.of(context).colorScheme.primary,
                  ),
                  child: const Text(
                    'Launch My Business →',
                    style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
                  ),
                ),
              );
            },
          ),

          if (state.errorMessage != null) ...[
             const SizedBox(height: 16),
             Text(state.errorMessage!, textAlign: TextAlign.center, style: TextStyle(color: Theme.of(context).colorScheme.error)),
          ],
        ],
      ),
    );
  }
}

class _SummaryRow extends StatelessWidget {
  final String label;
  final String value;
  const _SummaryRow(this.label, this.value);

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            width: 140,
            child: Text(label, style: const TextStyle(fontWeight: FontWeight.bold, color: Colors.grey)),
          ),
          Expanded(
            child: Text(value.isEmpty ? 'Not set' : value, style: const TextStyle(fontWeight: FontWeight.w500)),
          ),
        ],
      ),
    );
  }
}
