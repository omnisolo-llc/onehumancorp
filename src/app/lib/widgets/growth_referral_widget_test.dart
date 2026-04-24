import 'dart:convert';
import 'package:http/http.dart' as http;
import 'dart:io';
import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/growth_referral_widget.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:mocktail/mocktail.dart';

class MockHttpClient extends Mock implements http.Client {}

class MockAuthNotifier extends Mock implements AuthNotifier {}

class FakeUri extends Fake implements Uri {}

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
    print('Current directory: ${Directory.current.path}');
    try {
      print('Files in current directory:');
      Directory.current.listSync().forEach((file) => print(file.path));
      print('Files in ../..:');
      Directory('../../').listSync().forEach((file) => print(file.path));
    } catch (e) {
      print('Error listing files: $e');
    }
  });
  late MockHttpClient mockHttpClient;
  late ApiService apiService;

  setUp(() {
    mockHttpClient = MockHttpClient();
    apiService = ApiService(baseUrl: 'http://localhost', token: 'tok', client: mockHttpClient);
    
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger.setMockMethodCallHandler(
      SystemChannels.platform,
      (MethodCall methodCall) async {
        if (methodCall.method == 'Clipboard.setData') {
          return null;
        }
        return null;
      },
    );
  });

  testWidgets('GrowthReferralWidget renders title and quota', (WidgetTester tester) async {
    final quotaJson = '{"used": 5, "max": 10}';
    when(() => mockHttpClient.get(any(), headers: any(named: 'headers'))).thenAnswer(
      (_) async => http.Response(quotaJson, 200),
    );

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(apiService),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: GrowthReferralWidget(),
          ),
        ),
      ),
    );

    await tester.pump(); // Start future builder

    expect(find.text('Share OHC with a friend, both get 1 month free Pro.'), findsOneWidget);
    
    await tester.pumpAndSettle(); // Wait for future to complete
    
    expect(find.text('5 / 10 missions used'), findsOneWidget);
  });

  testWidgets('GrowthReferralWidget copies link on button tap', (WidgetTester tester) async {
    final mockAuthUser = const AuthUser(id: 'u1', email: 'test@example.com', name: 'Test User', role: 'admin', organizationId: 'org-1', token: 'tok');
    
    final quotaJson2 = '{"used": 5, "max": 10}';
    when(() => mockHttpClient.get(any(), headers: any(named: 'headers'))).thenAnswer(
      (_) async => http.Response(quotaJson2, 200),
    );
    when(() => mockHttpClient.post(
      any(that: predicate<Uri>((u) => u.path.endsWith('/referral'))),
      headers: any(named: 'headers'),
      body: any(named: 'body'),
    )).thenAnswer(
      (_) async => http.Response(
        jsonEncode({
          'link': 'http://localhost/?ref=TESTCODE',
          'pre_filled_message': 'Join me on OHC!',
        }),
        200,
      ),
    );
    when(() => mockHttpClient.post(
      any(that: predicate<Uri>((u) => u.path.endsWith('/referrals'))),
      headers: any(named: 'headers'),
      body: any(named: 'body'),
    )).thenAnswer((_) async => http.Response('', 200));
    
    final mockAuthNotifierForTest = MockAuthNotifier();
    when(() => mockAuthNotifierForTest.state).thenReturn(AsyncData(mockAuthUser));
    when(() => mockAuthNotifierForTest.build()).thenAnswer((_) async => mockAuthUser);

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(apiService),
          authStateProvider.overrideWith(() => mockAuthNotifierForTest),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: GrowthReferralWidget(),
          ),
        ),
      ),
    );

    await tester.pumpAndSettle();

    await tester.tap(find.text('Invite a Founder'));
    await tester.pumpAndSettle(); // Wait for async operations and animations

    // Verify service was called (now we verify the http client was called)
    verify(() => mockHttpClient.post(
      any(that: predicate<Uri>((u) => u.path.endsWith('/referral'))),
      headers: any(named: 'headers'),
      body: any(named: 'body'),
    )).called(1);

    // Verify snackbar is shown
    expect(find.byType(SnackBar), findsOneWidget);
    expect(find.text('Referral link copied to clipboard!'), findsOneWidget);
  });
}
