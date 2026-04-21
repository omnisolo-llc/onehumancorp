import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/widgets/autodream_pipeline_widget.dart';

void main() {
  testWidgets('AutoDreamPipelineWidget renders correctly with Semantics and Tooltips', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AutoDreamPipelineWidget(),
        ),
      ),
    );

    expect(find.text('AutoDream Pipeline Stream'), findsOneWidget);
    expect(find.text('Extract'), findsOneWidget);
    expect(find.text('Analyze'), findsOneWidget);
    expect(find.text('Embed'), findsOneWidget);
    expect(find.text('Store'), findsOneWidget);

    expect(find.byType(Semantics), findsWidgets);
    expect(find.byType(Tooltip), findsWidgets);
    expect(find.byTooltip('Processing node: Extract'), findsOneWidget);
    expect(find.byTooltip('Processing node: Analyze'), findsOneWidget);
    expect(find.byTooltip('Processing node: Embed'), findsOneWidget);
    expect(find.byTooltip('Processing node: Store'), findsOneWidget);
  });
}
