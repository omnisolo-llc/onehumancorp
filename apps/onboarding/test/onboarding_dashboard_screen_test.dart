import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_onboarding_app/screens/onboarding_dashboard_screen.dart';

void main() {
  testWidgets('OnboardingDashboardScreen renders successfully with styling', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: OnboardingDashboardScreen()));

    expect(find.text('Day One Setup Audit'), findsOneWidget);
    expect(find.text('Environment Provisioning'), findsOneWidget);
    expect(find.text('Status: Active'), findsOneWidget);
  });
}
