import 'package:flutter/material.dart';

class AiHelpChat extends StatefulWidget {
  const AiHelpChat({super.key});

  @override
  State<AiHelpChat> createState() => _AiHelpChatState();
}

class _AiHelpChatState extends State<AiHelpChat> {
  bool _isOpen = false;
  final TextEditingController _controller = TextEditingController();
  final List<String> _messages = [
    "Hi there! I'm your OHC Help Agent. Ask me anything about setting up or managing your business."
  ];

  void _toggleChat() {
    setState(() {
      _isOpen = !_isOpen;
    });
  }

  void _sendMessage() {
    if (_controller.text.trim().isEmpty) return;

    setState(() {
      _messages.add(_controller.text);
      _messages.add("That's a great question! For more details on this topic, please visit our Help Center articles.");
    });
    _controller.clear();
  }

  @override
  Widget build(BuildContext context) {
    if (!_isOpen) {
      return Positioned(
        bottom: 24,
        right: 24,
        child: FloatingActionButton(
          key: const Key('ai_help_chat_button'),
          onPressed: _toggleChat,
          child: const Icon(Icons.help_outline),
        ),
      );
    }

    return Positioned(
      bottom: 24,
      right: 24,
      child: Material(
        elevation: 8,
        borderRadius: BorderRadius.circular(16),
        child: Container(
          width: 350,
          height: 450,
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.surface,
            borderRadius: BorderRadius.circular(16),
          ),
          child: Column(
            children: [
              _buildHeader(),
              Expanded(child: _buildMessageList()),
              _buildInputArea(),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildHeader() {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.primaryContainer,
        borderRadius: const BorderRadius.vertical(top: Radius.circular(16)),
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          const Text(
            'Ask OHC Help',
            style: TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
          ),
          IconButton(
            icon: const Icon(Icons.close),
            onPressed: _toggleChat,
            padding: EdgeInsets.zero,
            constraints: const BoxConstraints(),
          ),
        ],
      ),
    );
  }

  Widget _buildMessageList() {
    return ListView.builder(
      padding: const EdgeInsets.all(16),
      itemCount: _messages.length,
      itemBuilder: (context, index) {
        final isUser = index % 2 != 0;
        return Align(
          alignment: isUser ? Alignment.centerRight : Alignment.centerLeft,
          child: Container(
            margin: const EdgeInsets.only(bottom: 12),
            padding: const EdgeInsets.all(12),
            decoration: BoxDecoration(
              color: isUser
                  ? Theme.of(context).colorScheme.primary
                  : Theme.of(context).colorScheme.surfaceContainerHighest,
              borderRadius: BorderRadius.circular(12),
            ),
            child: Text(
              _messages[index],
              style: TextStyle(
                color: isUser ? Theme.of(context).colorScheme.onPrimary : null,
                fontFamily: 'Inter',
              ),
            ),
          ),
        );
      },
    );
  }

  Widget _buildInputArea() {
    return Padding(
      padding: const EdgeInsets.all(12.0),
      child: Row(
        children: [
          Expanded(
            child: TextField(
              controller: _controller,
              decoration: const InputDecoration(
                hintText: 'Type your question...',
                border: OutlineInputBorder(),
                contentPadding: EdgeInsets.symmetric(horizontal: 16, vertical: 8),
              ),
              onSubmitted: (_) => _sendMessage(),
            ),
          ),
          const SizedBox(width: 8),
          IconButton(
            icon: const Icon(Icons.send),
            onPressed: _sendMessage,
            color: Theme.of(context).colorScheme.primary,
          ),
        ],
      ),
    );
  }
}
