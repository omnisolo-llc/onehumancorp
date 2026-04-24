import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class VideoTutorialsScreen extends StatelessWidget {
  const VideoTutorialsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final List<Map<String, String>> tutorials = [
      {'title': 'Set up your store in 5 minutes', 'duration': '1:20', 'category': 'Getting Started'},
      {'title': 'Add your first product', 'duration': '0:45', 'category': 'My Store'},
      {'title': 'Accept your first online payment', 'duration': '1:10', 'category': 'Payments'},
      {'title': 'Activate your AI Support Agent', 'duration': '0:55', 'category': 'AI Agents'},
      {'title': 'Connect your custom domain', 'duration': '1:15', 'category': 'My Store'},
      {'title': 'Create a social media campaign', 'duration': '1:30', 'category': 'Marketing'},
      {'title': 'Manage customer orders', 'duration': '0:50', 'category': 'Operations'},
      {'title': 'Set up automated email replies', 'duration': '1:00', 'category': 'AI Agents'},
      {'title': 'View business performance reports', 'duration': '0:40', 'category': 'Dashboard'},
      {'title': 'Configure tap-to-pay on your phone', 'duration': '1:25', 'category': 'Payments'},
    ];

    return Scaffold(
      appBar: AppBar(
        title: const Text('Video Tutorials', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: ListView.builder(
        padding: const EdgeInsets.all(16),
        itemCount: tutorials.length,
        itemBuilder: (context, index) {
          final tutorial = tutorials[index];
          return _VideoTutorialCard(
            title: tutorial['title']!,
            duration: tutorial['duration']!,
            category: tutorial['category']!,
            onTap: () {
              showModalBottomSheet(
                context: context,
                isScrollControlled: true,
                backgroundColor: Colors.transparent,
                builder: (context) => _VideoPlayerSheet(title: tutorial['title']!),
              );
            },
          );
        },
      ),
    );
  }
}

class _VideoTutorialCard extends StatelessWidget {
  final String title;
  final String duration;
  final String category;
  final VoidCallback onTap;

  const _VideoTutorialCard({
    required this.title,
    required this.duration,
    required this.category,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 16),
      child: GlassCard(
        child: InkWell(
          onTap: onTap,
          borderRadius: BorderRadius.circular(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Container(
                height: 160,
                decoration: BoxDecoration(
                  color: Theme.of(context).colorScheme.surfaceContainerHighest,
                  borderRadius: const BorderRadius.vertical(top: Radius.circular(16)),
                ),
                child: Center(
                  child: Icon(
                    Icons.play_circle_outline,
                    size: 64,
                    color: Theme.of(context).colorScheme.primary.withOpacity(0.8),
                  ),
                ),
              ),
              Padding(
                padding: const EdgeInsets.all(16),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Container(
                          padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                          decoration: BoxDecoration(
                            color: Theme.of(context).colorScheme.secondary.withOpacity(0.1),
                            borderRadius: BorderRadius.circular(4),
                          ),
                          child: Text(
                            category,
                            style: TextStyle(
                              fontFamily: 'Inter',
                              fontSize: 12,
                              fontWeight: FontWeight.bold,
                              color: Theme.of(context).colorScheme.secondary,
                            ),
                          ),
                        ),
                        Text(
                          duration,
                          style: TextStyle(
                            fontFamily: 'Inter',
                            fontSize: 12,
                            color: Theme.of(context).colorScheme.onSurfaceVariant,
                          ),
                        ),
                      ],
                    ),
                    const SizedBox(height: 8),
                    Text(
                      title,
                      style: const TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 18,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _VideoPlayerSheet extends StatelessWidget {
  final String title;

  const _VideoPlayerSheet({required this.title});

  @override
  Widget build(BuildContext context) {
    return Container(
      height: MediaQuery.of(context).size.height * 0.9,
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surface,
        borderRadius: const BorderRadius.vertical(top: Radius.circular(24)),
      ),
      child: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(16.0),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Expanded(
                  child: Text(
                    title,
                    style: const TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold),
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                  ),
                ),
                IconButton(
                  icon: const Icon(Icons.close),
                  onPressed: () => Navigator.pop(context),
                ),
              ],
            ),
          ),
          Expanded(
            child: Container(
              color: Colors.black,
              child: const Center(
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Icon(Icons.video_library, color: Colors.white54, size: 64),
                    SizedBox(height: 16),
                    Text('Video Player Placeholder', style: TextStyle(color: Colors.white, fontFamily: 'Outfit')),
                    SizedBox(height: 8),
                    Text('Portrait-optimized video goes here', style: TextStyle(color: Colors.white70, fontFamily: 'Inter', fontSize: 12)),
                  ],
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
