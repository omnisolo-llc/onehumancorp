import 'package:flutter/material.dart';

class HelpChatOverlay extends StatelessWidget {
  const HelpChatOverlay({super.key});

  @override
  Widget build(BuildContext context) {
    return FloatingActionButton(
      onPressed: () {},
      tooltip: 'Ask anything',
      child: const Icon(Icons.help_outline),
    );
  }
}
