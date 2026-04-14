import 'package:flutter/material.dart';
import '../../srcs/app/lib/widgets/glass_card.dart';

class SetupUI extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return GlassCard(
      padding: const EdgeInsets.all(20),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        children: [
          const Text(
            'OHC Hybrid OS Setup',
            style: TextStyle(
              fontFamily: 'Outfit',
              color: Colors.white,
              fontSize: 24,
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 20),
          _buildChecklistItem('1. Setup PostgreSQL', true),
          _buildChecklistItem('2. Configure Redis', true),
          _buildChecklistItem('3. Hire Initial Agent', false),
          _buildChecklistItem('4. Launch Standalone Mode', false),
        ],
      ),
    );
  }

  Widget _buildChecklistItem(String text, bool isDone) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0),
      child: Row(
        children: [
          Icon(
            isDone ? Icons.check_circle : Icons.radio_button_unchecked,
            color: isDone ? Colors.greenAccent : Colors.white54,
            size: 20,
          ),
          const SizedBox(width: 12),
          Text(
            text,
            style: TextStyle(
              fontFamily: 'Inter',
              color: isDone ? Colors.white : Colors.white70,
              fontSize: 16,
              decoration: isDone ? TextDecoration.lineThrough : null,
            ),
          ),
        ],
      ),
    );
  }
}
