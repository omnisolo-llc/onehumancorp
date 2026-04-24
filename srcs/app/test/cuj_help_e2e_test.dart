// CUJ: Help & Documentation
//
// Covers the full end-to-end critical user journey for documentation features:
//   1. Help Chat FAB is visible globally
//   2. Tooltips are visible on key interactive elements
//   3. Help Center screen is accessible and searchable
//   4. Release Notes screen displays changelog

import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/help_center_screen.dart';
import 'package:ohc_app/screens/release_notes_screen.dart';
import 'package:ohc_app/widgets/help_chat_fab.dart';
import 'package:ohc_app/widgets/ohc_tooltip.dart';

// ── Mocks & Fakes ─────────────────────────────────────────────────────────

class MockHttpClient extends Mock implements http.Client {}
class FakeUri extends Fake implements Uri {}

Widget _wrapScreen(Widget screen) {
  final router = GoRouter(
    routes: [
      GoRoute(path: '/', builder: (context, state) => Scaffold(body: screen)),
    ],
  );
  return ProviderScope(
    child: MaterialApp.router(routerConfig: router),
  );
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  group('CUJ: Documentation & Help Features', () {
    testWidgets('HelpChatFab opens and closes support chat', (tester) async {
      await tester.pumpWidget(_wrapScreen(const Stack(children: [HelpChatFab()])));
      await tester.pumpAndSettle();

      // Find the FAB by its icon or standard FAB locator
      final fab = find.byType(FloatingActionButton);
      expect(fab, findsOneWidget);

      // Verify the chat window is NOT open yet
      expect(find.text('OHC Support'), findsNothing);

      // Tap to open the chat window
      await tester.tap(fab);
      await tester.pumpAndSettle();

      // Verify the chat window IS open
      expect(find.text('Help Assistant'), findsOneWidget);
      expect(find.textContaining('How can I help you today?'), findsOneWidget);

      // Tap the close button
      final closeButton = find.byIcon(Icons.close);
      expect(closeButton, findsOneWidget);
      await tester.tap(closeButton);
      await tester.pumpAndSettle();

      // Verify the chat window is CLOSED
      expect(find.text('Help Assistant'), findsNothing);
    });

    testWidgets('OhcTooltip renders properly with its child', (tester) async {
      await tester.pumpWidget(const MaterialApp(
        home: Scaffold(
          body: Center(
            child: OhcTooltip(
              message: 'This is a test tooltip',
              child: Text('Hover me', key: Key('tooltip_text')),
            ),
          ),
        ),
      ));
      await tester.pumpAndSettle();

      // Ensure the child text is found
      final finder = find.byKey(const Key('tooltip_text'));
      expect(finder, findsOneWidget);

      // Ensure Tooltip widget exists
      final tooltipFinder = find.byType(Tooltip);
      expect(tooltipFinder, findsOneWidget);

      // Ensure tooltip message text is not visible initially
      expect(find.text('This is a test tooltip'), findsNothing);
    });

    testWidgets('HelpCenterScreen renders sections and responds to search', (tester) async {
      await tester.pumpWidget(_wrapScreen(const HelpCenterScreen()));
      await tester.pumpAndSettle();

      // Check standard sections are rendered
      expect(find.text('Help Center'), findsWidgets);
      expect(find.text('Getting Started'), findsOneWidget);
      expect(find.text('My Store'), findsOneWidget);

      // Check for search bar
      final searchField = find.byType(TextField);
      expect(searchField, findsOneWidget);

      // Enter search query
      await tester.enterText(searchField, 'Payments');
      await tester.pumpAndSettle();

      // The 'Payments' text should still be visible because it matches the query
      expect(find.textContaining('Payments'), findsWidgets);
    });

    testWidgets('ReleaseNotesScreen renders latest release info', (tester) async {
      await tester.pumpWidget(_wrapScreen(const ReleaseNotesScreen()));
      await tester.pumpAndSettle();

      // Check header - Actually title is "What's New" in the widget code.
      expect(find.text('What\'s New'), findsWidgets);

      // Check for content
      expect(find.textContaining('Latest Updates in One Human Corp'), findsWidgets);
      // Wait we used Column in SingleChildScrollView instead of ListView
      expect(find.byType(SingleChildScrollView), findsOneWidget);
    });
  });
}
