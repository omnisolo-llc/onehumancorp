import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

class InteractiveWalkthroughScreen extends StatelessWidget {
  final String flowId;

  const InteractiveWalkthroughScreen({super.key, required this.flowId});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Interactive Walkthrough', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(Icons.touch_app, size: 64, color: Colors.indigo),
            const SizedBox(height: 24),
            Text(
              'Interactive Walkthrough for: $flowId',
              style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
            ),
            const SizedBox(height: 16),
            const Text(
              'This is a placeholder for the interactive step-by-step tour. Imagine overlays and speech bubbles here!',
              textAlign: TextAlign.center,
              style: TextStyle(fontFamily: 'Inter'),
            ),
            const SizedBox(height: 32),
            FilledButton(
              onPressed: () => context.go('/help'),
              child: const Text('End Walkthrough'),
            ),
          ],
        ),
      ),
    );
  }
}
