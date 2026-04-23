import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class AiHelpChat extends StatefulWidget {
  const AiHelpChat({super.key});

  @override
  State<AiHelpChat> createState() => _AiHelpChatState();
}

class _AiHelpChatState extends State<AiHelpChat> {
  bool _isOpen = false;

  void _toggleChat() {
    setState(() {
      _isOpen = !_isOpen;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Positioned(
      bottom: 24,
      right: 24,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.end,
        children: [
          if (_isOpen)
            Container(
              width: 320,
              height: 400,
              margin: const EdgeInsets.only(bottom: 16),
              child: GlassCard(
                child: Column(
                  children: [
                    Container(
                      padding: const EdgeInsets.all(16),
                      decoration: BoxDecoration(
                        color: Theme.of(context).colorScheme.primaryContainer,
                        borderRadius: const BorderRadius.vertical(top: Radius.circular(12)),
                      ),
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Expanded(
                            child: Text(
                              'AI Support Agent',
                              style: TextStyle(
                                fontWeight: FontWeight.bold,
                                color: Theme.of(context).colorScheme.onPrimaryContainer,
                              ),
                            ),
                          ),
                          IconButton(
                            icon: const Icon(Icons.close, size: 20),
                            onPressed: _toggleChat,
                            padding: EdgeInsets.zero,
                            constraints: const BoxConstraints(),
                          ),
                        ],
                      ),
                    ),
                    Expanded(
                      child: ListView(
                        padding: const EdgeInsets.all(16),
                        children: [
                          _buildChatMessage('Hi! I\'m your AI Support Agent. What do you need help with today?', true),
                        ],
                      ),
                    ),
                    Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: TextField(
                        decoration: InputDecoration(
                          hintText: 'Ask anything...',
                          border: OutlineInputBorder(
                            borderRadius: BorderRadius.circular(20),
                          ),
                          contentPadding: const EdgeInsets.symmetric(horizontal: 16),
                          suffixIcon: const Icon(Icons.send),
                        ),
                      ),
                    ),
                  ],
                ),
              ),
            ),
          FloatingActionButton(
            onPressed: _toggleChat,
            tooltip: 'Ask AI Support Agent',
            child: Icon(_isOpen ? Icons.close : Icons.auto_awesome),
          ),
        ],
      ),
    );
  }

  Widget _buildChatMessage(String text, bool isAi) {
    return Align(
      alignment: isAi ? Alignment.centerLeft : Alignment.centerRight,
      child: Container(
        margin: const EdgeInsets.only(bottom: 8),
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
        decoration: BoxDecoration(
          color: isAi ? Colors.grey.withValues(alpha: 0.2) : Colors.blue.withValues(alpha: 0.2),
          borderRadius: BorderRadius.circular(16).copyWith(
            bottomLeft: isAi ? const Radius.circular(0) : const Radius.circular(16),
            bottomRight: !isAi ? const Radius.circular(0) : const Radius.circular(16),
          ),
        ),
        child: Text(text),
      ),
    );
  }
}
