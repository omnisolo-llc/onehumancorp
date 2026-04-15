import 'dart:convert';
import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_web_app/widgets/swarm_observability.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:stream_channel/stream_channel.dart';
import 'dart:async';

class MockWebSocketChannel extends StreamChannelMixin<dynamic> implements WebSocketChannel {
  final StreamController<dynamic> _streamController = StreamController<dynamic>();
  final StreamSink<dynamic> _sinkController = StreamController<dynamic>().sink;

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

  void emit(dynamic data) {
    _streamController.add(data);
  }
}

class _MockWebSocketSink implements WebSocketSink {
  final StreamSink<dynamic> _sink;
  _MockWebSocketSink(this._sink);

  @override
  void add(dynamic data) => _sink.add(data);

  @override
  void addError(Object error, [StackTrace? stackTrace]) => _sink.addError(error, stackTrace);

  @override
  Future<void> addStream(Stream<dynamic> stream) => _sink.addStream(stream);

  @override
  Future<void> close([int? closeCode, String? closeReason]) => _sink.close();

  @override
  Future<void> get done => _sink.done;
}

void main() {
  testWidgets('SwarmObservabilityWidget renders correctly initially', (WidgetTester tester) async {
    final mockChannel = MockWebSocketChannel();
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SwarmObservabilityWidget(channel: mockChannel),
        ),
      ),
    );

    expect(find.text('Swarm Intelligence'), findsOneWidget);
    expect(find.text('No agents active'), findsOneWidget);
  });

  testWidgets('SwarmObservabilityWidget updates when message received', (WidgetTester tester) async {
    final mockChannel = MockWebSocketChannel();
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SwarmObservabilityWidget(channel: mockChannel),
        ),
      ),
    );

    expect(find.text('No agents active'), findsOneWidget);

    mockChannel.emit(jsonEncode([
      {'name': 'Palette', 'status': 'HEALTHY'},
      {'name': 'Nova', 'status': 'WORKING'},
    ]));

    await tester.pumpAndSettle();

    expect(find.text('No agents active'), findsNothing);
    expect(find.text('Palette'), findsOneWidget);
    expect(find.text('HEALTHY'), findsOneWidget);
    expect(find.text('Nova'), findsOneWidget);
    expect(find.text('WORKING'), findsOneWidget);
  });
}
