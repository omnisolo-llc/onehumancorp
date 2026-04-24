import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class ReleaseNotesScreen extends StatelessWidget {
  const ReleaseNotesScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0D0D1A),
      appBar: AppBar(
        title: const Text('Release Notes', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              "What's New in OHC",
              style: TextStyle(fontFamily: 'Outfit', fontSize: 32, fontWeight: FontWeight.bold, color: Colors.white),
            ),
            const SizedBox(height: 24),
            _ReleaseNoteCard(
              version: 'v2.4',
              date: 'October 15, 2026',
              title: '2x Faster Orchestration & Custom Roles',
              description: 'We have completely rewritten the AI orchestration engine to be twice as fast. You can now also create custom agent roles tailored exactly to your business.',
              bullets: [
                'Faster task processing',
                'Custom agent roles (e.g., "Vegan Recipe Expert")',
                'Improved mobile dashboard loading times'
              ],
            ),
            const SizedBox(height: 16),
            _ReleaseNoteCard(
              version: 'v2.3',
              date: 'September 28, 2026',
              title: 'Apple Pay & Google Pay Support',
              description: 'Your customers can now check out with a single tap using Apple Pay and Google Pay on all storefronts.',
              bullets: [
                '1-tap checkout enabled by default',
                'New financial reports in the dashboard',
              ],
            ),
            const SizedBox(height: 16),
            _ReleaseNoteCard(
              version: 'v2.2',
              date: 'September 10, 2026',
              title: 'Instagram Auto-Posting',
              description: 'The Marketing Agent can now automatically post your new products to Instagram with AI-generated captions and hashtags.',
              bullets: [
                'Connect Instagram in Settings > Integrations',
                'Approve posts before they go live',
              ],
            ),
          ],
        ),
      ),
    );
  }
}

class _ReleaseNoteCard extends StatelessWidget {
  final String version;
  final String date;
  final String title;
  final String description;
  final List<String> bullets;

  const _ReleaseNoteCard({
    required this.version,
    required this.date,
    required this.title,
    required this.description,
    required this.bullets,
  });

  @override
  Widget build(BuildContext context) {
    return Semantics(
      label: 'Release $version: $title',
      child: GlassCard(
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Container(
                    padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
                    decoration: BoxDecoration(
                      color: Colors.blueAccent.withAlpha(51),
                      borderRadius: BorderRadius.circular(16),
                    ),
                    child: Text(
                      version,
                      style: const TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold, color: Colors.blueAccent),
                    ),
                  ),
                  Text(
                    date,
                    style: const TextStyle(fontFamily: 'Inter', color: Colors.white54, fontSize: 14),
                  ),
                ],
              ),
              const SizedBox(height: 16),
              Text(
                title,
                style: const TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white),
              ),
              const SizedBox(height: 8),
              Text(
                description,
                style: const TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 16, height: 1.5),
              ),
              const SizedBox(height: 16),
              ...bullets.map((b) => Padding(
                padding: const EdgeInsets.only(bottom: 8.0),
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text('• ', style: TextStyle(color: Colors.white70, fontSize: 16)),
                    Expanded(child: Text(b, style: const TextStyle(fontFamily: 'Inter', color: Colors.white70, fontSize: 16))),
                  ],
                ),
              )),
            ],
          ),
        ),
      ),
    );
  }
}
