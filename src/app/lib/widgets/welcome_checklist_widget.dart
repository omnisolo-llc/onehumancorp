import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class WelcomeChecklistWidget extends StatelessWidget {
  const WelcomeChecklistWidget({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(bottom: 24),
      child: GlassCard(
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                'Welcome Checklist',
                style: Theme.of(context).textTheme.titleLarge?.copyWith(
                      fontWeight: FontWeight.bold,
                      fontFamily: 'Outfit',
                    ),
              ),
              const SizedBox(height: 16),
              _ChecklistItem(
                isChecked: true,
                title: 'Business live',
                onTap: () {},
              ),
              const SizedBox(height: 8),
              _ChecklistItem(
                isChecked: false,
                title: 'Add 3 more products',
                onTap: () => context.go('/wizards/products'),
              ),
              const SizedBox(height: 8),
              _ChecklistItem(
                isChecked: false,
                title: 'Connect Instagram',
                onTap: () => context.go('/wizards/integrations'),
              ),
              const SizedBox(height: 8),
              _ChecklistItem(
                isChecked: false,
                title: 'Share your link with a friend',
                onTap: () => context.go('/wizards/share'),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _ChecklistItem extends StatelessWidget {
  final bool isChecked;
  final String title;
  final VoidCallback onTap;

  const _ChecklistItem({
    required this.isChecked,
    required this.title,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;

    return Semantics(
      button: true,
      label: 'Checklist item: $title',
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(8),
        child: Padding(
          padding: const EdgeInsets.symmetric(vertical: 8, horizontal: 8),
          child: Row(
            children: [
              Icon(
                isChecked ? Icons.check_circle : Icons.radio_button_unchecked,
                color: isChecked ? Colors.green : colorScheme.onSurfaceVariant,
                size: 24,
              ),
              const SizedBox(width: 12),
              Expanded(
                child: Text(
                  title,
                  style: TextStyle(
                    fontSize: 16,
                    fontFamily: 'Inter',
                    color: colorScheme.onSurface,
                    decoration:
                        isChecked ? TextDecoration.lineThrough : null,
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
