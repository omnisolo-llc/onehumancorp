
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';

Widget _wrapScreen(Widget screen) {
  final router = GoRouter(
    routes: [
      GoRoute(path: '/', builder: (context, state) => screen),
      GoRoute(path: '/dashboard', builder: (context, state) => const Scaffold(body: Text('Dashboard'))),
    ],
  );
  return ProviderScope(
    child: MaterialApp.router(routerConfig: router),
  );
}

void main() {
  group('Business Setup E2E Test', () {
    testWidgets('Full flow', (tester) async {
      tester.view.physicalSize = const Size(1920, 1080);
      tester.view.devicePixelRatio = 1.0;
      addTearDown(() {
        tester.view.resetPhysicalSize();
        tester.view.resetDevicePixelRatio();
      });

      await tester.pumpWidget(_wrapScreen(const BusinessSetupWizardScreen()));

      await tester.pumpAndSettle(const Duration(seconds: 1));

      // Wizard Step 0: Welcome
      expect(find.text('Next'), findsWidgets);

      await tester.tap(find.text('Next').first, warnIfMissed: false);
      await tester.pumpAndSettle(const Duration(seconds: 1));

      // Wizard Step 1: Company Name, Industry, Size
      expect(find.byType(TextField), findsWidgets);
      await tester.enterText(find.byType(TextField).at(0), 'Maya Cakes');
      await tester.enterText(find.byType(TextField).at(1), 'Bakery');
      await tester.tap(find.text('Next').first, warnIfMissed: false);
      await tester.pumpAndSettle(const Duration(seconds: 1));

      // Wizard Step 2: Goals
      expect(find.byType(CheckboxListTile), findsWidgets);
      await tester.tap(find.widgetWithText(CheckboxListTile, 'Marketing').first, warnIfMissed: false);
      await tester.pumpAndSettle(const Duration(seconds: 1));
      await tester.tap(find.text('Next').first, warnIfMissed: false);
      await tester.pumpAndSettle(const Duration(seconds: 1));

      // Wizard Step 3: Deployment
      // Since it's standalone, it will show Standalone Mode Detected message or Cloud if false
      expect(find.textContaining('Deployment'), findsWidgets);
      await tester.tap(find.text('Next').first, warnIfMissed: false);
      await tester.pumpAndSettle(const Duration(seconds: 1));

      // Wizard Step 4: Admin info
      expect(find.byType(TextField), findsWidgets);

      await tester.enterText(find.byType(TextField).at(0), 'Maya');
      await tester.enterText(find.byType(TextField).at(1), 'maya@example.com');
      await tester.enterText(find.byType(TextField).at(2), 'Password123');
      await tester.pumpAndSettle(const Duration(seconds: 1));

      // Launch My AI Team ->
      await tester.tap(find.text('Launch My AI Team →'), warnIfMissed: false);
      await tester.pumpAndSettle(const Duration(seconds: 4));

      // Assert we reach the dashboard after launch completes
      expect(find.textContaining('Dashboard'), findsWidgets);
    });
  });
}
