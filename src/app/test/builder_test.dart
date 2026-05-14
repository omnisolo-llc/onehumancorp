import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/builder/models.dart';
import 'package:ohc_app/builder/engine.dart';

void main() {
  test('Block fromJson', () {
    final block = Block.fromJson({
      'id': '123',
      'block_type': 'HeroBlock',
      'content': {'headline': 'Hello'},
      'sort_order': 0,
    });
    expect(block.id, '123');
    expect(block.blockType, 'HeroBlock');
    expect(block.content['headline'], 'Hello');
    expect(block.sortOrder, 0);
  });

  testWidgets('StorefrontEngine renders HeroBlock', (WidgetTester tester) async {
    final block = Block(
      id: '1',
      blockType: 'HeroBlock',
      content: {'headline': 'My Store', 'subtitle': 'Welcome'},
      sortOrder: 0,
    );

    await tester.pumpWidget(MaterialApp(
      home: Scaffold(
        body: StorefrontEngine(blocks: [block]),
      ),
    ));

    expect(find.text('My Store'), findsOneWidget);
    expect(find.text('Welcome'), findsOneWidget);
  });

  testWidgets('StorefrontEngine renders ProductGridBlock', (WidgetTester tester) async {
    final block = Block(
      id: '2',
      blockType: 'ProductGridBlock',
      content: {
          'items': ['Cake', 'Cookie', 'Bread', 'Pie']
      },
      sortOrder: 1,
    );
    await tester.pumpWidget(MaterialApp(
      home: Scaffold(
        body: StorefrontEngine(blocks: [block]),
      ),
    ));

    expect(find.text('Cake'), findsOneWidget);
    expect(find.text('Pie'), findsOneWidget);
  });

  testWidgets('StorefrontEngine renders ContactFormBlock', (WidgetTester tester) async {
    final block = Block(
      id: '3',
      blockType: 'ContactFormBlock',
      content: {'title': 'Contact Us Now'},
      sortOrder: 2,
    );

    await tester.pumpWidget(MaterialApp(
      home: Scaffold(
        body: StorefrontEngine(blocks: [block]),
      ),
    ));

    expect(find.text('Contact Us Now'), findsOneWidget);
    expect(find.text('Email'), findsOneWidget);
    expect(find.text('Message'), findsOneWidget);
    expect(find.text('Send'), findsOneWidget);
  });

  testWidgets('StorefrontEngine renders TestimonialBlock', (WidgetTester tester) async {
    final block = Block(
      id: '4',
      blockType: 'TestimonialBlock',
      content: {'testimonials': ['Great service', 'Awesome']},
      sortOrder: 3,
    );

    await tester.pumpWidget(MaterialApp(
      home: Scaffold(
        body: StorefrontEngine(blocks: [block]),
      ),
    ));

    expect(find.text('Great service'), findsOneWidget);
    expect(find.text('Awesome'), findsOneWidget);
  });

  testWidgets('StorefrontEngine renders ServiceBookingBlock', (WidgetTester tester) async {
    final block = Block(
      id: '5',
      blockType: 'ServiceBookingBlock',
      content: {'services': ['Plumbing']},
      sortOrder: 4,
    );

    await tester.pumpWidget(MaterialApp(
      home: Scaffold(
        body: StorefrontEngine(blocks: [block]),
      ),
    ));

    expect(find.text('Plumbing'), findsOneWidget);
    expect(find.text('Book'), findsOneWidget);
  });
}
