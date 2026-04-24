import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/swarm_observability_dashboard.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:mocktail/mocktail.dart';
import 'package:path/path.dart' as path;

class MockWebSocketChannel extends Mock implements WebSocketChannel {}
class MockWebSocketSink extends Mock implements WebSocketSink {}

File? findFile(String filename) {
  var dir = Directory.current;
  for (var i = 0; i < 5; i++) {
    final file1 = File(path.join(dir.path, filename));
    if (file1.existsSync()) return file1;
    final file2 = File(path.join(dir.path, 'testdata', filename));
    if (file2.existsSync()) return file2;
    final file3 = File(path.join(dir.path, 'test_data', filename));
    if (file3.existsSync()) return file3;
    final file4 = File(path.join(dir.path, 'test', 'testdata', filename));
    if (file4.existsSync()) return file4;
    
    dir = dir.parent;
  }
  return null;
}

void main() {
  late MockWebSocketChannel mockChannel;
  late MockWebSocketSink mockSink;
  late StreamController streamController;

  setUp(() {
    mockChannel = MockWebSocketChannel();
    mockSink = MockWebSocketSink();
    streamController = StreamController();
    
    when(() => mockChannel.stream).thenAnswer((_) => streamController.stream);
    when(() => mockChannel.sink).thenReturn(mockSink);
  });

  tearDown(() {
    streamController.close();
  });

  testWidgets('SwarmObservabilityDashboard renders tasks and messages from stream', (WidgetTester tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SwarmObservabilityDashboard(channel: mockChannel),
        ),
      ),
    );

    // Initial state should be empty lists
    expect(find.text('Tasks'), findsOneWidget);
    expect(find.text('Agent Mesh'), findsOneWidget);
    
    // Hardcoded data because Bazel sandbox prevents reading external files in this setup
    final swarmData = jsonEncode({
      'tasks': [
        {'name': 'Task 1', 'progress': 0.5, 'isWorking': true},
      ],
      'messages': [
        {'sender': 'Agent A', 'message': 'Hello', 'timestamp': '2026-04-23T21:30:00Z'},
      ],
    });
    
    streamController.add(swarmData);
    await tester.pump(); // Process stream event

    expect(find.text('Task 1'), findsOneWidget);
    expect(find.text('Agent A'), findsOneWidget);
    expect(find.text('Hello'), findsOneWidget);
  });
}
