import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/screens/user_management_screen.dart';

void main() {
  testWidgets('GrowthReferralWidget UI components present', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: Scaffold(body: GrowthReferralWidget())));
    expect(find.text('Invite a Collaborator'), findsOneWidget);
    expect(find.text('Generate Referral Link'), findsOneWidget);
  });
}
