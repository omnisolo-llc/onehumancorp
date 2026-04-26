import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import '../widgets/glass_card.dart';

class PromptTuningState {
  final int step;
  final String tone;
  final List<String> focusTags;
  final List<Map<String, String>> examples;
  final bool showRawPrompt;

  const PromptTuningState({
    this.step = 0,
    this.tone = 'Friendly',
    this.focusTags = const [],
    this.examples = const [],
    this.showRawPrompt = false,
  });

  PromptTuningState copyWith({
    int? step,
    String? tone,
    List<String>? focusTags,
    List<Map<String, String>>? examples,
    bool? showRawPrompt,
  }) {
    return PromptTuningState(
      step: step ?? this.step,
      tone: tone ?? this.tone,
      focusTags: focusTags ?? this.focusTags,
      examples: examples ?? this.examples,
      showRawPrompt: showRawPrompt ?? this.showRawPrompt,
    );
  }

  String generatePrompt() {
    String prompt = "You are an AI agent with a $tone personality.\n";
    if (focusTags.isNotEmpty) {
      prompt += "Domain Focus: ${focusTags.join(', ')}.\n";
    }
    if (examples.isNotEmpty) {
      prompt += "Examples:\n";
      for (var ex in examples) {
        prompt += "Q: ${ex['q']}\nA: ${ex['a']}\n";
      }
    }
    return prompt;
  }
}

class PromptTuningNotifier extends Notifier<PromptTuningState> {
  @override
  PromptTuningState build() => const PromptTuningState();

  void nextStep() {
    if (state.step < 3) state = state.copyWith(step: state.step + 1);
  }

  void previousStep() {
    if (state.step > 0) state = state.copyWith(step: state.step - 1);
  }

  void updateTone(String t) => state = state.copyWith(tone: t);
  void addFocusTag(String tag) => state = state.copyWith(focusTags: [...state.focusTags, tag]);
  void removeFocusTag(String tag) => state = state.copyWith(focusTags: state.focusTags.where((t) => t != tag).toList());
  void addExample(String q, String a) => state = state.copyWith(examples: [...state.examples, {'q': q, 'a': a}]);
  void toggleRawPrompt() => state = state.copyWith(showRawPrompt: !state.showRawPrompt);

  void save(BuildContext context) {
    ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text("Your agent has been updated ✓")));
    GoRouter.of(context).go('/dashboard');
  }
}

final promptTuningProvider = NotifierProvider<PromptTuningNotifier, PromptTuningState>(() {
  return PromptTuningNotifier();
});

class PromptTuningWizardScreen extends ConsumerWidget {
  final String agentId;
  const PromptTuningWizardScreen({super.key, required this.agentId});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(promptTuningProvider);
    final notifier = ref.read(promptTuningProvider.notifier);

    return Scaffold(
      appBar: AppBar(title: Text('Tune Agent: $agentId')),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 800),
          child: GlassCard(
            child: Padding(
              padding: const EdgeInsets.all(24.0),
              child: SingleChildScrollView(
                child: Wrap(
                  spacing: 24,
                  runSpacing: 24,
                  children: [
                    Container(
                      width: 350,
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          const Text('Prompt Tuning', style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold)),
                          const SizedBox(height: 16),
                          if (state.step == 0) ...[
                            const Text('Personality & tone', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
                            ...['Formal', 'Friendly', 'Concise', 'Detailed', 'Custom'].map((t) => RadioListTile<String>(
                              title: Text(t, style: const TextStyle(fontFamily: 'Inter')),
                              value: t,
                              groupValue: state.tone,
                              onChanged: (val) { if (val != null) notifier.updateTone(val); },
                            )),
                          ] else if (state.step == 1) ...[
                            const Text('Domain focus', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
                            Wrap(
                              spacing: 8.0,
                              children: ['Only discuss business', 'Avoid competitors', 'Always reply in Spanish'].map((tag) => ChoiceChip(
                                label: Text(tag, style: const TextStyle(fontFamily: 'Inter')),
                                selected: state.focusTags.contains(tag),
                                onSelected: (selected) {
                                  if (selected) {
                                    notifier.addFocusTag(tag);
                                  } else {
                                    notifier.removeFocusTag(tag);
                                  }
                                },
                              )).toList(),
                            ),
                          ] else if (state.step == 2) ...[
                            const Text('Example interactions', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
                            const Text('Provide up to 3 Q&A pairs.'),
                            ElevatedButton(
                              onPressed: state.examples.length < 3 ? () => notifier.addExample("Sample Q", "Sample A") : null,
                              child: const Text('Add Example'),
                            ),
                            ...state.examples.map((ex) => ListTile(title: Text(ex['q'] ?? ''), subtitle: Text(ex['a'] ?? ''))),
                          ] else if (state.step == 3) ...[
                             const Text('Review & Save', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
                             const Text('Ready to update the agent?'),
                          ],
                          const SizedBox(height: 16),
                          Row(
                            children: [
                              if (state.step > 0)
                                TextButton(onPressed: notifier.previousStep, child: const Text('Back')),
                              const Expanded(child: SizedBox()),
                              ElevatedButton(
                                onPressed: () {
                                  if (state.step < 3) {
                                    notifier.nextStep();
                                  } else {
                                    notifier.save(context);
                                  }
                                },
                                child: Text(state.step == 3 ? 'Save' : 'Next', style: const TextStyle(fontFamily: 'Inter')),
                              ),
                            ],
                          )
                        ],
                      ),
                    ),
                    Container(
                      width: 350,
                      child: Container(
                        padding: const EdgeInsets.all(16),
                        decoration: BoxDecoration(
                          border: Border.all(color: Colors.white24),
                          borderRadius: BorderRadius.circular(8),
                        ),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Wrap(
                              spacing: 8,
                              children: [
                                const Text('Live Preview', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
                                TextButton(
                                  onPressed: notifier.toggleRawPrompt,
                                  child: Text(state.showRawPrompt ? 'Hide raw prompt' : 'Edit raw prompt'),
                                ),
                              ],
                            ),
                            const SizedBox(height: 8),
                            if (state.showRawPrompt)
                              Text(state.generatePrompt(), style: const TextStyle(fontFamily: 'Inter', fontSize: 14)),
                            if (!state.showRawPrompt)
                              Container(height: 200, child: const Center(child: Text("Chat sandbox placeholder"))),
                          ],
                        ),
                      ),
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
