import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'dart:ui';
import '../services/api_service.dart';
import '../services/settings_service.dart';
import '../widgets/glass_card.dart';

class BusinessSetupState {
  final int step;
  final String aiPrompt;
  final String companyName;
  final String industry;
  final String size;
  final List<String> goals;
  final String deployment;
  final String adminName;
  final String adminEmail;
  final String adminPassword;
  final bool isLoading;
  final String? errorMessage;
  final bool useNlMode;
  final List<Map<String, String>> nlMessages;

  const BusinessSetupState({
    this.step = 0,
    this.aiPrompt = '',
    this.companyName = '',
    this.industry = '',
    this.size = 'S',
    this.goals = const [],
    this.deployment = 'Cloud',
    this.adminName = '',
    this.adminEmail = '',
    this.adminPassword = '',
    this.isLoading = false,
    this.errorMessage,
    this.useNlMode = false,
    this.nlMessages = const [],
  });

  BusinessSetupState copyWith({
    int? step,
    String? aiPrompt,
    String? companyName,
    String? industry,
    String? size,
    List<String>? goals,
    String? deployment,
    String? adminName,
    String? adminEmail,
    String? adminPassword,
    bool? isLoading,
    String? errorMessage,
    bool? useNlMode,
    List<Map<String, String>>? nlMessages,
  }) {
      return BusinessSetupState(
      step: step ?? this.step,
      aiPrompt: aiPrompt ?? this.aiPrompt,
      companyName: companyName ?? this.companyName,
      industry: industry ?? this.industry,
      size: size ?? this.size,
      goals: goals ?? this.goals,
      deployment: deployment ?? this.deployment,
      adminName: adminName ?? this.adminName,
      adminEmail: adminEmail ?? this.adminEmail,
      adminPassword: adminPassword ?? this.adminPassword,
      isLoading: isLoading ?? this.isLoading,
      errorMessage: errorMessage ?? this.errorMessage,
      useNlMode: useNlMode ?? this.useNlMode,
      nlMessages: nlMessages ?? this.nlMessages,
    );
  }
}

class BusinessSetupNotifier extends Notifier<BusinessSetupState> {
  @override
  BusinessSetupState build() => const BusinessSetupState();

  void nextStep() {
    if (state.step < 4) {
      state = state.copyWith(step: state.step + 1);
    }
  }

  void prevStep() {
    if (state.step > 0) {
      state = state.copyWith(step: state.step - 1);
    }
  }

  void updateCompany(String name) => state = state.copyWith(companyName: name);
  void updatePrompt(String prompt) => state = state.copyWith(aiPrompt: prompt);
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

  void toggleNlMode() => state = state.copyWith(useNlMode: !state.useNlMode);

  void addNlMessage(String role, String text) {
    final msgs = List<Map<String, String>>.from(state.nlMessages)
      ..add({'role': role, 'text': text});
    state = state.copyWith(nlMessages: msgs);
  }

  void applyNlFieldUpdates(Map<String, String> updates) {
    var s = state;
    if (updates.containsKey('company_name')) s = s.copyWith(companyName: updates['company_name']);
    if (updates.containsKey('industry')) s = s.copyWith(industry: updates['industry']);
    if (updates.containsKey('size')) s = s.copyWith(size: updates['size']);
    if (updates.containsKey('admin_name')) s = s.copyWith(adminName: updates['admin_name']);
    if (updates.containsKey('admin_email')) s = s.copyWith(adminEmail: updates['admin_email']);
    if (updates.containsKey('goals')) {
      final goals = List<String>.from(s.goals);
      final goal = updates['goals']!;
      if (!goals.contains(goal)) goals.add(goal);
      s = s.copyWith(goals: goals);
    }
    state = s;
  }

  Future<void> launch(BuildContext context, WidgetRef ref) async {
    final api = ref.read(apiServiceProvider);

    state = state.copyWith(isLoading: true, errorMessage: null);

    if (api != null) {
      try {
        await api.bootstrapBusiness(
          prompt: state.aiPrompt,
          companyName: state.companyName,
          industry: state.industry,
          companySize: state.size,
          goals: state.goals,
          deploymentPreference: state.deployment,
          adminName: state.adminName,
          adminEmail: state.adminEmail,
        );
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

class BusinessSetupWizardScreen extends ConsumerStatefulWidget {
  const BusinessSetupWizardScreen({super.key});

  @override
  ConsumerState<BusinessSetupWizardScreen> createState() => _BusinessSetupWizardScreenState();
}

class _BusinessSetupWizardScreenState extends ConsumerState<BusinessSetupWizardScreen> {
  bool _obscurePassword = true;
  final _nlMessageCtrl = TextEditingController();

  @override
  void dispose() {
    _nlMessageCtrl.dispose();
    super.dispose();
  }

  Future<void> _sendNlMessage() async {
    final msg = _nlMessageCtrl.text.trim();
    if (msg.isEmpty) return;
    final notifier = ref.read(businessSetupProvider.notifier);
    notifier.addNlMessage('user', msg);
    _nlMessageCtrl.clear();

    final api = ref.read(apiServiceProvider);
    if (api == null) {
      notifier.addNlMessage('assistant', 'No API connection available.');
      return;
    }
    try {
      final state = ref.read(businessSetupProvider);
      final result = await api.nlChatWizard(
        message: msg,
        partialState: {
          'company_name': state.companyName,
          'industry': state.industry,
          'admin_email': state.adminEmail,
          'admin_name': state.adminName,
        },
      );
      if (result['reply'] != null) {
        notifier.addNlMessage('assistant', result['reply'] as String);
      }
      final updates = result['field_updates'] as Map<String, dynamic>?;
      if (updates != null) {
        notifier.applyNlFieldUpdates(updates.map((k, v) => MapEntry(k, v.toString())));
      }
    } catch (e) {
      notifier.addNlMessage('assistant', 'Error: $e');
    }
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
                  const Text('Business Setup', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white)),
                  const SizedBox(height: 8),
                  // Mode toggle: Form ↔ Natural Language
                  Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      const Text('Form', style: TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 13)),
                      const SizedBox(width: 8),
                      Switch(
                        value: state.useNlMode,
                        onChanged: (_) => notifier.toggleNlMode(),
                        activeColor: Colors.blueAccent,
                      ),
                      const SizedBox(width: 8),
                      const Text('Natural Language', style: TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 13)),
                    ],
                  ),
                  const SizedBox(height: 8),
                  if (state.errorMessage != null) ...[
                    Text(state.errorMessage!, style: const TextStyle(color: Colors.red)),
                    const SizedBox(height: 16),
                  ],
                  if (state.useNlMode)
                    _NlChatView(
                      messages: state.nlMessages,
                      controller: _nlMessageCtrl,
                      onSend: _sendNlMessage,
                      isLoading: state.isLoading,
                      readyToSubmit: state.companyName.isNotEmpty && state.adminEmail.isNotEmpty,
                      onSubmit: () => notifier.launch(context, ref),
                    )
                  else ...[
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
                            const SizedBox(height: 16),
                            TextField(
                              minLines: 3,
                              maxLines: 5,
                              decoration: const InputDecoration(
                                labelText: 'Describe the business you want AI to create',
                                hintText: 'Help me create a real estate staging company that serves luxury listings.',
                                labelStyle: TextStyle(color: Colors.white70),
                              ),
                              onChanged: notifier.updatePrompt,
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                            ),
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
                              obscureText: _obscurePassword,
                              onChanged: notifier.updateAdminPassword,
                              style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                              decoration: InputDecoration(
                                labelText: 'Admin Password',
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
                          ],
                        ],
                      ),
                    ),
                  ),
                  // Navigation buttons shown only in form mode.
                  if (!state.useNlMode) ...[
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
                          if (state.step < 4) {
                            notifier.nextStep();
                          } else {
                            notifier.launch(context, ref);
                          }
                        },
                        child: state.isLoading
                            ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2))
                            : Text(state.step == 4 ? 'Launch My AI Team →' : 'Next', style: const TextStyle(fontFamily: 'Inter')),
                      ),
                    ],
                  ),
                  ], // end form mode navigation
                ], // end else (form mode) block
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

// ── Natural-language chat view ────────────────────────────────────────────────

class _NlChatView extends StatelessWidget {
  final List<Map<String, String>> messages;
  final TextEditingController controller;
  final VoidCallback onSend;
  final bool isLoading;
  final bool readyToSubmit;
  final VoidCallback onSubmit;

  const _NlChatView({
    required this.messages,
    required this.controller,
    required this.onSend,
    required this.isLoading,
    required this.readyToSubmit,
    required this.onSubmit,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        ConstrainedBox(
          constraints: const BoxConstraints(maxHeight: 260),
          child: messages.isEmpty
              ? const Padding(
                  padding: EdgeInsets.symmetric(vertical: 16),
                  child: Text(
                    'Describe your business in natural language and I\'ll fill in the form for you.',
                    style: TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 14),
                    textAlign: TextAlign.center,
                  ),
                )
              : ListView.builder(
                  shrinkWrap: true,
                  itemCount: messages.length,
                  itemBuilder: (_, i) {
                    final msg = messages[i];
                    final isUser = msg['role'] == 'user';
                    return Align(
                      alignment: isUser ? Alignment.centerRight : Alignment.centerLeft,
                      child: Container(
                        margin: const EdgeInsets.symmetric(vertical: 4),
                        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                        decoration: BoxDecoration(
                          color: isUser
                              ? Colors.blueAccent.withOpacity(0.3)
                              : Colors.white.withOpacity(0.1),
                          borderRadius: BorderRadius.circular(12),
                        ),
                        child: Text(
                          msg['text'] ?? '',
                          style: const TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 13),
                        ),
                      ),
                    );
                  },
                ),
        ),
        const SizedBox(height: 12),
        Row(
          children: [
            Expanded(
              child: TextField(
                controller: controller,
                style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                decoration: const InputDecoration(
                  hintText: 'e.g. I want to start a real estate staging company...',
                  hintStyle: TextStyle(color: Colors.white38),
                  border: OutlineInputBorder(),
                ),
                onSubmitted: (_) => onSend(),
              ),
            ),
            const SizedBox(width: 8),
            IconButton(
              icon: const Icon(Icons.send, color: Colors.blueAccent),
              onPressed: isLoading ? null : onSend,
            ),
          ],
        ),
        if (readyToSubmit) ...[
          const SizedBox(height: 12),
          ElevatedButton(
            onPressed: isLoading ? null : onSubmit,
            child: isLoading
                ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2))
                : const Text('Launch My AI Team →', style: TextStyle(fontFamily: 'Inter')),
          ),
        ],
      ],
    );
  }
}
