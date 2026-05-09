import 'package:flutter_riverpod/flutter_riverpod.dart';

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

  void connectInstagram() {
    state = state.copyWith(
      instagramConnected: true,
      messages: [...state.messages],
    );
  }

  void connectWhatsApp() {
    state = state.copyWith(
      whatsappConnected: true,
      messages: [...state.messages],
    );
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
