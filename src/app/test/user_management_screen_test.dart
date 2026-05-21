import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/screens/user_management_screen.dart';

void main() {
  testWidgets('UserManagementScreen contains GrowthReferralWidget', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: UserManagementScreen()));

    // Verify the widget exists
    expect(find.byType(GrowthReferralWidget), findsOneWidget);

    // Verify text exists
    expect(find.text('Invite Collaborator'), findsOneWidget);
    expect(find.text('Bridge your Standalone sovereignty with Cloud-Native team expansion.'), findsOneWidget);
    expect(find.text('Generate Viral Link'), findsOneWidget);
  });
}
