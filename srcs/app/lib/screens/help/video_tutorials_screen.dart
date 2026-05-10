import 'package:flutter/material.dart';
import '../../main.dart'; // For GlassContainer

class VideoTutorialsScreen extends StatelessWidget {
  const VideoTutorialsScreen({super.key});

  final List<Map<String, dynamic>> videos = const [
    {
      "title": "How to add your first product",
      "description": "Learn the basics of listing an item.",
      "duration": "1:25"
    },
    {
      "title": "Setting up payments",
      "description": "Accept Apple Pay and credit cards.",
      "duration": "1:00"
    },
    {
      "title": "Connect your Instagram",
      "description": "Link your social media easily.",
      "duration": "0:45"
    },
    {
      "title": "Create a discount code",
      "description": "Boost sales with a promo code.",
      "duration": "0:50"
    },
    {
      "title": "Managing orders",
      "description": "How to fulfill and ship orders.",
      "duration": "1:15"
    },
    {
      "title": "Using the AI Helper",
      "description": "Let AI draft your emails.",
      "duration": "1:28"
    },
    {
      "title": "Customize your storefront",
      "description": "Change colors and fonts.",
      "duration": "1:30"
    },
    {
      "title": "View your analytics",
      "description": "Track your sales and visitors.",
      "duration": "0:55"
    },
    {
      "title": "Set up a custom domain",
      "description": "Use your own website address.",
      "duration": "1:20"
    },
    {
      "title": "Invite your team",
      "description": "Add staff members to your store.",
      "duration": "0:40"
    }
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      appBar: AppBar(
        title: const Text('Video Tutorials', style: TextStyle(fontFamily: 'Outfit', color: Colors.white)),
        backgroundColor: Colors.transparent,
        elevation: 0,
        iconTheme: const IconThemeData(color: Colors.white),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 600),
          child: ListView.builder(
            padding: const EdgeInsets.all(20),
            itemCount: videos.length,
            itemBuilder: (context, index) {
              final video = videos[index];
              return Padding(
                padding: const EdgeInsets.only(bottom: 20),
                child: GlassContainer(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      Container(
                        height: 200,
                        decoration: BoxDecoration(
                          color: const Color(0xFF0F172A),
                          borderRadius: BorderRadius.circular(8),
                        ),
                        child: const Center(
                          child: Text('▶ Play Video', style: TextStyle(fontWeight: FontWeight.bold, color: Colors.white)),
                        ),
                      ),
                      const SizedBox(height: 12),
                      Text(
                        video['title']!,
                        style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 18, color: Colors.white),
                      ),
                      const SizedBox(height: 5),
                      Text(
                        video['description']!,
                        style: const TextStyle(color: Colors.white70),
                      ),
                    ],
                  ),
                ),
              );
            },
          ),
        ),
      ),
    );
  }
}
