import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/autodream_pipeline_widget.dart';

void main() {
  testWidgets('AutoDreamPipelineWidget renders correctly with all nodes', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: Center(
            child: AutoDreamPipelineWidget(),
          ),
        ),
      ),
    );

    // Verify Title
    expect(find.text('AutoDream Pipeline Stream'), findsOneWidget);

    // Verify Nodes
    expect(find.text('Extract'), findsOneWidget);
    expect(find.text('Analyze'), findsOneWidget);
    expect(find.text('Embed'), findsOneWidget);
    expect(find.text('Store'), findsOneWidget);

    // Verify Icons
    expect(find.byIcon(Icons.data_object), findsOneWidget);
    expect(find.byIcon(Icons.psychology), findsOneWidget);
    expect(find.byIcon(Icons.scatter_plot), findsOneWidget);
    expect(find.byIcon(Icons.save_alt), findsOneWidget);

    // Let animation run briefly
    await tester.pump(const Duration(milliseconds: 500));

    // Verify it doesn't crash during animation
    expect(find.byType(AutoDreamPipelineWidget), findsOneWidget);
  });
}
