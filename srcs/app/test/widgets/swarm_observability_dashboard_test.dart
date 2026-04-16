import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:stream_channel/stream_channel.dart';
import 'dart:async';
import 'package:ohc_app/widgets/swarm_observability_dashboard.dart';
import 'package:ohc_app/widgets/agent_task_progress.dart';
import 'package:ohc_app/widgets/agent_mesh_message_tile.dart';

class MockWebSocketChannel extends StreamChannelMixin implements WebSocketChannel {
  final StreamController<dynamic> _streamController = StreamController<dynamic>.broadcast();
  final StreamController<dynamic> _sinkController = StreamController<dynamic>();

  @override
  Stream<dynamic> get stream => _streamController.stream;

  @override
  WebSocketSink get sink => _MockWebSocketSink(_sinkController);

  @override
  Future<void> get ready => Future.value();

  @override
  String? get protocol => null;

  @override
  int? get closeCode => null;

  @override
  String? get closeReason => null;

  void addMessage(String message) {
    _streamController.add(message);
  }

  void closeWebSocket() {
    _streamController.close();
    _sinkController.close();
  }
}

class _MockWebSocketSink implements WebSocketSink {
  final StreamController<dynamic> controller;
  _MockWebSocketSink(this.controller);

  @override
  void add(dynamic data) => controller.add(data);

  @override
  void addError(Object error, [StackTrace? stackTrace]) => controller.addError(error, stackTrace);

  @override
  Future addStream(Stream<dynamic> stream) => controller.addStream(stream);

  @override
  Future close([int? closeCode, String? closeReason]) => controller.close();

  @override
  Future get done => controller.done;
}

void main() {
  testWidgets('SwarmObservabilityDashboard displays tasks from websocket', (WidgetTester tester) async {
    final channel = MockWebSocketChannel();

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SwarmObservabilityDashboard(channel: channel),
        ),
      ),
    );

    expect(find.text('Swarm Observability'), findsOneWidget);

    channel.addMessage(jsonEncode({
      'tasks': [
        {'id': '1', 'name': 'Data processing', 'progress': 0.4, 'isWorking': true},
      ]
    }));

    // Use pump instead of pumpAndSettle because AgentTaskProgressWidget has infinite animation
    await tester.pump(const Duration(milliseconds: 500));

    expect(find.text('Data processing'), findsOneWidget);

    // Check if AgentTaskProgressWidget has the correct progress
    final progressBarFinder = find.byType(LinearProgressIndicator);
    expect(progressBarFinder, findsOneWidget);
    final LinearProgressIndicator progressBar = tester.widget(progressBarFinder);
    expect(progressBar.value, 0.4);

    // Now test messages
    channel.addMessage(jsonEncode({
      'messages': [
        {'sender': 'Guide', 'message': 'Hello from Guide', 'timestamp': '2026-04-15T08:00:00Z'},
      ]
    }));

    await tester.pump(const Duration(milliseconds: 500));

    expect(find.text('Agent Mesh'), findsOneWidget);
    expect(find.text('Guide'), findsOneWidget);
    expect(find.text('Hello from Guide'), findsOneWidget);

    channel.closeWebSocket();
  });
}
