import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class WalkthroughOverlay extends StatelessWidget {
  final String message;
  const WalkthroughOverlay({super.key, required this.message});

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        Container(color: Colors.black54),
        Center(
          child: GlassCard(
            child: Text(message, style: const TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 16)),
          ),
        ),
      ],
    );
  }
}
