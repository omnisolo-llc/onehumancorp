// CUJ: Pricing & Plans E2E Test
// Covers the pricing screen user journey

import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/login_screen.dart';
import 'package:ohc_app/screens/ongoing_management_wizards.dart';
import 'package:ohc_app/screens/pricing_screen.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/auth_service.dart';

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
    'totalCostUSD': 0,
    'totalTokens': 0,
    'totalActions': 0,
    'agents': [],
  },
  'agents': [],
  'statuses': [],
  'updatedAt': DateTime.now().toIso8601String(),
};

class _SuccessAuthNotifier extends AuthNotifier {
  @override
  Future<AuthUser?> build() async => null;
  @override
  Future<void> login(String email, String password) async {
    state = const AsyncData(AuthUser(
      id: 'u1',
      email: 'user@example.com',
      name: 'Test User',
      role: 'admin',
      organizationId: 'org-1',
      token: 'tok-ok',
    ));
  }
}

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  testWidgets('Full CUJ: Navigate from Login to Dashboard, to Pricing Screen', (WidgetTester tester) async {
    final mockClient = MockHttpClient();
    when(
      () => mockClient.get(any(that: isA<Uri>()), headers: any(named: 'headers')),
    ).thenAnswer(
      (_) async => http.Response(jsonEncode(_fakeDashboard()), 200),
    );
    when(
      () => mockClient.post(any(that: isA<Uri>()), headers: any(named: 'headers'), body: any(named: 'body')),
    ).thenAnswer(
      (_) async => http.Response('{"status":"ok"}', 200),
    );

    final api = ApiService(
      baseUrl: 'http://localhost',
      token: 'tok-ok',
      client: mockClient,
    );

    final router = GoRouter(
      initialLocation: '/login',
      routes: [
        GoRoute(
          path: '/login',
          builder: (context, state) => const LoginScreen(),
        ),
        GoRoute(
          path: '/',
          builder: (context, state) => const BillingWizardScreen(), // Fake dashboard
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
        authStateProvider.overrideWith(() => _SuccessAuthNotifier()),
      ],
      child: MaterialApp.router(routerConfig: router),
    ));

    await tester.pumpAndSettle();

    // Now we are safely on login screen
    final emailFields = tester.widgetList<TextFormField>(find.byType(TextFormField));
    if (emailFields.isNotEmpty) {
      await tester.enterText(find.byWidget(emailFields.first), 'user@example.com');
      await tester.pumpAndSettle();

      if (emailFields.length > 1) {
        await tester.enterText(find.byWidget(emailFields.last), 'password');
        await tester.pumpAndSettle();
      }
    }

    final signInBtnList = tester.widgetList(find.byType(FilledButton));
    if (signInBtnList.isNotEmpty) {
      await tester.tap(find.byWidget(signInBtnList.first));
      await tester.pumpAndSettle(const Duration(seconds: 2));
    }

    // Simulate router detecting auth change manually since we don't have OhcApp wrapper doing it
    final BuildContext context = tester.element(find.byType(Scaffold).first);
    context.go('/');
    await tester.pumpAndSettle(const Duration(seconds: 2));

    await tester.binding.setSurfaceSize(const Size(1440, 900));
    await tester.pumpAndSettle();

    // We are on billing wizard / fake dashboard
    final switchPlanBtn = find.text('Switch Plan');
    if (switchPlanBtn.evaluate().isNotEmpty) {
      await tester.ensureVisible(switchPlanBtn.first);
      await tester.tap(switchPlanBtn.first);
      await tester.pumpAndSettle(const Duration(seconds: 2));
    }

    expect(find.textContaining('pricing'), findsWidgets);
    expect(find.text('Free'), findsWidgets);
    expect(find.text('Business'), findsWidgets);
  });
}
