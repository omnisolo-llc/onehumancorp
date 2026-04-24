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
}

class BusinessSetupNotifier extends Notifier<BusinessSetupState> {
  @override
  BusinessSetupState build() {
    _loadDraft();
    return const BusinessSetupState();
  }

  Future<void> _loadDraft() async {
    try {
      final user = ref.read(authStateProvider).valueOrNull;
      final baseUrl = ref.read(backendUrlProvider);
      if (user != null && baseUrl.isNotEmpty) {
        final res = await http.get(
          Uri.parse('$baseUrl/api/wizard/draft'),
          headers: {'Authorization': 'Bearer ${user.token}'},
        );
        if (res.statusCode == 200 && res.body.isNotEmpty && res.body != '{}') {
          final data = jsonDecode(res.body);
          state = state.copyWith(
            step: data['step'] as int?,
            businessType: data['businessType'] as String?,
            companyName: data['companyName'] as String?,
            businessDescription: data['businessDescription'] as String?,
            whatYouSell: (data['whatYouSell'] as List<dynamic>?)?.map((e) => e as String).toList(),
            paymentMethod: data['paymentMethod'] as String?,
            adminName: data['adminName'] as String?,
            adminEmail: data['adminEmail'] as String?,
          );
        }
      }
    } catch (e) {
      // Ignore draft loading errors
    }
  }

  Future<void> _saveDraft() async {
    try {
      final user = ref.read(authStateProvider).valueOrNull;
      final baseUrl = ref.read(backendUrlProvider);
      if (user != null && baseUrl.isNotEmpty) {
        final body = jsonEncode({
          'step': state.step,
          'businessType': state.businessType,
          'companyName': state.companyName,
          'businessDescription': state.businessDescription,
          'whatYouSell': state.whatYouSell,
          'paymentMethod': state.paymentMethod,
          'adminName': state.adminName,
          'adminEmail': state.adminEmail,
        });
        await http.post(
          Uri.parse('$baseUrl/api/wizard/draft'),
          headers: {
            'Authorization': 'Bearer ${user.token}',
            'Content-Type': 'application/json',
          },
          body: body,
        );
      }
    } catch (e) {
      // Ignore draft saving errors
    }
  }

  void nextStep() {
    if (state.step < 6) {
      state = state.copyWith(step: state.step + 1);
      _saveDraft();
    }
  }

  void prevStep() {
    if (state.step > 0) {
      state = state.copyWith(step: state.step - 1);
      _saveDraft();
    }
  }

  void updateBusinessType(String type) {
    state = state.copyWith(businessType: type);
    _saveDraft();
    nextStep();
  }

  void updateCompany(String name) { state = state.copyWith(companyName: name); _saveDraft(); }
  void updateDescription(String desc) { state = state.copyWith(businessDescription: desc); _saveDraft(); }

  void toggleWhatYouSell(String item) {
    final list = List<String>.from(state.whatYouSell);
    if (list.contains(item)) {
      list.remove(item);
    } else {
      list.add(item);
    }
    state = state.copyWith(whatYouSell: list);
    _saveDraft();
  }

  void updatePaymentMethod(String method) { state = state.copyWith(paymentMethod: method); _saveDraft(); }

  void updateAdminName(String name) { state = state.copyWith(adminName: name); _saveDraft(); }
  void updateAdminEmail(String val) { state = state.copyWith(adminEmail: val); _saveDraft(); }
  void updateAdminPassword(String val) { state = state.copyWith(adminPassword: val); _saveDraft(); }

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
          'what_you_sell': state.whatYouSell.join(','),
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
        } else {
          state = state.copyWith(isLoading: false);
          if (context.mounted) {
            context.go('/dashboard');
          }
          // Clear the draft on success
          try {
             await http.post(
                Uri.parse('$baseUrl/api/wizard/draft'),
                headers: {
                  'Authorization': 'Bearer ${user.token}',
                  'Content-Type': 'application/json',
                },
                body: '{}',
             );
          } catch (_) {}
        }
      } catch (e) {
        state = state.copyWith(isLoading: false, errorMessage: e.toString());
      }
    } else {
      state = state.copyWith(isLoading: false, errorMessage: 'Not authenticated');
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

  Widget _buildStep(BusinessSetupState state, BusinessSetupNotifier notifier) {
    if (state.step == 0) {
      return const Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(Icons.rocket_launch, size: 64, color: Colors.blueAccent),
          SizedBox(height: 24),
          Text(
            'Your business, live in minutes',
            style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white),
            textAlign: TextAlign.center,
          ),
          SizedBox(height: 16),
          Text(
            'No coding required. AI agents will set up your entire backend.',
            style: TextStyle(fontFamily: 'Inter', fontSize: 16, color: Colors.white70),
            textAlign: TextAlign.center,
          ),
        ],
      );
    } else if (state.step == 1) {
      final types = [
        {'label': 'Online Store', 'icon': Icons.shopping_bag},
        {'label': 'Service Business', 'icon': Icons.build},
        {'label': 'Restaurant / Food', 'icon': Icons.restaurant},
        {'label': 'Creative / Portfolio', 'icon': Icons.brush},
        {'label': 'Local Business', 'icon': Icons.store},
        {'label': 'Other', 'icon': Icons.category},
      ];
      return Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Text('What kind of business are you building?', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, fontSize: 18, color: Colors.white)),
          const SizedBox(height: 16),
          Wrap(
            spacing: 16,
            runSpacing: 16,
            children: types.map((t) {
              final isSelected = state.businessType == t['label'];
              return InkWell(
                onTap: () => notifier.updateBusinessType(t['label'] as String),
                child: Container(
                  width: 140,
                  height: 120,
                  decoration: BoxDecoration(
                    color: isSelected ? Colors.blueAccent.withAlpha(51) : Colors.white.withAlpha(13),
                    border: Border.all(color: isSelected ? Colors.blueAccent : Colors.white.withAlpha(26)),
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(t['icon'] as IconData, size: 32, color: isSelected ? Colors.blueAccent : Colors.white70),
                      const SizedBox(height: 12),
                      Text(t['label'] as String, textAlign: TextAlign.center, style: TextStyle(fontFamily: 'Inter', color: isSelected ? Colors.white : Colors.white70, fontWeight: isSelected ? FontWeight.bold : FontWeight.normal)),
                    ],
                  ),
                ),
              );
            }).toList(),
          ),
        ],
      );
    } else if (state.step == 2) {
      return Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Text('Tell us about your business', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, fontSize: 18, color: Colors.white)),
          const SizedBox(height: 16),
          TextField(
            decoration: const InputDecoration(labelText: 'Business Name', labelStyle: TextStyle(color: Colors.white70)),
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
        ],
      );
    } else if (state.step == 3) {
      final items = [
        'Physical products',
        'Digital downloads',
        'Services / appointments',
        'Food & beverages',
        'Subscriptions',
      ];
      return Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Text('What do you sell?', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, fontSize: 18, color: Colors.white)),
          const SizedBox(height: 16),
          ...items.map((item) {
            final isSelected = state.whatYouSell.contains(item);
            return Padding(
              padding: const EdgeInsets.only(bottom: 12.0),
              child: InkWell(
                onTap: () => notifier.toggleWhatYouSell(item),
                child: Container(
                  padding: const EdgeInsets.all(16),
                  decoration: BoxDecoration(
                    color: isSelected ? Colors.blueAccent.withAlpha(51) : Colors.white.withAlpha(13),
                    border: Border.all(color: isSelected ? Colors.blueAccent : Colors.white.withAlpha(26)),
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: Row(
                    children: [
                      Icon(isSelected ? Icons.check_circle : Icons.radio_button_unchecked, color: isSelected ? Colors.blueAccent : Colors.white70),
                      const SizedBox(width: 16),
                      Text(item, style: TextStyle(fontFamily: 'Inter', color: isSelected ? Colors.white : Colors.white70, fontSize: 16)),
                    ],
                  ),
                ),
              ),
            );
          }),
        ],
      );
    } else if (state.step == 4) {
      final items = [
        {'label': 'Online only', 'time': 'Receive in 2 days'},
        {'label': 'In-person (POS)', 'time': 'Instant'},
        {'label': 'Both', 'time': 'Varies'},
        {'label': 'Skip for now', 'time': ''},
      ];
      return Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Text('How do you want to receive payments?', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, fontSize: 18, color: Colors.white)),
          const SizedBox(height: 16),
          ...items.map((item) {
            final isSelected = state.paymentMethod == item['label'];
            return Padding(
              padding: const EdgeInsets.only(bottom: 12.0),
              child: InkWell(
                onTap: () => notifier.updatePaymentMethod(item['label'] as String),
                child: Container(
                  padding: const EdgeInsets.all(16),
                  decoration: BoxDecoration(
                    color: isSelected ? Colors.blueAccent.withAlpha(51) : Colors.white.withAlpha(13),
                    border: Border.all(color: isSelected ? Colors.blueAccent : Colors.white.withAlpha(26)),
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: Row(
                    children: [
                      Expanded(
                        child: Text(item['label'] as String, style: TextStyle(fontFamily: 'Inter', color: isSelected ? Colors.white : Colors.white70, fontSize: 16, fontWeight: isSelected ? FontWeight.bold : FontWeight.normal)),
                      ),
                      if ((item['time'] as String).isNotEmpty)
                        Text(item['time'] as String, style: const TextStyle(fontFamily: 'Inter', color: Colors.greenAccent, fontSize: 12)),
                    ],
                  ),
                ),
              ),
            );
          }),
        ],
      );
    } else if (state.step == 5) {
      return Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Text('Administrator account', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, fontSize: 18, color: Colors.white)),
          const SizedBox(height: 16),
          TextField(
            decoration: const InputDecoration(labelText: 'Name', labelStyle: TextStyle(color: Colors.white70)),
            onChanged: notifier.updateAdminName,
            style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
          ),
          const SizedBox(height: 16),
          TextField(
            keyboardType: TextInputType.emailAddress,
            decoration: const InputDecoration(labelText: 'Email', labelStyle: TextStyle(color: Colors.white70)),
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
          if (state.adminPassword.isNotEmpty)
             Padding(
               padding: const EdgeInsets.only(top: 8.0),
               child: LinearProgressIndicator(
                 value: state.adminPassword.length / 10.0 > 1.0 ? 1.0 : state.adminPassword.length / 10.0,
                 backgroundColor: Colors.white.withAlpha(26),
                 color: state.adminPassword.length > 6 ? Colors.green : Colors.orange,
               ),
             ),
        ],
      );
    } else {
      return Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Text('Review & Launch', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold, fontSize: 24, color: Colors.white)),
          const SizedBox(height: 24),
          _buildSummaryRow('Business Name', state.companyName),
          _buildSummaryRow('Type', state.businessType),
          _buildSummaryRow('Selling', state.whatYouSell.join(', ')),
          _buildSummaryRow('Payments', state.paymentMethod),
          _buildSummaryRow('Admin Email', state.adminEmail),
        ],
      );
    }
  }

  Widget _buildSummaryRow(String label, String value) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 12.0),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            width: 120,
            child: Text(label, style: const TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 14)),
          ),
          Expanded(
            child: Text(value.isEmpty ? 'Not set' : value, style: const TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 14, fontWeight: FontWeight.w500)),
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(businessSetupProvider);
    final notifier = ref.read(businessSetupProvider.notifier);

    return Scaffold(
      backgroundColor: const Color(0xFF0D0D1A),
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
                        child: _buildStep(state, notifier),
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
                          onPressed: state.isLoading ? null : () {
                            if (state.step < 6) {
                              notifier.nextStep();
                            } else {
                              notifier.launch(context, ref);
                            }
                          },
                          style: ElevatedButton.styleFrom(
                            backgroundColor: state.step == 6 ? Colors.green : Colors.blueAccent,
                            padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
                            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                          ),
                          child: state.isLoading
                              ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white))
                              : Text(
                                  state.step == 6 ? 'Launch My Business →' : (state.step == 0 ? 'Get Started' : 'Continue'),
                                  style: const TextStyle(fontFamily: 'Inter', fontSize: 16, fontWeight: FontWeight.bold, color: Colors.white),
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
