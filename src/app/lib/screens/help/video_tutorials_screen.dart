import 'dart:ui';
import 'package:flutter/material.dart';

class VideoTutorialsScreen extends StatelessWidget {
  const VideoTutorialsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final tutorials = [
      {'title': 'Setting up your storefront in 5 minutes', 'duration': '1:20', 'id': 'setup'},
      {'title': 'How to connect Stripe for payments', 'duration': '0:45', 'id': 'stripe'},
      {'title': 'Configuring your AI Support Agent', 'duration': '1:10', 'id': 'ai'},
      {'title': 'Understanding your daily analytics', 'duration': '0:55', 'id': 'analytics'},
      {'title': 'Managing custom orders and deposits', 'duration': '1:30', 'id': 'orders'},
    ];

    return Scaffold(
      appBar: AppBar(
        title: const Text('Video Tutorials', style: TextStyle(fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      extendBodyBehindAppBar: true,
      body: Container(
        decoration: BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [
              Theme.of(context).colorScheme.surface,
              Theme.of(context).colorScheme.surfaceContainerHighest.withOpacity(0.5),
            ],
          ),
        ),
        child: SafeArea(
          child: ListView.builder(
            padding: const EdgeInsets.all(16),
            itemCount: tutorials.length,
            itemBuilder: (context, index) {
              final tutorial = tutorials[index];
              return Padding(
                padding: const EdgeInsets.only(bottom: 16.0),
                child: ClipRRect(
                  borderRadius: BorderRadius.circular(16),
                  child: BackdropFilter(
                    filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
                    child: Container(
                      decoration: BoxDecoration(
                        color: Theme.of(context).colorScheme.surface.withOpacity(0.5),
                        borderRadius: BorderRadius.circular(16),
                        border: Border.all(color: Colors.white.withOpacity(0.2)),
                      ),
                      child: Column(
                        children: [
                          ListTile(
                            contentPadding: const EdgeInsets.all(16),
                            leading: Container(
                              width: 80,
                              height: 60,
                              decoration: BoxDecoration(
                                color: Theme.of(context).colorScheme.primaryContainer,
                                borderRadius: BorderRadius.circular(8),
                              ),
                              child: Icon(Icons.play_circle_fill, color: Theme.of(context).colorScheme.primary),
                            ),
                            title: Text(
                              tutorial['title']!,
                              style: const TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.w600),
                            ),
                            subtitle: Text(
                              tutorial['duration']!,
                              style: TextStyle(color: Theme.of(context).colorScheme.onSurfaceVariant),
                            ),
                            trailing: IconButton(
                              icon: const Icon(Icons.open_in_new),
                              onPressed: () {
                                _showVideoPlayer(context, tutorial['title']!);
                              },
                            ),
                            onTap: () {
                              _showVideoPlayer(context, tutorial['title']!);
                            },
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
              );
            },
          ),
        ),
      ),
    );
  }

  void _showVideoPlayer(BuildContext context, String title) {
    showDialog(
      context: context,
      builder: (context) => Dialog(
        backgroundColor: Colors.transparent,
        insetPadding: const EdgeInsets.all(16),
        child: ClipRRect(
          borderRadius: BorderRadius.circular(24),
          child: BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
            child: Container(
              height: 300,
              decoration: BoxDecoration(
                color: Theme.of(context).colorScheme.surface.withOpacity(0.85),
                borderRadius: BorderRadius.circular(24),
                border: Border.all(color: Colors.white.withOpacity(0.2)),
              ),
              child: Column(
                children: [
                  Container(
                    padding: const EdgeInsets.all(16),
                    decoration: BoxDecoration(
                      color: Theme.of(context).colorScheme.primary.withOpacity(0.1),
                      border: Border(bottom: BorderSide(color: Colors.white.withOpacity(0.1))),
                    ),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Expanded(
                          child: Text(
                            title,
                            style: const TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Outfit', fontSize: 16),
                            overflow: TextOverflow.ellipsis,
                          ),
                        ),
                        IconButton(
                          icon: const Icon(Icons.close),
                          onPressed: () => Navigator.of(context).pop(),
                        ),
                      ],
                    ),
                  ),
                  Expanded(
                    child: Center(
                      child: Column(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: [
                          Icon(Icons.play_circle_outline, size: 64, color: Theme.of(context).colorScheme.primary),
                          const SizedBox(height: 16),
                          const Text('Video Player Implementation'),
                          const Text('(Mocked for demo)'),
                        ],
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
