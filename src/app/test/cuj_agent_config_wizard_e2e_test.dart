import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:mocktail/mocktail.dart';
import 'package:http/http.dart' as http;
import 'package:ohc_app/screens/login_screen.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/settings_service.dart';
import 'package:ohc_app/screens/agent_config_wizard_screen.dart';

class MockHttpClient extends Mock implements http.Client {}
class FakeUri extends Fake implements Uri {}
class _SuccessAuthNotifier extends AuthNotifier {
  @override
  Future<AuthUser?> build() async => null;
  @override
  Future<void> login(String email, String password) async {
    state = const AsyncData(AuthUser(id: 'u1', email: 'user@example.com', name: 'Test User', role: 'admin', organizationId: 'org-1', token: 'tok_123'));
  }
}

class _FakeLoginScreen extends StatelessWidget {
  const _FakeLoginScreen();
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Column(
        children: [
          const TextField(),
          const TextField(),
          ElevatedButton(
            onPressed: () => context.go('/dashboard'),
            child: const Text('Sign In'),
          ),
        ],
      ),
    );
  }
}

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  testWidgets('E2E Test: AgentConfigWizardScreen flow', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1200, 1000));
    final router = GoRouter(
      initialLocation: '/login',
      routes: [
        GoRoute(path: '/login', builder: (context, state) => const _FakeLoginScreen()),
        GoRoute(path: '/dashboard', builder: (context, state) => Scaffold(body: ElevatedButton(onPressed: () => context.go('/wizard'), child: const Text('Start')))),
        GoRoute(path: '/wizard', builder: (context, state) => const AgentConfigWizardScreen()),
      ],
    );

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          authStateProvider.overrideWith(() => _SuccessAuthNotifier()),
          clientSettingsProvider.overrideWith((ref) => ClientSettingsNotifier(ref)..state = const AsyncValue.data(ClientSettings(backendUrl: 'http://localhost', standaloneMode: false))),
        ],
        child: MaterialApp.router(routerConfig: router),
      ),
    );

    await tester.pumpAndSettle();
    await tester.enterText(find.byType(TextField).first, 'test@example.com');
    await tester.enterText(find.byType(TextField).last, 'password');
    await tester.tap(find.text('Sign In'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Start'));
    await tester.pumpAndSettle();

    expect(find.text('Choose an agent to add to your team'), findsOneWidget);
    await tester.tap(find.text('Customer Support'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Reply to customer messages'));
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Review & Activate'), findsOneWidget);
    await tester.tap(find.text('Activate'));
    await tester.pump(const Duration(seconds: 5));
    await tester.pumpAndSettle();
  });
}
