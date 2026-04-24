import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class VideoTutorial {
  final String id;
  final String title;
  final String description;
  final String videoUrl;
  final int durationSeconds;

  VideoTutorial({
    required this.id,
    required this.title,
    required this.description,
    required this.videoUrl,
    required this.durationSeconds,
  });

  factory VideoTutorial.fromJson(Map<String, dynamic> json) {
    return VideoTutorial(
      id: json['id'] as String,
      title: json['title'] as String,
      description: json['description'] as String,
      videoUrl: json['videoUrl'] as String,
      durationSeconds: json['durationSeconds'] as int,
    );
  }
}

final videoTutorialsProvider = FutureProvider<List<VideoTutorial>>((ref) async {
  final api = ref.read(apiServiceProvider);
  if (api == null) throw Exception('API Service not available');
  final response = await api.get('/api/v1/help/video-tutorials');

  if (response.statusCode == 200) {
    final List<dynamic> data = jsonDecode(response.body);
    return data.map((json) => VideoTutorial.fromJson(json)).toList();
  } else {
    throw Exception('Failed to load video tutorials');
  }
});

class VideoTutorialsScreen extends ConsumerWidget {
  const VideoTutorialsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tutorialsAsyncValue = ref.watch(videoTutorialsProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Video Tutorials', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: tutorialsAsyncValue.when(
        data: (tutorials) {
          return ListView.builder(
            padding: const EdgeInsets.all(16),
            itemCount: tutorials.length,
            itemBuilder: (context, index) {
              return _VideoTutorialCard(tutorial: tutorials[index]);
            },
          );
        },
        loading: () => const Center(child: CircularProgressIndicator()),
        error: (error, stack) => Center(child: Text('Error: $error')),
      ),
    );
  }
}

class _VideoTutorialCard extends StatelessWidget {
  final VideoTutorial tutorial;

  const _VideoTutorialCard({required this.tutorial});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 16.0),
      child: GlassCard(
        child: InkWell(
          onTap: () {
            _showVideoPlayer(context, tutorial);
          },
          borderRadius: BorderRadius.circular(16),
          child: Padding(
            padding: const EdgeInsets.all(20),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Container(
                  height: 200, // Portrait-optimized aspect ratio representation
                  decoration: BoxDecoration(
                    color: Theme.of(context).colorScheme.surfaceContainerHighest,
                    borderRadius: BorderRadius.circular(16),
                  ),
                  child: Center(
                    child: Container(
                      padding: const EdgeInsets.all(12),
                      decoration: const BoxDecoration(
                        color: Colors.black54,
                        shape: BoxShape.circle,
                      ),
                      child: const Icon(Icons.play_arrow, size: 48, color: Colors.white),
                    ),
                  ),
                ),
                const SizedBox(height: 16),
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Expanded(
                      child: Text(
                        tutorial.title,
                        style: const TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 18,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                    ),
                    Text(
                      '${tutorial.durationSeconds}s',
                      style: TextStyle(
                        fontFamily: 'Inter',
                        fontSize: 12,
                        color: Theme.of(context).colorScheme.onSurfaceVariant,
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 4),
                Text(
                  tutorial.description,
                  style: TextStyle(
                    fontFamily: 'Inter',
                    fontSize: 14,
                    color: Theme.of(context).colorScheme.onSurfaceVariant,
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  void _showVideoPlayer(BuildContext context, VideoTutorial tutorial) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      useSafeArea: true,
      builder: (context) {
        return Container(
          color: Colors.black,
          child: SafeArea(
            child: Column(
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: [
                    IconButton(
                      icon: const Icon(Icons.close, color: Colors.white),
                      onPressed: () => Navigator.pop(context),
                    ),
                  ],
                ),
                Expanded(
                  child: Center(
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        _VideoPlayerWidget(tutorial: tutorial),
                      ],
                    ),
                  ),
                ),
              ],
            ),
          ),
        );
      },
    );
  }
}

class _VideoPlayerWidget extends StatelessWidget {
  final VideoTutorial tutorial;

  const _VideoPlayerWidget({required this.tutorial});

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        const AspectRatio(
          aspectRatio: 16 / 9,
          child: Center(
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Icon(Icons.play_circle_outline, size: 64, color: Colors.white),
                SizedBox(height: 16),
                Text(
                  'Portrait-Optimized Player Placeholder',
                  style: TextStyle(color: Colors.white, fontSize: 16),
                ),
                SizedBox(height: 16),
              ],
            ),
          ),
        ),
        const SizedBox(height: 16),
        Text(
          'Playing: ${tutorial.title}',
          style: const TextStyle(color: Colors.white, fontSize: 18, fontFamily: 'Outfit'),
          textAlign: TextAlign.center,
        ),
      ],
    );
  }
}
