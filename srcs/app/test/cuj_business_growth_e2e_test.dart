// CUJ: Business Growth Features – Share, Viral Storefront, and Referrals
//
// Covers the newly added growth-focused user journeys:
//   1. User can click "Share my business" on the dashboard and verify link is copied.
//   2. User can view the "Live Storefront" from the dashboard and see the viral footer.
//   3. User can click the "Share OHC & Get Pro" button to copy their referral link.

import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:flutter/services.dart';

class MockHttpClient extends Mock implements http.Client {}
class FakeUri extends Fake implements Uri {}

Widget _wrapScreen(Widget screen, {ApiService? api}) {
  final router = GoRouter(
    routes: [
      GoRoute(path: '/', builder: (context, state) => screen),
      GoRoute(path: '/wizards/upgrade', builder: (context, state) => const Scaffold(body: Text('Upgrade'))),
      GoRoute(path: '/wizards/billing', builder: (context, state) => const Scaffold(body: Text('Billing'))),
    ],
  );
  return ProviderScope(
    overrides: [
      if (api != null) apiServiceProvider.overrideWithValue(api),
    ],
    child: MaterialApp.router(routerConfig: router),
  );
}

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  group('CUJ: Business Growth Features', () {
    late ApiService mockApi;
    late MockHttpClient mockClient;

    setUp(() {
      mockClient = MockHttpClient();
      mockApi = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      final mockDashboardJson = {
        'organization': {
          'id': 'org-1',
          'name': 'Maya\\'s Cakes',
          'tier': 'free',
          'roleProfiles': [],
          'members': [],
        },
        'agents': [],
        'meetings': [],
        'statuses': [],
        'metrics': {
          'uptime': 99.9,
          'tasksCompleted': 10,
          'activeMissions': 1,
        }
      };

      when(() => mockClient.get(any(), headers: any(named: 'headers')))
          .thenAnswer((invocation) async {
            final uri = invocation.positionalArguments[0] as Uri;
            if (uri.path.contains('/api/dashboard')) {
              return http.Response(jsonEncode(mockDashboardJson), 200);
            }
            if (uri.path.contains('/api/growth/quota')) {
              return http.Response(jsonEncode({'used': 1, 'max': 5}), 200);
            }
            return http.Response('{}', 200);
          });

      when(() => mockClient.post(any(), headers: any(named: 'headers'), body: any(named: 'body')))
          .thenAnswer((_) async => http.Response('{}', 201));
    });

    testWidgets('Dashboard displays Growth Elements and they are interactive', (tester) async {
      await tester.pumpWidget(_wrapScreen(const DashboardScreen(), api: mockApi));
      // wait for APIs
      await tester.pump(const Duration(milliseconds: 500));
      await tester.pump(const Duration(milliseconds: 500));

      // Since it's a ListView, we need to drag it to make elements visible.
      // But text matching should find elements built in the widget tree.

      final scrollable = find.byType(Scrollable);

      // 1. Verify Share My Business flow
      final shareBtn = find.text('Share my business');
      await tester.scrollUntilVisible(shareBtn, 200, scrollable: scrollable);
      expect(shareBtn, findsOneWidget);

      await tester.tap(shareBtn);
      await tester.pump(const Duration(milliseconds: 100)); // allow snackbar to show
      expect(find.text('Link copied! Share to Instagram/WhatsApp/X'), findsOneWidget);

      // Dismiss Snackbar by waiting
      await tester.pump(const Duration(seconds: 5));

      // 2. Verify View Live Storefront flow
      final storefrontBtn = find.text('View Live Storefront');
      await tester.scrollUntilVisible(storefrontBtn, 200, scrollable: scrollable);
      expect(storefrontBtn, findsOneWidget);

      await tester.tap(storefrontBtn);
      await tester.pump(const Duration(milliseconds: 500)); // allow dialog to show

      // Dialog content
      expect(find.text('Built with OHC — Start your free business →'), findsOneWidget);

      // Close dialog
      await tester.tap(find.byIcon(Icons.close));
      await tester.pump(const Duration(milliseconds: 500));

      // 3. Verify Share OHC & Get Pro
      final referralBtn = find.text('Share OHC & Get Pro');
      await tester.scrollUntilVisible(referralBtn.last, 200, scrollable: scrollable);

      // the widget has 2 texts with same string (Title and button). We want the button.
      expect(referralBtn, findsWidgets);

      await tester.tap(referralBtn.last);
      await tester.pump(const Duration(milliseconds: 100));

      expect(find.textContaining('Start your business in 10 minutes and we both get 1 month free Pro'), findsOneWidget);
    });
  });
}
