import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/settings_service.dart';

class SetupUI extends ConsumerWidget {
  const SetupUI({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final clientSettings = ref.watch(clientSettingsProvider).valueOrNull;
    final isStandalone = clientSettings?.standaloneMode ?? false;

    return Container(
      decoration: BoxDecoration(
        color: Colors.white.withOpacity(0.05),
        border: Border.all(color: Colors.white.withOpacity(0.1)),
        borderRadius: BorderRadius.circular(12),
      ),
      padding: EdgeInsets.all(20),
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
          if (isStandalone) ...[
            _buildChecklistItem('1. Initialize Local SQLite', true),
            _buildChecklistItem('2. Bypassed Redis & Postgres', true),
            _buildChecklistItem('3. Hire Initial Agent', false),
            _buildChecklistItem('4. Launch Standalone Mode', false),
          ] else ...[
            _buildChecklistItem('1. Setup PostgreSQL', true),
            _buildChecklistItem('2. Configure Redis', true),
            _buildChecklistItem('3. Hire Initial Agent', false),
            _buildChecklistItem('4. Launch Standalone Mode', false),
          ],
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
