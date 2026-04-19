import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'ongoing_management_wizards.dart';

void main() {
  testWidgets('FixThisWizard renders stepper', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: FixThisWizard(),
          ),
        ),
      ),
    );
    expect(find.byType(Stepper), findsOneWidget);
    expect(find.text('Error Summary'), findsOneWidget);
  });

  testWidgets('UpgradeWizard renders stepper', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: UpgradeWizard(),
          ),
        ),
      ),
    );
    expect(find.byType(Stepper), findsOneWidget);
    expect(find.text('Release Notes'), findsOneWidget);
  });

  testWidgets('BillingWizard renders stepper', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: BillingWizard(),
          ),
        ),
      ),
    );
    expect(find.byType(Stepper), findsOneWidget);
    expect(find.text('Usage'), findsOneWidget);
  });
}
