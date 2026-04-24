import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/services/settings_service.dart';
import '../widgets/glass_card.dart';

class GrowMyBusinessState {
  final int step;
  final String selectedStrategy;
  final bool isDeploying;

  const GrowMyBusinessState({
    this.step = 0,
    this.selectedStrategy = '',
    this.isDeploying = false,
  });

  GrowMyBusinessState copyWith({
    int? step,
    String? selectedStrategy,
    bool? isDeploying,
  }) {
    return GrowMyBusinessState(
      step: step ?? this.step,
      selectedStrategy: selectedStrategy ?? this.selectedStrategy,
      isDeploying: isDeploying ?? this.isDeploying,
    );
  }
}

class GrowMyBusinessNotifier extends Notifier<GrowMyBusinessState> {
  @override
  GrowMyBusinessState build() => const GrowMyBusinessState();

  void nextStep() {
    if (state.step < 1) state = state.copyWith(step: state.step + 1);
  }

  void previousStep() {
    if (state.step > 0) state = state.copyWith(step: state.step - 1);
  }

  void updateStrategy(String val) => state = state.copyWith(selectedStrategy: val);

  Future<void> execute(BuildContext context) async {
    state = state.copyWith(isDeploying: true);
    // Replace with real backend API in the future
    if (context.mounted) {
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Action started: ${state.selectedStrategy} ✓')));
      GoRouter.of(context).go('/dashboard');
    }
  }
}

final growMyBusinessProvider = NotifierProvider<GrowMyBusinessNotifier, GrowMyBusinessState>(() {
  return GrowMyBusinessNotifier();
});

class GrowMyBusinessWizardScreen extends ConsumerWidget {
  const GrowMyBusinessWizardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(growMyBusinessProvider);
    final notifier = ref.read(growMyBusinessProvider.notifier);
    final isAdvanced = ref.watch(clientSettingsProvider).valueOrNull?.expertMode ?? false;

    final strategies = [
      {'title': 'Add 5 more products', 'desc': 'Expand your catalog with AI suggestions', 'icon': Icons.add_business},
      {'title': 'Connect Instagram', 'desc': 'Let your agents post automatically', 'icon': Icons.camera_alt},
      {'title': 'Run your first email campaign', 'desc': 'Re-engage past customers', 'icon': Icons.email},
    ];

    return Scaffold(
      appBar: AppBar(title: const Text('Grow My Business', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold))),
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
                        const Text('Growth Strategies', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold)),
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
                      const Text('Select a next step to grow your business', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
                      const SizedBox(height: 16),
                      ...strategies.map((s) {
                        final isSelected = state.selectedStrategy == s['title'];
                        return Padding(
                          padding: const EdgeInsets.only(bottom: 8.0),
                          child: InkWell(
                            onTap: () => notifier.updateStrategy(s['title'] as String),
                            child: Container(
                              padding: const EdgeInsets.all(16),
                              decoration: BoxDecoration(
                                color: Theme.of(context).colorScheme.surfaceContainerHighest,
                                border: Border.all(color: isSelected ? Colors.blue : Colors.grey, width: 2),
                                borderRadius: BorderRadius.circular(12),
                              ),
                              child: Row(
                                children: [
                                  Icon(s['icon'] as IconData, size: 32, color: isSelected ? Colors.blue : Colors.grey),
                                  const SizedBox(width: 16),
                                  Expanded(
                                    child: Column(
                                      crossAxisAlignment: CrossAxisAlignment.start,
                                      children: [
                                        Text(s['title'] as String, style: const TextStyle(fontWeight: FontWeight.bold)),
                                        Text(s['desc'] as String),
                                      ],
                                    ),
                                  ),
                                  if (isSelected) const Icon(Icons.check_circle, color: Colors.blue),
                                ],
                              ),
                            ),
                          ),
                        );
                      }).toList(),
                    ] else if (state.step == 1) ...[
                      const Text('Confirm Action', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, fontSize: 20)),
                      const SizedBox(height: 16),
                      Text('You are about to start: ${state.selectedStrategy}'),
                      const SizedBox(height: 8),
                      const Text('Your AI agents will be instructed to begin this growth strategy immediately.', style: TextStyle(color: Colors.grey)),
                      if (isAdvanced) ...[
                        const SizedBox(height: 24),
                        const Text('Advanced Target KPIs', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
                        const SizedBox(height: 8),
                        const TextField(
                          decoration: InputDecoration(
                            labelText: 'Conversion Target (%)',
                            border: OutlineInputBorder(),
                            hintText: 'e.g. 5.0',
                          ),
                          keyboardType: TextInputType.number,
                        ),
                      ],
                    ],
                    const SizedBox(height: 32),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        if (state.step > 0)
                          OutlinedButton(onPressed: state.isDeploying ? null : notifier.previousStep, child: const Text('Back')),
                        if (state.step == 0) const SizedBox(),
                        ElevatedButton(
                          onPressed: (state.step == 0 && state.selectedStrategy.isEmpty) || state.isDeploying ? null : () {
                            if (state.step < 1) {
                              notifier.nextStep();
                            } else {
                              notifier.execute(context);
                            }
                          },
                          child: state.isDeploying
                            ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2))
                            : Text(state.step == 1 ? 'Execute' : 'Next'),
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
