import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'fix_this_wizard.dart';

void main() {
  testWidgets('FixThisWizard renders steps and elements correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: FixThisWizard(),
          ),
        ),
      ),
    );

    // Initial state: Step 1
    expect(find.text('Error Summary'), findsOneWidget);
    expect(find.text('The agent is experiencing high latency and connection drops to the external provider.'), findsOneWidget);

    // Toggle expert mode
    final switchFinder = find.byType(Switch);
    expect(switchFinder, findsOneWidget);
    await tester.tap(switchFinder);
    await tester.pumpAndSettle();

    // Verify expert mode content appears
    expect(find.text('Raw Log:'), findsOneWidget);

    // Go to next step
    await tester.tap(find.text('Next').first);
    await tester.pumpAndSettle();

    // Step 2
    expect(find.text('Suggested Fix'), findsOneWidget);
    expect(find.text('Restart the agent and clear its current session cache.'), findsOneWidget);
    expect(find.text('Apply fix'), findsOneWidget);

    // Tap apply fix
    await tester.tap(find.text('Apply fix'));
    await tester.pump();

    // TestWidgetsFlutterBinding overrides http and fails immediately, returning a 400.
    // Our code handles this by showing a SnackBar with "Failed to apply fix."
    expect(find.text('Failed to apply fix.'), findsOneWidget);
  });
}
