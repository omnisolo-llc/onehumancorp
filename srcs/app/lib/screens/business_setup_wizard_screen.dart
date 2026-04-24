import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/settings_service.dart';
import 'package:ohc_app/widgets/glass_card.dart';

// Update state to include new fields
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

  // New fields
  final String selectedTemplate;
  final String firstProductName;
  final String firstProductPrice;
  final String subdomain;

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
    this.selectedTemplate = '',
    this.firstProductName = '',
    this.firstProductPrice = '',
    this.subdomain = '',
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
    String? selectedTemplate,
    String? firstProductName,
    String? firstProductPrice,
    String? subdomain,
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
      selectedTemplate: selectedTemplate ?? this.selectedTemplate,
      firstProductName: firstProductName ?? this.firstProductName,
      firstProductPrice: firstProductPrice ?? this.firstProductPrice,
      subdomain: subdomain ?? this.subdomain,
      isLoading: isLoading ?? this.isLoading,
      errorMessage: errorMessage ?? this.errorMessage,
      obscurePassword: obscurePassword ?? this.obscurePassword,
    );
  }
}

class BusinessSetupNotifier extends Notifier<BusinessSetupState> {
  @override
  BusinessSetupState build() {
    _loadState();
    return const BusinessSetupState();
  }

  Future<void> _loadState() async {
    try {
      final user = ref.read(authStateProvider).valueOrNull;
      final baseUrl = ref.read(backendUrlProvider);
      if (user == null || baseUrl.isEmpty) return;

      final res = await http.get(
        Uri.parse('$baseUrl/api/wizard/state/load'),
        headers: {
          'Authorization': 'Bearer ${user.token}',
        },
      );

      if (res.statusCode == 200) {
        final data = jsonDecode(res.body);
        state = state.copyWith(
          step: data['step'] ?? 0,
          businessType: data['businessType'] ?? '',
          companyName: data['companyName'] ?? '',
          description: data['description'] ?? '',
          whatYouSell: (data['whatYouSell'] as List<dynamic>?)?.map((e) => e.toString()).toList() ?? [],
          payments: data['payments'] ?? '',
          adminName: data['adminName'] ?? '',
          adminEmail: data['adminEmail'] ?? '',
          selectedTemplate: data['selectedTemplate'] ?? '',
          firstProductName: data['firstProductName'] ?? '',
          firstProductPrice: data['firstProductPrice'] ?? '',
          subdomain: data['subdomain'] ?? '',
        );
      }
    } catch (e) {
      // Ignore load error
    }
  }

  Future<void> _saveState() async {
    try {
      final user = ref.read(authStateProvider).valueOrNull;
      final baseUrl = ref.read(backendUrlProvider);
      if (user == null || baseUrl.isEmpty) return;

      await http.post(
        Uri.parse('$baseUrl/api/wizard/state/save'),
        headers: {
          'Authorization': 'Bearer ${user.token}',
          'Content-Type': 'application/json',
        },
        body: jsonEncode({
          'step': state.step,
          'businessType': state.businessType,
          'companyName': state.companyName,
          'description': state.description,
          'whatYouSell': state.whatYouSell,
          'payments': state.payments,
          'adminName': state.adminName,
          'adminEmail': state.adminEmail,
          'selectedTemplate': state.selectedTemplate,
          'firstProductName': state.firstProductName,
          'firstProductPrice': state.firstProductPrice,
          'subdomain': state.subdomain,
        }),
      );
    } catch (e) {
      // Ignore save error
    }
  }

  void nextStep() {
    if (state.step < 5) {
      state = state.copyWith(step: state.step + 1);
      _saveState();
    }
  }

  void prevStep() {
    if (state.step > 0) {
      state = state.copyWith(step: state.step - 1);
      _saveState();
    }
  }

  void updateBusinessType(String val) {
    state = state.copyWith(businessType: val);
    _saveState();
  }
  void updateCompany(String name) {
    // Auto-generate subdomain from company name
    final generatedSubdomain = name.toLowerCase().replaceAll(RegExp(r'[^a-z0-9]'), '') + '.ohc.app';
    state = state.copyWith(companyName: name, subdomain: generatedSubdomain);
    _saveState();
  }
  void updateDescription(String val) {
    state = state.copyWith(description: val);
    _saveState();
  }
  void toggleWhatYouSell(String val) {
    final list = List<String>.from(state.whatYouSell);
    if (list.contains(val)) list.remove(val);
    else list.add(val);
    state = state.copyWith(whatYouSell: list);
    _saveState();
  }
  void updatePayments(String val) {
    state = state.copyWith(payments: val);
    _saveState();
  }

  void updateTemplate(String val) {
    state = state.copyWith(selectedTemplate: val);
    _saveState();
  }
  void updateFirstProductName(String val) {
    state = state.copyWith(firstProductName: val);
    _saveState();
  }
  void updateFirstProductPrice(String val) {
    state = state.copyWith(firstProductPrice: val);
    _saveState();
  }
  void updateSubdomain(String val) {
    state = state.copyWith(subdomain: val);
    _saveState();
  }

  void updateAdminName(String name) {
    state = state.copyWith(adminName: name);
    _saveState();
  }
  void updateAdminEmail(String val) {
    state = state.copyWith(adminEmail: val);
    _saveState();
  }
  void updateAdminPassword(String val) {
    state = state.copyWith(adminPassword: val);
    _saveState();
  }
  void toggleObscurePassword() {
    state = state.copyWith(obscurePassword: !state.obscurePassword);
    _saveState();
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
          'description': state.description,
          'what_you_sell': state.whatYouSell.join(','),
          'payments': state.payments,
          'admin_name': state.adminName,
          'admin_email': state.adminEmail,
          'template': state.selectedTemplate,
          'first_product_name': state.firstProductName,
          'first_product_price': state.firstProductPrice,
          'subdomain': state.subdomain,
          'checklist_pending': 'true',
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

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(businessSetupProvider);
    final notifier = ref.read(businessSetupProvider.notifier);

    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      body: Stack(
        children: [
          // Background blobs
          Positioned(top: -100, left: -100, child: _buildBlob(Colors.indigo.withOpacity(0.3))),
          Positioned(bottom: -100, right: -100, child: _buildBlob(Colors.purple.withOpacity(0.3))),

          SafeArea(
            child: Center(
              child: ConstrainedBox(
                constraints: const BoxConstraints(maxWidth: 480),
                child: Padding(
                  padding: const EdgeInsets.all(24.0),
                  child: Column(
                    children: [
                      // Header
                      Row(
                        children: [
                          if (state.step > 0)
                            IconButton(
                              icon: const Icon(Icons.arrow_back, color: Colors.white70),
                              onPressed: notifier.prevStep,
                            ),
                          const Expanded(
                            child: Text(
                              'Business Setup',
                              textAlign: TextAlign.center,
                              style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white),
                            ),
                          ),
                          if (state.step > 0) const SizedBox(width: 48), // balance back button
                        ],
                      ),
                      const SizedBox(height: 16),

                      // Progress bar
                      LinearProgressIndicator(
                        value: (state.step + 1) / 6, // 6 total steps
                        backgroundColor: Colors.white12,
                        color: Colors.indigoAccent,
                        minHeight: 4,
                      ),
                      const SizedBox(height: 32),

                      Expanded(
                        child: AnimatedSwitcher(
                          duration: const Duration(milliseconds: 300),
                          child: SingleChildScrollView(
                            key: ValueKey(state.step),
                            child: _buildCurrentStep(state, notifier),
                          ),
                        ),
                      ),

                      // Footer actions
                      const SizedBox(height: 24),
                      SizedBox(
                        width: double.infinity,
                        height: 56,
                        child: ElevatedButton(
                          style: ElevatedButton.styleFrom(
                            backgroundColor: Colors.indigo,
                            foregroundColor: Colors.white,
                            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                          ),
                          onPressed: state.isLoading ? null : () {
                            if (state.step < 5) {
                              notifier.nextStep();
                            } else {
                              notifier.launch(context, ref);
                            }
                          },
                          child: state.isLoading
                              ? const SizedBox(width: 24, height: 24, child: CircularProgressIndicator(color: Colors.white, strokeWidth: 2))
                              : Text(
                                  state.step == 5 ? 'Publish Business 🎉' : 'Next',
                                  style: const TextStyle(fontFamily: 'Inter', fontSize: 18, fontWeight: FontWeight.w600),
                                ),
                        ),
                      ),
                      if (state.errorMessage != null) ...[
                        const SizedBox(height: 16),
                        Text(state.errorMessage!, style: const TextStyle(color: Colors.redAccent, fontFamily: 'Inter')),
                      ],
                    ],
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildBlob(Color color) {
    return Container(
      width: 300,
      height: 300,
      decoration: BoxDecoration(shape: BoxShape.circle, color: color),
    );
  }

  Widget _buildCurrentStep(BusinessSetupState state, BusinessSetupNotifier notifier) {
    switch (state.step) {
      case 0: return _buildStep0_Idea(state, notifier);
      case 1: return _buildStep1_Template(state, notifier);
      case 2: return _buildStep2_FirstProduct(state, notifier);
      case 3: return _buildStep3_Domain(state, notifier);
      case 4: return _buildStep4_Admin(state, notifier);
      case 5: return _buildStep5_Review(state);
      default: return const SizedBox();
    }
  }

  Widget _buildGlassInput(String label, String value, Function(String) onChanged, {int maxLines = 1, bool obscure = false}) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(label, style: const TextStyle(color: Colors.white70, fontFamily: 'Inter')),
        const SizedBox(height: 8),
        TextFormField(
          initialValue: value,
          onChanged: onChanged,
          maxLines: maxLines,
          obscureText: obscure,
          style: const TextStyle(color: Colors.white),
          decoration: InputDecoration(
            filled: true,
            fillColor: Colors.white.withOpacity(0.1),
            border: OutlineInputBorder(
              borderRadius: BorderRadius.circular(12),
              borderSide: BorderSide.none,
            ),
          ),
        ),
        const SizedBox(height: 16),
      ],
    );
  }

  Widget _buildStep0_Idea(BusinessSetupState state, BusinessSetupNotifier notifier) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('What do you do?', style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 24, fontWeight: FontWeight.bold)),
        const SizedBox(height: 8),
        const Text('Tell us about your business idea.', style: TextStyle(color: Colors.white54, fontFamily: 'Inter')),
        const SizedBox(height: 24),
        _buildGlassInput('Business Name', state.companyName, notifier.updateCompany),
        _buildGlassInput('Description (e.g. "I bake custom vegan cakes")', state.description, notifier.updateDescription, maxLines: 3),
      ],
    );
  }

  Widget _buildStep1_Template(BusinessSetupState state, BusinessSetupNotifier notifier) {
    final templates = ['Modern Minimal', 'Vibrant Boutique', 'Service Pro', 'Creative Portfolio'];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Choose a Template', style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 24, fontWeight: FontWeight.bold)),
        const SizedBox(height: 8),
        const Text('Pick a style that fits your brand. You can change it later.', style: TextStyle(color: Colors.white54, fontFamily: 'Inter')),
        const SizedBox(height: 24),
        ...templates.map((t) => Padding(
          padding: const EdgeInsets.only(bottom: 12),
          child: InkWell(
            onTap: () => notifier.updateTemplate(t),
            borderRadius: BorderRadius.circular(12),
            child: Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: state.selectedTemplate == t ? Colors.indigo.withOpacity(0.3) : Colors.white.withOpacity(0.05),
                border: Border.all(color: state.selectedTemplate == t ? Colors.indigoAccent : Colors.transparent),
                borderRadius: BorderRadius.circular(12),
              ),
              child: Row(
                children: [
                  Container(
                    width: 40, height: 40,
                    decoration: BoxDecoration(
                      color: Colors.white12,
                      borderRadius: BorderRadius.circular(8),
                    ),
                    child: const Icon(Icons.web, color: Colors.white),
                  ),
                  const SizedBox(width: 16),
                  Text(t, style: const TextStyle(color: Colors.white, fontFamily: 'Inter', fontSize: 16)),
                  const Spacer(),
                  if (state.selectedTemplate == t)
                    const Icon(Icons.check_circle, color: Colors.indigoAccent)
                ],
              ),
            ),
          ),
        )),
      ],
    );
  }

  Widget _buildStep2_FirstProduct(BusinessSetupState state, BusinessSetupNotifier notifier) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Add your first product or service', style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 24, fontWeight: FontWeight.bold)),
        const SizedBox(height: 8),
        const Text('Let\'s get something on your storefront.', style: TextStyle(color: Colors.white54, fontFamily: 'Inter')),
        const SizedBox(height: 24),
        _buildGlassInput('Item Name (e.g. "Chocolate Cake")', state.firstProductName, notifier.updateFirstProductName),
        _buildGlassInput('Price', state.firstProductPrice, notifier.updateFirstProductPrice),
      ],
    );
  }

  Widget _buildStep3_Domain(BusinessSetupState state, BusinessSetupNotifier notifier) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Your unique link', style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 24, fontWeight: FontWeight.bold)),
        const SizedBox(height: 8),
        const Text('Customers will use this link to visit your business.', style: TextStyle(color: Colors.white54, fontFamily: 'Inter')),
        const SizedBox(height: 24),
        _buildGlassInput('Storefront Link', state.subdomain, notifier.updateSubdomain),
      ],
    );
  }

  Widget _buildStep4_Admin(BusinessSetupState state, BusinessSetupNotifier notifier) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Create Admin Account', style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 24, fontWeight: FontWeight.bold)),
        const SizedBox(height: 8),
        const Text('You will use this to log into the dashboard.', style: TextStyle(color: Colors.white54, fontFamily: 'Inter')),
        const SizedBox(height: 24),
        _buildGlassInput('Your Name', state.adminName, notifier.updateAdminName),
        _buildGlassInput('Email', state.adminEmail, notifier.updateAdminEmail),
        _buildGlassInput('Password', state.adminPassword, notifier.updateAdminPassword, obscure: state.obscurePassword),
      ],
    );
  }

  Widget _buildStep5_Review(BusinessSetupState state) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Ready to launch!', style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 24, fontWeight: FontWeight.bold)),
        const SizedBox(height: 8),
        const Text('Here is what your AI Marketing Department will build.', style: TextStyle(color: Colors.white54, fontFamily: 'Inter')),
        const SizedBox(height: 24),
        GlassCard(
          color: Colors.white.withOpacity(0.05),
          child: Padding(
            padding: const EdgeInsets.all(16.0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                _buildSummaryRow('Business', state.companyName),
                _buildSummaryRow('Template', state.selectedTemplate.isEmpty ? 'Modern Minimal' : state.selectedTemplate),
                _buildSummaryRow('First Item', '${state.firstProductName} - \$${state.firstProductPrice}'),
                _buildSummaryRow('Link', state.subdomain),
              ],
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildSummaryRow(String label, String val) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(width: 100, child: Text(label, style: const TextStyle(color: Colors.white54, fontFamily: 'Inter'))),
          Expanded(child: Text(val.isEmpty ? 'N/A' : val, style: const TextStyle(color: Colors.white, fontFamily: 'Inter', fontWeight: FontWeight.w500))),
        ],
      ),
    );
  }
}
