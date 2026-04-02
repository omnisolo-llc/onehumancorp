import 'dart:convert';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'settings_service.dart';

class MeshMessage {
  final String senderId;
  final String role;
  final String content;
  final DateTime timestamp;

  MeshMessage({
    required this.senderId,
    required this.role,
    required this.content,
    required this.timestamp,
  });

  factory MeshMessage.fromJson(Map<String, dynamic> json) {
    return MeshMessage(
      senderId: json['sender_id'] ?? '',
      role: json['role'] ?? '',
      content: json['content'] ?? '',
      timestamp: json['timestamp'] != null ? DateTime.parse(json['timestamp']) : DateTime.now(),
    );
  }
}

final teammateMeshProvider = StreamProvider.family<MeshMessage, String>((ref, roomId) {
  final settings = ref.watch(clientSettingsProvider).valueOrNull;
  final baseUrl = settings?.backendUrl ?? 'http://localhost:18789';

  // Convert http:// to ws://
  final wsUrl = baseUrl.replaceFirst('http://', 'ws://').replaceFirst('https://', 'wss://');

  final channel = WebSocketChannel.connect(
    Uri.parse('$wsUrl/api/v1/mesh/rooms?room=$roomId'),
  );

  ref.onDispose(() => channel.sink.close());

  return channel.stream.map((message) {
    final decoded = jsonDecode(message);
    return MeshMessage.fromJson(decoded);
  });
});
