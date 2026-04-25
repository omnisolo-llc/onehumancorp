import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:go_router/go_router.dart';

final checklistStateProvider = StateProvider<Map<String, bool>>((ref) {
  return {
    'business_live': true,
    'add_products': false,
    'connect_instagram': false,
    'share_link': false,
  };
});

class WelcomeChecklistWidget extends ConsumerWidget {
  const WelcomeChecklistWidget({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(checklistStateProvider);

    return GlassCard(
      padding: const EdgeInsets.all(24.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            'Welcome Checklist',
            style: TextStyle(
              fontSize: 20,
              fontWeight: FontWeight.bold,
              fontFamily: 'Outfit',
            ),
          ),
          const SizedBox(height: 16),
          _ChecklistItem(
            title: 'Business live',
            isChecked: state['business_live'] ?? true,
            onChanged: null,
          ),
          _ChecklistItem(
            title: 'Add 3 more products',
            isChecked: state['add_products'] ?? false,
            onChanged: (val) {
              ref.read(checklistStateProvider.notifier).update((state) => {...state, 'add_products': val ?? false});
            },
          ),
          _ChecklistItem(
            title: 'Connect Instagram',
            isChecked: state['connect_instagram'] ?? false,
            onChanged: (val) {
              ref.read(checklistStateProvider.notifier).update((state) => {...state, 'connect_instagram': val ?? false});
            },
          ),
          _ChecklistItem(
            title: 'Share your link with a friend',
            isChecked: state['share_link'] ?? false,
            onChanged: (val) {
              ref.read(checklistStateProvider.notifier).update((state) => {...state, 'share_link': val ?? false});
              if (val == true) {
                Clipboard.setData(const ClipboardData(text: 'https://onehumancorp.com/my-business'));
                ScaffoldMessenger.of(context).showSnackBar(
                  const SnackBar(content: Text('Link copied to clipboard!')),
                );
              }
            },
          ),
        ],
      ),
    );
  }
}

class _ChecklistItem extends StatelessWidget {
  final String title;
  final bool isChecked;
  final ValueChanged<bool?>? onChanged;

  const _ChecklistItem({
    required this.title,
    required this.isChecked,
    this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Checkbox(
          value: isChecked,
          onChanged: onChanged,
        ),
        Expanded(
          child: Text(
            title,
            style: TextStyle(
              fontSize: 16,
              fontFamily: 'Inter',
              decoration: isChecked ? TextDecoration.lineThrough : null,
              color: isChecked ? Colors.grey : null,
            ),
          ),
        ),
      ],
    );
  }
}
