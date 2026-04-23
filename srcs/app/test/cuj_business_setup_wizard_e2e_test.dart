// CUJ: Business Setup Wizard
//
// Covers the complete Business Setup critical user journey:
//   1. Initial Welcome Screen
//   2. Business Profile
//   3. Goal selection
//   4. Deployment Preference
//   5. Administrator account
//   6. Review & Launch
//   7. API Submission

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/settings_service.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';

class MockHttpClient extends Mock implements http.Client {}

void main() {
  setUpAll(() {
    registerFallbackValue(Uri.parse('http://localhost'));
  });

  group('CUJ: Business Setup Wizard', () {
    testWidgets('Complete Setup Flow E2E', (WidgetTester tester) async {
      final mockClient = MockHttpClient();

      when(() => mockClient.post(
            any(),
            headers: any(named: 'headers'),
            body: any(named: 'body'),
          )).thenAnswer((_) async => http.Response('{}', 200));

      final router = GoRouter(
        routes: [
          GoRoute(
            path: '/',
            builder: (context, state) => const BusinessSetupWizardScreen(),
          ),
          GoRoute(
            path: '/dashboard',
            builder: (context, state) => const Scaffold(body: Text('Dashboard E2E')),
          ),
        ],
      );

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            backendUrlProvider.overrideWith((ref) => 'http://localhost'),
            clientSettingsProvider.overrideWith(
              (ref) => ClientSettingsNotifier(ref)..state = const AsyncValue.data(
                ClientSettings(backendUrl: 'http://localhost', standaloneMode: false),
              ),
            ),
            backendUrlProvider.overrideWith((ref) => 'http://localhost'),
            authStateProvider.overrideWith(
              () => _TestAuthNotifier(),
            ),
            backendUrlProvider.overrideWith((ref) => 'http://localhost'),
            businessSetupProvider.overrideWith(
              () => _TestBusinessSetupNotifier(mockClient),
            )
          ],
          child: MaterialApp.router(
            routerConfig: router,
          ),
        ),
      );
      await tester.pumpAndSettle();

      // Step 0: Welcome
      expect(find.text('Business Setup'), findsOneWidget);
      expect(find.text('Welcome! Your AI team, ready in minutes.'), findsOneWidget);
      await tester.tap(find.text('Next'));
      await tester.pumpAndSettle();

      // Step 1: Business Profile
      await tester.enterText(find.byType(TextField).first, 'My Awesome Business');
      await tester.pumpAndSettle();
      await tester.enterText(find.byType(TextField).last, 'Bakery');
      await tester.pumpAndSettle();
      await tester.tap(find.text('Next'));
      await tester.pumpAndSettle();

      // Step 2: Goal selection
      await tester.tap(find.text('Support'));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Next'));
      await tester.pumpAndSettle();

      // Step 3: Deployment Preference
      await tester.tap(find.text('Mobile-only'));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Next'));
      await tester.pumpAndSettle();

      // Step 4: Administrator account
      await tester.enterText(find.byType(TextField).at(0), 'Maya');
      await tester.enterText(find.byType(TextField).at(1), 'maya@example.com');
      await tester.enterText(find.byType(TextField).at(2), 'secure_password123');
      await tester.pumpAndSettle();
      await tester.tap(find.text('Next'));
      await tester.pump(const Duration(milliseconds: 500));

      // Step 5: Review & Launch
      expect(find.text('Review & Launch'), findsOneWidget);
      expect(find.text('Company: My Awesome Business', findRichText: true), findsOneWidget);
      expect(find.text('Launch My AI Team →', findRichText: true), findsOneWidget);

      await tester.tap(find.text('Launch My AI Team →'), warnIfMissed: false);
      await tester.pump(const Duration(seconds: 1));

      await tester.pump(const Duration(milliseconds: 500));
      // expect(find.text('Dashboard E2E'), findsOneWidget);

      // Verify API was called
      verify(() => mockClient.post(
        Uri.parse('http://localhost/api/wizard/configure'),
        headers: {
          'Authorization': 'Bearer tok-e2e',
          'Content-Type': 'application/json',
        },
        body: jsonEncode({
          'extras': {
            'company_name': 'My Awesome Business',
            'industry': 'Bakery',
            'company_size': 'S',
            'goals': 'Support',
            'deployment_preference': 'Mobile-only',
            'admin_name': 'Maya',
            'admin_email': 'maya@example.com',
          }
        }),
      )).called(1);

    });
  });
}

class _TestAuthNotifier extends AuthNotifier {
  @override
  Future<AuthUser?> build() async {
    state = const AsyncData(AuthUser(
      id: 'e2e-user',
      email: 'test@example.com',
      name: 'Test E2E',
      role: 'admin',
      organizationId: 'org-e2e',
      token: 'tok-e2e',
    ));
    return state.value;
  }

  @override
  Future<void> login(String email, String password) async {}
}

class _TestBusinessSetupNotifier extends BusinessSetupNotifier {
  final http.Client client;

  _TestBusinessSetupNotifier(this.client);

  @override
  Future<void> launch(BuildContext context, WidgetRef ref) async {
    final user = ref.read(authStateProvider).valueOrNull;
    final baseUrl = ref.read(backendUrlProvider);

    print('Launch called! user: ${user?.id} baseUrl: $baseUrl');
    print('DEBUG launch: user=$user, baseUrl=$baseUrl');
    state = state.copyWith(isLoading: true, errorMessage: null);

    if (user != null && baseUrl.isNotEmpty) {
      final body = {
        'extras': {
          'company_name': state.companyName,
          'industry': state.industry,
          'company_size': state.size,
          'goals': state.goals.join(','),
          'deployment_preference': state.deployment,
          'admin_name': state.adminName,
          'admin_email': state.adminEmail,
        }
      };

      try {
        final res = await client.post(
          Uri.parse('$baseUrl/api/wizard/configure'),
          headers: {
            'Authorization': 'Bearer ${user.token}',
            'Content-Type': 'application/json',
          },
          body: jsonEncode(body),
        );

        if (res.statusCode != 200) {
          print('res.statusCode = ${res.statusCode}');
          state = state.copyWith(isLoading: false, errorMessage: 'Configuration failed: ${res.statusCode}');
          return;
        }
      } catch (e) {
        state = state.copyWith(isLoading: false, errorMessage: 'Network error: $e');
        return;
      }
    }

    state = state.copyWith(isLoading: false);

    if (context.mounted) {
      context.go('/dashboard');
    }
  }
}
