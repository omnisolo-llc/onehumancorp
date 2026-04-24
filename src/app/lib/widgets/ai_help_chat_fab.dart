import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

/// A floating "Ask anything" chat button accessible from every page.
/// Routes to a specialized Help Agent.
class AiHelpChatFAB extends StatelessWidget {
  const AiHelpChatFAB({super.key});

  @override
  Widget build(BuildContext context) {
    return Positioned(
      bottom: 24.0,
      right: 24.0,
      child: FloatingActionButton.extended(
        heroTag: 'ai_help_fab',
        onPressed: () {
          // Navigate to the chat screen, optionally passing a parameter to start
          // a conversation with the Help Agent specifically.
          context.push('/chat');
        },
        icon: const Icon(Icons.support_agent),
        label: const Text(
          'Ask anything',
          style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold),
        ),
        backgroundColor: Theme.of(context).primaryColor,
        foregroundColor: Colors.white,
        elevation: 8.0,
      ),
    );
  }
}
