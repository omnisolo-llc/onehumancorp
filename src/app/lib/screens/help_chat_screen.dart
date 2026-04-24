import 'package:flutter/material.dart';

class HelpChatScreen extends StatefulWidget {
  const HelpChatScreen({super.key});
  @override
  State<HelpChatScreen> createState() => _HelpChatScreenState();
}

class _HelpChatScreenState extends State<HelpChatScreen> {
  final TextEditingController _ctrl = TextEditingController();
  final List<Map<String, String>> _messages = [
    {'role': 'agent', 'text': 'Hi! I am your OHC Support Agent. How can I help you today?'},
  ];
  bool _isTyping = false;

  void _send() {
    final text = _ctrl.text.trim();
    if (text.isEmpty) return;

    setState(() {
      _messages.add({'role': 'user', 'text': text});
      _isTyping = true;
    });
    _ctrl.clear();

    Future.delayed(const Duration(seconds: 2), () {
      if (mounted) {
        setState(() {
          _isTyping = false;
          _messages.add({
            'role': 'agent',
            'text': 'I understand you are asking about that. You can read more in our Help Center -> My Store section. Let me know if you need specific guidance!'
          });
        });
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    return Scaffold(
      appBar: AppBar(
        title: const Text('AI Help Agent', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: Column(
        children: [
          Expanded(
            child: ListView.builder(
              padding: const EdgeInsets.all(16),
              itemCount: _messages.length,
              itemBuilder: (context, i) {
                final isMe = _messages[i]['role'] == 'user';
                return Align(
                  alignment: isMe ? Alignment.centerRight : Alignment.centerLeft,
                  child: Container(
                    margin: const EdgeInsets.only(bottom: 12),
                    padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                    constraints: const BoxConstraints(maxWidth: 300),
                    decoration: BoxDecoration(
                      color: isMe ? colors.primary : colors.surfaceContainerHighest,
                      borderRadius: BorderRadius.circular(16),
                    ),
                    child: Text(
                      _messages[i]['text']!,
                      style: TextStyle(color: isMe ? colors.onPrimary : colors.onSurface, fontFamily: 'Inter'),
                    ),
                  ),
                );
              },
            ),
          ),
          if (_isTyping)
            const Padding(
              padding: EdgeInsets.symmetric(vertical: 8.0, horizontal: 16),
              child: Align(
                alignment: Alignment.centerLeft,
                child: Text('Agent is typing...', style: TextStyle(fontStyle: FontStyle.italic, color: Colors.grey)),
              ),
            ),
          SafeArea(
            child: Container(
              padding: const EdgeInsets.all(8),
              child: Row(
                children: [
                  Expanded(
                    child: TextField(
                      controller: _ctrl,
                      onSubmitted: (_) => _send(),
                      decoration: InputDecoration(
                        hintText: 'Ask anything...',
                        border: OutlineInputBorder(borderRadius: BorderRadius.circular(24)),
                        contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                      ),
                    ),
                  ),
                  const SizedBox(width: 8),
                  IconButton(
                    icon: const Icon(Icons.send),
                    color: colors.primary,
                    onPressed: _send,
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
