import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/kairos_dashboard.dart';
import 'package:ohc_app/widgets/glass_card.dart';

void main() {
  testWidgets('KairosDashboardScreen renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: KairosDashboardScreen(),
        ),
      ),
    );

    expect(find.text('KAIROS Swarm Analytics'), findsOneWidget);
    expect(find.text('Shared Task Queue'), findsOneWidget);
    expect(find.text('Teammate Mesh Stream'), findsOneWidget);
    expect(find.text('AutoDream Memory'), findsOneWidget);
    expect(find.byType(GlassCard), findsNWidgets(3));
  });
}
