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
    when(() => mockApiService.generateReferralLink(any())).thenAnswer((_) async => {
      'link': 'ohc://join?ref=TESTCODE',
      'pre_filled_message': 'Share OHC with a friend, both get 1 month free Pro! Join here: ohc://join?ref=TESTCODE',
    });
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

    expect(find.text('Free Tier Quota'), findsOneWidget);
    expect(find.text('10 / 100 missions used'), findsOneWidget);
    expect(find.text('Share OHC with a friend, both get 1 month free Pro.'), findsOneWidget);
    expect(find.text('Invite a Founder'), findsOneWidget);

    await tester.tap(find.text('Invite a Founder'));
    await tester.pumpAndSettle();

    verify(() => mockApiService.generateReferralLink("anonymous")).called(1);
    expect(find.byType(SnackBar), findsOneWidget);
    expect(find.textContaining('Referral link copied to clipboard!'), findsOneWidget);
  });
}
