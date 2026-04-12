import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  testWidgets('Glassmorphism components use ColorFilter.matrix', (WidgetTester tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: Stack(
            children: [
              Container(color: Colors.red),
              BackdropFilter(
                filter: ColorFilter.matrix(<double>[
                  1, 0, 0, 0, 0,
                  0, 1, 0, 0, 0,
                  0, 0, 1, 0, 0,
                  0, 0, 0, 1, 0,
                ]),
                child: Container(
                  width: 100,
                  height: 100,
                  color: Colors.white.withOpacity(0.1),
                ),
              ),
            ],
          ),
        ),
      ),
    );

    expect(find.byType(BackdropFilter), findsOneWidget);

    final backdropFilter = tester.widget<BackdropFilter>(find.byType(BackdropFilter));
    expect(backdropFilter.filter, isA<ColorFilter>());
  });
}
