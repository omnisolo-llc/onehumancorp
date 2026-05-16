import 'package:flutter/material.dart';

class VideoTutorialsScreen extends StatelessWidget {
  const VideoTutorialsScreen({super.key});

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
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(20),
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 800),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                const Text(
                  'Learn how to use OneHumanCorp',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 28,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                ),
                const SizedBox(height: 10),
                const Text(
                  'Short and simple tutorials to help you master the platform.',
                  style: TextStyle(fontSize: 16, color: Colors.white70),
                ),
                const SizedBox(height: 30),
                _buildVideoCard('How to add your first product', '1:20', 'Learn how to easily add products to your store, including images, descriptions, and pricing.'),
                const SizedBox(height: 15),
                _buildVideoCard('Setting up automated support', '0:55', 'Configure your AI Agent to instantly answer common customer questions.'),
                const SizedBox(height: 15),
                _buildVideoCard('Understanding your dashboard', '2:10', 'A quick tour of your main dashboard and what each metric means for your business.'),
                const SizedBox(height: 15),
                _buildVideoCard('Connecting a payment gateway', '1:45', 'Link your Stripe account and start accepting payments securely.'),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildVideoCard(String title, String duration, String description) {
    return Card(
      color: Colors.white.withAlpha(15),
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: InkWell(
        onTap: () {
          // In a real app, this would open a video player.
        },
        borderRadius: BorderRadius.circular(12),
        child: Padding(
          padding: const EdgeInsets.all(15.0),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Container(
                width: 120,
                height: 80,
                decoration: BoxDecoration(
                  color: Colors.black26,
                  borderRadius: BorderRadius.circular(8),
                  border: Border.all(color: Colors.white24),
                ),
                child: const Center(
                  child: Icon(Icons.play_circle_fill, color: Color(0xFF6B4EFF), size: 40),
                ),
              ),
              const SizedBox(width: 15),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Expanded(
                          child: Text(
                            title,
                            style: const TextStyle(fontWeight: FontWeight.bold, color: Colors.white, fontSize: 18),
                          ),
                        ),
                        Container(
                          padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                          decoration: BoxDecoration(
                            color: Colors.black45,
                            borderRadius: BorderRadius.circular(4),
                          ),
                          child: Text(
                            duration,
                            style: const TextStyle(color: Colors.white, fontSize: 12, fontWeight: FontWeight.bold),
                          ),
                        ),
                      ],
                    ),
                    const SizedBox(height: 8),
                    Text(
                      description,
                      style: const TextStyle(color: Colors.white70, fontSize: 14),
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
