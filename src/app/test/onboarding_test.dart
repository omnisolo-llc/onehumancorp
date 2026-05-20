import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/screens/onboarding.dart';

void main() {
  testWidgets('Onboarding Screen - Welcome State UI components present', (WidgetTester tester) async {
    // To avoid overflow errors on small virtual test screens
    tester.view.physicalSize = const Size(1080, 2400);
    tester.view.devicePixelRatio = 3.0;

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen()));
    await tester.pumpAndSettle();

    expect(find.text('What are you building today?'), findsOneWidget);
    expect(find.text('Review and add any extra details to help our AI generate the perfect store.'), findsOneWidget);
    expect(find.text('Build My Storefront'), findsOneWidget);

    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
  });
}
