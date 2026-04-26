import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/widgets/growth_referral_widget.dart';
import 'package:ohc_app/services/api_service.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  late MockApiService mockApiService;

  setUp(() {
    mockApiService = MockApiService();
  });

  testWidgets('GrowthReferralWidget displays quota and invite button', (WidgetTester tester) async {
    when(() => mockApiService.getQuota()).thenAnswer((_) async => {'used': 10, 'max': 100});
    when(() => mockApiService.createReferral(any(), any())).thenAnswer((_) async {});

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(mockApiService),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: GrowthReferralWidget(),
          ),
        ),
      ),
    );

    await tester.pump(const Duration(milliseconds: 100));

    expect(find.text('Free Plan Usage'), findsOneWidget);
    expect(find.text('10 / 100 tasks used'), findsOneWidget);
    expect(find.text('Invite Team for More Tasks'), findsOneWidget);

    await tester.tap(find.text('Invite Team for More Tasks'));
    await tester.pumpAndSettle();

    verify(() => mockApiService.createReferral("anonymous", "xYz8vQ_secure_invite")).called(1);
    expect(find.byType(SnackBar), findsOneWidget);
    expect(find.textContaining('Secure invite link copied'), findsOneWidget);
  });
}
