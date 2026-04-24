import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class VideoTutorialsScreen extends StatelessWidget {
  const VideoTutorialsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Video Tutorials', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
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
                'Learn how to use OneHumanCorp',
                style: TextStyle(fontFamily: 'Outfit', fontSize: 32, fontWeight: FontWeight.bold),
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 32),
              _buildVideoCard(
                context,
                title: '1. Setting up your store',
                duration: '1:45',
                thumbnailColor: Colors.blueAccent,
              ),
              const SizedBox(height: 16),
              _buildVideoCard(
                context,
                title: '2. Adding your first product',
                duration: '2:10',
                thumbnailColor: Colors.purpleAccent,
              ),
              const SizedBox(height: 16),
              _buildVideoCard(
                context,
                title: '3. Activating your AI Support Agent',
                duration: '1:30',
                thumbnailColor: Colors.tealAccent,
              ),
              const SizedBox(height: 16),
              _buildVideoCard(
                context,
                title: '4. Reading your dashboard analytics',
                duration: '3:05',
                thumbnailColor: Colors.orangeAccent,
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildVideoCard(BuildContext context, {required String title, required String duration, required Color thumbnailColor}) {
    return GlassCard(
      child: InkWell(
        onTap: () {},
        borderRadius: BorderRadius.circular(16),
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: Row(
            children: [
              Container(
                width: 120,
                height: 80,
                decoration: BoxDecoration(
                  color: thumbnailColor.withValues(alpha: 0.2),
                  borderRadius: BorderRadius.circular(12),
                  border: Border.all(color: thumbnailColor.withValues(alpha: 0.5)),
                ),
                child: Center(
                  child: Icon(Icons.play_circle_fill, size: 40, color: thumbnailColor),
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      title,
                      style: const TextStyle(
                        fontFamily: 'Inter',
                        fontSize: 18,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    const SizedBox(height: 8),
                    Row(
                      children: [
                        const Icon(Icons.timer, size: 16, color: Colors.grey),
                        const SizedBox(width: 4),
                        Text(
                          duration,
                          style: const TextStyle(
                            fontFamily: 'Inter',
                            color: Colors.grey,
                          ),
                        ),
                      ],
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
