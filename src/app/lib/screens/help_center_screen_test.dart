import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/help_center_screen.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

void main() {
  testWidgets('HelpCenterScreen renders topics and Ask AI button', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: HelpCenterScreen())));
    await tester.pumpAndSettle();
    expect(find.text('Help Center'), findsOneWidget);
    expect(find.text('Ask AI'), findsOneWidget);
    expect(find.text('Getting Started'), findsOneWidget);
    expect(find.text('My Store'), findsOneWidget);
    expect(find.text('Payments'), findsOneWidget);
    expect(find.text('AI Agents'), findsOneWidget);
    expect(find.text('Marketing'), findsOneWidget);
    expect(find.text('Account & Billing'), findsOneWidget);
  });
}
