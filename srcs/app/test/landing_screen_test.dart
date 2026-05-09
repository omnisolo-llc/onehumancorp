import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:app/screens/landing_screen.dart';

void main() {
  testWidgets('Landing Screen displays OS specific buttons', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: LandingScreen()));

    expect(find.text('Welcome to One Human Corp'), findsOneWidget);

    expect(find.byKey(const Key('downloadMacBtn')), findsOneWidget);
    expect(find.text('Download for Mac'), findsOneWidget);

    expect(find.byKey(const Key('downloadWindowsBtn')), findsOneWidget);
    expect(find.text('Download for Windows'), findsOneWidget);

    expect(find.byKey(const Key('downloadLinuxBtn')), findsOneWidget);
    expect(find.text('Download for Linux'), findsOneWidget);
  });

  testWidgets('Landing Screen layout matches design intent', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: LandingScreen()));
    expect(find.text('Welcome to One Human Corp'), findsOneWidget);
    expect(find.byType(ElevatedButton), findsNWidgets(3));
  });

  testWidgets('Mac Download Button correctly taps', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: LandingScreen()));
    await tester.tap(find.byKey(const Key('downloadMacBtn')));
    await tester.pump();
  });

  testWidgets('Windows Download Button correctly taps', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: LandingScreen()));
    await tester.tap(find.byKey(const Key('downloadWindowsBtn')));
    await tester.pump();
  });

  testWidgets('Linux Download Button correctly taps', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: LandingScreen()));
    await tester.tap(find.byKey(const Key('downloadLinuxBtn')));
    await tester.pump();
  });

  testWidgets('Continue to Setup navigates to BusinessSetupWizardScreen', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: LandingScreen()));
    await tester.tap(find.byKey(const Key('continueSetupBtn')));
    await tester.pump(const Duration(milliseconds: 500));
    await tester.pump(const Duration(milliseconds: 500));
    expect(find.text('Setup your Business'), findsOneWidget);
  });
}
