import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class KairosDashboardScreen extends ConsumerWidget {
  const KairosDashboardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('KAIROS Swarm Analytics', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: Padding(
        padding: const EdgeInsets.all(24.0),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Expanded(
              child: Column(
                children: [
                  const Text('Shared Task Queue', style: TextStyle(fontFamily: 'Outfit', fontSize: 20)),
                  const SizedBox(height: 16),
                  Expanded(
                    child: GlassCard(
                      child: ListView(
                        children: const [
                          ListTile(title: Text('Task 1', style: TextStyle(fontFamily: 'Inter'))),
                        ],
                      ),
                    ),
                  ),
                ],
              ),
            ),
            const SizedBox(width: 24),
            Expanded(
              child: Column(
                children: [
                  const Text('Teammate Mesh Stream', style: TextStyle(fontFamily: 'Outfit', fontSize: 20)),
                  const SizedBox(height: 16),
                  Expanded(
                    child: GlassCard(
                      child: ListView(
                        children: const [
                          ListTile(title: Text('Agent Ping', style: TextStyle(fontFamily: 'Inter'))),
                        ],
                      ),
                    ),
                  ),
                ],
              ),
            ),
            const SizedBox(width: 24),
            Expanded(
              child: Column(
                children: [
                  const Text('AutoDream Memory', style: TextStyle(fontFamily: 'Outfit', fontSize: 20)),
                  const SizedBox(height: 16),
                  Expanded(
                    child: GlassCard(
                      child: ListView(
                        children: const [
                          ListTile(title: Text('Vector Embed', style: TextStyle(fontFamily: 'Inter'))),
                        ],
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
