import 'package:flutter/material.dart';

class ViralStorefrontFooterWidget extends StatelessWidget {
  const ViralStorefrontFooterWidget({super.key});

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;

    return Container(
      padding: const EdgeInsets.symmetric(vertical: 24, horizontal: 16),
      width: double.infinity,
      decoration: BoxDecoration(
        color: colorScheme.surface,
        border: Border(
          top: BorderSide(
            color: colorScheme.outline.withValues(alpha: 0.1),
            width: 1,
          ),
        ),
      ),
      child: Center(
        child: InkWell(
          onTap: () async {
            // Simplified link without url_launcher
          },
          borderRadius: BorderRadius.circular(8),
          child: Padding(
            padding: const EdgeInsets.all(8.0),
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                Icon(
                  Icons.blur_on,
                  size: 20,
                  color: colorScheme.primary,
                ),
                const SizedBox(width: 8),
                Text(
                  'Built with OHC — Start your free business →',
                  style: TextStyle(
                    fontFamily: 'Inter',
                    fontSize: 14,
                    color: colorScheme.onSurfaceVariant,
                    fontWeight: FontWeight.w500,
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
