import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/tooltip_registry.dart';

class AiHelpChatButton extends StatelessWidget {
  const AiHelpChatButton({super.key});

  void _showHelpChatOverlay(BuildContext context) {
    showDialog(
      context: context,
      barrierColor: Colors.black54,
      builder: (context) {
        return Dialog(
          backgroundColor: Colors.transparent,
          insetPadding: const EdgeInsets.all(16),
          child: Container(
            width: 400,
            height: 600,
            decoration: BoxDecoration(
              color: const Color.fromRGBO(30, 30, 35, 0.95),
              borderRadius: BorderRadius.circular(24),
              border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
            ),
            child: Column(
              children: [
                Container(
                  padding: const EdgeInsets.all(20),
                  decoration: BoxDecoration(
                    border: Border(bottom: BorderSide(color: Colors.white.withValues(alpha: 0.1))),
                  ),
                  child: Row(
                    children: [
                      const Icon(Icons.smart_toy, color: Colors.blueAccent),
                      const SizedBox(width: 12),
                      const Expanded(
                        child: Text(
                          'AI Support Agent',
                          style: TextStyle(
                            fontFamily: 'Outfit',
                            fontSize: 18,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ),
                      IconButton(
                        icon: const Icon(Icons.close),
                        onPressed: () => Navigator.of(context).pop(),
                      ),
                    ],
                  ),
                ),
                Expanded(
                  child: ListView(
                    padding: const EdgeInsets.all(20),
                    children: [
                      _buildChatBubble('Hello! I am your AI Support Agent. What do you need help with today?', true),
                      const SizedBox(height: 16),
                      _buildChatBubble('How do I add a new product?', false),
                      const SizedBox(height: 16),
                      _buildChatBubble(
                        'To add a new product, go to My Store > Inventory and click the "Add Product" button. You can add photos, prices, and variants there.',
                        true,
                        linkText: 'Read the full article →',
                      ),
                    ],
                  ),
                ),
                Padding(
                  padding: const EdgeInsets.all(20),
                  child: TextField(
                    decoration: InputDecoration(
                      hintText: 'Ask anything...',
                      suffixIcon: const Icon(Icons.send),
                      border: OutlineInputBorder(
                        borderRadius: BorderRadius.circular(24),
                      ),
                      filled: true,
                      fillColor: Colors.white.withValues(alpha: 0.05),
                    ),
                  ),
                ),
              ],
            ),
          ),
        );
      },
    );
  }

  Widget _buildChatBubble(String text, bool isAgent, {String? linkText}) {
    return Align(
      alignment: isAgent ? Alignment.centerLeft : Alignment.centerRight,
      child: Container(
        margin: EdgeInsets.only(
          left: isAgent ? 0 : 40,
          right: isAgent ? 40 : 0,
        ),
        padding: const EdgeInsets.all(16),
        decoration: BoxDecoration(
          color: isAgent ? Colors.white.withValues(alpha: 0.1) : Colors.blueAccent.withValues(alpha: 0.8),
          borderRadius: BorderRadius.circular(16).copyWith(
            bottomLeft: isAgent ? Radius.zero : const Radius.circular(16),
            bottomRight: isAgent ? const Radius.circular(16) : Radius.zero,
          ),
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              text,
              style: const TextStyle(fontFamily: 'Inter', fontSize: 15),
            ),
            if (linkText != null) ...[
              const SizedBox(height: 8),
              InkWell(
                onTap: () {},
                child: Text(
                  linkText,
                  style: const TextStyle(
                    fontFamily: 'Inter',
                    fontSize: 14,
                    color: Colors.lightBlueAccent,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return ContextualTooltip(
      tooltipKey: 'help_center_chat',
      child: FloatingActionButton.extended(
        onPressed: () => _showHelpChatOverlay(context),
        icon: const Icon(Icons.help_outline),
        label: const Text('Ask anything'),
        backgroundColor: Theme.of(context).colorScheme.primary,
        foregroundColor: Theme.of(context).colorScheme.onPrimary,
      ),
    );
  }
}
