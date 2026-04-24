import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class BusinessShareWidget extends ConsumerWidget {
  const BusinessShareWidget({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final colors = Theme.of(context).colorScheme;

    return Semantics(
      label: 'Share my business card',
      child: GlassCard(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(Icons.share, color: colors.primary),
                const SizedBox(width: 8),
                Text(
                  'Share My Business',
                  style: Theme.of(context).textTheme.titleLarge?.copyWith(
                    fontFamily: 'Outfit',
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 16),
            Text(
              'Get a beautifully designed shareable link card for your business. Share it directly to your favorite social platforms.',
              style: TextStyle(
                fontFamily: 'Inter',
                color: colors.onSurfaceVariant,
              ),
            ),
            const SizedBox(height: 24),
            Row(
              children: [
                ElevatedButton.icon(
                  onPressed: () {
                    Clipboard.setData(const ClipboardData(text: 'https://ohc.app/my-business'));
                    ScaffoldMessenger.of(context).showSnackBar(
                      SnackBar(
                        content: const Text(
                          'Business link copied to clipboard!',
                          style: TextStyle(fontFamily: 'Inter'),
                        ),
                        backgroundColor: colors.primary,
                        behavior: SnackBarBehavior.floating,
                      ),
                    );
                  },
                  icon: const Icon(Icons.copy),
                  label: const Text('Copy Link', style: TextStyle(fontFamily: 'Outfit')),
                  style: ElevatedButton.styleFrom(
                    backgroundColor: colors.primaryContainer,
                    foregroundColor: colors.onPrimaryContainer,
                  ),
                ),
                const SizedBox(width: 16),
                OutlinedButton.icon(
                  onPressed: () {},
                  icon: const Icon(Icons.post_add),
                  label: const Text('Post to Social', style: TextStyle(fontFamily: 'Outfit')),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
