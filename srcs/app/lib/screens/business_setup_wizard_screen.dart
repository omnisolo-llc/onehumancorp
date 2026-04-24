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
  final bool obscurePassword;
  final int step;
  final String companyName;
  final String industry;
  final String size;
  final List<String> goals;
  final String deployment;
  final String adminName;
  final String adminEmail;
  final String adminPassword;
  final String productName;
  final String productDesc;
  final String productPrice;
  final String domainName;
  final bool isLoading;
  final String? errorMessage;

      const BusinessSetupState({
    this.step = 0,
    this.companyName = '',
    this.industry = '',
    this.size = 'S',
    this.goals = const [],
    this.deployment = 'Cloud',
    this.adminName = '',
    this.adminEmail = '',
    this.adminPassword = '',
    this.productName = '',
    this.productDesc = '',
    this.productPrice = '',
    this.domainName = '',
    this.isLoading = false,
    this.errorMessage,
    this.obscurePassword = true,
  });

  BusinessSetupState copyWith({
    int? step,
    String? companyName,
    String? industry,
    String? size,
    List<String>? goals,
    String? deployment,
    String? adminName,
    String? adminEmail,
    String? adminPassword,
    String? productName,
    String? productDesc,
    String? productPrice,
    String? domainName,
    bool? isLoading,
    String? errorMessage,
    bool? obscurePassword,
  }) {
    return BusinessSetupState(
      step: step ?? this.step,
      companyName: companyName ?? this.companyName,
      industry: industry ?? this.industry,
      size: size ?? this.size,
      goals: goals ?? this.goals,
      deployment: deployment ?? this.deployment,
      adminName: adminName ?? this.adminName,
      adminEmail: adminEmail ?? this.adminEmail,
      adminPassword: adminPassword ?? this.adminPassword,
      productName: productName ?? this.productName,
      productDesc: productDesc ?? this.productDesc,
      productPrice: productPrice ?? this.productPrice,
      domainName: domainName ?? this.domainName,
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

  void updateCompany(String name) => state = state.copyWith(companyName: name);
  void updateIndustry(String val) => state = state.copyWith(industry: val);
  void updateSize(String val) => state = state.copyWith(size: val);
  void toggleGoal(String goal) {
    final goals = List<String>.from(state.goals);
    if (goals.contains(goal)) {
      goals.remove(goal);
    } else {
      goals.add(goal);
    }
    state = state.copyWith(goals: goals);
  }
  void updateDeployment(String val) => state = state.copyWith(deployment: val);
  void updateAdminName(String name) => state = state.copyWith(adminName: name);
  void updateAdminEmail(String val) => state = state.copyWith(adminEmail: val);
  void updateAdminPassword(String val) => state = state.copyWith(adminPassword: val);
  void toggleObscurePassword() => state = state.copyWith(obscurePassword: !state.obscurePassword);

  void updateProductName(String val) => state = state.copyWith(productName: val);
  void updateProductDesc(String val) => state = state.copyWith(productDesc: val);
  void updateProductPrice(String val) => state = state.copyWith(productPrice: val);
  void updateDomainName(String val) => state = state.copyWith(domainName: val);

  Future<void> generateProductDesc() async {
    // Mock AI generation
    state = state.copyWith(isLoading: true);
    await Future.delayed(const Duration(milliseconds: 800));
    final prompt = state.productName;
    final desc = "A high-quality, premium ${prompt.isNotEmpty ? prompt : 'product'} crafted with care to deliver exceptional results and satisfaction.";
    state = state.copyWith(productDesc: desc, isLoading: false);
  }


  Future<void> launch(BuildContext context, WidgetRef ref) async {
    final user = ref.read(authStateProvider).valueOrNull;
    final baseUrl = ref.read(backendUrlProvider);

    state = state.copyWith(isLoading: true, errorMessage: null);

    if (user != null && baseUrl.isNotEmpty) {
      final body = {
        'extras': {
          'company_name': state.companyName,
          'industry': state.industry,
          'company_size': state.size,
          'goals': state.goals.join(','),
          'deployment_preference': state.deployment,
          'admin_name': state.adminName,
          'admin_email': state.adminEmail,
          'product_name': state.productName,
          'product_desc': state.productDesc,
          'product_price': state.productPrice,
          'domain_name': state.domainName,
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

class BusinessSetupWizardScreen extends ConsumerWidget {
  const BusinessSetupWizardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
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
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          if (state.step == 0) ...[
                            const Text('Welcome! Your AI team, ready in minutes.', style: TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 16)),
                          ] else if (state.step == 1) ...[
                            TextField(
                              decoration: const InputDecoration(labelText: 'Company Name', labelStyle: TextStyle(color: Colors.white70)),
                              onChanged: notifier.updateCompany,
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                            ),
                            const SizedBox(height: 16),
                            TextField(
                              decoration: const InputDecoration(labelText: 'Industry', labelStyle: TextStyle(color: Colors.white70)),
                              onChanged: notifier.updateIndustry,
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                            ),
                            const SizedBox(height: 16),
                            DropdownButtonFormField<String>(
                              value: state.size,
                              decoration: const InputDecoration(labelText: 'Size', labelStyle: TextStyle(color: Colors.white70)),
                              dropdownColor: const Color(0xFF1A1A33),
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                              items: const [
                                DropdownMenuItem(value: 'S', child: Text('Small')),
                                DropdownMenuItem(value: 'M', child: Text('Medium')),
                                DropdownMenuItem(value: 'L', child: Text('Large')),
                              ],
                              onChanged: (val) {
                                if (val != null) notifier.updateSize(val);
                              },
                            ),
                          ] else if (state.step == 2) ...[
                             const Text('Select Goals', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
                             ...['Support', 'Build software', 'Marketing', 'Data', 'Custom'].map((goal) => CheckboxListTile(
                              title: Text(goal, style: const TextStyle(fontFamily: 'Inter', color: Colors.white)),
                              value: state.goals.contains(goal),
                              checkColor: Colors.black,
                              activeColor: Colors.white,
                              onChanged: (bool? value) {
                                notifier.toggleGoal(goal);
                              },
                            )),
                          ] else if (state.step == 3) ...[
                             const Text('Deployment Preference', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
                             if (isStandalone)
                               Padding(
                                 padding: const EdgeInsets.only(top: 16.0),
                                 child: ClipRRect(
                                   borderRadius: BorderRadius.circular(12),
                                   child: BackdropFilter(
                                     filter: ImageFilter.compose(outer: const ColorFilter.matrix(<double>[1.168, -0.153, -0.015, 0, 0, -0.046, 1.061, -0.015, 0, 0, -0.046, -0.152, 1.198, 0, 0, 0, 0, 0, 1, 0]), inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0)),
                                     child: Container(
                                       padding: const EdgeInsets.all(16),
                                       decoration: BoxDecoration(
                                         color: Colors.white.withOpacity(0.05),
                                         border: Border.all(color: Colors.white.withOpacity(0.1)),
                                       ),
                                       child: const Text(
                                         'Standalone Mode Detected. Multi-tenant cloud databases and Redis configurations bypassed for local execution.',
                                         style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 16),
                                       ),
                                     ),
                                   ),
                                 ),
                               )
                             else
                               ...['Cloud', 'Desktop', 'Mobile-only'].map((dep) => RadioListTile<String>(
                                title: Text(dep, style: const TextStyle(fontFamily: 'Inter', color: Colors.white)),
                                value: dep,
                                groupValue: state.deployment,
                                activeColor: Colors.blueAccent,
                                onChanged: (String? value) {
                                  if (value != null) notifier.updateDeployment(value);
                                },
                              )),
                          ] else if (state.step == 4) ...[
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
                                  onPressed: () {
                                    notifier.toggleObscurePassword();
                                  },
                                ),
                              ),
                            ),

                          ] else if (state.step == 5) ...[
                            const Text('First Product / Service', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white, fontSize: 18)),
                            const SizedBox(height: 16),
                            TextField(
                              decoration: const InputDecoration(labelText: 'Product Name', labelStyle: TextStyle(color: Colors.white70)),
                              onChanged: notifier.updateProductName,
                              controller: TextEditingController.fromValue(TextEditingValue(text: state.productName, selection: TextSelection.collapsed(offset: state.productName.length))),
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                            ),
                            const SizedBox(height: 16),
                            ElevatedButton.icon(
                              onPressed: state.productName.isNotEmpty && !state.isLoading ? notifier.generateProductDesc : null,
                              icon: const Icon(Icons.auto_awesome, size: 16),
                              label: const Text('AI Auto-Generate Description'),
                            ),
                            const SizedBox(height: 16),
                            TextField(
                              decoration: const InputDecoration(labelText: 'Description', labelStyle: TextStyle(color: Colors.white70)),
                              onChanged: notifier.updateProductDesc,
                              maxLines: 3,
                              controller: TextEditingController.fromValue(TextEditingValue(text: state.productDesc, selection: TextSelection.collapsed(offset: state.productDesc.length))),
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                            ),
                            const SizedBox(height: 16),
                            TextField(
                              decoration: const InputDecoration(labelText: 'Price', labelStyle: TextStyle(color: Colors.white70), prefixText: '\$'),
                              keyboardType: TextInputType.numberWithOptions(decimal: true),
                              onChanged: notifier.updateProductPrice,
                              controller: TextEditingController.fromValue(TextEditingValue(text: state.productPrice, selection: TextSelection.collapsed(offset: state.productPrice.length))),
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                            ),
                          ] else if (state.step == 6) ...[
                            const Text('Domain & Go-Live', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white, fontSize: 18)),
                            const SizedBox(height: 16),
                            TextField(
                              decoration: const InputDecoration(labelText: 'Choose your free subdomain', labelStyle: TextStyle(color: Colors.white70), suffixText: '.ohc.app', suffixStyle: TextStyle(color: Colors.white54)),
                              onChanged: notifier.updateDomainName,
                              controller: TextEditingController.fromValue(TextEditingValue(text: state.domainName, selection: TextSelection.collapsed(offset: state.domainName.length))),
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
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
                          onPressed: state.isLoading ? null : notifier.prevStep,
                          child: const Text('Back', style: TextStyle(fontFamily: 'Inter')),
                        )
                      else
                        const SizedBox(),
                      state.step == 6 ? PulseAnimation(child: ElevatedButton(
                        onPressed: state.isLoading ? null : () {
                          if (state.step < 6) {
                            notifier.nextStep();
                          } else {
                            notifier.launch(context, ref);
                          }
                        },
                        child: state.isLoading
                            ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2))
                            : Text(state.step == 6 ? 'Launch My AI Team →' : 'Next', style: const TextStyle(fontFamily: 'Inter')),
                      ),) : ElevatedButton(
                        onPressed: state.isLoading ? null : () {
                          if (state.step < 6) {
                            notifier.nextStep();
                          } else {
                            notifier.launch(context, ref);
                          }
                        },
                        child: state.isLoading
                            ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2))
                            : Text(state.step == 6 ? 'Launch My AI Team →' : 'Next', style: const TextStyle(fontFamily: 'Inter')),
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
