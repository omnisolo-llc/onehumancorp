import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import '../../lib/screens/ongoing_management_wizards.dart';

void main() {
  testWidgets('FixThisWizardScreen shows raw error logs when expert mode is enabled', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: FixThisWizardScreen(agentId: 'test-agent'),
        ),
      ),
    );

    // Initial state: Step 0, Expert mode off. Log container should not be visible.
    expect(find.text('ERROR: postgresql connection timeout\n  at db.go:142\n  at agent.go:34\nCLI: psql -h localhost -U admin'), findsNothing);

    // Turn on expert mode.
    final switchFinder = find.byType(Switch);
    expect(switchFinder, findsOneWidget);
    await tester.tap(switchFinder);
    await tester.pumpAndSettle();

    // Log container should now be visible.
    expect(find.text('ERROR: postgresql connection timeout\n  at db.go:142\n  at agent.go:34\nCLI: psql -h localhost -U admin'), findsOneWidget);
  });
}
