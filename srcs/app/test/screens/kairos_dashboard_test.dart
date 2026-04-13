import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/kairos_dashboard.dart';

void main() {
  testWidgets('KairosDashboardScreen renders correctly', (WidgetTester tester) async {
    await tester.runAsync(() async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: KairosDashboardScreen(),
        ),
      ),
    );
    expect(find.text('Swarm Analytics Dashboard'), findsOneWidget);
    expect(find.text('Shared Task Queue'), findsOneWidget);
    expect(find.text('Teammate Mesh Stream'), findsOneWidget);
    expect(find.text('AutoDream Memory'), findsOneWidget);
    });
  });
}
