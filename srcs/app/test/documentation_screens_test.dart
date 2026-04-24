import 'package:flutter/material.dart';
import "package:flutter_riverpod/flutter_riverpod.dart";
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/api_docs_screen.dart';
import 'package:ohc_app/screens/release_notes_screen.dart';
import 'package:ohc_app/screens/video_tutorials_screen.dart';
import 'package:ohc_app/widgets/walkthrough_overlay.dart';

void main() {
  testWidgets('ApiDocsScreen renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(
      home: ProviderScope(child: ApiDocsScreen()),
    ));
    await tester.pumpAndSettle();

    expect(find.textContaining('API Documentation'), findsOneWidget);
    expect(find.text('/api/v1/agents'), findsOneWidget);
    expect(find.text('Hire a new agent with a specific role.'), findsOneWidget);
  });

  testWidgets('ReleaseNotesScreen renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(
      home: ReleaseNotesScreen(),
    ));

    expect(find.text('Release Notes'), findsOneWidget);
    expect(find.text('v1.4.0'), findsOneWidget);
    expect(find.text('October 24, 2023'), findsOneWidget);
  });

  testWidgets('VideoTutorialsScreen renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(
      home: VideoTutorialsScreen(),
    ));

    expect(find.text('Video Tutorials'), findsOneWidget);
    expect(find.text('Set up your store in 5 minutes'), findsOneWidget);
    expect(find.text('4:30'), findsOneWidget);
  });

  testWidgets('WalkthroughOverlay renders using Overlay', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(
      home: Scaffold(
        body: Builder(
          builder: (context) {
            return Center(
              child: ElevatedButton(
                onPressed: () {
                  WalkthroughOverlay.show(
                    context,
                    title: 'Test Walkthrough',
                    content: 'This is a test.',
                    onDismiss: () {},
                  );
                },
                child: const Text('Show'),
              ),
            );
          },
        ),
      ),
    ));

    // Tap button to show overlay
    await tester.tap(find.text('Show'));
    await tester.pumpAndSettle();

    expect(find.text('Test Walkthrough'), findsOneWidget);
    expect(find.text('This is a test.'), findsOneWidget);
  });
}
