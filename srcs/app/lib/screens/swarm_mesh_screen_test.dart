import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/swarm_mesh_screen.dart';

void main() {
  testWidgets('SwarmMeshScreen renders properly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: SwarmMeshScreen(),
        ),
      ),
    );

    // Verify app bar title
    expect(find.text('Swarm Observability Mesh'), findsOneWidget);

    // Verify panel title
    expect(find.text('Realtime Teammate Mesh'), findsOneWidget);

    // Verify initial awaiting message
    expect(find.text('Awaiting swarm activity...'), findsOneWidget);
  });
}
