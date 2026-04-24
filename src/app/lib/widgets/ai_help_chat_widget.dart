import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../models/tier.dart';
import 'upgrade_bottom_sheet.dart';

class AiHelpChatWidget extends StatelessWidget {
  const AiHelpChatWidget({super.key});

  @override
  Widget build(BuildContext context) {
    return FloatingActionButton.extended(
      onPressed: () {
        showModalBottomSheet(
          context: context,
          isScrollControlled: true,
          backgroundColor: Colors.transparent,
          builder: (context) => const _ChatBottomSheet(),
        );
      },
      icon: const Icon(Icons.help_outline),
      label: const Text('Ask AI Support', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
    );
  }
}

class _ChatBottomSheet extends ConsumerWidget {
  const _ChatBottomSheet();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tierState = ref.watch(tierProvider);
    final limitExceeded = tierState.aiActionsUsed >= tierState.aiActionsLimit;

    return Container(
      height: MediaQuery.of(context).size.height * 0.8,
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surface,
        borderRadius: const BorderRadius.vertical(top: Radius.circular(24)),
      ),
      child: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(16.0),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                const Text(
                  'AI Support Agent',
                  style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold),
                ),
                IconButton(
                  icon: const Icon(Icons.close),
                  onPressed: () => Navigator.pop(context),
                ),
              ],
            ),
          ),
          const Divider(),
          Expanded(
            child: ListView(
              padding: const EdgeInsets.all(16),
              children: [
                _ChatBubble(
                  message: 'Hi there! I am your AI Support Agent. I can help you find answers in the Help Center or guide you through setting up your business. What can I help you with today?',
                  isMe: false,
                ),
              ],
            ),
          ),
          Padding(
            padding: const EdgeInsets.all(16.0),
            child: limitExceeded
                ? Column(
                    children: [
                      const Text(
                        'AI replies limit reached for the Free plan.',
                        style: TextStyle(color: Colors.redAccent, fontFamily: 'Inter', fontSize: 12),
                      ),
                      const SizedBox(height: 8),
                      Row(
                        children: [
                          Expanded(
                            child: TextField(
                              decoration: InputDecoration(
                                hintText: 'Manual reply mode...',
                                border: OutlineInputBorder(
                                  borderRadius: BorderRadius.circular(24),
                                ),
                                contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                              ),
                            ),
                          ),
                          const SizedBox(width: 8),
                          TextButton(
                            onPressed: () {
                              Navigator.pop(context); // Close chat
                              showModalBottomSheet(
                                context: context,
                                isScrollControlled: true,
                                backgroundColor: Colors.transparent,
                                builder: (context) => const UpgradeBottomSheet(limitReason: 'ai_actions'),
                              );
                            },
                            child: const Text('Upgrade to restore AI replies', style: TextStyle(fontSize: 12)),
                          )
                        ],
                      )
                    ],
                  )
                : Row(
                    children: [
                      Expanded(
                        child: TextField(
                          decoration: InputDecoration(
                            hintText: 'Type your question...',
                            border: OutlineInputBorder(
                              borderRadius: BorderRadius.circular(24),
                            ),
                            contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                          ),
                        ),
                      ),
                      const SizedBox(width: 8),
                      IconButton(
                        icon: const Icon(Icons.send),
                        color: Theme.of(context).colorScheme.primary,
                        onPressed: () {
                          ref.read(tierProvider.notifier).addAiAction();
                        },
                      ),
                    ],
                  ),
          ),
        ],
      ),
    );
  }
}

class _ChatBubble extends StatelessWidget {
  final String message;
  final bool isMe;

  const _ChatBubble({required this.message, required this.isMe});

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    return Align(
      alignment: isMe ? Alignment.centerRight : Alignment.centerLeft,
      child: Container(
        margin: const EdgeInsets.only(bottom: 16), constraints: const BoxConstraints(maxWidth: 300),
        padding: const EdgeInsets.all(16),
        decoration: BoxDecoration(
          color: isMe ? colorScheme.primary : colorScheme.surfaceContainerHighest,
          borderRadius: BorderRadius.circular(16).copyWith(
            bottomRight: isMe ? const Radius.circular(0) : null,
            bottomLeft: !isMe ? const Radius.circular(0) : null,
          ),
        ),
        child: Text(
          message,
          style: TextStyle(
            color: isMe ? colorScheme.onPrimary : colorScheme.onSurfaceVariant,
            fontFamily: 'Inter',
          ),
        ),
      ),
    );
  }
}
