import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/router.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/screens/my_plan_screen.dart';
import 'package:ohc_app/screens/pricing_screen.dart';

class MockHttpClient extends Mock implements http.Client {}
class FakeUri extends Fake implements Uri {}

Map<String, dynamic> _fakeDashboard() => {
  'organization': {
    'id': 'org-1',
    'name': 'Test Corp',
    'domain': 'test.com',
    'tier': 'Free',
    'members': [],
  },
  'meetings': [],
  'costs': {
    'totalCostUsd': 123.45,
    'totalTokens': 1000,
    'totalActions': 50,
    'breakdown': {},
  },
  'storage': {
    'usedBytes': 50000000,
    'limitBytes': 500000000,
  },
  'agents': [],
  'statuses': [],
  'updatedAt': DateTime.now().toIso8601String(),
};

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  group('CUJ: My Plan & Pricing Dashboard', () {
    testWidgets('starts from home, navigates to my plan, then pricing', (tester) async {
      await tester.binding.setSurfaceSize(const Size(1440, 900));

      final mockClient = MockHttpClient();
      when(() => mockClient.get(any(), headers: any(named: 'headers')))
          .thenAnswer((_) async => http.Response(jsonEncode(_fakeDashboard()), 200));

      final api = ApiService(baseUrl: 'http://localhost', token: 'tok', client: mockClient);
      final authService = AuthService(baseUrl: 'http://localhost', client: mockClient);

      final router = GoRouter(
        initialLocation: '/my-plan',
        routes: [
          GoRoute(
            path: '/my-plan',
            builder: (context, state) => const MyPlanScreen(),
          ),
          GoRoute(
            path: '/pricing',
            builder: (context, state) => const PricingScreen(),
          ),
        ],
      );

      await tester.pumpWidget(ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(api),
          authServiceProvider.overrideWithValue(authService),
        ],
        child: MaterialApp.router(routerConfig: router),
      ));
      await tester.pumpAndSettle();

      await tester.pumpAndSettle();
      // Verify My Plan screen
      expect(find.text('Free'), findsWidgets);
      expect(find.textContaining('AI Actions'), findsOneWidget);

      // Tap Upgrade and navigate to Pricing
      await tester.tap(find.text('Upgrade'));
      await tester.pumpAndSettle();

      expect(find.text('Pricing & Billing'), findsOneWidget);
    });
  });
}
