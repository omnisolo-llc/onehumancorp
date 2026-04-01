import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:ohc_app/main.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

void main() {
  testWidgets('App loads smoke test', (WidgetTester tester) async {
    // Build our app and trigger a frame.
    await tester.pumpWidget(const ProviderScope(child: OhcApp()));

    // Smoke test to ensure the app loads
    expect(find.byType(MaterialApp), findsOneWidget);
  });
}
