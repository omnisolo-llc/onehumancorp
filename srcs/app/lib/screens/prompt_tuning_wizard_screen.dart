import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import '../widgets/glass_card.dart';
import 'dart:ui';

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
  void addFocusTag(String tag) =>
      state = state.copyWith(focusTags: [...state.focusTags, tag]);
  void removeFocusTag(String tag) => state = state.copyWith(
    focusTags: state.focusTags.where((t) => t != tag).toList(),
  );
  void addExample(String q, String a) => state = state.copyWith(
    examples: [
      ...state.examples,
      {'q': q, 'a': a},
    ],
  );
  void toggleRawPrompt() =>
      state = state.copyWith(showRawPrompt: !state.showRawPrompt);

  void save(BuildContext context) {
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text("Your agent has been updated ✓")),
    );
    GoRouter.of(context).go('/dashboard');
  }
}

final promptTuningProvider =
    NotifierProvider<PromptTuningNotifier, PromptTuningState>(() {
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
      appBar: AppBar(
        title: Text(
          'Tune Agent: $agentId',
          style: const TextStyle(
            fontFamily: 'Outfit',
            fontWeight: FontWeight.bold,
          ),
        ),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      extendBodyBehindAppBar: true,
      backgroundColor: Colors.transparent,
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Color(0xFF0D0D1A), Color(0xFF1A1A33)],
          ),
        ),
        child: SafeArea(
          child: Row(
            children: [
              Expanded(
                flex: 1,
                child: ClipRRect(
                  child: BackdropFilter(
                    filter: ImageFilter.compose(
                      outer: ColorFilter.matrix(const <double>[
                        1.787,
                        -0.715,
                        -0.072,
                        0,
                        0,
                        -0.213,
                        1.285,
                        -0.072,
                        0,
                        0,
                        -0.213,
                        -0.715,
                        1.928,
                        0,
                        0,
                        0,
                        0,
                        0,
                        1,
                        0,
                      ]),
                      inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                    ),
                    child: Stepper(
                      type: StepperType.vertical,
                      currentStep: state.step,
                      onStepContinue: () {
                        if (state.step < 3) {
                          notifier.nextStep();
                        } else {
                          notifier.save(context);
                        }
                      },
                      onStepCancel: () {
                        if (state.step > 0) {
                          notifier.previousStep();
                        }
                      },
                      steps: [
                        Step(
                          title: const Text(
                            'Personality & Tone',
                            style: TextStyle(
                              fontFamily: 'Outfit',
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                          isActive: state.step >= 0,
                          content: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              ...[
                                'Formal',
                                'Friendly',
                                'Concise',
                                'Detailed',
                                'Custom',
                              ].map(
                                (t) => RadioListTile<String>(
                                  title: Text(
                                    t,
                                    style: const TextStyle(fontFamily: 'Inter'),
                                  ),
                                  value: t,
                                  groupValue: state.tone,
                                  onChanged: (val) {
                                    if (val != null) notifier.updateTone(val);
                                  },
                                ),
                              ),
                            ],
                          ),
                        ),
                        Step(
                          title: const Text(
                            'Domain Focus',
                            style: TextStyle(
                              fontFamily: 'Outfit',
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                          isActive: state.step >= 1,
                          content: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Wrap(
                                spacing: 8.0,
                                children:
                                    [
                                          'Only discuss business',
                                          'Avoid competitors',
                                          'Always reply in Spanish',
                                        ]
                                        .map(
                                          (tag) => ChoiceChip(
                                            label: Text(
                                              tag,
                                              style: const TextStyle(
                                                fontFamily: 'Inter',
                                              ),
                                            ),
                                            selected: state.focusTags.contains(
                                              tag,
                                            ),
                                            onSelected: (selected) {
                                              if (selected) {
                                                notifier.addFocusTag(tag);
                                              } else {
                                                notifier.removeFocusTag(tag);
                                              }
                                            },
                                          ),
                                        )
                                        .toList(),
                              ),
                            ],
                          ),
                        ),
                        Step(
                          title: const Text(
                            'Example Interactions',
                            style: TextStyle(
                              fontFamily: 'Outfit',
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                          isActive: state.step >= 2,
                          content: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              const Text(
                                'Provide up to 3 Q&A pairs.',
                                style: TextStyle(fontFamily: 'Inter'),
                              ),
                              const SizedBox(height: 8),
                              ElevatedButton(
                                onPressed: state.examples.length < 3
                                    ? () => notifier.addExample(
                                        "Sample Q",
                                        "Sample A",
                                      )
                                    : null,
                                child: const Text(
                                  'Add Example',
                                  style: TextStyle(fontFamily: 'Inter'),
                                ),
                              ),
                              const SizedBox(height: 8),
                              ...state.examples.map(
                                (ex) => ListTile(
                                  title: Text(ex['q'] ?? ''),
                                  subtitle: Text(ex['a'] ?? ''),
                                ),
                              ),
                            ],
                          ),
                        ),
                        Step(
                          title: const Text(
                            'Review & Save',
                            style: TextStyle(
                              fontFamily: 'Outfit',
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                          isActive: state.step >= 3,
                          content: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              const Text(
                                'Ready to update the agent?',
                                style: TextStyle(fontFamily: 'Inter'),
                              ),
                            ],
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
              Expanded(
                flex: 1,
                child: Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: GlassCard(
                    child: Padding(
                      padding: const EdgeInsets.all(16.0),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Row(
                            mainAxisAlignment: MainAxisAlignment.spaceBetween,
                            children: [
                              const Text(
                                'Live Preview',
                                style: TextStyle(
                                  fontFamily: 'Outfit',
                                  fontWeight: FontWeight.bold,
                                  fontSize: 18,
                                  color: Colors.white,
                                ),
                              ),
                              TextButton(
                                onPressed: notifier.toggleRawPrompt,
                                child: Text(
                                  state.showRawPrompt
                                      ? 'Hide raw prompt'
                                      : 'Edit raw prompt',
                                  style: const TextStyle(fontFamily: 'Inter'),
                                ),
                              ),
                            ],
                          ),
                          const SizedBox(height: 16),
                          Expanded(
                            child: state.showRawPrompt
                                ? SingleChildScrollView(
                                    child: TextField(
                                      maxLines: null,
                                      controller: TextEditingController(
                                        text: state.generatePrompt(),
                                      ),
                                      style: const TextStyle(
                                        fontFamily: 'Inter',
                                        fontSize: 14,
                                        color: Colors.white,
                                      ),
                                      decoration: const InputDecoration(
                                        border: OutlineInputBorder(),
                                      ),
                                    ),
                                  )
                                : Container(
                                    decoration: BoxDecoration(
                                      color: Colors.black26,
                                      borderRadius: BorderRadius.circular(8),
                                      border: Border.all(color: Colors.white12),
                                    ),
                                    child: const Center(
                                      child: Text(
                                        "Chat sandbox placeholder",
                                        style: TextStyle(
                                          fontFamily: 'Inter',
                                          color: Colors.white70,
                                        ),
                                      ),
                                    ),
                                  ),
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
