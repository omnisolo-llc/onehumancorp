import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import '../widgets/glass_card.dart';

class WelcomeChecklistScreen extends StatelessWidget {
  const WelcomeChecklistScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Color(0xFF0D0D1A), Color(0xFF1A1A33)],
          ),
        ),
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 600),
            child: GlassCard(
              child: Padding(
                padding: const EdgeInsets.all(24.0),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    const Text('You\'re set up! Here\'s what to do next:', style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white, fontFamily: 'Outfit')),
                    const SizedBox(height: 16),
                    const ListTile(leading: Icon(Icons.check_circle, color: Colors.green), title: Text('Business live', style: TextStyle(color: Colors.white, fontFamily: 'Inter'))),
                    const ListTile(leading: Icon(Icons.circle_outlined, color: Colors.white), title: Text('Add 3 more products', style: TextStyle(color: Colors.white, fontFamily: 'Inter'))),
                    const ListTile(leading: Icon(Icons.circle_outlined, color: Colors.white), title: Text('Connect Instagram', style: TextStyle(color: Colors.white, fontFamily: 'Inter'))),
                    const ListTile(leading: Icon(Icons.circle_outlined, color: Colors.white), title: Text('Share your link with a friend', style: TextStyle(color: Colors.white, fontFamily: 'Inter'))),
                    const SizedBox(height: 24),
                    ElevatedButton(onPressed: () => context.go('/dashboard'), child: const Text('Go to Dashboard', style: TextStyle(fontFamily: 'Inter'))),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}