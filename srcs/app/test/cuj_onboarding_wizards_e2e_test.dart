import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';
import 'package:ohc_app/screens/agent_hire_wizard_screen.dart';
import 'package:ohc_app/screens/prompt_tuning_wizard_screen.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/settings_service.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';

class MockHttpClient extends Mock implements http.Client {}

class FakeUri extends Fake implements Uri {}

class _SuccessAuthNotifier extends AuthNotifier {
  @override
  Future<AuthUser?> build() async =>
      const AuthUser(id: '1', email: 'test@example.com', name: 'Test User', role: 'admin', organizationId: '1', token: 'fake_token');
  @override
  Future<void> login(String email, String password) async {}
  @override
  Future<void> logout() async {}
}

class _SuccessAuthService extends AuthService {
  _SuccessAuthService() : super(baseUrl: 'http://localhost');
}

class _FakeClientSettingsNotifier extends ClientSettingsNotifier {
  _FakeClientSettingsNotifier(Ref ref) : super(ref) {
    state = const AsyncData(
      ClientSettings(backendUrl: 'http://localhost', standaloneMode: false),
    );
  }
}

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  testWidgets('E2E: Onboarding Wizards (Business Setup)', (WidgetTester tester) async {
    final mockClient = MockHttpClient();
    final api = ApiService(
      baseUrl: 'http://localhost',
      token: 'tok',
      client: mockClient,
    );

    final router = GoRouter(
      initialLocation: '/',
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const BusinessSetupWizardScreen(),
        ),
      ],
    );

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(api),
          clientSettingsProvider.overrideWith((ref) => _FakeClientSettingsNotifier(ref)),
          authStateProvider.overrideWith(() => _SuccessAuthNotifier()),
          authServiceProvider.overrideWith((ref) => _SuccessAuthService()),
        ],
        child: MaterialApp.router(routerConfig: router),
      ),
    );
    await tester.pumpAndSettle();

    // Verify Business Setup Welcome
    expect(find.text('Your business, live in minutes.'), findsOneWidget);
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 1: Business Type
    expect(find.text('Business Type'), findsOneWidget);
    await tester.tap(find.text('Online Store'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();
  });

  testWidgets('E2E: AI Agent Config & Prompt Tuning Wizard', (WidgetTester tester) async {
    final mockClient = MockHttpClient();
    final api = ApiService(
      baseUrl: 'http://localhost',
      token: 'tok',
      client: mockClient,
    );

    when(() => mockClient.get(any(), headers: any(named: 'headers')))
        .thenAnswer((_) async => http.Response('{"providers":[], "roles":[]}', 200));

    final router = GoRouter(
      initialLocation: '/',
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const AgentHireWizardScreen(),
        ),
        GoRoute(
          path: '/agents/:id/tune',
          builder: (context, state) => PromptTuningWizardScreen(
            agentId: state.pathParameters['id'] ?? 'unknown',
          ),
        ),
      ],
    );

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(api),
          clientSettingsProvider.overrideWith((ref) => _FakeClientSettingsNotifier(ref)),
        ],
        child: MaterialApp.router(routerConfig: router),
      ),
    );
    await tester.pumpAndSettle();

    // Step 1: Agent Gallery
    expect(find.text('Step 1 — Select Agent Role'), findsOneWidget);
  });
}
