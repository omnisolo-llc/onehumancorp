import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'dart:async';
import '../lib/screens/kairos_dashboard.dart';
import 'package:stream_channel/stream_channel.dart';

class MockWebSocketChannel extends StreamChannelMixin implements WebSocketChannel {
  final StreamController<dynamic> _streamController = StreamController<dynamic>();

  @override
  Stream<dynamic> get stream => _streamController.stream;

  @override
  WebSocketSink get sink => MockWebSocketSink();

  void addMessage(dynamic message) {
    _streamController.add(message);
  }

  @override
  String? get protocol => null;

  @override
  int? get closeCode => null;

  @override
  String? get closeReason => null;

  @override
  Future<void> get ready => Future.value();
}

class MockWebSocketSink implements WebSocketSink {
  @override
  void add(dynamic data) {}

  @override
  void addError(Object error, [StackTrace? stackTrace]) {}

  @override
  Future addStream(Stream stream) async {}

  @override
  Future close([int? closeCode, String? closeReason]) async {}

  @override
  Future get done => Future.value();
}

void main() {
  testWidgets('KairosDashboardScreen displays three panels', (WidgetTester tester) async {
    final mockChannel = MockWebSocketChannel();
    await tester.pumpWidget(MaterialApp(home: KairosDashboardScreen(channel: mockChannel)));

    expect(find.text('Shared Task Queue'), findsOneWidget);
    expect(find.text('Teammate Mesh Stream'), findsOneWidget);
    expect(find.text('AutoDream Memory'), findsOneWidget);
  });
}
