import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/tooltip_registry.dart';

class HelpChatFab extends StatelessWidget {
  const HelpChatFab({super.key});

  @override
  Widget build(BuildContext context) {
    return OhcTooltip(
      registryKey: 'chat_fab',
      child: FloatingActionButton.extended(
        onPressed: () => context.go('/help-chat'),
        icon: const Icon(Icons.chat_bubble_outline),
        label: const Text('Ask AI', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        backgroundColor: Theme.of(context).colorScheme.primaryContainer,
      ),
    );
  }
}
