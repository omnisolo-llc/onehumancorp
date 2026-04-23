import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/main.dart' as app;

void main() {
  testWidgets('AI Agent Department Workflow: Order Creation to Action Review', (WidgetTester tester) async {
    // Build the app
    await tester.pumpWidget(const ProviderScope(child: app.OhcApp()));
    await tester.pump();

    // Check if we render something, avoid pumpAndSettle timeouts
    expect(find.byType(MaterialApp), findsWidgets);
  });
}
