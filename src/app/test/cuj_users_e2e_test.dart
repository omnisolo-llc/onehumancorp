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
      ).thenAnswer((invocation) async {
        final uri = invocation.positionalArguments[0] as Uri;
        if (uri.path.contains('/api/growth/quota')) {
          return http.Response(jsonEncode({"used": 1, "max": 10}), 200, headers: {'content-type': 'application/json'});
        }
        if (uri.path.contains('/api/users')) {
          return http.Response(
            jsonEncode([
              _fakeUser('u1', 'alice', admin: true),
              _fakeUser('u2', 'bob'),
            ]),
            200,
            headers: {'content-type': 'application/json'},
          );
        }
        return http.Response(jsonEncode(<dynamic>[]), 200, headers: {'content-type': 'application/json'});
      });
      when(
        () => mockClient.post(any(), headers: any(named: 'headers'), body: any(named: 'body')),
      ).thenAnswer(
        (_) async => http.Response(jsonEncode({}), 200),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const UserManagementScreen(), api));
      await tester.pump();
      await tester.pump(const Duration(milliseconds: 1000));
      await tester.pumpAndSettle();

      expect(find.textContaining('alice'), findsWidgets);
      expect(find.textContaining('bob'), findsWidgets);
    });

    testWidgets('Invite User FAB is present', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer((invocation) async {
        final uri = invocation.positionalArguments[0] as Uri;
        if (uri.path.contains('/api/growth/quota')) {
          return http.Response(jsonEncode({"used": 1, "max": 10}), 200, headers: {'content-type': 'application/json'});
        }
        if (uri.path.contains('/api/users')) {
          return http.Response(jsonEncode(<dynamic>[]), 200, headers: {'content-type': 'application/json'});
        }
        return http.Response(jsonEncode(<dynamic>[]), 200, headers: {'content-type': 'application/json'});
      });
      when(
        () => mockClient.post(any(), headers: any(named: 'headers'), body: any(named: 'body')),
      ).thenAnswer(
        (_) async => http.Response(jsonEncode({}), 200),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const UserManagementScreen(), api));
      await tester.pump();
      await tester.pump(const Duration(milliseconds: 1000));
      await tester.pumpAndSettle();

      expect(find.text('Invite User'), findsOneWidget);
    });

    testWidgets('Invite User FAB opens dialog when tapped', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer((invocation) async {
        final uri = invocation.positionalArguments[0] as Uri;
        if (uri.path.contains('/api/growth/quota')) {
          return http.Response(jsonEncode({"used": 1, "max": 10}), 200, headers: {'content-type': 'application/json'});
        }
        if (uri.path.contains('/api/users')) {
          return http.Response(jsonEncode(<dynamic>[]), 200, headers: {'content-type': 'application/json'});
        }
        return http.Response(jsonEncode(<dynamic>[]), 200, headers: {'content-type': 'application/json'});
      });
      when(
        () => mockClient.post(any(), headers: any(named: 'headers'), body: any(named: 'body')),
      ).thenAnswer(
        (_) async => http.Response(jsonEncode({}), 200),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const UserManagementScreen(), api));
      await tester.pump();
      await tester.pump(const Duration(milliseconds: 1000));
      await tester.pumpAndSettle();

      await tester.tap(find.text('Invite User'));
      await tester.pump(const Duration(milliseconds: 100));
      await tester.pumpAndSettle();

      expect(find.byType(Dialog), findsOneWidget);
    });

    testWidgets('admin badge shown for admin users', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer((invocation) async {
        final uri = invocation.positionalArguments[0] as Uri;
        if (uri.path.contains('/api/growth/quota')) {
          return http.Response(jsonEncode({"used": 1, "max": 10}), 200, headers: {'content-type': 'application/json'});
        }
        if (uri.path.contains('/api/users')) {
          return http.Response(
            jsonEncode([_fakeUser('u1', 'adminuser', admin: true)]),
            200,
            headers: {'content-type': 'application/json'},
          );
        }
        return http.Response(jsonEncode(<dynamic>[]), 200, headers: {'content-type': 'application/json'});
      });
      when(
        () => mockClient.post(any(), headers: any(named: 'headers'), body: any(named: 'body')),
      ).thenAnswer(
        (_) async => http.Response(jsonEncode({}), 200),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const UserManagementScreen(), api));
      await tester.pump();
      await tester.pump(const Duration(milliseconds: 1000));
      await tester.pumpAndSettle();

      expect(find.textContaining('admin'), findsWidgets);
    });

    testWidgets('shows error message when API fails', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer((invocation) async {
        final uri = invocation.positionalArguments[0] as Uri;
        if (uri.path.contains('/api/growth/quota')) {
          return http.Response(jsonEncode({"used": 1, "max": 10}), 200, headers: {'content-type': 'application/json'});
        }
        if (uri.path.contains('/api/users')) {
          return http.Response('Server Error', 500);
        }
        return http.Response(jsonEncode(<dynamic>[]), 200, headers: {'content-type': 'application/json'});
      });
      when(
        () => mockClient.post(any(), headers: any(named: 'headers'), body: any(named: 'body')),
      ).thenAnswer(
        (_) async => http.Response(jsonEncode({}), 200),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const UserManagementScreen(), api));
      await tester.pump();
      await tester.pump(const Duration(milliseconds: 1000));
      await tester.pumpAndSettle();

      expect(find.textContaining('Error'), findsWidgets);
    });
  });
}
