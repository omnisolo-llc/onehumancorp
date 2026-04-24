import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/main.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/router.dart';

void main() {
  testWidgets('E2E: CUJ - Help Documentation Flow', (WidgetTester tester) async {
    // Start the application
    await tester.pumpWidget(const ProviderScope(child: OhcApp()));
    await tester.pumpAndSettle();

    // Verify we are at the Landing screen initially
    if (find.text('Login').evaluate().isNotEmpty) {
      // Tap Login to go to login screen
      await tester.tap(find.text('Login').first);
      await tester.pumpAndSettle();

      // Enter credentials
      await tester.enterText(find.byType(TextField).first, 'admin@onehumancorp.com');
      await tester.enterText(find.byType(TextField).last, 'admin');
      await tester.pumpAndSettle();

      // Tap Login
      await tester.tap(find.text('Login').last);
      await tester.pumpAndSettle();
    }

    // We might be in the business setup wizard or dashboard
    // If in wizard, skip to dashboard
    if (find.text('Let\'s get your business online in minutes.').evaluate().isNotEmpty) {
        // Just mock the location
        final router = ProviderScope.containerOf(tester.element(find.byType(OhcApp))).read(routerProvider);
        router.go('/dashboard');
        await tester.pumpAndSettle();
    }

    // Look for the Help icon button in the AppBar. It's now visible.

    // We'll navigate to /help via the router since sidebars/appbars might be collapsed on mobile view during test
    final router = ProviderScope.containerOf(tester.element(find.byType(OhcApp))).read(routerProvider);
    router.go('/help');
    await tester.pumpAndSettle();

    // Verify we are on the Help Center screen
    expect(find.text('How can we help you?'), findsOneWidget);
    expect(find.text('Getting Started'), findsOneWidget);

    // Check for the AI Help Chat FAB
    final aiHelpFabFinder = find.byIcon(Icons.support_agent);
    expect(aiHelpFabFinder, findsOneWidget);

    // Tap the Release Notes card to navigate to Changelog.
    // We scroll it into view first.
    await tester.ensureVisible(find.text('Release Notes'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Release Notes'), warnIfMissed: false);
    await tester.pumpAndSettle();

    // Verify we are on the Release Notes screen
    expect(find.text('What\'s New'), findsOneWidget);
    expect(find.text('Version 1.4.2'), findsOneWidget);
  });
}
