import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/ongoing_management_wizards.dart';

void main() {
  group('GrowMyBusinessWizardScreen', () {
    testWidgets('renders first step and navigates to second', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(child: MaterialApp(home: GrowMyBusinessWizardScreen())),
      );
      await tester.pumpAndSettle();

      expect(find.text('Let\'s grow your business 🚀'), findsOneWidget);
      expect(find.text('See Suggestions'), findsOneWidget);

      await tester.tap(find.text('See Suggestions'));
      await tester.pumpAndSettle();

      expect(find.text('Suggested Next Steps:'), findsOneWidget);
      expect(find.text('Add 5 more products'), findsOneWidget);
      expect(find.text('Connect Instagram'), findsOneWidget);
      expect(find.text('Run your first email campaign'), findsOneWidget);
    });

    testWidgets('can apply an action', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(child: MaterialApp(home: GrowMyBusinessWizardScreen())),
      );
      await tester.pumpAndSettle();

      await tester.tap(find.text('See Suggestions'));
      await tester.pumpAndSettle();

      await tester.tap(find.widgetWithText(ElevatedButton, 'Do it'));
      await tester.pump(); // Start processing

      expect(find.byType(CircularProgressIndicator), findsOneWidget);
      expect(find.text('Working on it...'), findsOneWidget);

      await tester.pumpAndSettle(const Duration(seconds: 2));

      expect(find.text('Successfully applied: Add 5 more products'), findsOneWidget);
      expect(find.text('Back to Dashboard'), findsOneWidget);
    });
  });
}
