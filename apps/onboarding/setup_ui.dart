import 'package:flutter/material.dart';

class SetupUI extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: Colors.white.withOpacity(0.05),
        border: Border.all(color: Colors.white.withOpacity(0.1)),
        borderRadius: BorderRadius.circular(12),
      ),
      padding: EdgeInsets.all(20),
      child: const Text(
        'OHC Hybrid OS Setup',
        style: TextStyle(
          fontFamily: 'Outfit', // Or 'Inter'
          color: Colors.white,
        ),
      ),
    );
  }
}
