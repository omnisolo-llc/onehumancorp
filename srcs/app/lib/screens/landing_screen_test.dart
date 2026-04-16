import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/landing_screen.dart';

void main() {
  testWidgets('Landing screen shows app mode entry options', (tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: LandingScreen(),
        ),
      ),
    );

    expect(find.text('One Human Corp App'), findsOneWidget);
    expect(find.text('Standalone Mode'), findsOneWidget);
    expect(find.text('Cloud Mode'), findsOneWidget);
    expect(find.text('Open App'), findsOneWidget);
    expect(find.text('Switch to Cloud mode'), findsOneWidget);
  });
}
