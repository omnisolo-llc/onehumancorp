import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class ReleaseNotesScreen extends StatelessWidget {
  const ReleaseNotesScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('What\'s New', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => context.pop(),
        ),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 800),
          child: ListView(
            padding: const EdgeInsets.all(24.0),
            children: [
              const Text(
                'Recent Updates',
                style: TextStyle(fontFamily: 'Outfit', fontSize: 32, fontWeight: FontWeight.bold),
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 32),
              _buildReleaseNote(
                context,
                version: 'Version 1.1',
                date: 'Just now',
                title: 'New Help Center & AI Chat',
                description: 'We have completely redesigned our Help Center. You can now search for articles, watch video tutorials, and ask our new AI Support Agent for help directly from your dashboard.',
                features: [
                  'Searchable knowledge base',
                  'Contextual tooltips throughout the app',
                  'Interactive guided walkthroughs',
                  'Floating AI Help Chat agent'
                ],
              ),
              const SizedBox(height: 24),
              _buildReleaseNote(
                context,
                version: 'Version 1.0',
                date: 'Last month',
                title: 'Launch of OneHumanCorp',
                description: 'Welcome to OneHumanCorp! The simplest way to run your business with AI agents.',
                features: [
                  'Business Setup Wizard',
                  'AI Agent Dashboard',
                  'Stripe Payments Integration',
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildReleaseNote(BuildContext context, {
    required String version,
    required String date,
    required String title,
    required String description,
    required List<String> features,
  }) {
    return GlassCard(
      child: Padding(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                  decoration: BoxDecoration(
                    color: Theme.of(context).colorScheme.primary.withValues(alpha: 0.2),
                    borderRadius: BorderRadius.circular(16),
                    border: Border.all(color: Theme.of(context).colorScheme.primary.withValues(alpha: 0.5)),
                  ),
                  child: Text(
                    version,
                    style: TextStyle(
                      fontFamily: 'Inter',
                      fontWeight: FontWeight.bold,
                      color: Theme.of(context).colorScheme.primary,
                    ),
                  ),
                ),
                Text(
                  date,
                  style: const TextStyle(
                    fontFamily: 'Inter',
                    color: Colors.white54,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 16),
            Text(
              title,
              style: const TextStyle(
                fontFamily: 'Outfit',
                fontSize: 24,
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 12),
            Text(
              description,
              style: const TextStyle(
                fontFamily: 'Inter',
                fontSize: 16,
                color: Colors.white70,
                height: 1.5,
              ),
            ),
            const SizedBox(height: 16),
            ...features.map((feature) => Padding(
              padding: const EdgeInsets.only(bottom: 8.0),
              child: Row(
                children: [
                  const Icon(Icons.check_circle, size: 20, color: Colors.greenAccent),
                  const SizedBox(width: 8),
                  Expanded(
                    child: Text(
                      feature,
                      style: const TextStyle(
                        fontFamily: 'Inter',
                        fontSize: 15,
                      ),
                    ),
                  ),
                ],
              ),
            )),
          ],
        ),
      ),
    );
  }
}
