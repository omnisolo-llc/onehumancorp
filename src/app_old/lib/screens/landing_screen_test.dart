import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/landing_screen.dart';

void main() {
  testWidgets('Landing screen displays key local-first features', (tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: LandingScreen(),
        ),
      ),
    );

    expect(find.text('The Hybrid Agentic OS'), findsOneWidget);
    expect(find.text('Data Privacy'), findsOneWidget);
    expect(find.text('Works Offline'), findsOneWidget);
    expect(find.text('Referrals'), findsOneWidget);
    expect(find.text('Download for Mac'), findsOneWidget);
    expect(find.text('Download for Windows'), findsOneWidget);
    expect(find.text('Download for Linux'), findsOneWidget);
  });
}
