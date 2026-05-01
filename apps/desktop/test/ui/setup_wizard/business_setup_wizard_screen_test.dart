import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../lib/ui/setup_wizard/business_setup_wizard_screen.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen renders and manages state correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: BusinessSetupWizardScreen(),
        ),
      ),
    );

    // Initial step
    expect(find.text('Welcome'), findsOneWidget);
    expect(find.text('Next'), findsOneWidget);

    // Tap Next
    await tester.tap(find.text('Next'));
    await tester.pump();

    // Step 1
    expect(find.text('Business Type'), findsOneWidget);

    // Select type and type custom
    await tester.tap(find.text('Online Store'));
    await tester.enterText(find.byType(TextField), 'Custom Store');
    await tester.pump();

    // Tap Next
    await tester.tap(find.text('Next'));
    await tester.pump();

    // Step 2
    expect(find.byType(TextField), findsOneWidget);
    await tester.enterText(find.byType(TextField), 'Test Company');
    await tester.pump();

    // Tap Next
    await tester.tap(find.text('Next'));
    await tester.pump();

    // Step 3
    expect(find.text('What do you sell'), findsOneWidget);

    // Tap Back
    await tester.tap(find.text('Back'));
    await tester.pump();

    // Should be back to Step 2
    expect(find.byType(TextField), findsOneWidget);
    expect(find.text('Test Company'), findsOneWidget);
  });
}
