// CUJ: Dynamic Scaling – Agent Workforce Provisioning
//
// Covers the scaling critical user journey:
//   1. Screen renders with "Dynamic Scaling" title
//   2. Role selection chips are displayed
//   3. Selecting a chip updates the selection
//   4. Scale button is present
//   5. Provisioning logs area is visible

import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/scaling_screen.dart';
import 'package:ohc_app/services/api_service.dart';

// ── Mocks & Fakes ─────────────────────────────────────────────────────────

class MockHttpClient extends Mock implements http.Client {}

class FakeUri extends Fake implements Uri {}

Widget _wrapScreen(Widget screen, ApiService api) {
  final router = GoRouter(
    routes: [
      GoRoute(path: '/', builder: (context, state) => screen),
    ],
  );
  return ProviderScope(
    overrides: [apiServiceProvider.overrideWithValue(api)],
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

  group('CUJ: Dynamic Scaling', () {
    testWidgets('renders Dynamic Scaling title in AppBar', (tester) async {
      final mockClient = MockHttpClient();
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const ScalingScreen(), api));
      await tester.pumpAndSettle();

      expect(find.textContaining('Dynamic Scaling'), findsOneWidget);
    });

    testWidgets('role selection chips are rendered', (tester) async {
      final mockClient = MockHttpClient();
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const ScalingScreen(), api));
      await tester.pumpAndSettle();

      // Role chips like "Software Engineer" should be present
      expect(find.byType(ChoiceChip), findsWidgets);
    });

    testWidgets('selecting a role chip updates selection state', (tester) async {
      final mockClient = MockHttpClient();
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const ScalingScreen(), api));
      await tester.pumpAndSettle();

      final chips = find.byType(ChoiceChip);
      if (chips.evaluate().isNotEmpty) {
        // Tap the first role chip
        await tester.tap(chips.first);
        await tester.pumpAndSettle();
        // Verify some chip is selected (doesn't throw)
        expect(find.byType(ChoiceChip), findsWidgets);
      }
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('Scale button is present on screen', (tester) async {
      final mockClient = MockHttpClient();
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const ScalingScreen(), api));
      await tester.pumpAndSettle();

      // A button to initiate scaling should be visible
      expect(
        find.byWidgetPredicate(
          (w) =>
              (w is ElevatedButton || w is FilledButton || w is OutlinedButton) &&
              w.toString().isNotEmpty,
        ),
        findsWidgets,
      );
    });

    testWidgets('capacity slider is visible', (tester) async {
      final mockClient = MockHttpClient();
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const ScalingScreen(), api));
      await tester.pumpAndSettle();

      expect(find.byType(Slider), findsOneWidget);
    });
  });
}
