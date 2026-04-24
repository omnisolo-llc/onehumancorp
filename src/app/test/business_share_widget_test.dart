import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/business_share_widget.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/models/agent.dart';

void main() {
  testWidgets('BusinessShareWidget displays correct info and copy button', (tester) async {
    final dashboardData = DashboardSnapshot(
      organization: Organization(
        id: 'org-test',
        name: 'Maya Bakery',
        domain: 'maya.ohc.io',
        members: [],
        roleProfiles: [],
      ),
      meetings: [],
      costs: CostSummary(totalCostUSD: 0, totalTokens: 0, agents: []),
      agents: [],
      statuses: [],
      updatedAt: DateTime.now(),
    );

    await tester.pumpWidget(
      ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: BusinessShareWidget(data: dashboardData),
          ),
        ),
      ),
    );

    expect(find.text('Share my business'), findsOneWidget);
    expect(find.text('Maya Bakery'), findsOneWidget);
    expect(find.text('Check out my new storefront built with OHC!'), findsOneWidget);
    expect(find.text('Copy Link'), findsOneWidget);
    expect(find.bySemanticsLabel('Share to Instagram'), findsOneWidget);
    expect(find.bySemanticsLabel('Share to X'), findsOneWidget);
    expect(find.bySemanticsLabel('Share to WhatsApp'), findsOneWidget);

    // Tap copy link
    await tester.tap(find.text('Copy Link'));
    await tester.pumpAndSettle();

    expect(find.textContaining('Storefront link copied to clipboard: https://maya.ohc.io'), findsOneWidget);
  });
}
