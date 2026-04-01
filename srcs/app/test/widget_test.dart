// This is a basic Flutter widget test.
//
// To perform an interaction with a widget in your test, use the WidgetTester
// utility in the flutter_test package. For example, you can send tap and scroll
// gestures. You can also use WidgetTester to find child widgets in the widget
// tree, read text, and verify that the values of widget properties are correct.

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:ohc_app/main.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

void main() {
  testWidgets('Counter increments smoke test', (WidgetTester tester) async {
    // Build our app and trigger a frame.
    await tester.pumpWidget(const ProviderScope(child: OhcApp()));

    // Wait for the app to render fully, especially since there might be async initialization
    await tester.pumpAndSettle();

    // Verify that the title of the app renders since it's the OhcApp router.
    // The default counter test template is meaningless if we don't have a counter.
    // Let's just expect the router or app to spin up successfully without crashing.
    expect(find.byType(MaterialApp), findsOneWidget);
  });
}
