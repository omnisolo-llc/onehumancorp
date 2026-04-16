import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'prompt_tuning_wizard.dart';

void main() {
  testWidgets('PromptTuningWizard renders Stepper and initial text', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: PromptTuningWizard(),
        ),
      ),
    );
    expect(find.byType(Stepper), findsOneWidget);
    expect(find.text('Personality & Tone'), findsOneWidget);
    expect(find.text('Friendly'), findsOneWidget);
  });
}
