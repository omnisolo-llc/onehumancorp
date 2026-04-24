import 'dart:ui';
import 'package:flutter/material.dart';

class AiHelpChatWidget extends StatefulWidget {
  const AiHelpChatWidget({super.key});

  @override
  State<AiHelpChatWidget> createState() => _AiHelpChatWidgetState();
}

class _AiHelpChatWidgetState extends State<AiHelpChatWidget> {
  bool _isOpen = false;
  final TextEditingController _controller = TextEditingController();
  final List<String> _messages = [
    "Hi there! I'm your One Human Corp AI Help Agent. How can I assist you with your business today?"
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
      _messages.add("I'm looking that up for you! Please refer to the Help Center for more detailed guides. Read the full article →");
      _controller.clear();
    });
  }

  @override
  Widget build(BuildContext context) {
    if (!_isOpen) {
      return FloatingActionButton.extended(
        onPressed: _toggleChat,
        backgroundColor: const Color.fromRGBO(30, 30, 30, 0.9),
        icon: const Icon(Icons.help_outline, color: Colors.blueAccent),
        label: const Text(
          'Ask AI Help',
          style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold, color: Colors.white),
        ),
      );
    }

    return ClipRRect(
      borderRadius: BorderRadius.circular(20),
      child: BackdropFilter(
        filter: ImageFilter.compose(
          outer: const ColorFilter.matrix(<double>[
            1.787, -0.715, -0.072, 0, 0,
            -0.213, 1.285, -0.072, 0, 0,
            -0.213, -0.715, 1.928, 0, 0,
            0, 0, 0, 1, 0,
          ]),
          inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        ),
        child: Container(
          width: 350,
          height: 500,
          decoration: BoxDecoration(
            color: const Color.fromRGBO(30, 30, 30, 0.7),
            borderRadius: BorderRadius.circular(20),
            border: Border.all(color: Colors.white.withValues(alpha: 0.2)),
            boxShadow: const [
              BoxShadow(
                color: Colors.black26,
                blurRadius: 10,
                spreadRadius: 2,
              )
            ],
          ),
          child: Column(
            children: [
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                decoration: BoxDecoration(
                  border: Border(bottom: BorderSide(color: Colors.white.withValues(alpha: 0.1))),
                ),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    const Row(
                      children: [
                        Icon(Icons.smart_toy, color: Colors.blueAccent),
                        SizedBox(width: 8),
                        Text(
                          'Help Agent',
                          style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold, color: Colors.white),
                        ),
                      ],
                    ),
                    IconButton(
                      icon: const Icon(Icons.close, color: Colors.white70),
                      onPressed: _toggleChat,
                    ),
                  ],
                ),
              ),
              Expanded(
                child: ListView.builder(
                  padding: const EdgeInsets.all(16),
                  itemCount: _messages.length,
                  itemBuilder: (context, index) {
                    final isUser = index > 0 && index % 2 != 0;
                    return Align(
                      alignment: isUser ? Alignment.centerRight : Alignment.centerLeft,
                      child: Container(
                        margin: const EdgeInsets.only(bottom: 12),
                        padding: const EdgeInsets.all(12),
                        decoration: BoxDecoration(
                          color: isUser ? Colors.blueAccent.withValues(alpha: 0.8) : Colors.black45,
                          borderRadius: BorderRadius.circular(12),
                        ),
                        child: Text(
                          _messages[index],
                          style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
                        ),
                      ),
                    );
                  },
                ),
              ),
              Padding(
                padding: const EdgeInsets.all(16.0),
                child: Row(
                  children: [
                    Expanded(
                      child: TextField(
                        controller: _controller,
                        style: const TextStyle(color: Colors.white, fontFamily: 'Inter'),
                        decoration: InputDecoration(
                          hintText: 'Ask anything...',
                          hintStyle: const TextStyle(color: Colors.white54),
                          filled: true,
                          fillColor: Colors.black45,
                          border: OutlineInputBorder(
                            borderRadius: BorderRadius.circular(20),
                            borderSide: BorderSide.none,
                          ),
                          contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                        ),
                        onSubmitted: (_) => _sendMessage(),
                      ),
                    ),
                    const SizedBox(width: 8),
                    Container(
                      decoration: const BoxDecoration(
                        color: Colors.blueAccent,
                        shape: BoxShape.circle,
                      ),
                      child: IconButton(
                        icon: const Icon(Icons.send, color: Colors.white, size: 20),
                        onPressed: _sendMessage,
                      ),
                    ),
                  ],
                ),
              )
            ],
          ),
        ),
      ),
    );
  }
}
