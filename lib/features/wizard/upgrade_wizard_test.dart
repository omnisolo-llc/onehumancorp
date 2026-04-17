import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'upgrade_wizard.dart';

void main() {
  testWidgets('UpgradeWizard renders steps and elements correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: UpgradeWizard(),
          ),
        ),
      ),
    );

    // Initial state: Step 1
    expect(find.text('What\'s new ✨'), findsOneWidget);

    // Go to next step
    await tester.tap(find.text('Next').first);
    await tester.pump(); await tester.pump(const Duration(milliseconds: 500));

    // Step 2
    expect(find.text('Upgrading System...'), findsOneWidget);
    expect(find.text('Upgrade in 1 click'), findsOneWidget);

    // Tap upgrade
    await tester.tap(find.text('Upgrade in 1 click'));
    await tester.pump();

    // TestWidgetsFlutterBinding overrides http and fails immediately, returning a 400.
    // Our code handles this by showing a SnackBar with "Failed to upgrade."
    expect(find.text('Failed to upgrade.'), findsOneWidget);
  });
}
