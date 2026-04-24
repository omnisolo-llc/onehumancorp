import 'package:flutter/material.dart';

class HelpChatButton extends StatelessWidget {
  const HelpChatButton({super.key});

  @override
  Widget build(BuildContext context) {
    return FloatingActionButton.extended(
      onPressed: () {
        showDialog(
          context: context,
          builder: (context) => AlertDialog(
            title: const Text('AI Help Chat', style: TextStyle(fontFamily: 'Outfit')),
            content: const Text('Hello! I am your AI Help Agent. How can I assist you with OneHumanCorp today?'),
            actions: [
              TextButton(
                onPressed: () => Navigator.of(context).pop(),
                child: const Text('Close'),
              ),
              TextButton(
                onPressed: () {},
                child: const Text('Read the full article →'),
              ),
            ],
          ),
        );
      },
      icon: const Icon(Icons.help_outline),
      label: const Text('Ask anything'),
    );
  }
}
