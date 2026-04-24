// CUJ: Business Setup Wizard
import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';
import 'package:ohc_app/services/settings_service.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/api_service.dart';

class MockHttpClient extends Mock implements http.Client {}
class FakeUri extends Fake implements Uri {}

class _FakeClientSettingsNotifier extends ClientSettingsNotifier {
  _FakeClientSettingsNotifier(Ref ref) : super(ref) {
    state = const AsyncData(ClientSettings(backendUrl: 'http://localhost', standaloneMode: false));
  }
}

class _SuccessAuthNotifier extends AuthNotifier {
  @override
  Future<AuthUser?> build() async => const AuthUser(
    id: 'u1',
    email: 'user@example.com',
    name: 'Test User',
    role: 'admin',
    organizationId: 'org-1',
    token: 'tok-ok',
  );
}

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
    registerFallbackValue(http.Request('POST', Uri.parse('http://localhost')));
  });

  Widget wrapWizard(MockHttpClient mockClient) {
    final router = GoRouter(
      initialLocation: '/',
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => Scaffold(
            body: Center(
              child: ElevatedButton(
                onPressed: () => context.go('/setup'),
                child: const Text('Start Setup Wizard'),
              ),
            ),
          ),
        ),
        GoRoute(
          path: '/setup',
          builder: (context, state) => const BusinessSetupWizardScreen(),
        ),
        GoRoute(
          path: '/dashboard',
          builder: (context, state) => const Scaffold(body: Text('Dashboard Mock')),
        ),
      ],
    );

    final api = ApiService(
      baseUrl: 'http://localhost',
      token: 'tok-ok',
      client: mockClient,
    );

    return ProviderScope(
      overrides: [
        apiServiceProvider.overrideWithValue(api),
        clientSettingsProvider.overrideWith((ref) => _FakeClientSettingsNotifier(ref)),
        authStateProvider.overrideWith(() => _SuccessAuthNotifier()),
      ],
      child: MaterialApp.router(routerConfig: router),
    );
  }

  group('CUJ: Business Setup Wizard E2E', () {
    testWidgets('Full successful business setup path', (tester) async {
      final mockClient = MockClient((request) async {
        return http.Response('{}', 200);
      });

      when(() => mockClient.send(any())).thenAnswer((invocation) async {
        return http.StreamedResponse(
          Stream.value(utf8.encode('{"status":"ok"}')),
          200,
        );
      });


      await tester.pumpWidget(wrapWizard(mockClient));

      await tester.pumpAndSettle();

      // Home page
      expect(find.text('Start Setup Wizard'), findsOneWidget);
      await tester.tap(find.text('Start Setup Wizard'));
      await tester.pumpAndSettle();

      // Step 0: Welcome screen
      expect(find.text('Your business, live in minutes.'), findsOneWidget);
      await tester.tap(find.text('Get Started'));
      await tester.pumpAndSettle();

      // Step 1: Business type
      expect(find.text('What kind of business are you building?'), findsOneWidget);
      await tester.tap(find.text('Online Store'));
      await tester.pumpAndSettle();
      await tester.ensureVisible(find.text('Next'));
      await tester.tap(find.text('Next'));
      await tester.pumpAndSettle();

      // Step 2: Business name and description
      expect(find.text('What is your business called?'), findsOneWidget);
      await tester.enterText(
        find.widgetWithText(TextField, 'Business Name'),
        'My Cool Shop',
      );
      await tester.pumpAndSettle();
      await tester.ensureVisible(find.text('Next'));
      await tester.tap(find.text('Next'));
      await tester.pumpAndSettle();

      // Step 3: What do you sell?
      expect(find.text('What do you sell?'), findsOneWidget);
      await tester.tap(find.text('Physical products'));
      await tester.pumpAndSettle();
      await tester.ensureVisible(find.text('Next'));
      await tester.tap(find.text('Next'));
      await tester.pumpAndSettle();

      // Step 4: Payments
      expect(find.text('How do you want to receive payments?'), findsOneWidget);
      await tester.tap(find.text('Online only'));
      await tester.pumpAndSettle();
      await tester.ensureVisible(find.text('Next'));
      await tester.tap(find.text('Next'));
      await tester.pumpAndSettle();

      // Step 5: Admin Account
      expect(find.text('Create your admin account'), findsOneWidget);
      final fields = find.byType(TextField);
      await tester.enterText(fields.at(0), 'Admin User');
      await tester.enterText(fields.at(1), 'admin@example.com');
      await tester.enterText(fields.at(2), 'password123');
      await tester.pumpAndSettle();

      // Verify SSO buttons exist
      expect(find.text('Google SSO'), findsOneWidget);
      expect(find.text('Apple SSO'), findsOneWidget);

      // Verify password strength meter updates
      expect(find.text('Strong'), findsOneWidget);

      await tester.ensureVisible(find.text('Next'));
      await tester.tap(find.text('Next'));
      await tester.pumpAndSettle();

      // Step 6: Review & Launch
      expect(find.text('Ready to launch'), findsOneWidget);
      expect(find.text('Launch My Business →'), findsOneWidget);

      await tester.ensureVisible(find.text('Launch My Business →'));
      await tester.tap(find.text('Launch My Business →'));

      // Pump frame to start loading
      await tester.pump(const Duration(milliseconds: 50));

      // We don't verify loading indicator since mock client resolves instantly and navigates.
      await tester.pumpAndSettle(const Duration(seconds: 1));

      expect(find.textContaining('Setup failed'), findsNothing);
      expect(find.textContaining('Network error'), findsNothing);
      final errorText = find.textContaining('Network error');
      if (tester.any(errorText)) {
        print('Error: ${tester.widget<Text>(errorText.first).data}');
      }
      expect(find.text('Dashboard Mock'), findsOneWidget);
    });
  });
}
