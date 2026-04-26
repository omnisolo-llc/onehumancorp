import 'package:flutter/material.dart';

class AiHelpChatButton extends StatefulWidget {
  const AiHelpChatButton({super.key});

  @override
  State<AiHelpChatButton> createState() => _AiHelpChatButtonState();
}

class _AiHelpChatButtonState extends State<AiHelpChatButton> {
  bool _isOpen = false;
  final TextEditingController _controller = TextEditingController();
  final List<String> _messages = [];

  void _toggleChat() {
    setState(() {
      _isOpen = !_isOpen;
    });
  }

  void _sendMessage() {
    if (_controller.text.isNotEmpty) {
      setState(() {
        _messages.add(_controller.text);
        _controller.clear();
        // Simulate AI response
        Future.delayed(const Duration(milliseconds: 500), () {
          setState(() {
            _messages.add("That's a great question! For more details on this topic, please visit our Help Center articles.");
          });
        });
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Stack(
      alignment: Alignment.bottomRight,
      children: [
        if (_isOpen)
          Positioned(
            bottom: 80,
            right: 16,
            child: Material(
              elevation: 8,
              borderRadius: BorderRadius.circular(16),
              child: Container(
                width: 300,
                height: 400,
                decoration: BoxDecoration(
                  color: Theme.of(context).colorScheme.surface,
                  borderRadius: BorderRadius.circular(16),
                  border: Border.all(color: Colors.grey.withValues(alpha: 0.2)),
                ),
                child: Column(
                  children: [
                    Container(
                      padding: const EdgeInsets.all(12),
                      decoration: BoxDecoration(
                        color: Theme.of(context).primaryColor,
                        borderRadius: const BorderRadius.vertical(top: Radius.circular(16)),
                      ),
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          const Text(
                            'Ask OHC Help',
                            style: TextStyle(color: Colors.white, fontWeight: FontWeight.bold),
                          ),
                          IconButton(
                            icon: const Icon(Icons.close, color: Colors.white, size: 20),
                            onPressed: _toggleChat,
                            padding: EdgeInsets.zero,
                            constraints: const BoxConstraints(),
                          ),
                        ],
                      ),
                    ),
                    Expanded(
                      child: ListView.builder(
                        padding: const EdgeInsets.all(8),
                        itemCount: _messages.length,
                        itemBuilder: (context, index) {
                          final isUser = index % 2 == 0;
                          return Align(
                            alignment: isUser ? Alignment.centerRight : Alignment.centerLeft,
                            child: Container(
                              margin: const EdgeInsets.symmetric(vertical: 4),
                              padding: const EdgeInsets.all(12),
                              decoration: BoxDecoration(
                                color: isUser ? Colors.blue.withValues(alpha: 0.1) : Colors.grey.withValues(alpha: 0.1),
                                borderRadius: BorderRadius.circular(12),
                              ),
                              child: Text(_messages[index]),
                            ),
                          );
                        },
                      ),
                    ),
                    Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: Row(
                        children: [
                          Expanded(
                            child: TextField(
                              controller: _controller,
                              decoration: const InputDecoration(
                                hintText: 'Type your question...',
                                border: OutlineInputBorder(),
                                contentPadding: EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                              ),
                              onSubmitted: (_) => _sendMessage(),
                            ),
                          ),
                          IconButton(
                            icon: const Icon(Icons.send),
                            onPressed: _sendMessage,
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
        Positioned(
          bottom: 16,
          right: 16,
          child: FloatingActionButton(
            key: const Key('ai_help_chat_button'),
            onPressed: _toggleChat,
            child: Icon(_isOpen ? Icons.close : Icons.chat),
          ),
        ),
      ],
    );
  }
}
