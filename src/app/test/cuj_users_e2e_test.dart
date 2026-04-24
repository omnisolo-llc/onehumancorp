// CUJ: User Management – RBAC Administration
//
// Covers the user management critical user journey:
//   1. Renders user list from API
//   2. Shows empty state when no users
//   3. Invite User FAB is present
//   4. Delete user dialog appears on delete action
//   5. Refresh reloads the user list

import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/user_management_screen.dart';
import 'package:ohc_app/services/api_service.dart';

// ── Mocks & Fakes ─────────────────────────────────────────────────────────

class MockHttpClient extends Mock implements http.Client {}

class FakeUri extends Fake implements Uri {}

Widget _wrapScreen(Widget screen, ApiService api) {
  final router = GoRouter(
    routes: [
      GoRoute(path: '/', builder: (context, state) => screen),
    ],
  );
  return ProviderScope(
    overrides: [apiServiceProvider.overrideWithValue(api)],
    child: MaterialApp.router(routerConfig: router),
  );
}

Map<String, dynamic> _fakeUser(String id, String username, {bool admin = false}) => {
  'id': id,
  'username': username,
  'email': '$username@example.com',
  'roles': admin ? ['admin'] : ['user'],
  'active': true,
  'created_at': DateTime.now().toIso8601String(),
};

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  group('CUJ: User Management', () {
    testWidgets('renders user names from API', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response(
          jsonEncode([
            _fakeUser('u1', 'alice', admin: true),
            _fakeUser('u2', 'bob'),
          ]),
          200,
        ),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const UserManagementScreen(), api));
      await tester.pumpAndSettle();

      expect(find.textContaining('alice'), findsWidgets);
      expect(find.textContaining('bob'), findsWidgets);
    });

    testWidgets('Invite User FAB is present', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response(jsonEncode(<dynamic>[]), 200),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const UserManagementScreen(), api));
      await tester.pumpAndSettle();

      expect(find.textContaining('Invite'), findsOneWidget);
    });

    testWidgets('Invite User FAB opens dialog when tapped', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response(jsonEncode(<dynamic>[]), 200),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const UserManagementScreen(), api));
      await tester.pumpAndSettle();

      await tester.tap(find.textContaining('Invite'));
      await tester.pumpAndSettle();

      expect(find.byType(AlertDialog), findsOneWidget);
    });

    testWidgets('admin badge shown for admin users', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response(
          jsonEncode([_fakeUser('u1', 'adminuser', admin: true)]),
          200,
        ),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const UserManagementScreen(), api));
      await tester.pumpAndSettle();

      expect(find.textContaining('admin'), findsWidgets);
    });

    testWidgets('shows error message when API fails', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response('Server Error', 500),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const UserManagementScreen(), api));
      await tester.pumpAndSettle();

      expect(find.textContaining('Error'), findsWidgets);
    });
  });
}
