import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/builder/models.dart';
import 'package:ohc_app/builder/editor.dart';

void main() {
  testWidgets('StorefrontEditor renders and allows editing', (WidgetTester tester) async {
    final blocks = [
      Block(
        id: '1',
        blockType: 'HeroBlock',
        content: {'headline': 'Initial'},
        sortOrder: 0,
      )
    ];

    bool published = false;
    List<Block> savedBlocks = [];

    await tester.pumpWidget(MaterialApp(
      home: StorefrontEditor(
        initialBlocks: blocks,
        onSave: (b) { savedBlocks = b; },
        onPublish: () { published = true; },
      ),
    ));

    expect(find.text('HeroBlock'), findsOneWidget);

    await tester.tap(find.text('HeroBlock'));
    await tester.pumpAndSettle();

    expect(find.text('Edit HeroBlock'), findsOneWidget);

    await tester.enterText(find.byType(TextField), 'Updated Headline');
    await tester.tap(find.text('Save Block'));
    await tester.pumpAndSettle();

    expect(savedBlocks[0].content['headline'], 'Updated Headline');

    await tester.tap(find.text('Publish'));
    await tester.pumpAndSettle();

    expect(published, true);
  });
}
