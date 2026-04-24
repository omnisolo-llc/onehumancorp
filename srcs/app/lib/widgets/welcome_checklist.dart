import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class WelcomeChecklist extends StatelessWidget {
  const WelcomeChecklist({super.key});

  @override
  Widget build(BuildContext context) {
    return GlassCard(
      child: Padding(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'You\'re set up! Here\'s what to do next',
              style: TextStyle(
                fontSize: 20,
                fontWeight: FontWeight.bold,
                fontFamily: 'Outfit',
              ),
            ),
            const SizedBox(height: 16),
            _ChecklistItem(
              isCompleted: true,
              label: 'Business live',
            ),
            _ChecklistItem(
              isCompleted: false,
              label: 'Add 3 more products',
            ),
            _ChecklistItem(
              isCompleted: false,
              label: 'Connect Instagram',
            ),
            _ChecklistItem(
              isCompleted: false,
              label: 'Share your link with a friend',
            ),
          ],
        ),
      ),
    );
  }
}

class _ChecklistItem extends StatelessWidget {
  final bool isCompleted;
  final String label;

  const _ChecklistItem({
    required this.isCompleted,
    required this.label,
  });

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0),
      child: Row(
        children: [
          Icon(
            isCompleted ? Icons.check_box : Icons.check_box_outline_blank,
            color: isCompleted ? Colors.green : colors.onSurfaceVariant,
          ),
          const SizedBox(width: 12),
          Text(
            label,
            style: TextStyle(
              fontSize: 16,
              fontFamily: 'Inter',
              color: colors.onSurface,
              decoration: isCompleted ? TextDecoration.lineThrough : null,
            ),
          ),
        ],
      ),
    );
  }
}