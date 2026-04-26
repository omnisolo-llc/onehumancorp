import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class ChangelogScreen extends StatelessWidget {
  const ChangelogScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.transparent,
      appBar: AppBar(title: const Text("What's New", style: TextStyle(fontFamily: 'Outfit')), backgroundColor: Colors.transparent),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          GlassCard(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: const [
                Text('Recent Updates', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
                SizedBox(height: 8),
                Text('We added a new help center to make running your business easier.', style: TextStyle(fontFamily: 'Inter')),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
