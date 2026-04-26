import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/settings_service.dart';
import '../widgets/glass_card.dart';

class AgentConfigState {
  final int step;
  final String selectedAgent;
  final bool canReplyMessages;
  final bool canPostSocial;
  final bool canWriteProductDesc;
  final bool canSendOrderUpdates;
  final double frequency; // 0: Weekly, 1: Daily, 2: Hourly, 3: Real-time
  final bool isDeploying;

  const AgentConfigState({
    this.step = 0,
    this.selectedAgent = '',
    this.canReplyMessages = false,
    this.canPostSocial = false,
    this.canWriteProductDesc = false,
    this.canSendOrderUpdates = false,
    this.frequency = 1.0,
    this.isDeploying = false,
  });

  AgentConfigState copyWith({
    int? step,
    String? selectedAgent,
    bool? canReplyMessages,
    bool? canPostSocial,
    bool? canWriteProductDesc,
    bool? canSendOrderUpdates,
    double? frequency,
    bool? isDeploying,
  }) {
    return AgentConfigState(
      step: step ?? this.step,
      selectedAgent: selectedAgent ?? this.selectedAgent,
      canReplyMessages: canReplyMessages ?? this.canReplyMessages,
      canPostSocial: canPostSocial ?? this.canPostSocial,
      canWriteProductDesc: canWriteProductDesc ?? this.canWriteProductDesc,
      canSendOrderUpdates: canSendOrderUpdates ?? this.canSendOrderUpdates,
      frequency: frequency ?? this.frequency,
      isDeploying: isDeploying ?? this.isDeploying,
    );
  }
}

class AgentConfigNotifier extends Notifier<AgentConfigState> {
  @override
  AgentConfigState build() => const AgentConfigState();

  void nextStep() {
    if (state.step < 3) state = state.copyWith(step: state.step + 1);
  }

  void previousStep() {
    if (state.step > 0) state = state.copyWith(step: state.step - 1);
  }

  void updateAgent(String val) => state = state.copyWith(selectedAgent: val);

  void toggleCapability(String cap) {
    if (cap == 'reply') state = state.copyWith(canReplyMessages: !state.canReplyMessages);
    if (cap == 'social') state = state.copyWith(canPostSocial: !state.canPostSocial);
    if (cap == 'desc') state = state.copyWith(canWriteProductDesc: !state.canWriteProductDesc);
    if (cap == 'order') state = state.copyWith(canSendOrderUpdates: !state.canSendOrderUpdates);
  }

  void updateFrequency(double val) => state = state.copyWith(frequency: val);

  Future<void> activate(BuildContext context, WidgetRef ref) async {
    state = state.copyWith(isDeploying: true);

    try {
      final api = ref.read(apiServiceProvider);
      if (api != null) {
        await api.hireAgent(
          state.selectedAgent,
          state.selectedAgent.replaceAll(' ', '_'),
        );
      }
    } catch (e) {
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Failed to deploy agent: $e')));
        state = state.copyWith(isDeploying: false);
        return;
      }
    }

    if (context.mounted) {
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('${state.selectedAgent} is now active and on the team! ✓')));
      GoRouter.of(context).go('/dashboard');
    }
  }
}

final agentConfigProvider = NotifierProvider<AgentConfigNotifier, AgentConfigState>(() {
  return AgentConfigNotifier();
});

class AgentConfigWizardScreen extends ConsumerWidget {
  const AgentConfigWizardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(agentConfigProvider);
    final notifier = ref.read(agentConfigProvider.notifier);

    final agents = [
      {'name': 'Customer Support', 'icon': Icons.support_agent},
      {'name': 'Social Media Manager', 'icon': Icons.thumb_up},
      {'name': 'SEO Booster', 'icon': Icons.trending_up},
      {'name': 'Order Manager', 'icon': Icons.local_shipping},
      {'name': 'Email Marketer', 'icon': Icons.email},
    ];

    String getFrequencyLabel() {
      if (state.frequency == 0) return 'Weekly';
      if (state.frequency == 1) return 'Daily';
      if (state.frequency == 2) return 'Hourly';
      return 'Real-time';
    }

    final isAdvanced = ref.watch(clientSettingsProvider).valueOrNull?.expertMode ?? false;

    return Scaffold(
      appBar: AppBar(title: const Text('Manage my AI team', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold))),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 800),
          child: GlassCard(
            child: Padding(
              padding: const EdgeInsets.all(24.0),
              child: SingleChildScrollView(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        const Text('Configure AI Agent', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold)),
                        Switch(
                          value: isAdvanced,
                          onChanged: (val) {
                            final settingsNotifier = ref.read(clientSettingsProvider.notifier);
                            settingsNotifier.updateExpertMode(val);
                          },
                        ),
                      ],
                    ),
                    const SizedBox(height: 16),
                    if (state.step == 0) ...[
                      const Text('Choose an agent to add to your team', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
                      const SizedBox(height: 16),
                      Wrap(
                        spacing: 16,
                        runSpacing: 16,
                        children: agents.map((a) {
                          final name = a['name'] as String;
                          final icon = a['icon'] as IconData;
                          final isSelected = state.selectedAgent == name;
                          return InkWell(
                            onTap: () => notifier.updateAgent(name),
                            child: Container(
                              width: 160,
                              padding: const EdgeInsets.all(16),
                              decoration: BoxDecoration(
                                color: Theme.of(context).colorScheme.surfaceContainerHighest,
                                border: Border.all(color: isSelected ? Colors.blue : Colors.grey, width: 2),
                                borderRadius: BorderRadius.circular(12),
                              ),
                              child: Column(
                                children: [
                                  Icon(icon, size: 48, color: isSelected ? Colors.blue : Colors.grey),
                                  const SizedBox(height: 8),
                                  Text(name, textAlign: TextAlign.center, style: const TextStyle(fontFamily: 'Inter')),
                                  const SizedBox(height: 8),
                                  Switch(value: isSelected, onChanged: (_) => notifier.updateAgent(name)),
                                ],
                              ),
                            ),
                          );
                        }).toList(),
                      ),
                    ] else if (state.step == 1) ...[
                      const Text('What should this agent do?', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
                      const SizedBox(height: 16),
                      SwitchListTile(
                        title: const Text('Reply to customer messages'),
                        value: state.canReplyMessages,
                        onChanged: (_) => notifier.toggleCapability('reply'),
                      ),
                      SwitchListTile(
                        title: const Text('Post to Instagram & Facebook'),
                        value: state.canPostSocial,
                        onChanged: (_) => notifier.toggleCapability('social'),
                      ),
                      SwitchListTile(
                        title: const Text('Write product descriptions'),
                        value: state.canWriteProductDesc,
                        onChanged: (_) => notifier.toggleCapability('desc'),
                      ),
                      SwitchListTile(
                        title: const Text('Send order updates'),
                        value: state.canSendOrderUpdates,
                        onChanged: (_) => notifier.toggleCapability('order'),
                      ),
                      if (isAdvanced) ...[
                        const SizedBox(height: 24),
                        const Text('Advanced Capabilities', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
                        const SizedBox(height: 8),
                        const TextField(
                          decoration: InputDecoration(
                            labelText: 'Custom JSON Prompt Configuration',
                            border: OutlineInputBorder(),
                            hintText: 'e.g. {"custom_tool": true}',
                          ),
                          maxLines: 3,
                        ),
                      ],
                    ] else if (state.step == 2) ...[
                      const Text('How often should this agent work?', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
                      const SizedBox(height: 16),
                      Text(getFrequencyLabel(), style: const TextStyle(fontSize: 24, fontWeight: FontWeight.bold, color: Colors.blue)),
                      const SizedBox(height: 16),
                      Slider(
                        value: state.frequency,
                        min: 0,
                        max: 3,
                        divisions: 3,
                        label: getFrequencyLabel(),
                        onChanged: notifier.updateFrequency,
                      ),
                    ] else if (state.step == 3) ...[
                      const Text('Review & Activate', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, fontSize: 20)),
                      const SizedBox(height: 16),
                      Container(
                        padding: const EdgeInsets.all(16),
                        decoration: BoxDecoration(
                          color: Theme.of(context).colorScheme.surfaceContainerHighest,
                          borderRadius: BorderRadius.circular(12),
                        ),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text('Agent: ${state.selectedAgent}', style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 18)),
                            const Divider(),
                            const Text('Capabilities:', style: TextStyle(fontWeight: FontWeight.bold)),
                            if (state.canReplyMessages) const Text('• Reply to customer messages'),
                            if (state.canPostSocial) const Text('• Post to Instagram & Facebook'),
                            if (state.canWriteProductDesc) const Text('• Write product descriptions'),
                            if (state.canSendOrderUpdates) const Text('• Send order updates'),
                            if (!state.canReplyMessages && !state.canPostSocial && !state.canWriteProductDesc && !state.canSendOrderUpdates)
                              const Text('• None selected'),
                            const Divider(),
                            Text('Schedule: ${getFrequencyLabel()}', style: const TextStyle(fontWeight: FontWeight.bold)),
                          ],
                        ),
                      ),
                    ],
                    const SizedBox(height: 32),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        if (state.step > 0)
                          OutlinedButton(onPressed: state.isDeploying ? null : notifier.previousStep, child: const Text('Back')),
                        if (state.step == 0) const SizedBox(),
                        ElevatedButton(
                          onPressed: (state.step == 0 && state.selectedAgent.isEmpty) || state.isDeploying ? null : () {
                            if (state.step < 3) {
                              notifier.nextStep();
                            } else {
                              notifier.activate(context, ref);
                            }
                          },
                          child: state.isDeploying
                            ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2))
                            : Text(state.step == 3 ? 'Activate' : 'Next'),
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
