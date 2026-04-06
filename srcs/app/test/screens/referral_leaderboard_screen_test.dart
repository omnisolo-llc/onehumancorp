import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/referral_leaderboard_screen.dart';
import 'package:ohc_app/services/api_service.dart';

class MockApiService extends ApiService {
  MockApiService() : super(baseUrl: 'http://localhost', token: 'fake');

  @override
  Future<List<Map<String, dynamic>>> getReferralLeaderboard() async {
    return [
      {'userId': 'alice', 'conversions': 5, 'totalClicks': 20},
      {'userId': 'bob', 'conversions': 3, 'totalClicks': 15},
      {'userId': 'charlie', 'conversions': 1, 'totalClicks': 5},
    ];
  }
}

void main() {
  testWidgets('ReferralLeaderboardScreen renders and displays leaderboard', (WidgetTester tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(MockApiService()),
        ],
        child: const MaterialApp(
          home: ReferralLeaderboardScreen(),
        ),
      ),
    );

    // Initial loading state
    expect(find.byType(CircularProgressIndicator), findsOneWidget);

    // Wait for the future to resolve
    await tester.pumpAndSettle();

    // Check if the leaderboard entries are rendered
    expect(find.text('alice'), findsOneWidget);
    expect(find.text('5 Conversions'), findsOneWidget);
    expect(find.text('20 Clicks'), findsOneWidget);

    expect(find.text('bob'), findsOneWidget);
    expect(find.text('3 Conversions'), findsOneWidget);

    expect(find.text('charlie'), findsOneWidget);
    expect(find.text('1 Conversions'), findsOneWidget);

    // Check ranks
    expect(find.text('#1'), findsOneWidget);
    expect(find.text('#2'), findsOneWidget);
    expect(find.text('#3'), findsOneWidget);
  });
}
