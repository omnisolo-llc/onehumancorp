import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/swarm_observability_screen.dart';

void main() {
  testWidgets('SwarmObservabilityScreen renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: SwarmObservabilityScreen(),
        ),
      ),
    );

    expect(find.text('Swarm Observability'), findsOneWidget);
    expect(find.text('Swarm Intelligence Mesh'), findsOneWidget);
  });
}
