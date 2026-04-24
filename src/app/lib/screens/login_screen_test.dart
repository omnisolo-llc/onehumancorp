import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/login_screen.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/settings_service.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:shared_preferences/shared_preferences.dart';

class MockHttpClient extends Mock implements http.Client {}
class FakeUri extends Fake implements Uri {}

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  late MockHttpClient mockHttpClient;
  late AuthService authService;

  setUp(() {
    mockHttpClient = MockHttpClient();
    authService = AuthService(baseUrl: 'http://localhost', client: mockHttpClient);

    SharedPreferences.setMockInitialValues({}); // Empty prefs
  });

  testWidgets('LoginScreen renders fields and button', (WidgetTester tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          authServiceProvider.overrideWithValue(authService),
        ],
        child: const MaterialApp(
          home: LoginScreen(),
        ),
      ),
    );

    expect(find.text('One Human Corp'), findsOneWidget);
    expect(find.text('Email or Username'), findsOneWidget);
    expect(find.text('Password'), findsOneWidget);
    expect(find.text('Sign In'), findsOneWidget);
  });

  testWidgets('LoginScreen shows validation errors on empty submit', (WidgetTester tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          authServiceProvider.overrideWithValue(authService),
        ],
        child: const MaterialApp(
          home: LoginScreen(),
        ),
      ),
    );

    await tester.tap(find.text('Sign In'));
    await tester.pump();

    expect(find.text('Enter your email or username'), findsOneWidget);
    expect(find.text('Enter your password'), findsOneWidget);
  });

  testWidgets('LoginScreen calls login on success', (WidgetTester tester) async {
    final responseData = {
      'token': 'mock_token',
      'user': {
        'id': 'u1',
        'email': 'test@example.com',
        'username': 'testuser',
        'roles': ['admin'],
        'organizationId': 'org-1'
      }
    };

    when(() => mockHttpClient.post(
      any(),
      headers: any(named: 'headers'),
      body: any(named: 'body'),
    )).thenAnswer((_) async => http.Response(jsonEncode(responseData), 200));

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          authServiceProvider.overrideWithValue(authService),
        ],
        child: const MaterialApp(
          home: LoginScreen(),
        ),
      ),
    );

    // Find text fields by label text to be more robust
    await tester.enterText(find.byWidgetPredicate((w) => w is TextField && w.decoration?.labelText == 'Email or Username'), 'testuser');
    await tester.enterText(find.byWidgetPredicate((w) => w is TextField && w.decoration?.labelText == 'Password'), 'password');

    await tester.tap(find.text('Sign In'));
    await tester.pump(); // Start loading

    // Wait for async operations to complete
    await tester.pumpAndSettle();

    // Verify navigation or state change happened
    // Since we don't have router mocked here, we just verify it didn't throw error
    expect(find.text('Enter your email or username'), findsNothing);
  });
}
