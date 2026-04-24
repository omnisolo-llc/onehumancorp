import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/org_tree_widget.dart';
import 'package:ohc_app/models/dashboard.dart';

void main() {
  testWidgets('OrgTreeWidget renders root members', (WidgetTester tester) async {
    final members = [
      const OrganizationMember(id: 'm1', name: 'Manager', role: 'CEO', isHuman: true),
      const OrganizationMember(id: 'm2', name: 'Employee', role: 'Dev', managerId: 'm1', isHuman: false),
    ];

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: OrgTreeWidget(members: members),
        ),
      ),
    );

    expect(find.text('Manager'), findsOneWidget);
    expect(find.text('CEO'), findsOneWidget);
    expect(find.text('YOU'), findsOneWidget); // Manager is human

    // Employee should also be rendered as a child
    expect(find.text('Employee'), findsOneWidget);
    expect(find.text('Dev'), findsOneWidget);
  });

  testWidgets('OrgTreeWidget applies indentation for depth', (WidgetTester tester) async {
    final members = [
      const OrganizationMember(id: 'm1', name: 'Manager', role: 'CEO', isHuman: false),
      const OrganizationMember(id: 'm2', name: 'Employee', role: 'Dev', managerId: 'm1', isHuman: false),
    ];

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: OrgTreeWidget(members: members),
        ),
      ),
    );

    final orgTreePaddings = find.byWidgetPredicate((widget) {
      if (widget is Padding) {
        final p = widget.padding;
        if (p is EdgeInsets) {
          return p.left == 20.0;
        }
      }
      return false;
    });

    expect(orgTreePaddings, findsOneWidget); // Only the child should have 20.0 left padding
  });

  testWidgets('OrgTreeWidget shows initials correctly', (WidgetTester tester) async {
    final members = [
      const OrganizationMember(id: 'm1', name: 'John Doe', role: 'software_engineer', isHuman: false),
      const OrganizationMember(id: 'm2', name: 'Alice Smith', role: 'Manager', isHuman: false),
    ];

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: OrgTreeWidget(members: members),
        ),
      ),
    );

    // 'software_engineer' should give initials 'SE'
    expect(find.text('SE'), findsOneWidget);

    // 'Manager' should give initials 'MA'
    expect(find.text('MA'), findsOneWidget);
  });
}
