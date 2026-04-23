import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/api_docs_screen.dart';

void main() {
  testWidgets('ApiDocsScreen builds correctly', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: ApiDocsScreen()));

    expect(find.text('Developer API Docs'), findsWidgets);
    expect(find.text('Advanced Feature'), findsOneWidget);
    expect(find.text('API Reference'), findsOneWidget);

    expect(find.text('/api/v1/store/products'), findsOneWidget);
    expect(find.text('Retrieve a list of all products in your store.'), findsOneWidget);
  });
}
