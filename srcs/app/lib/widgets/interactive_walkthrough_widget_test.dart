import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/interactive_walkthrough_widget.dart';

void main() {
  testWidgets('InteractiveWalkthrough renders child', (WidgetTester tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: InteractiveWalkthrough(
            steps: [],
            child: const Text('Child Widget'),
          ),
        ),
      ),
    );

    expect(find.text('Child Widget'), findsOneWidget);
  });

  testWidgets('InteractiveWalkthrough shows snackbar on start', (WidgetTester tester) async {
    final key = GlobalKey();
    final steps = [
      WalkthroughStep(key: key, title: 'Step 1', description: 'Description 1'),
    ];

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: InteractiveWalkthrough(
            steps: steps,
            child: Container(key: key, child: const Text('Target')),
          ),
        ),
      ),
    );

    final state = tester.state<InteractiveWalkthroughState>(find.byType(InteractiveWalkthrough));
    state.startWalkthrough();
    await tester.pumpAndSettle();

    expect(find.text('Step 1'), findsOneWidget);
    expect(find.text('Description 1'), findsOneWidget);
    expect(find.text('Finish'), findsOneWidget);
  });

  testWidgets('InteractiveWalkthrough goes to next step', (WidgetTester tester) async {
    final key1 = GlobalKey();
    final key2 = GlobalKey();
    final steps = [
      WalkthroughStep(key: key1, title: 'Step 1', description: 'Description 1'),
      WalkthroughStep(key: key2, title: 'Step 2', description: 'Description 2'),
    ];

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: InteractiveWalkthrough(
            steps: steps,
            child: Column(
              children: [
                Container(key: key1, child: const Text('Target 1')),
                Container(key: key2, child: const Text('Target 2')),
              ],
            ),
          ),
        ),
      ),
    );

    final state = tester.state<InteractiveWalkthroughState>(find.byType(InteractiveWalkthrough));
    state.startWalkthrough();
    await tester.pumpAndSettle();

    expect(find.text('Step 1'), findsOneWidget);
    expect(find.text('Next'), findsOneWidget);

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Step 2'), findsOneWidget);
    expect(find.text('Description 2'), findsOneWidget);
    expect(find.text('Finish'), findsOneWidget);
  });
}
