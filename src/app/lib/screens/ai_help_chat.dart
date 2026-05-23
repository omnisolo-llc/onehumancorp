import 'package:flutter/material.dart';
import '../help_registry.dart';

class ChatMessage {
  final String text;
  final bool isMe;
  final HelpArticle? linkedArticle;

  ChatMessage({required this.text, required this.isMe, this.linkedArticle});
}

class AiHelpChatScreen extends StatefulWidget {
  @override
  _AiHelpChatScreenState createState() => _AiHelpChatScreenState();
}

class _AiHelpChatScreenState extends State<AiHelpChatScreen> {
  final TextEditingController _controller = TextEditingController();
  final List<ChatMessage> _messages = [
    ChatMessage(text: "Hi! I'm your OHC Help Assistant. I can answer any questions you have about using the app. What do you need help with today?", isMe: false),
  ];

  void _sendMessage() {
    if (_controller.text.trim().isEmpty) return;

    final query = _controller.text;
    setState(() {
      _messages.add(ChatMessage(text: query, isMe: true));
    });
    _controller.clear();

    // Simulate AI response based on HelpRegistry
    Future.delayed(const Duration(seconds: 1), () {
      final lowerQuery = query.toLowerCase();
      HelpArticle? match;

      try {
        match = HelpRegistry().articles.firstWhere(
          (article) => article.title.toLowerCase().contains(lowerQuery) ||
                       article.description.toLowerCase().contains(lowerQuery)
        );
      } catch (e) {
        match = null;
      }

      setState(() {
        if (match != null) {
          _messages.add(ChatMessage(
            text: "I found something that might help: ${match.description}",
            isMe: false,
            linkedArticle: match
          ));
        } else {
          _messages.add(ChatMessage(text: "I'm still learning! Could you try asking about adding products, payments, or subscriptions?", isMe: false));
        }
      });
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.white,
      appBar: AppBar(
        title: const Text('AI Help Assistant', style: TextStyle(fontFamily: 'Outfit', color: Colors.black87, fontWeight: FontWeight.bold)),
        backgroundColor: Colors.white,
        elevation: 1,
        iconTheme: const IconThemeData(color: Colors.black87),
      ),
      body: Center(
        child: Container(
          maxWidth: 600,
          child: Column(
            children: [
              Expanded(
                child: ListView.builder(
                  padding: const EdgeInsets.all(16),
                  itemCount: _messages.length,
                  itemBuilder: (context, index) {
                    final msg = _messages[index];
                    return _buildChatBubble(msg);
                  },
                ),
              ),
              Container(
                padding: const EdgeInsets.all(16),
                decoration: BoxDecoration(
                  color: Colors.white,
                  border: Border(top: BorderSide(color: Colors.grey[200]!)),
                ),
                child: Row(
                  children: [
                    Expanded(
                      child: TextField(
                        controller: _controller,
                        decoration: InputDecoration(
                          hintText: 'Type your question...',
                          border: OutlineInputBorder(borderRadius: BorderRadius.circular(24)),
                          contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                        ),
                        onSubmitted: (_) => _sendMessage(),
                      ),
                    ),
                    const SizedBox(width: 8),
                    FloatingActionButton(
                      mini: true,
                      onPressed: _sendMessage,
                      backgroundColor: const Color(0xFF0EA5E9),
                      elevation: 0,
                      child: const Icon(Icons.send, size: 20),
                    )
                  ],
                ),
              )
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildChatBubble(ChatMessage msg) {
    return Align(
      alignment: msg.isMe ? Alignment.centerRight : Alignment.centerLeft,
      child: Container(
        margin: const EdgeInsets.only(bottom: 12),
        constraints: BoxConstraints(maxWidth: MediaQuery.of(context).size.width * 0.7),
        child: Column(
          crossAxisAlignment: msg.isMe ? CrossAxisAlignment.end : CrossAxisAlignment.start,
          children: [
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: msg.isMe ? const Color(0xFF0EA5E9) : const Color(0xFFF1F5F9),
                borderRadius: BorderRadius.only(
                  topLeft: const Radius.circular(16),
                  topRight: const Radius.circular(16),
                  bottomLeft: Radius.circular(msg.isMe ? 16 : 0),
                  bottomRight: Radius.circular(msg.isMe ? 0 : 16),
                ),
              ),
              child: Text(
                msg.text,
                style: TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 14,
                  color: msg.isMe ? Colors.white : const Color(0xFF0F172A),
                ),
              ),
            ),
            if (msg.linkedArticle != null)
              Padding(
                padding: const EdgeInsets.only(top: 4, left: 8),
                child: InkWell(
                  onTap: () {
                    // Could show modal here similar to HelpCenter
                    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Opening: ${msg.linkedArticle!.title}')));
                  },
                  child: const Text(
                    'Read the full article →',
                    style: TextStyle(color: Color(0xFF0EA5E9), fontWeight: FontWeight.bold, fontSize: 12),
                  ),
                ),
              )
          ],
        ),
      ),
    );
  }
}

extension on Container {
  get maxWidth => 600.0;
}
