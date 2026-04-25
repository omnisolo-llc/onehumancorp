import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/widgets/business_share_widget.dart';

Widget _wrapWidget(Widget child) {
  return ProviderScope(
    child: MaterialApp(
      home: Scaffold(
        body: child,
      ),
    ),
  );
}

void main() {
  group('CUJ: Growth & Share', () {
    testWidgets('BusinessShareWidget renders and displays appropriate action buttons', (WidgetTester tester) async {
      final mockData = DashboardSnapshot(
        organization: Organization(
          id: 'org-123',
          name: 'Maya\'s Cakes',
          domain: 'mayascakes.ohc.io',
          tier: 'Free',
          members: [],
          roleProfiles: [],
        ),
        costs: CostSummary(
          totalCostUSD: 0,
          totalTokens: 0,
          totalActions: 0,
          agents: [],
        ),
        agents: [],
        meetings: [],
        statuses: [],
        updatedAt: DateTime.now(),
      );

      await tester.pumpWidget(_wrapWidget(BusinessShareWidget(data: mockData)));
      await tester.pumpAndSettle();

      expect(find.text('Share my business'), findsOneWidget);
      expect(find.text('Maya\'s Cakes'), findsOneWidget);
      expect(find.text('Copy Link'), findsOneWidget);
      expect(find.byType(Wrap), findsWidgets);
    });
  });
}
