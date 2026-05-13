import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:app/src/blocks.dart';

void main() {
  testWidgets('HeroBlock renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: Scaffold(body: HeroBlock())));
    expect(find.text('Hero Headline'), findsOneWidget);
    expect(find.text('Hero Subtitle'), findsOneWidget);
    expect(find.text('Book Now'), findsOneWidget);
  });

  testWidgets('ProductGridBlock renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: Scaffold(body: ProductGridBlock())));
    expect(find.text('Product 0'), findsOneWidget);
  });

  testWidgets('CalendarBlock renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: Scaffold(body: CalendarBlock())));
    expect(find.text('Calendar'), findsOneWidget);
  });
}
