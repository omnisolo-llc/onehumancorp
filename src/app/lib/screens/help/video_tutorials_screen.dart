import 'package:flutter/material.dart';

class VideoTutorialsScreen extends StatelessWidget {
  const VideoTutorialsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Video Tutorials', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          const Text('Top 10 Tasks', style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
          const SizedBox(height: 16),
          _VideoCard(title: 'Set up your store in 5 minutes', duration: '1:20'),
          _VideoCard(title: 'How to accept your first payment', duration: '0:45'),
          _VideoCard(title: 'Hiring an AI Support Agent', duration: '1:10'),
        ],
      ),
    );
  }
}

class _VideoCard extends StatelessWidget {
  final String title;
  final String duration;

  const _VideoCard({required this.title, required this.duration});

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.only(bottom: 16),
      child: ListTile(
        leading: const Icon(Icons.play_circle_fill, size: 40, color: Colors.indigo),
        title: Text(title, style: const TextStyle(fontWeight: FontWeight.bold)),
        subtitle: Text('Duration: $duration'),
        trailing: const Icon(Icons.open_in_new),
      ),
    );
  }
}
