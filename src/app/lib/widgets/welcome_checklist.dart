import 'package:flutter/material.dart';
import 'glass_card.dart';

class WelcomeChecklistWidget extends StatelessWidget {
  const WelcomeChecklistWidget({super.key});

  @override
  Widget build(BuildContext context) {
    return GlassCard(
      child: Padding(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          children: [
            const Text(
              'Welcome! Your Checklist',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 20,
                fontWeight: FontWeight.bold,
                color: Colors.white,
              ),
            ),
            const SizedBox(height: 8),
            const Text(
              "You're set up! Here's what to do next to grow your business.",
              style: TextStyle(
                fontFamily: 'Inter',
                fontSize: 14,
                color: Colors.white70,
              ),
            ),
            const SizedBox(height: 24),
            _buildItem(context, 'Business live & ready', true),
            _buildItem(context, 'Add 3 more products', false),
            _buildItem(context, 'Connect Instagram / Social', false),
            _buildItem(context, 'Share your link with a friend', false),
          ],
        ),
      ),
    );
  }

  Widget _buildItem(BuildContext context, String title, bool completed) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 16.0),
      child: Row(
        children: [
          Icon(
            completed ? Icons.check_circle : Icons.radio_button_unchecked,
            color: completed ? Colors.greenAccent : Colors.white38,
            size: 20,
          ),
          const SizedBox(width: 12),
          Text(
            title,
            style: TextStyle(
              fontFamily: 'Inter',
              fontSize: 15,
              color: completed ? Colors.white70 : Colors.white,
              decoration: completed ? TextDecoration.lineThrough : null,
            ),
          ),
        ],
      ),
    );
  }
}
