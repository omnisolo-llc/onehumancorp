import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../providers/inbox_provider.dart';

class UnifiedInboxScreen extends ConsumerStatefulWidget {
  const UnifiedInboxScreen({super.key});

  @override
  ConsumerState<UnifiedInboxScreen> createState() => _UnifiedInboxScreenState();
}

class _UnifiedInboxScreenState extends ConsumerState<UnifiedInboxScreen> {
  final TextEditingController _replyController = TextEditingController();
  final ScrollController _scrollController = ScrollController();

  void _connectInstagram() {
    ref.read(inboxProvider.notifier).connectInstagram();
    _scrollToBottom();
  }

  void _connectWhatsApp() {
    ref.read(inboxProvider.notifier).connectWhatsApp();
    _scrollToBottom();
  }

  void _sendReply() {
    ref.read(inboxProvider.notifier).sendReply(_replyController.text);
    _replyController.clear();
    _scrollToBottom();
  }

  void _scrollToBottom() {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (_scrollController.hasClients) {
        _scrollController.animateTo(
          _scrollController.position.maxScrollExtent,
          duration: const Duration(milliseconds: 300),
          curve: Curves.easeOut,
        );
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    final inboxState = ref.watch(inboxProvider);

    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
        title: const Text(
          "Unified Inbox",
          style: TextStyle(
            fontFamily: 'Outfit',
            fontWeight: FontWeight.bold,
          ),
        ),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: BoxConstraints(maxWidth: MediaQuery.of(context).size.width > 600 ? 600 : MediaQuery.of(context).size.width),
          child: Column(
            children: [
              if (!inboxState.instagramConnected || !inboxState.whatsappConnected)
                Padding(
                  padding: const EdgeInsets.all(20.0),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      const Text(
                        "Connect Platforms",
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 18,
                          fontWeight: FontWeight.bold,
                          color: Colors.white,
                        ),
                      ),
                      const SizedBox(height: 10),
                      if (!inboxState.instagramConnected)
                        ElevatedButton(
                          key: const Key('connectInstagramBtn'),
                          onPressed: _connectInstagram,
                          style: ElevatedButton.styleFrom(
                            backgroundColor: const Color(0xFFE1306C),
                            padding: const EdgeInsets.symmetric(vertical: 15),
                            shape: RoundedRectangleBorder(
                              borderRadius: BorderRadius.circular(10),
                            ),
                          ),
                          child: const Text("Connect Instagram", style: TextStyle(color: Colors.white)),
                        ),
                      const SizedBox(height: 10),
                      if (!inboxState.whatsappConnected)
                        ElevatedButton(
                          key: const Key('connectWhatsappBtn'),
                          onPressed: _connectWhatsApp,
                          style: ElevatedButton.styleFrom(
                            backgroundColor: const Color(0xFF25D366),
                            padding: const EdgeInsets.symmetric(vertical: 15),
                            shape: RoundedRectangleBorder(
                              borderRadius: BorderRadius.circular(10),
                            ),
                          ),
                          child: const Text("Connect WhatsApp", style: TextStyle(color: Colors.white)),
                        ),
                    ],
                  ),
                ),
              Expanded(
                child: ListView.builder(
                  controller: _scrollController,
                  padding: const EdgeInsets.all(20),
                  itemCount: inboxState.messages.length,
                  itemBuilder: (context, index) {
                    final msg = inboxState.messages[index];
                    final isMe = msg.isMe;
                    return Padding(
                      padding: const EdgeInsets.only(bottom: 15.0),
                      child: Column(
                        crossAxisAlignment: isMe ? CrossAxisAlignment.end : CrossAxisAlignment.start,
                        children: [
                          Row(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              Text(
                                msg.sender,
                                style: TextStyle(
                                  color: Colors.white.withAlpha(153),
                                  fontSize: 12,
                                  fontWeight: FontWeight.bold,
                                ),
                              ),
                              const SizedBox(width: 5),
                              Text(
                                "via ${msg.platform}",
                                style: TextStyle(
                                  color: Colors.white.withAlpha(102),
                                  fontSize: 10,
                                ),
                              ),
                            ],
                          ),
                          const SizedBox(height: 5),
                          Container(
                            padding: const EdgeInsets.all(15),
                            decoration: BoxDecoration(
                              color: isMe ? const Color(0xFF6B4EFF) : Colors.white.withAlpha(25),
                              borderRadius: BorderRadius.circular(15),
                              border: Border.all(
                                color: isMe ? Colors.transparent : Colors.white.withAlpha(51),
                              ),
                            ),
                            child: Text(
                              msg.message,
                              style: const TextStyle(color: Colors.white),
                            ),
                          ),
                          const SizedBox(height: 5),
                          Text(
                            msg.time,
                            style: TextStyle(
                              color: Colors.white.withAlpha(102),
                              fontSize: 10,
                            ),
                          ),
                        ],
                      ),
                    );
                  },
                ),
              ),
              if (inboxState.messages.isNotEmpty)
                Padding(
                  padding: const EdgeInsets.all(20.0),
                  child: Row(
                    children: [
                      Expanded(
                        child: TextField(
                          key: const Key('replyTextField'),
                          controller: _replyController,
                          style: const TextStyle(color: Colors.white),
                          decoration: InputDecoration(
                            hintText: "Type a reply...",
                            hintStyle: TextStyle(color: Colors.white.withAlpha(128)),
                            filled: true,
                            fillColor: Colors.white.withAlpha(25),
                            border: OutlineInputBorder(
                              borderRadius: BorderRadius.circular(20),
                              borderSide: BorderSide.none,
                            ),
                            contentPadding: const EdgeInsets.symmetric(horizontal: 20, vertical: 15),
                          ),
                        ),
                      ),
                      const SizedBox(width: 10),
                      Container(
                        decoration: const BoxDecoration(
                          color: Color(0xFF6B4EFF),
                          shape: BoxShape.circle,
                        ),
                        child: IconButton(
                          key: const Key('sendReplyBtn'),
                          icon: const Icon(Icons.send, color: Colors.white),
                          onPressed: _sendReply,
                        ),
                      ),
                    ],
                  ),
                ),
            ],
          ),
        ),
      ),
    );
  }
}
