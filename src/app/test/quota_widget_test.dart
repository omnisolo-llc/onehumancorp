import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/screens/quota_widget.dart';

void main() {
  testWidgets('FreeTierQuotaWidget - Displays correctly and interaction works', (WidgetTester tester) async {
    bool invitePressed = false;

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: FreeTierQuotaWidget(
            currentReferrals: 2,
            maxReferrals: 5,
            onInvitePressed: () {
              invitePressed = true;
            },
          ),
        ),
      ),
    );

    // Verify text
    expect(find.text('Free-Tier Quota'), findsOneWidget);
    expect(find.text('2 / 5 Referrals'), findsOneWidget);
    expect(find.text('Expand your quota by inviting more businesses.'), findsOneWidget);
    expect(find.text('Invite Team to Expand Quota'), findsOneWidget);

    // Test button tap
    await tester.tap(find.text('Invite Team to Expand Quota'));
    await tester.pumpAndSettle();

    expect(invitePressed, isTrue);
  });
}
