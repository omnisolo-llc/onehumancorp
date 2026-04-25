import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class ReleaseNotesScreen extends ConsumerWidget {
  const ReleaseNotesScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("What's New in OHC", style: TextStyle(fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
      ),
      backgroundColor: Colors.transparent,
      body: ListView(
        padding: const EdgeInsets.all(24),
        children: const [
          _ReleaseNote(
            version: 'v1.4.0',
            date: 'October 15, 2023',
            title: 'AI Marketing Agent Update',
            description: 'The Marketing Agent can now automatically generate and post Facebook ad campaigns directly from your dashboard.',
            isNew: true,
          ),
          SizedBox(height: 16),
          _ReleaseNote(
            version: 'v1.3.2',
            date: 'October 1, 2023',
            title: 'Storefront Enhancements',
            description: 'Added support for 3 new modern storefront themes and improved mobile checkout speeds by 25%.',
          ),
          SizedBox(height: 16),
          _ReleaseNote(
            version: 'v1.3.0',
            date: 'September 15, 2023',
            title: 'Introducing Agent Handoffs',
            description: 'You can now seamlessly take over conversations from your AI Customer Support agent if a customer requests human assistance.',
          ),
        ],
      ),
    );
  }
}

class _ReleaseNote extends StatelessWidget {
  final String version;
  final String date;
  final String title;
  final String description;
  final bool isNew;

  const _ReleaseNote({
    required this.version,
    required this.date,
    required this.title,
    required this.description,
    this.isNew = false,
  });

  @override
  Widget build(BuildContext context) {
    return GlassCard(
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Text(
                  version,
                  style: TextStyle(
                    fontWeight: FontWeight.bold,
                    color: Theme.of(context).colorScheme.primary,
                    fontFamily: 'Outfit',
                  ),
                ),
                const SizedBox(width: 8),
                Text(
                  date,
                  style: const TextStyle(color: Colors.white54, fontSize: 12),
                ),
                const Spacer(),
                if (isNew)
                  Container(
                    padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                    decoration: BoxDecoration(
                      color: Colors.green.withValues(alpha: 0.2),
                      borderRadius: BorderRadius.circular(12),
                      border: Border.all(color: Colors.green.withValues(alpha: 0.5)),
                    ),
                    child: const Text('NEW', style: TextStyle(color: Colors.green, fontSize: 10, fontWeight: FontWeight.bold)),
                  ),
              ],
            ),
            const SizedBox(height: 12),
            Text(
              title,
              style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
            ),
            const SizedBox(height: 8),
            Text(
              description,
              style: const TextStyle(fontFamily: 'Inter', fontSize: 15),
            ),
          ],
        ),
      ),
    );
  }
}
