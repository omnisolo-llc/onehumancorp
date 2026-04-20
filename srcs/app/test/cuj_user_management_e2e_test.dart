// CUJ: User Management Screen
//
// Covers user management CUJ using seeded ApiService subclass (no direct HTTP mocks).

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/user_management_screen.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/models/user.dart';

class _SeededUserApiService extends ApiService {
  final List<UserPublic> _users;
  _SeededUserApiService(this._users)
      : super(baseUrl: 'http://test-host', token: 'seed-token');

  @override
  Future<List<UserPublic>> listUsers() async => _users;
}

UserPublic _user(String id, String email, {String role = 'member'}) =>
    UserPublic(
      id: id,
      username: 'user_$id',
      email: email,
      roles: [role],
      active: true,
      createdAt: DateTime(2025, 1, 1),
    );

Widget _wrapUsers(List<UserPublic> users) {
  final api = _SeededUserApiService(users);
  return ProviderScope(
    overrides: [apiServiceProvider.overrideWithValue(api)],
    child: const MaterialApp(home: UserManagementScreen()),
  );
}

void main() {
  group('CUJ: User Management Screen', () {
    testWidgets('empty users list renders Scaffold', (tester) async {
      await tester.pumpWidget(_wrapUsers([]));
      await tester.pumpAndSettle();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('single seeded user renders Scaffold', (tester) async {
      await tester.pumpWidget(_wrapUsers([_user('u1', 'alice@example.com')]));
      await tester.pumpAndSettle();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('multiple seeded users render without crash', (tester) async {
      await tester.pumpWidget(_wrapUsers([
        _user('u1', 'alice@example.com'),
        _user('u2', 'bob@example.com'),
        _user('u3', 'carol@example.com'),
      ]));
      await tester.pumpAndSettle();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('admin user role renders without crash', (tester) async {
      await tester.pumpWidget(_wrapUsers([_user('u1', 'admin@example.com', role: 'admin')]));
      await tester.pumpAndSettle();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('user management screen AppBar is present', (tester) async {
      await tester.pumpWidget(_wrapUsers([]));
      await tester.pumpAndSettle();
      expect(find.byType(AppBar), findsAtLeastNWidgets(1));
    });

    testWidgets('narrow viewport renders without crash', (tester) async {
      tester.view.physicalSize = const Size(360, 640);
      tester.view.devicePixelRatio = 1.0;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      await tester.pumpWidget(_wrapUsers([]));
      await tester.pumpAndSettle();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('wide viewport renders without crash', (tester) async {
      tester.view.physicalSize = const Size(1280, 800);
      tester.view.devicePixelRatio = 1.0;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      await tester.pumpWidget(_wrapUsers([]));
      await tester.pumpAndSettle();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('10 seeded users render without crash', (tester) async {
      final users = List.generate(10, (i) => _user('u$i', 'user$i@example.com'));
      await tester.pumpWidget(_wrapUsers(users));
      await tester.pumpAndSettle();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('user email with long name renders without overflow', (tester) async {
      await tester.pumpWidget(_wrapUsers([
        _user('u1', 'averylongemail@example.com'),
      ]));
      await tester.pumpAndSettle();
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('user management screen rebuild does not crash', (tester) async {
      await tester.pumpWidget(_wrapUsers([]));
      await tester.pumpAndSettle();
      await tester.pumpWidget(_wrapUsers([_user('u1', 'test@example.com')]));
      await tester.pumpAndSettle();
      expect(find.byType(Scaffold), findsOneWidget);
    });
  });
}
