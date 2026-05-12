import 'package:flutter/material.dart';

class AiHelpChatScreen extends StatefulWidget {
  const AiHelpChatScreen({super.key});

  @override
  State<AiHelpChatScreen> createState() => _AiHelpChatScreenState();
}

class _AiHelpChatScreenState extends State<AiHelpChatScreen> {
  final List<Map<String, String>> _messages = [
    {
      'role': 'assistant',
      'text': 'Hi! I am your Support Agent. How can I help you with OneHumanCorp today?'
    }
  ];
  final TextEditingController _controller = TextEditingController();

  void _sendMessage() {
    if (_controller.text.trim().isEmpty) return;
    setState(() {
      _messages.add({'role': 'user', 'text': _controller.text});
      _messages.add({'role': 'assistant', 'text': 'This is a simulated AI response. Please visit the Help Center to read the full article.'});
    });
    _controller.clear();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      appBar: AppBar(
        title: const Text('Ask Anything', style: TextStyle(fontFamily: 'Outfit', color: Colors.white)),
        backgroundColor: Colors.transparent,
        elevation: 0,
        iconTheme: const IconThemeData(color: Colors.white),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 600),
          child: Column(
            children: [
              Expanded(
                child: ListView.builder(
                  padding: const EdgeInsets.all(20),
                  itemCount: _messages.length,
                  itemBuilder: (context, index) {
                    final msg = _messages[index];
                    final isUser = msg['role'] == 'user';
                    return Align(
                      alignment: isUser ? Alignment.centerRight : Alignment.centerLeft,
                      child: Container(
                        margin: const EdgeInsets.only(bottom: 15),
                        padding: const EdgeInsets.all(15),
                        decoration: BoxDecoration(
                          color: isUser ? const Color(0xFF6B4EFF) : Colors.white.withAlpha(20),
                          borderRadius: BorderRadius.circular(15),
                        ),
                        child: Text(
                          msg['text'] ?? '',
                          style: const TextStyle(color: Colors.white),
                        ),
                      ),
                    );
                  },
                ),
              ),
              Padding(
                padding: const EdgeInsets.all(20),
                child: Row(
                  children: [
                    Expanded(
                      child: TextField(
                        controller: _controller,
                        style: const TextStyle(color: Colors.white),
                        decoration: InputDecoration(
                          hintText: 'Type your question...',
                          hintStyle: const TextStyle(color: Colors.white70),
                          filled: true,
                          fillColor: Colors.white.withAlpha(20),
                          border: OutlineInputBorder(
                            borderRadius: BorderRadius.circular(20),
                            borderSide: BorderSide.none,
                          ),
                        ),
                        onSubmitted: (_) => _sendMessage(),
                      ),
                    ),
                    const SizedBox(width: 10),
                    IconButton(
                      icon: const Icon(Icons.send, color: Color(0xFF6B4EFF)),
                      onPressed: _sendMessage,
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
