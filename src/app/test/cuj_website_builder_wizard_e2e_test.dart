import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:mocktail/mocktail.dart';
import 'package:http/http.dart' as http;
import 'package:ohc_app/screens/login_screen.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/settings_service.dart';
import 'package:ohc_app/screens/website_builder_wizard_screen.dart';

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

  testWidgets('E2E Test: WebsiteBuilderWizardScreen flow', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1200, 1000));
    final router = GoRouter(
      initialLocation: '/login',
      routes: [
        GoRoute(path: '/login', builder: (context, state) => const _FakeLoginScreen()),
        GoRoute(path: '/dashboard', builder: (context, state) => Scaffold(body: ElevatedButton(onPressed: () => context.go('/wizard'), child: const Text('Start')))),
        GoRoute(path: '/wizard', builder: (context, state) => const WebsiteBuilderWizardScreen()),
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

    expect(find.text('Choose a template'), findsOneWidget);
    await tester.tap(find.text('E-commerce'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Use this template →'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    await tester.enterText(find.byType(TextField).first, 'Product 1');
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Use my own domain'));
    await tester.pumpAndSettle();
    await tester.enterText(find.byType(TextField).first, 'custom.com');
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Go Live: Review your site'), findsOneWidget);
    await tester.tap(find.text('Publish'));
    await tester.pumpAndSettle();
  });
}
