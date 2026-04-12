import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/team_invites_dashboard_screen.dart';
import 'package:ohc_app/services/api_service.dart';

class MockApiService extends ApiService {
  MockApiService() : super(baseUrl: '', token: '');

  @override
  Future<List<Map<String, dynamic>>> listTeamInvites() async {
    return [
      {
        'id': 'inv-123',
        'inviterId': 'user1',
        'inviteeId': 'user2',
        'status': 'PENDING',
        'createdAt': '2026-04-12T00:00:00Z',
      }
    ];
  }
}

void main() {
  testWidgets('TeamInvitesDashboardScreen renders invites', (WidgetTester tester) async {
    final mockApi = MockApiService();

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(mockApi),
        ],
        child: const MaterialApp(
          home: TeamInvitesDashboardScreen(),
        ),
      ),
    );

    await tester.pumpAndSettle();

    expect(find.text('Team Invites Dashboard'), findsOneWidget);
    expect(find.textContaining('user1'), findsOneWidget);
    expect(find.textContaining('user2'), findsOneWidget);
    expect(find.textContaining('PENDING'), findsOneWidget);
  });
}
