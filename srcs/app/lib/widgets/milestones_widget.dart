import 'package:flutter/material.dart';

class SuccessMilestonesWidget extends StatelessWidget {
  const SuccessMilestonesWidget({super.key});

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    return Container(
      padding: const EdgeInsets.all(16),
      margin: const EdgeInsets.only(bottom: 16),
      decoration: BoxDecoration(
        color: colorScheme.tertiaryContainer,
        borderRadius: BorderRadius.circular(12),
      ),
      child: Row(
        children: [
          Icon(Icons.celebration, color: colorScheme.onTertiaryContainer, size: 28),
          const SizedBox(width: 16),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  '🎉 Success Milestone!',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontWeight: FontWeight.bold,
                    color: colorScheme.onTertiaryContainer,
                    fontSize: 16,
                  ),
                ),
                Text(
                  'You just got your 10th order! Keep it up.',
                  style: TextStyle(
                    fontFamily: 'Inter',
                    color: colorScheme.onTertiaryContainer,
                    fontSize: 14,
                  ),
                ),
              ],
            ),
          ),
          IconButton(
            icon: Icon(Icons.close, color: colorScheme.onTertiaryContainer),
            onPressed: () {
              // Dismiss
            },
          )
        ],
      ),
    );
  }
}
