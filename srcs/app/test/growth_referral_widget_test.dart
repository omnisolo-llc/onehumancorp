import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../lib/widgets/growth_referral_widget.dart';

void main() {
  testWidgets('GrowthReferralWidget displays quota and invite button', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: GrowthReferralWidget(),
          ),
        ),
      ),
    );

    await tester.pump(const Duration(milliseconds: 100));

    expect(find.text('Free Tier Quota'), findsOneWidget);
    expect(find.text('Invite Team to Expand Quota'), findsOneWidget);
  });
}
