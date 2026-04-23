import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class ReleaseNotesScreen extends StatelessWidget {
  const ReleaseNotesScreen({super.key});

  final List<Map<String, dynamic>> _releases = const [
    {
      'version': 'v1.4.0',
      'date': 'October 24, 2026',
      'title': 'New AI Social Media Manager & UI Improvements',
      'features': [
        'Added the Promoter Agent to automate your Instagram posts.',
        'Improved the Dashboard with fresh Glassmorphism styling.',
        'Fixed an issue where some users could not connect their domain.',
      ],
      'icon': Icons.campaign,
    },
    {
      'version': 'v1.3.2',
      'date': 'October 10, 2026',
      'title': 'Faster Loading & Bug Fixes',
      'features': [
        'Optimized images so your store loads 3x faster on mobile.',
        'The Sales Agent now correctly handles partial refunds.',
        'Added more plain language explanations to the Settings page.',
      ],
      'icon': Icons.speed,
    },
    {
      'version': 'v1.3.0',
      'date': 'September 15, 2026',
      'title': 'Introducing AI Chat Support',
      'features': [
        'You can now chat with our AI Help Agent directly in the app.',
        'New Stripe integration makes checkout smoother.',
      ],
      'icon': Icons.chat,
    },
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('What\'s New', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      extendBodyBehindAppBar: true,
      body: SafeArea(
        child: LayoutBuilder(
          builder: (context, constraints) {
            return Center(
              child: ConstrainedBox(
                constraints: const BoxConstraints(maxWidth: 800),
                child: Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text(
                        'Release Notes',
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 28,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      const SizedBox(height: 8),
                      const Text(
                        'See the latest improvements we\'ve made to help your business grow.',
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 16,
                          color: Colors.grey,
                        ),
                      ),
                      const SizedBox(height: 32),
                      Expanded(
                        child: ListView.builder(
                          itemCount: _releases.length,
                          itemBuilder: (context, index) {
                            return _buildReleaseCard(context, _releases[index]);
                          },
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            );
          },
        ),
      ),
    );
  }

  Widget _buildReleaseCard(BuildContext context, Map<String, dynamic> release) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 24.0),
      child: GlassCard(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                  decoration: BoxDecoration(
                    color: Theme.of(context).colorScheme.primary.withValues(alpha: 0.2),
                    borderRadius: BorderRadius.circular(16),
                  ),
                  child: Text(
                    release['version'],
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontWeight: FontWeight.bold,
                      color: Theme.of(context).colorScheme.primary,
                    ),
                  ),
                ),
                const SizedBox(width: 12),
                Text(
                  release['date'],
                  style: const TextStyle(
                    fontFamily: 'Outfit',
                    color: Colors.grey,
                  ),
                ),
                const Spacer(),
                Icon(release['icon'], color: Colors.grey.withValues(alpha: 0.5)),
              ],
            ),
            const SizedBox(height: 16),
            Text(
              release['title'],
              style: const TextStyle(
                fontFamily: 'Outfit',
                fontSize: 20,
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 16),
            ...List.generate(
              (release['features'] as List<String>).length,
              (index) => Padding(
                padding: const EdgeInsets.only(bottom: 8.0),
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Padding(
                      padding: EdgeInsets.only(top: 6.0, right: 12.0),
                      child: Icon(Icons.circle, size: 8, color: Colors.grey),
                    ),
                    Expanded(
                      child: Text(
                        release['features'][index],
                        style: const TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 16,
                          height: 1.5,
                        ),
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
