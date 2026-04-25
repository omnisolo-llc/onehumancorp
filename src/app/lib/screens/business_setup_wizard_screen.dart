import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
import '../services/auth_service.dart';
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
      errorMessage: errorMessage ?? this.errorMessage,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'step': step,
      'businessType': businessType,
      'companyName': companyName,
      'businessDescription': businessDescription,
      'whatYouSell': whatYouSell,
      'paymentMethod': paymentMethod,
      'adminName': adminName,
      'adminEmail': adminEmail,
    };
  }

  factory BusinessSetupState.fromJson(Map<String, dynamic> json) {
    return BusinessSetupState(
      step: json['step'] as int? ?? 0,
      businessType: json['businessType'] as String? ?? '',
      companyName: json['companyName'] as String? ?? '',
      businessDescription: json['businessDescription'] as String? ?? '',
      whatYouSell: (json['whatYouSell'] as List<dynamic>?)?.map((e) => e as String).toList() ?? [],
      paymentMethod: json['paymentMethod'] as String? ?? '',
      adminName: json['adminName'] as String? ?? '',
      adminEmail: json['adminEmail'] as String? ?? '',
    );
  }
}

final businessSetupProvider = NotifierProvider<BusinessSetupNotifier, BusinessSetupState>(() {
  return BusinessSetupNotifier();
});

class BusinessSetupNotifier extends Notifier<BusinessSetupState> {
  @override
  BusinessSetupState build() => const BusinessSetupState();

  Future<void> loadState(WidgetRef ref) async {
    final user = ref.read(authStateProvider).valueOrNull;
    final baseUrl = ref.read(backendUrlProvider);
    if (user == null) return;
    try {
      final res = await http.get(
        Uri.parse('$baseUrl/api/wizard/state'),
        headers: {'Authorization': 'Bearer ${user.token}'},
      );
      if (res.statusCode == 200) {
        final data = jsonDecode(res.body);
        if (data.isNotEmpty) {
          state = BusinessSetupState.fromJson(data);
        }
      }
    } catch (_) {}
  }

  Future<void> _saveState(WidgetRef ref) async {
    final user = ref.read(authStateProvider).valueOrNull;
    final baseUrl = ref.read(backendUrlProvider);
    if (user == null) return;
    try {
      await http.post(
        Uri.parse('$baseUrl/api/wizard/state/save'),
        headers: {
          'Content-Type': 'application/json',
          'Authorization': 'Bearer ${user.token}',
        },
        body: jsonEncode(state.toJson()),
      );
    } catch (_) {}
  }

  void nextStep(WidgetRef ref) {
    if (state.step < 6) {
      state = state.copyWith(step: state.step + 1);
      _saveState(ref);
    }
  }

  void prevStep(WidgetRef ref) {
    if (state.step > 0) {
      state = state.copyWith(step: state.step - 1);
      _saveState(ref);
    }
  }

  void updateBusinessType(String type, WidgetRef ref) {
    state = state.copyWith(businessType: type);
    nextStep(ref);
  }

  void updateCompany(String name, WidgetRef ref) {
    state = state.copyWith(companyName: name);
  }
  void updateDescription(String desc, WidgetRef ref) {
    state = state.copyWith(businessDescription: desc);
  }

  void toggleWhatYouSell(String item, WidgetRef ref) {
    final list = List<String>.from(state.whatYouSell);
    if (list.contains(item)) {
      list.remove(item);
    } else {
      list.add(item);
    }
    state = state.copyWith(whatYouSell: list);
    _saveState(ref);
  }

  void updatePaymentMethod(String method, WidgetRef ref) {
    state = state.copyWith(paymentMethod: method);
    nextStep(ref);
  }

  void updateAdminName(String name, WidgetRef ref) {
    state = state.copyWith(adminName: name);
  }
  void updateAdminEmail(String val, WidgetRef ref) {
    state = state.copyWith(adminEmail: val);
  }
  void updateAdminPassword(String val, WidgetRef ref) {
    state = state.copyWith(adminPassword: val);
  }

  Future<void> launch(BuildContext context, WidgetRef ref) async {
    final user = ref.read(authStateProvider).valueOrNull;
    final baseUrl = ref.read(backendUrlProvider);

    state = state.copyWith(isLoading: true, errorMessage: null);

    if (user != null && baseUrl.isNotEmpty) {
      final body = {
        'extras': {
          'business_type': state.businessType,
          'company_name': state.companyName,
          'business_description': state.businessDescription,
          'what_you_sell': state.whatYouSell,
          'payment_method': state.paymentMethod,
          'admin_name': state.adminName,
          'admin_email': state.adminEmail,
          'admin_password': state.adminPassword,
        }
      };

      try {
        final res = await http.post(
          Uri.parse('$baseUrl/api/wizard/configure'),
          headers: {
            'Content-Type': 'application/json',
            'Authorization': 'Bearer ${user.token}',
          },
          body: jsonEncode(body),
        );

        if (res.statusCode == 200) {
          state = state.copyWith(isLoading: false);
          if (context.mounted) {
            context.go('/dashboard');
          }
        } else {
          state = state.copyWith(
            isLoading: false,
            errorMessage: 'Failed to save configuration: ${res.statusCode}',
          );
        }
      } catch (e) {
        state = state.copyWith(
          isLoading: false,
          errorMessage: 'Network error: $e',
        );
      }
    } else {
      state = state.copyWith(isLoading: false);
      if (context.mounted) {
        context.go('/dashboard');
      }
    }
  }
}

class BusinessSetupWizardScreen extends ConsumerStatefulWidget {
  const BusinessSetupWizardScreen({Key? key}) : super(key: key);

  @override
  ConsumerState<BusinessSetupWizardScreen> createState() => _BusinessSetupWizardScreenState();
}

class _BusinessSetupWizardScreenState extends ConsumerState<BusinessSetupWizardScreen> {
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      ref.read(businessSetupProvider.notifier).loadState(ref);
    });
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(businessSetupProvider);
    final notifier = ref.read(businessSetupProvider.notifier);

    Widget currentStepWidget;
    switch (state.step) {
      case 0:
        currentStepWidget = _buildWelcome(notifier);
        break;
      case 1:
        currentStepWidget = _buildBusinessType(state, notifier);
        break;
      case 2:
        currentStepWidget = _buildBusinessName(state, notifier);
        break;
      case 3:
        currentStepWidget = _buildWhatYouSell(state, notifier);
        break;
      case 4:
        currentStepWidget = _buildPayments(state, notifier);
        break;
      case 5:
        currentStepWidget = _buildAdminAccount(state, notifier);
        break;
      case 6:
        currentStepWidget = _buildReviewLaunch(state, notifier);
        break;
      default:
        currentStepWidget = _buildWelcome(notifier);
    }

    return Scaffold(
      backgroundColor: Colors.black,
      body: SafeArea(
        child: Center(
          child: SingleChildScrollView(
            child: Padding(
              padding: const EdgeInsets.all(16.0),
              child: AnimatedSwitcher(
                duration: const Duration(milliseconds: 300),
                child: GlassCard(
                  key: ValueKey<int>(state.step),
                  child: Padding(
                    padding: const EdgeInsets.all(24.0),
                    child: currentStepWidget,
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildWelcome(BusinessSetupNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        const Text(
          'Your business, live in minutes',
          style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 24),
        ElevatedButton(
          onPressed: () => notifier.nextStep(ref),
          child: const Text('Get Started'),
        ),
      ],
    );
  }

  Widget _buildBusinessType(BusinessSetupState state, BusinessSetupNotifier notifier) {
    final types = [
      {'label': 'Online Store', 'icon': Icons.shopping_cart},
      {'label': 'Service Business', 'icon': Icons.build},
      {'label': 'Restaurant / Food', 'icon': Icons.restaurant},
      {'label': 'Creative / Portfolio', 'icon': Icons.brush},
      {'label': 'Local Business', 'icon': Icons.store},
      {'label': 'Other', 'icon': Icons.category}
    ];
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Row(
          children: [
            IconButton(icon: const Icon(Icons.arrow_back, color: Colors.white), onPressed: () => notifier.prevStep(ref)),
            const Expanded(child: Text('What kind of business are you building?', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white))),
          ],
        ),
        const SizedBox(height: 16),
                ...types.map((type) => Padding(
          padding: const EdgeInsets.only(bottom: 8.0),
          child: ListTile(
            leading: Icon((type as Map)['icon'] as IconData, size: 32, color: Colors.blueAccent),
            title: Text(type['label'] as String, style: const TextStyle(color: Colors.white, fontFamily: 'Inter', fontSize: 18)),
            tileColor: Colors.white.withValues(alpha: 0.1),
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
            onTap: () => notifier.updateBusinessType(type['label'] as String, ref),
          ),
        )),

      ],
    );
  }

  Widget _buildBusinessName(BusinessSetupState state, BusinessSetupNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Row(
          children: [
            IconButton(icon: const Icon(Icons.arrow_back, color: Colors.white), onPressed: () => notifier.prevStep(ref)),
            const Expanded(child: Text('Tell us about your business', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white))),
          ],
        ),
        const SizedBox(height: 16),
        TextFormField(
          initialValue: state.companyName,
          onChanged: (v) => notifier.updateCompany(v, ref),
          decoration: const InputDecoration(labelText: 'Business Name', labelStyle: TextStyle(color: Colors.white70)),
          style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
        ),
        const SizedBox(height: 16),
        TextFormField(
          initialValue: state.businessDescription,
          onChanged: (v) => notifier.updateDescription(v, ref),
          decoration: const InputDecoration(labelText: 'Short Description', labelStyle: TextStyle(color: Colors.white70)),
          style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
          maxLines: 3,
        ),
        const SizedBox(height: 24),
        ElevatedButton(
          onPressed: () => notifier.nextStep(ref),
          child: const Text('Continue'),
        ),
      ],
    );
  }

  Widget _buildWhatYouSell(BusinessSetupState state, BusinessSetupNotifier notifier) {
    final options = [
      'Physical products',
      'Digital downloads',
      'Services / appointments',
      'Food & beverages',
      'Subscriptions'
    ];
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Row(
          children: [
            IconButton(icon: const Icon(Icons.arrow_back, color: Colors.white), onPressed: () => notifier.prevStep(ref)),
            const Expanded(child: Text('What do you sell?', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white))),
          ],
        ),
        const SizedBox(height: 16),
        ...options.map((option) => CheckboxListTile(
          title: Text(option, style: const TextStyle(color: Colors.white, fontFamily: 'Inter')),
          value: state.whatYouSell.contains(option),
          onChanged: (_) => notifier.toggleWhatYouSell(option, ref),
          checkColor: Colors.black,
          activeColor: Colors.white,
        )),
        const SizedBox(height: 24),
        ElevatedButton(
          onPressed: () => notifier.nextStep(ref),
          child: const Text('Continue'),
        ),
      ],
    );
  }

  Widget _buildPayments(BusinessSetupState state, BusinessSetupNotifier notifier) {
    final methods = [
      {'label': 'Online only', 'time': 'Est. 2 days to first payment'},
      {'label': 'In-person (POS)', 'time': 'Est. instant access'},
      {'label': 'Both', 'time': 'Varies by method'},
      {'label': 'Skip for now', 'time': ''}
    ];
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Row(
          children: [
            IconButton(icon: const Icon(Icons.arrow_back, color: Colors.white), onPressed: () => notifier.prevStep(ref)),
            const Expanded(child: Text('How do you want to receive payments?', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white))),
          ],
        ),
        const SizedBox(height: 16),
                ...methods.map((method) => Padding(
          padding: const EdgeInsets.only(bottom: 8.0),
          child: ListTile(
            title: Text((method as Map)['label'] as String, style: const TextStyle(color: Colors.white, fontFamily: 'Inter')),
            subtitle: (method['time'] as String).isNotEmpty ? Text(method['time'] as String, style: const TextStyle(color: Colors.white54, fontSize: 12)) : null,
            tileColor: Colors.white.withValues(alpha: 0.1),
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
            onTap: () => notifier.updatePaymentMethod(method['label'] as String, ref),
          ),
        )),

      ],
    );
  }

  Widget _buildAdminAccount(BusinessSetupState state, BusinessSetupNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Row(
          children: [
            IconButton(icon: const Icon(Icons.arrow_back, color: Colors.white), onPressed: () => notifier.prevStep(ref)),
            const Expanded(child: Text('Administrator account', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white))),
          ],
        ),
        const SizedBox(height: 16),
        TextFormField(
          initialValue: state.adminName,
          onChanged: (v) => notifier.updateAdminName(v, ref),
          decoration: const InputDecoration(labelText: 'Name', labelStyle: TextStyle(color: Colors.white70)),
          style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
        ),
        const SizedBox(height: 16),
        TextFormField(
          initialValue: state.adminEmail,
          onChanged: (v) => notifier.updateAdminEmail(v, ref),
          decoration: const InputDecoration(labelText: 'Email', labelStyle: TextStyle(color: Colors.white70)),
          style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
          keyboardType: TextInputType.emailAddress,
        ),
        const SizedBox(height: 16),
        TextFormField(
          initialValue: state.adminPassword,
          onChanged: (v) => notifier.updateAdminPassword(v, ref),
          decoration: const InputDecoration(labelText: 'Password', labelStyle: TextStyle(color: Colors.white70)),
          style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
          obscureText: true,
        ),
        const SizedBox(height: 8),
        LinearProgressIndicator(
          value: state.adminPassword.length > 8 ? 1.0 : (state.adminPassword.length > 4 ? 0.5 : 0.1),
          backgroundColor: Colors.white24,
          color: state.adminPassword.length > 8 ? Colors.green : (state.adminPassword.length > 4 ? Colors.orange : Colors.red),
        ),
        const SizedBox(height: 4),
        Text(
          state.adminPassword.length > 8 ? 'Strong' : (state.adminPassword.length > 4 ? 'Fair' : 'Weak'),
          style: TextStyle(color: state.adminPassword.length > 8 ? Colors.green : (state.adminPassword.length > 4 ? Colors.orange : Colors.red), fontSize: 12),
        ),
        const SizedBox(height: 24),
        ElevatedButton(
          onPressed: () => notifier.nextStep(ref),
          child: const Text('Continue'),
        ),
      ],
    );
  }

  Widget _buildReviewLaunch(BusinessSetupState state, BusinessSetupNotifier notifier) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Row(
          children: [
            IconButton(icon: const Icon(Icons.arrow_back, color: Colors.white), onPressed: () => notifier.prevStep(ref)),
            const Expanded(child: Text('Review & Launch', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white))),
          ],
        ),
        const SizedBox(height: 16),
        Text('Business: ${state.companyName}', style: const TextStyle(color: Colors.white, fontFamily: 'Inter')),
        Text('Type: ${state.businessType}', style: const TextStyle(color: Colors.white, fontFamily: 'Inter')),
        const SizedBox(height: 24),
        if (state.errorMessage != null)
          Text(state.errorMessage!, style: const TextStyle(color: Colors.red)),
        ElevatedButton(
          onPressed: state.isLoading ? null : () => notifier.launch(context, ref),
          child: state.isLoading ? const CircularProgressIndicator() : const Text('Launch My Business →'),
        ),
      ],
    );
  }
}
