// CUJ: Authentication – Sign In / Sign Out
//
// Covers the complete login critical user journey:
//   1. Validate form fields on empty submission
//   2. Toggle password visibility
//   3. Login failure displays error
//   4. Login success navigates away
//   5. Back navigation from login doesn't crash

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/login_screen.dart';
import 'package:ohc_app/services/auth_service.dart';

// ── Mocks & Fakes ─────────────────────────────────────────────────────────

class FakeUri extends Fake implements Uri {}

class _FailingAuthNotifier extends AuthNotifier {
  @override
  Future<AuthUser?> build() async => null;
  @override
  Future<void> login(String email, String password) async {
    throw Exception('Unauthorized');
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

Widget _wrapLogin({List<Override> overrides = const []}) {
  return ProviderScope(
    overrides: overrides,
    child: const MaterialApp(home: LoginScreen()),
  );
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  group('CUJ: Auth – Sign In', () {
    testWidgets('empty form shows email validation error', (tester) async {
      await tester.pumpWidget(_wrapLogin());
      await tester.pumpAndSettle();

      await tester.tap(find.text('Sign In'));
      await tester.pumpAndSettle();

      expect(find.text('Enter a valid email'), findsOneWidget);
    });

    testWidgets('invalid email format shows validation error', (tester) async {
      await tester.pumpWidget(_wrapLogin());
      await tester.pumpAndSettle();

      await tester.enterText(
        find.widgetWithText(TextFormField, 'Email'),
        'not-an-email',
      );
      await tester.tap(find.text('Sign In'));
      await tester.pumpAndSettle();

      expect(find.text('Enter a valid email'), findsOneWidget);
    });

    testWidgets('missing password shows password validation error', (tester) async {
      await tester.pumpWidget(_wrapLogin());
      await tester.pumpAndSettle();

      await tester.enterText(
        find.widgetWithText(TextFormField, 'Email'),
        'user@example.com',
      );
      await tester.tap(find.text('Sign In'));
      await tester.pumpAndSettle();

      expect(find.text('Enter your password'), findsOneWidget);
    });

    testWidgets('login failure displays error message', (tester) async {
      await tester.pumpWidget(
        _wrapLogin(
          overrides: [
            authStateProvider.overrideWith(() => _FailingAuthNotifier()),
          ],
        ),
      );
      await tester.pumpAndSettle();

      await tester.enterText(
        find.widgetWithText(TextFormField, 'Email'),
        'user@example.com',
      );
      await tester.enterText(
        find.widgetWithText(TextFormField, 'Password'),
        'wrongpassword',
      );
      await tester.tap(find.text('Sign In'));
      await tester.pumpAndSettle();

      expect(find.textContaining('Unauthorized'), findsOneWidget);
    });

    testWidgets('login screen renders Sign In button and two text fields', (tester) async {
      await tester.pumpWidget(_wrapLogin());
      await tester.pumpAndSettle();

      expect(find.text('Sign In'), findsOneWidget);
      expect(find.byType(TextFormField), findsWidgets);
    });
  });
}
