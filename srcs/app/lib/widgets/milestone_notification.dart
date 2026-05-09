import 'package:flutter/material.dart';
import '../main.dart'; // For GlassContainer

class MilestoneNotification extends StatelessWidget {
  final String title;
  final String message;
  final IconData icon;

  const MilestoneNotification({
    super.key,
    required this.title,
    required this.message,
    this.icon = Icons.emoji_events,
  });

  @override
  Widget build(BuildContext context) {
    return GlassContainer(
      child: Row(
        children: [
          Container(
            padding: const EdgeInsets.all(12),
            decoration: BoxDecoration(
              color: const Color(0xFF6B4EFF).withAlpha(50),
              shape: BoxShape.circle,
            ),
            child: Icon(icon, color: const Color(0xFF6B4EFF), size: 28),
          ),
          const SizedBox(width: 15),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  title,
                  style: const TextStyle(
                    fontSize: 16,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                ),
                const SizedBox(height: 5),
                Text(
                  message,
                  style: const TextStyle(fontSize: 14, color: Colors.white70),
                ),
              ],
            ),
          ),
          IconButton(
            icon: const Icon(Icons.close, color: Colors.white54),
            onPressed: () {
              // Dismiss logic would go here
            },
          ),
        ],
      ),
    );
  }
}
