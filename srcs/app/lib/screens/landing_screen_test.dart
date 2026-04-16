import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:ohc_app/screens/landing_screen.dart';

void main() {
  testWidgets('Landing screen displays key local-first features', (tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: LandingScreen(),
      ),
    );

    expect(find.text('The Hybrid Agentic OS'), findsOneWidget);
    expect(find.text('Zero Data Leakage'), findsOneWidget);
    expect(find.text('Air-Gapped Autonomy'), findsOneWidget);
    expect(find.text('Viral Referral Loop'), findsOneWidget);
    expect(find.text('Start Business Setup'), findsOneWidget);
    expect(find.text('Or continue to Cloud Dashboard'), findsOneWidget);
  });
}
