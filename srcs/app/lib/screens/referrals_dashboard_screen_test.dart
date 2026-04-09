import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/referrals_dashboard_screen.dart';
import 'package:ohc_app/services/api_service.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  late MockApiService mockApiService;

  setUp(() {
    mockApiService = MockApiService();
  });

  Widget buildTestWidget() {
    return ProviderScope(
      overrides: [
        apiServiceProvider.overrideWithValue(mockApiService),
      ],
      child: const MaterialApp(
        home: ReferralsDashboardScreen(),
      ),
    );
  }

  testWidgets('displays list of referrals and K-Factor', (tester) async {
    when(() => mockApiService.listReferrals()).thenAnswer(
      (_) async => [
        {
          'id': 'ref-1',
          'userId': 'jules',
          'referralCode': 'JULES2026',
          'clicks': 42,
          'conversions': 10,
          'createdAt': '2026-04-05T12:00:00Z',
        },
      ],
    );
    when(() => mockApiService.getViralCoefficient()).thenAnswer(
      (_) async => {
        'totalReferrals': 100,
        'totalConversions': 25,
        'uniqueInviters': 10,
        'kFactor': 2.5,
      },
    );

    await tester.pumpWidget(buildTestWidget());
    expect(find.byType(CircularProgressIndicator), findsOneWidget);

    await tester.pumpAndSettle();

    expect(find.text('Viral Loop Dashboard'), findsOneWidget);

    // K-Factor checks
    expect(find.text('Viral Coefficient (K-Factor)'), findsOneWidget);
    expect(find.text('2.50'), findsOneWidget);
    expect(find.text('100'), findsOneWidget);
    expect(find.text('25'), findsOneWidget);
    expect(find.text('10'), findsOneWidget);

    // Referral card checks
    expect(find.text('Ref: JULES2026'), findsOneWidget);
    expect(find.text('User: jules'), findsOneWidget);
    expect(find.text('42'), findsOneWidget);
    expect(find.text('10'), findsOneWidget);
    expect(find.text('Clicks'), findsOneWidget);
    expect(find.text('Conversions'), findsOneWidget);
  });

  testWidgets('displays empty state', (tester) async {
    when(() => mockApiService.listReferrals()).thenAnswer((_) async => []);
    when(() => mockApiService.getViralCoefficient()).thenAnswer(
      (_) async => {
        'totalReferrals': 0,
        'totalConversions': 0,
        'uniqueInviters': 0,
        'kFactor': 0.0,
      },
    );

    await tester.pumpWidget(buildTestWidget());
    await tester.pumpAndSettle();

    expect(find.text('No referrals tracked yet.'), findsOneWidget);
    expect(find.text('0.00'), findsOneWidget);
  });

  testWidgets('displays error state', (tester) async {
    when(() => mockApiService.listReferrals())
        .thenAnswer((_) => Future.error(Exception('API failure')));
    when(() => mockApiService.getViralCoefficient())
        .thenAnswer((_) => Future.error(Exception('API failure')));

    await tester.pumpWidget(buildTestWidget());
    await tester.pumpAndSettle();

    expect(find.textContaining('API failure'), findsOneWidget);
  });
}
