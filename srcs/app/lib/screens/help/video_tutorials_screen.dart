import 'package:flutter/material.dart';

class VideoTutorialsScreen extends StatelessWidget {
  const VideoTutorialsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Video Tutorials')),
      body: const Center(child: Text('Video Tutorials')),
    );
  }
}
