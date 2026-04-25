import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/skeleton_loading.dart';
import 'package:flutter/material.dart';

void main() {
  testWidgets('SkeletonLoading renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: SkeletonLoading(width: 100, height: 100),
        ),
      ),
    );

    expect(find.byType(SkeletonLoading), findsOneWidget);
  });

  testWidgets('DashboardSkeleton renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: DashboardSkeleton(),
        ),
      ),
    );

    expect(find.byType(DashboardSkeleton), findsOneWidget);
    expect(find.byType(SkeletonLoading), findsWidgets);
  });


  testWidgets('ListSkeleton renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: ListSkeleton(),
        ),
      ),
    );

    expect(find.byType(ListSkeleton), findsOneWidget);
    expect(find.byType(SkeletonLoading), findsWidgets);
  });
}
