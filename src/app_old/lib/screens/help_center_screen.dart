import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class HelpCenterScreen extends StatelessWidget {
  const HelpCenterScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.transparent,
      appBar: AppBar(title: const Text('Help Center', style: TextStyle(fontFamily: 'Outfit')), backgroundColor: Colors.transparent),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          const TextField(decoration: InputDecoration(hintText: 'Search for help...', prefixIcon: Icon(Icons.search))),
          const SizedBox(height: 16),
          GlassCard(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: const [
                Text('Getting Started', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
                SizedBox(height: 8),
                Text('Learn how to set up your store and accept your first payment.', style: TextStyle(fontFamily: 'Inter')),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
