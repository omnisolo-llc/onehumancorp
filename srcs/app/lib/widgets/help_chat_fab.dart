import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class HelpChatFab extends StatefulWidget {
  const HelpChatFab({super.key});

  @override
  State<HelpChatFab> createState() => _HelpChatFabState();
}

class _HelpChatFabState extends State<HelpChatFab> {
  bool _isOpen = false;
  final TextEditingController _controller = TextEditingController();
  final List<String> _messages = [
    "Hi there! I'm your OHC Help Assistant. How can I help you today?"
  ];

  void _sendMessage() {
    if (_controller.text.isEmpty) return;
    setState(() {
      _messages.add(_controller.text);
      // Simulate simple plain-language AI response
      _messages.add(
        "I'm an AI assistant in training. Please check the Help Center using the '?' icon in the menu for more details on that topic!"
      );
      _controller.clear();
    });
  }

  @override
  Widget build(BuildContext context) {
    if (!_isOpen) {
      return FloatingActionButton.extended(
        onPressed: () => setState(() => _isOpen = true),
        icon: const Icon(Icons.help_outline),
        label: const Text('Ask anything'),
        backgroundColor: Theme.of(context).colorScheme.primaryContainer,
        foregroundColor: Theme.of(context).colorScheme.onPrimaryContainer,
      );
    }

    // Chat interface
    return GlassCard(
      padding: EdgeInsets.zero,
      child: SizedBox(
        width: 320,
        height: 450,
        child: Column(
          children: [
            // Header
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
              decoration: BoxDecoration(
                color: Theme.of(context).colorScheme.surfaceContainerHighest.withOpacity(0.5),
                borderRadius: const BorderRadius.vertical(top: Radius.circular(16)),
              ),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  const Row(
                    children: [
                      Icon(Icons.support_agent, size: 20),
                      SizedBox(width: 8),
                      Text('Help Assistant', style: TextStyle(fontWeight: FontWeight.bold)),
                    ],
                  ),
                  IconButton(
                    icon: const Icon(Icons.close, size: 20),
                    onPressed: () => setState(() => _isOpen = false),
                    padding: EdgeInsets.zero,
                    constraints: const BoxConstraints(),
                  ),
                ],
              ),
            ),

            // Messages
            Expanded(
              child: ListView.builder(
                padding: const EdgeInsets.all(16),
                itemCount: _messages.length,
                itemBuilder: (context, index) {
                  final isUser = index % 2 != 0;
                  return Align(
                    alignment: isUser ? Alignment.centerRight : Alignment.centerLeft,
                    child: Container(
                      margin: const EdgeInsets.only(bottom: 12),
                      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                      decoration: BoxDecoration(
                        color: isUser
                            ? Theme.of(context).colorScheme.primary.withOpacity(0.8)
                            : Theme.of(context).colorScheme.surfaceContainerHighest,
                        borderRadius: BorderRadius.circular(12),
                      ),
                      child: Text(
                        _messages[index],
                        style: TextStyle(
                          color: isUser
                              ? Theme.of(context).colorScheme.onPrimary
                              : Theme.of(context).colorScheme.onSurface,
                        ),
                      ),
                    ),
                  );
                },
              ),
            ),

            // Input
            Padding(
              padding: const EdgeInsets.all(12),
              child: Row(
                children: [
                  Expanded(
                    child: TextField(
                      controller: _controller,
                      decoration: InputDecoration(
                        hintText: 'Ask a question...',
                        border: OutlineInputBorder(
                          borderRadius: BorderRadius.circular(20),
                          borderSide: BorderSide.none,
                        ),
                        filled: true,
                        fillColor: Theme.of(context).colorScheme.surfaceContainerHighest.withOpacity(0.3),
                        contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
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
            ),
          ],
        ),
      ),
    );
  }
}
