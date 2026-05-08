import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';

class InboxMessage {
  final String platform;
  final String sender;
  final String message;
  final String time;
  final bool isMe;

  InboxMessage({
    required this.platform,
    required this.sender,
    required this.message,
    required this.time,
    required this.isMe,
  });

  factory InboxMessage.fromJson(Map<String, dynamic> json) {
    return InboxMessage(
      platform: json['platform'],
      sender: json['sender'],
      message: json['message'],
      time: json['time'],
      isMe: json['isMe'] ?? false,
    );
  }
}

class InboxState {
  final bool instagramConnected;
  final bool whatsappConnected;
  final List<InboxMessage> messages;

  InboxState({
    this.instagramConnected = false,
    this.whatsappConnected = false,
    this.messages = const [],
  });

  InboxState copyWith({
    bool? instagramConnected,
    bool? whatsappConnected,
    List<InboxMessage>? messages,
  }) {
    return InboxState(
      instagramConnected: instagramConnected ?? this.instagramConnected,
      whatsappConnected: whatsappConnected ?? this.whatsappConnected,
      messages: messages ?? this.messages,
    );
  }
}

class InboxNotifier extends StateNotifier<InboxState> {
  InboxNotifier() : super(InboxState());

  Future<void> connectInstagram() async {
    try {
      final response = await http.get(Uri.parse('http://localhost:8080/api/v1/inbox/instagram'));
      if (response.statusCode == 200) {
        final List<dynamic> data = jsonDecode(response.body);
        final newMessages = data.map((m) => InboxMessage.fromJson(m)).toList();
        state = state.copyWith(
          instagramConnected: true,
          messages: [...state.messages, ...newMessages],
        );
      } else {
        state = state.copyWith(instagramConnected: true);
      }
    } catch (_) {
      state = state.copyWith(instagramConnected: true);
    }
  }

  Future<void> connectWhatsApp() async {
    try {
      final response = await http.get(Uri.parse('http://localhost:8080/api/v1/inbox/whatsapp'));
      if (response.statusCode == 200) {
        final List<dynamic> data = jsonDecode(response.body);
        final newMessages = data.map((m) => InboxMessage.fromJson(m)).toList();
        state = state.copyWith(
          whatsappConnected: true,
          messages: [...state.messages, ...newMessages],
        );
      } else {
        state = state.copyWith(whatsappConnected: true);
      }
    } catch (_) {
      state = state.copyWith(whatsappConnected: true);
    }
  }

  void sendReply(String replyText) {
    if (replyText.trim().isEmpty) return;

    final platform = state.messages.isNotEmpty ? state.messages.last.platform : "System";

    state = state.copyWith(
      messages: [
        ...state.messages,
        InboxMessage(
          platform: platform,
          sender: "Me",
          message: replyText,
          time: "Just now",
          isMe: true,
        )
      ],
    );
  }
}

final inboxProvider = StateNotifierProvider<InboxNotifier, InboxState>((ref) {
  return InboxNotifier();
});
