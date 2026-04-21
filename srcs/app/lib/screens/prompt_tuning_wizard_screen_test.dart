import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/prompt_tuning_wizard_screen.dart';

void main() {
  group('PromptTuningNotifier', () {
    testWidgets('initial state is correct', (tester) async {
      final container = ProviderContainer();
      final state = container.read(promptTuningProvider);

      expect(state.step, 0);
      expect(state.tone, 'Friendly');
      expect(state.focusTags, isEmpty);
      expect(state.examples, isEmpty);
      expect(state.showRawPrompt, isFalse);
    });

    testWidgets('updateTone changes state correctly', (tester) async {
      final container = ProviderContainer();
      final notifier = container.read(promptTuningProvider.notifier);

      notifier.updateTone('Detailed');
      expect(container.read(promptTuningProvider).tone, 'Detailed');
    });

    testWidgets('addFocusTag changes state correctly', (tester) async {
      final container = ProviderContainer();
      final notifier = container.read(promptTuningProvider.notifier);

      notifier.addFocusTag('Tag1');
      expect(container.read(promptTuningProvider).focusTags, contains('Tag1'));
    });

    testWidgets('removeFocusTag changes state correctly', (tester) async {
      final container = ProviderContainer();
      final notifier = container.read(promptTuningProvider.notifier);

      notifier.addFocusTag('Tag1');
      expect(container.read(promptTuningProvider).focusTags, contains('Tag1'));

      notifier.removeFocusTag('Tag1');
      expect(container.read(promptTuningProvider).focusTags, isNot(contains('Tag1')));
    });

    testWidgets('addExample changes state correctly', (tester) async {
      final container = ProviderContainer();
      final notifier = container.read(promptTuningProvider.notifier);

      notifier.addExample('Question', 'Answer');
      expect(container.read(promptTuningProvider).examples.length, 1);
      expect(container.read(promptTuningProvider).examples.first['q'], 'Question');
      expect(container.read(promptTuningProvider).examples.first['a'], 'Answer');
    });

    testWidgets('toggleRawPrompt toggles state correctly', (tester) async {
      final container = ProviderContainer();
      final notifier = container.read(promptTuningProvider.notifier);

      expect(container.read(promptTuningProvider).showRawPrompt, isFalse);

      notifier.toggleRawPrompt();
      expect(container.read(promptTuningProvider).showRawPrompt, isTrue);

      notifier.toggleRawPrompt();
      expect(container.read(promptTuningProvider).showRawPrompt, isFalse);
    });

    testWidgets('nextStep and previousStep manage state correctly', (tester) async {
      final container = ProviderContainer();
      final notifier = container.read(promptTuningProvider.notifier);

      expect(container.read(promptTuningProvider).step, 0);

      notifier.nextStep();
      expect(container.read(promptTuningProvider).step, 1);

      for (int i = 0; i < 5; i++) {
         notifier.nextStep();
      }
      expect(container.read(promptTuningProvider).step, 3); // Max step

      notifier.previousStep();
      expect(container.read(promptTuningProvider).step, 2);

      for (int i = 0; i < 5; i++) {
         notifier.previousStep();
      }
      expect(container.read(promptTuningProvider).step, 0); // Min step
    });

    test('generatePrompt outputs correctly', () {
      final state = const PromptTuningState(
        tone: 'Formal',
        focusTags: ['Tag1', 'Tag2'],
        examples: [
          {'q': 'Q1', 'a': 'A1'}
        ],
      );

      final prompt = state.generatePrompt();

      expect(prompt, contains('You are an AI agent with a Formal personality.'));
      expect(prompt, contains('Domain Focus: Tag1, Tag2.'));
      expect(prompt, contains('Examples:'));
      expect(prompt, contains('Q: Q1'));
      expect(prompt, contains('A: A1'));
    });
  });
}