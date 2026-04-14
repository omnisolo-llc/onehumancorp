import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/swarm/swarm_observability_dashboard.dart';

void main() {
  testWidgets('SwarmObservabilityDashboard renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: SwarmObservabilityDashboard(),
          ),
        ),
      ),
    );

    expect(find.text('Swarm Intelligence Mesh'), findsOneWidget);
    expect(find.text('Monitoring agent swarm...'), findsOneWidget);
  });
}
