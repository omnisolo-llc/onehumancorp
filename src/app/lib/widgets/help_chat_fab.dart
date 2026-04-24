import 'package:flutter/material.dart';

class HelpChatFAB extends StatelessWidget {
  const HelpChatFAB({super.key});

  @override
  Widget build(BuildContext context) {
    return Positioned(
      bottom: 24,
      right: 24,
      child: FloatingActionButton.extended(
        onPressed: () {
          showModalBottomSheet(
            context: context,
            backgroundColor: const Color(0xFF1A1A33),
            shape: const RoundedRectangleBorder(
              borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
            ),
            builder: (context) => const _HelpChatSheet(),
          );
        },
        backgroundColor: Colors.blueAccent,
        icon: const Icon(Icons.support_agent, color: Colors.white),
        label: const Text('Ask anything', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.white)),
      ),
    );
  }
}

class _HelpChatSheet extends StatelessWidget {
  const _HelpChatSheet();

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: EdgeInsets.only(
        bottom: MediaQuery.of(context).viewInsets.bottom,
      ),
      child: Container(
        height: 400,
        padding: const EdgeInsets.all(24.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            const Text(
              'Help Agent',
              style: TextStyle(fontFamily: 'Outfit', fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white),
            ),
            const SizedBox(height: 16),
            Expanded(
              child: ListView(
                children: [
                  _ChatBubble(
                    text: 'Hi! I am your OHC Help Agent. What do you need help with today?',
                    isBot: true,
                  ),
                ],
              ),
            ),
            const SizedBox(height: 16),
            TextField(
              decoration: InputDecoration(
                hintText: 'Type your question...',
                hintStyle: const TextStyle(color: Colors.white54),
                filled: true,
                fillColor: Colors.white.withAlpha(13),
                border: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(24),
                  borderSide: BorderSide.none,
                ),
                suffixIcon: IconButton(
                  icon: const Icon(Icons.send, color: Colors.blueAccent),
                  onPressed: () {},
                ),
              ),
              style: const TextStyle(color: Colors.white),
            ),
          ],
        ),
      ),
    );
  }
}

class _ChatBubble extends StatelessWidget {
  final String text;
  final bool isBot;

  const _ChatBubble({required this.text, required this.isBot});

  @override
  Widget build(BuildContext context) {
    return Align(
      alignment: isBot ? Alignment.centerLeft : Alignment.centerRight,
      child: Container(
        margin: const EdgeInsets.only(bottom: 12),
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
        decoration: BoxDecoration(
          color: isBot ? Colors.white.withAlpha(26) : Colors.blueAccent,
          borderRadius: BorderRadius.circular(16).copyWith(
            bottomLeft: isBot ? const Radius.circular(0) : const Radius.circular(16),
            bottomRight: !isBot ? const Radius.circular(0) : const Radius.circular(16),
          ),
        ),
        child: Text(
          text,
          style: const TextStyle(fontFamily: 'Inter', color: Colors.white),
        ),
      ),
    );
  }
}
