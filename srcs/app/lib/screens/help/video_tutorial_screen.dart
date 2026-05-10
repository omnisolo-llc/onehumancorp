import 'package:flutter/material.dart';

class VideoTutorialScreen extends StatelessWidget {
  final Map<String, dynamic> videoData;

  const VideoTutorialScreen({super.key, required this.videoData});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      appBar: AppBar(
        title: const Text('Video Tutorial', style: TextStyle(fontFamily: 'Outfit', color: Colors.white)),
        backgroundColor: Colors.transparent,
        elevation: 0,
        iconTheme: const IconThemeData(color: Colors.white),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 600),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              // Mock Portrait Video Player
              Container(
                width: double.infinity,
                height: MediaQuery.of(context).size.height * 0.5,
                margin: const EdgeInsets.all(20),
                decoration: BoxDecoration(
                  color: Colors.black,
                  borderRadius: BorderRadius.circular(16),
                  border: Border.all(color: Colors.white24),
                ),
                child: Stack(
                  alignment: Alignment.center,
                  children: [
                    const Icon(Icons.play_circle_fill, color: Colors.white, size: 80),
                    Positioned(
                      bottom: 10,
                      left: 10,
                      right: 10,
                      child: Row(
                        children: [
                          const Icon(Icons.pause, color: Colors.white, size: 24),
                          const SizedBox(width: 10),
                          Expanded(
                            child: Container(
                              height: 4,
                              color: Colors.white30,
                              child: Align(
                                alignment: Alignment.centerLeft,
                                child: Container(width: 50, color: const Color(0xFF6B4EFF)),
                              ),
                            ),
                          ),
                          const SizedBox(width: 10),
                          const Text('0:10', style: TextStyle(color: Colors.white)),
                          const Text(' / ', style: TextStyle(color: Colors.white54)),
                          Text(videoData['duration'] ?? '1:00', style: const TextStyle(color: Colors.white)),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 20),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      videoData['title'] ?? 'Tutorial',
                      style: const TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 24,
                        fontWeight: FontWeight.bold,
                        color: Colors.white,
                      ),
                    ),
                    const SizedBox(height: 10),
                    Text(
                      videoData['description'] ?? '',
                      style: const TextStyle(fontSize: 16, color: Colors.white70),
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
