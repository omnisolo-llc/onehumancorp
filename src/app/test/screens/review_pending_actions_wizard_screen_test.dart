import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/ongoing_management_wizards.dart';
import 'package:ohc_app/services/api_service.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  testWidgets('ReviewPendingActionsWizardScreen test', (WidgetTester tester) async {
    final mockApi = MockApiService();

    when(() => mockApi.getApprovals()).thenAnswer((_) async => [
      {
        'id': '1',
        'agentId': 'Customer Success Agent',
        'action': 'Send refund email',
        'riskLevel': 'High',
      },
      {
        'id': '2',
        'agentId': 'Marketing & Advertising Agent',
        'action': 'Publish Instagram post',
        'riskLevel': 'High',
      }
    ]);

    when(() => mockApi.decideApproval(any(), any())).thenAnswer((_) async {});

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(mockApi),
        ],
        child: const MaterialApp(
          home: ReviewPendingActionsWizardScreen(),
        ),
      ),
    );

    // Initial state is loading
    expect(find.byType(CircularProgressIndicator), findsOneWidget);

    // Wait for the mock API call to complete
    await tester.pumpAndSettle(const Duration(milliseconds: 500));

    // After loading, we should see the mock actions
    expect(find.text('Pending Approvals'), findsOneWidget);
    expect(find.text('Customer Success Agent'), findsOneWidget);
    expect(find.text('Marketing & Advertising Agent'), findsOneWidget);

    // There should be 2 Approve and 2 Reject buttons
    expect(find.text('Approve'), findsNWidgets(2));
    expect(find.text('Reject'), findsNWidgets(2));

    // Test rejecting the first action
    await tester.tap(find.text('Reject').first);
    await tester.pumpAndSettle();

    // Verify toast is shown
    expect(find.text('Action rejected and discarded.'), findsOneWidget);

    // One action should be removed, leaving 1
    expect(find.text('Customer Success Agent'), findsNothing);
    expect(find.text('Marketing & Advertising Agent'), findsOneWidget);
    expect(find.text('Approve'), findsNWidgets(1));

    // Wait for the snackbar to disappear to avoid "multiple widgets found" for the next tap
    await tester.pumpAndSettle(const Duration(seconds: 4));

    // Test approving the remaining action
    await tester.tap(find.text('Approve').first);
    await tester.pumpAndSettle();

    // Verify toast is shown
    expect(find.text('Action approved and executed.'), findsOneWidget);

    // List should be empty now
    expect(find.text('Marketing & Advertising Agent'), findsNothing);
    expect(find.text('All caught up! No pending actions.'), findsOneWidget);
  });
}
