import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/login_screen.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';
import 'package:ohc_app/screens/welcome_checklist_screen.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/settings_service.dart';

class _MockApiService extends Fake implements ApiService {
  @override
  Future<void> configureWizard(Map<String, dynamic> payload) async {
    // Mock the API call
  }
}

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
  testWidgets('CUJ: Full Onboarding Flow (Login -> Setup -> Checklist)', (tester) async {
    final router = GoRouter(
      initialLocation: '/login',
      routes: [
        GoRoute(
          path: '/login',
          builder: (context, state) => const LoginScreen(),
        ),
        GoRoute(
          path: '/business_setup',
          builder: (context, state) => const BusinessSetupWizardScreen(),
        ),
        GoRoute(
          path: '/welcome_checklist',
          builder: (context, state) => const WelcomeChecklistScreen(),
        ),
        GoRoute(
          path: '/dashboard',
          builder: (context, state) => const Scaffold(body: Text('Dashboard')),
        ),
      ],
    );

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          authStateProvider.overrideWith(() => _SuccessAuthNotifier()),
          apiServiceProvider.overrideWithValue(_MockApiService()),
          clientSettingsProvider.overrideWith(
            (ref) => ClientSettingsNotifier(ref)..state = const AsyncValue.data(
              ClientSettings(backendUrl: 'http://localhost', standaloneMode: false),
            ),
          ),
        ],
        child: MaterialApp.router(
          routerConfig: router,
        ),
      ),
    );

    await tester.pumpAndSettle();

    // 1. Login (SignUp mode to trigger onboarding)
    expect(find.byType(LoginScreen), findsOneWidget);

    // Toggle to Sign Up to trigger /business_setup logic
    await tester.tap(find.text("Don't have an account? Sign Up").first);
    await tester.pumpAndSettle();

    final emailField = tester.widgetList<TextFormField>(find.byType(TextFormField)).first;
    await tester.enterText(find.byWidget(emailField), 'user@example.com');
    await tester.pumpAndSettle();

    final passwordField = tester.widgetList<TextFormField>(find.byType(TextFormField)).last;
    await tester.enterText(find.byWidget(passwordField), 'password123');
    await tester.pumpAndSettle();

    // Now the button says Sign Up. It uses FilledButton, which has text inside.
    await tester.tap(find.widgetWithText(FilledButton, 'Sign Up').first);
    await tester.pumpAndSettle();

    // 2. Business Setup Wizard
    await tester.pumpAndSettle();
    expect(find.byType(BusinessSetupWizardScreen), findsOneWidget);
    expect(find.text('Welcome! Your AI team, ready in minutes.'), findsOneWidget);

    // Step 0 -> 1
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 1: Business Profile
    expect(find.byType(TextField), findsNWidgets(2)); // Business Type, Company Name
    await tester.enterText(find.byType(TextField).first, 'Baker');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2: What they sell
    expect(find.byType(TextField), findsOneWidget); // Products Services
    await tester.enterText(find.byType(TextField).first, 'Cakes');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3: Template
    expect(find.byType(DropdownButtonFormField<String>), findsOneWidget);
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 4: First Product
    expect(find.byType(TextField), findsNWidgets(3));
    await tester.enterText(find.byType(TextField).first, 'Chocolate Cake');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 5: Domain
    expect(find.byType(TextField), findsNWidgets(1));
    await tester.enterText(find.byType(TextField).first, 'mybakery');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Launch My AI Team →'));
    await tester.pumpAndSettle();

    // 3. Welcome Checklist
    expect(find.byType(WelcomeChecklistScreen), findsOneWidget);
    expect(find.text("You're set up! Here's what to do next:"), findsOneWidget);

    // Finish checklist and go to dashboard
    await tester.tap(find.text('Go to Dashboard'));
    await tester.pumpAndSettle();

    expect(find.text('Dashboard'), findsOneWidget);
  });
}
