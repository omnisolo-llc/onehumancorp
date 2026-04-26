import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class AiHelpChatWidget extends StatefulWidget {
  const AiHelpChatWidget({super.key});

  @override
  State<AiHelpChatWidget> createState() => _AiHelpChatWidgetState();
}

class _AiHelpChatWidgetState extends State<AiHelpChatWidget> {
  bool _isOpen = false;
  final TextEditingController _controller = TextEditingController();
  final List<String> _messages = [];

  void _sendMessage() {
    if (_controller.text.trim().isEmpty) return;
    setState(() {
      _messages.add('User: ${_controller.text}');
      _messages.add("AI: That's a great question! For more details on this topic, please visit our Help Center articles.");
      _controller.clear();
    });
  }

  @override
  Widget build(BuildContext context) {
    if (!_isOpen) {
      return Positioned(
        bottom: 24,
        right: 24,
        child: FloatingActionButton(
          key: const Key('ai_help_chat_button'),
          onPressed: () => setState(() => _isOpen = true),
          child: const Icon(Icons.help_outline),
        ),
      );
    }

    return Positioned(
      bottom: 24,
      right: 24,
      width: 350,
      height: 450,
      child: GlassCard(
        child: Column(
          children: [
            Container(
              padding: const EdgeInsets.all(16),
              decoration: const BoxDecoration(
                border: Border(bottom: BorderSide(color: Colors.black12)),
              ),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  const Text('Ask OHC Help', style: TextStyle(fontWeight: FontWeight.bold, fontSize: 16)),
                  IconButton(
                    icon: const Icon(Icons.close),
                    onPressed: () => setState(() => _isOpen = false),
                  ),
                ],
              ),
            ),
            Expanded(
              child: ListView.builder(
                padding: const EdgeInsets.all(16),
                itemCount: _messages.length,
                itemBuilder: (context, index) {
                  return Padding(
                    padding: const EdgeInsets.only(bottom: 8),
                    child: Text(_messages[index]),
                  );
                },
              ),
            ),
            Padding(
              padding: const EdgeInsets.all(16),
              child: Row(
                children: [
                  Expanded(
                    child: TextField(
                      controller: _controller,
                      decoration: const InputDecoration(
                        hintText: 'Type your question...',
                        border: OutlineInputBorder(),
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
    );
  }
}
