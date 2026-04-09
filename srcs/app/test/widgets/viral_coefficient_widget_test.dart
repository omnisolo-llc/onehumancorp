import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/viral_coefficient_widget.dart';
import 'package:ohc_app/services/api_service.dart';

class FakeApiService extends ApiService {
  FakeApiService() : super(baseUrl: 'http://localhost', token: 'token');

  @override
  Future<Map<String, dynamic>> getViralCoefficient() async {
    return {
      'totalReferrals': 100,
      'totalConversions': 25,
      'uniqueInviters': 10,
      'kFactor': 2.5,
    };
  }
}

void main() {
  testWidgets('ViralCoefficientWidget displays data correctly', (tester) async {
    final fakeApi = FakeApiService();

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(fakeApi),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: ViralCoefficientWidget(),
          ),
        ),
      ),
    );

    expect(find.byType(CircularProgressIndicator), findsOneWidget);

    await tester.pumpAndSettle();

    expect(find.text('Viral Coefficient (K-Factor)'), findsOneWidget);
    expect(find.text('2.50'), findsOneWidget);
    expect(find.text('100'), findsOneWidget);
    expect(find.text('25'), findsOneWidget);
    expect(find.text('10'), findsOneWidget);
  });
}
