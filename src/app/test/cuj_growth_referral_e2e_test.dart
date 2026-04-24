import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:ohc_app/screens/user_management_screen.dart';
import 'package:ohc_app/services/api_service.dart';

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

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  group('CUJ: Growth Referral Loops', () {
    testWidgets('Dashboard Growth Referral Flow', (tester) async {
      await tester.binding.setSurfaceSize(const Size(1440, 2000));
      final mockClient = MockHttpClient();

      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (Invocation invocation) async {
          final uri = invocation.positionalArguments[0] as Uri;
          if (uri.path.contains('quota')) {
              return http.Response(jsonEncode({
                'used': 10,
                'max': 100,
              }), 200);
          }
          return http.Response(jsonEncode({
            "organization": {
                "id": "org1",
                "name": "My Org",
                "domain": "org1.com",
                "is_standalone": false,
                "members": [],
                "role_profiles": [],
            },
            "agents": [],
            "rooms": [],
            "tasks": [],
            "recent_activities": [],
            "tasks_in_progress": 0,
            "agents_hired": 0,
            "tenant_id": "test",
            'used': 10,
            'max': 100,
            "has_update": false
          }), 200);
        }
      );

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

      await tester.pumpWidget(_wrapScreen(const DashboardScreen(), api));
      await tester.pumpAndSettle();

      expect(find.textContaining('Grow Your Swarm. Maintain Sovereignty.', skipOffstage: false), findsWidgets);
      expect(find.textContaining('10 / 100 missions used', skipOffstage: false), findsWidgets);

      final inviteButton = find.widgetWithText(ElevatedButton, 'Invite Team to Expand Quota', skipOffstage: false);
      expect(inviteButton, findsOneWidget);
      await tester.ensureVisible(inviteButton);
      await tester.tap(inviteButton, warnIfMissed: false);
      await tester.pump();
      await tester.pumpAndSettle();

      expect(find.byType(SnackBar), findsWidgets);
    });

    testWidgets('User Management Growth Referral Flow', (tester) async {
      await tester.binding.setSurfaceSize(const Size(1440, 2000));
      final mockClient = MockHttpClient();

      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (Invocation invocation) async {
          final uri = invocation.positionalArguments[0] as Uri;
          if (uri.path.contains('users')) {
            return http.Response(jsonEncode([]), 200);
          }
          return http.Response(jsonEncode({
            'used': 10,
            'max': 100,
          }), 200);
        }
      );

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
      await tester.pumpAndSettle();

      expect(find.text('User Management', skipOffstage: false), findsWidgets);
      expect(find.textContaining('Grow Your Swarm. Maintain Sovereignty.', skipOffstage: false), findsWidgets);

      final inviteUserButton = find.widgetWithText(FloatingActionButton, 'Invite User', skipOffstage: false);
      await tester.ensureVisible(inviteUserButton);
      await tester.tap(inviteUserButton, warnIfMissed: false);
      await tester.pumpAndSettle();

      expect(find.text('Invite New User', skipOffstage: false), findsWidgets);
      expect(find.textContaining('Expand your sovereign swarm', skipOffstage: false), findsWidgets);

      await tester.enterText(find.byType(TextField).first, 'new_collaborator');
      await tester.tap(find.widgetWithText(FilledButton, 'Generate Secure Invite', skipOffstage: false), warnIfMissed: false);
      await tester.pump();

      // Need to find a snackbar, but we don't pumpAndSettle. We do just a quick pump
      await tester.pump(const Duration(milliseconds: 100));

      expect(find.byType(SnackBar), findsWidgets);
    });
  });
}
