import 'package:flutter/material.dart';

class VideoTutorialPlayer extends StatelessWidget {
  final String title;
  final String thumbnailUrl; // In a real implementation, you'd use a video player package
  final String duration;

  const VideoTutorialPlayer({
    super.key,
    required this.title,
    required this.thumbnailUrl,
    required this.duration,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 250,
      margin: const EdgeInsets.only(right: 16),
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surface,
        borderRadius: BorderRadius.circular(16),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withValues(alpha: 0.1),
            blurRadius: 8,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Stack(
            alignment: Alignment.center,
            children: [
              Container(
                height: 140,
                decoration: const BoxDecoration(
                  color: Colors.black12,
                  borderRadius: BorderRadius.vertical(top: Radius.circular(16)),
                ),
                child: const Center(
                  child: Icon(Icons.image, size: 48, color: Colors.grey),
                ),
              ),
              const CircleAvatar(
                backgroundColor: Colors.black54,
                child: Icon(Icons.play_arrow, color: Colors.white),
              ),
              Positioned(
                bottom: 8,
                right: 8,
                child: Container(
                  padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                  decoration: BoxDecoration(
                    color: Colors.black87,
                    borderRadius: BorderRadius.circular(4),
                  ),
                  child: Text(
                    duration,
                    style: const TextStyle(color: Colors.white, fontSize: 12),
                  ),
                ),
              ),
            ],
          ),
          Padding(
            padding: const EdgeInsets.all(12.0),
            child: Text(
              title,
              style: const TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
              maxLines: 2,
              overflow: TextOverflow.ellipsis,
            ),
          ),
        ],
      ),
    );
  }
}
