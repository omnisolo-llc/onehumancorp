import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class WebDashboardWidget extends StatelessWidget {
  final String title;
  final String value;
  final IconData icon;

  const WebDashboardWidget({
    Key? key,
    required this.title,
    required this.value,
    required this.icon,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return GlassCard(
      padding: const EdgeInsets.all(24),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(icon, size: 32, color: Colors.white),
          const SizedBox(height: 16),
          Text(
            value,
            style: const TextStyle(
              fontSize: 36,
              fontWeight: FontWeight.bold,
              color: Colors.white,
              fontFamily: 'Inter',
            ),
          ),
          const SizedBox(height: 8),
          Text(
            title,
            style: TextStyle(
              fontSize: 16,
              color: Colors.white.withOpacity(0.7),
              fontFamily: 'Outfit',
            ),
          ),
        ],
      ),
    );
  }
}
