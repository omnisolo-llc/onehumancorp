import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/main.dart';
import 'package:app/screens/referral_program_screen.dart';

void main() {
  testWidgets('Dashboard navigation to ReferralProgramScreen', (WidgetTester tester) async {
    // We build the DashboardScreen instead of OHCApp to avoid the wizard
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: DashboardScreen(),
        ),
      ),
    );

    // Verify milestone notification exists
    expect(find.text('🎉 You just got your 10th order!'), findsOneWidget);
    expect(find.text('Keep up the great work!'), findsOneWidget);

    // Verify "Share your link with a friend" checklist item exists
    final shareItem = find.text('Share your link with a friend');
    expect(shareItem, findsOneWidget);

    // Tap the item to navigate
    await tester.ensureVisible(shareItem);
    await tester.tap(shareItem);
    await tester.pumpAndSettle();

    // Verify we are on the ReferralProgramScreen
    expect(find.byType(ReferralProgramScreen), findsOneWidget);
    expect(find.text('Refer a Friend'), findsOneWidget);
    expect(find.text('Share OHC, get 1 month free Pro!'), findsOneWidget);
  });

  testWidgets('ReferralProgramScreen UI elements', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: ReferralProgramScreen(),
        ),
      ),
    );

    // Verify main texts
    expect(find.text('Refer a Friend'), findsOneWidget);
    expect(find.text('Share OHC, get 1 month free Pro!'), findsOneWidget);
    expect(find.text('Your Invite Link'), findsOneWidget);

    // Verify link and buttons
    expect(find.text('ohc://join?ref=user123'), findsOneWidget);
    expect(find.byTooltip('Copy Link'), findsOneWidget);
    expect(find.text('Share Link'), findsOneWidget);

    // Verify invites list
    expect(find.text('Your Invites'), findsOneWidget);
    expect(find.text('friend@example.com'), findsOneWidget);
    expect(find.text('ACCEPTED'), findsOneWidget);
    expect(find.text('another@example.com'), findsOneWidget);
    expect(find.text('PENDING'), findsOneWidget);
  });
}
