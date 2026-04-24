import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class VideoTutorial {
  final String id;
  final String title;
  final String duration;
  final String description;

  const VideoTutorial({
    required this.id,
    required this.title,
    required this.duration,
    required this.description,
  });
}

class VideoTutorialsScreen extends StatelessWidget {
  const VideoTutorialsScreen({super.key});

  static const List<VideoTutorial> tutorials = [
    VideoTutorial(
      id: 'v1',
      title: 'Set up your store in 5 minutes',
      duration: '4:30',
      description: 'Learn how to add products and choose a theme.',
    ),
    VideoTutorial(
      id: 'v2',
      title: 'Accept your first payment',
      duration: '2:15',
      description: 'Connect Stripe and start accepting payments securely.',
    ),
    VideoTutorial(
      id: 'v3',
      title: 'Activate your AI Support Agent',
      duration: '3:45',
      description: 'Train your AI agent to answer customer questions automatically.',
    ),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Video Tutorials', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () {
            if (context.canPop()) {
              context.pop();
            } else {
              context.go('/help');
            }
          },
        ),
      ),
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Color(0xFF1E1E28), Color(0xFF0F0F14)],
          ),
        ),
        child: ListView.builder(
          padding: const EdgeInsets.all(16.0),
          itemCount: tutorials.length,
          itemBuilder: (context, index) {
            final tut = tutorials[index];
            return Padding(
              padding: const EdgeInsets.only(bottom: 16.0),
              child: GlassCard(
                child: ListTile(
                  contentPadding: const EdgeInsets.all(16.0),
                  leading: Container(
                    width: 80,
                    height: 60,
                    decoration: BoxDecoration(
                      color: Colors.black45,
                      borderRadius: BorderRadius.circular(8),
                    ),
                    child: const Center(
                      child: Icon(Icons.play_circle_fill, color: Colors.indigoAccent, size: 36),
                    ),
                  ),
                  title: Text(
                    tut.title,
                    style: const TextStyle(color: Colors.white, fontWeight: FontWeight.bold, fontSize: 16),
                  ),
                  subtitle: Padding(
                    padding: const EdgeInsets.only(top: 8.0),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(tut.description, style: const TextStyle(color: Colors.white70)),
                        const SizedBox(height: 4),
                        Text(tut.duration, style: const TextStyle(color: Colors.white54, fontSize: 12)),
                      ],
                    ),
                  ),
                  onTap: () {
                    ScaffoldMessenger.of(context).showSnackBar(
                      SnackBar(content: Text('Playing video: ${tut.title}')),
                    );
                  },
                ),
              ),
            );
          },
        ),
      ),
    );
  }
}
