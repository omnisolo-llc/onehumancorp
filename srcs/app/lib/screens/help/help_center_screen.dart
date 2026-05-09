import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import '../../main.dart'; // For GlassContainer
import 'release_notes_screen.dart';
import 'api_reference_screen.dart';
import 'video_tutorial_screen.dart';

class HelpCenterScreen extends StatefulWidget {
  const HelpCenterScreen({super.key});

  @override
  State<HelpCenterScreen> createState() => _HelpCenterScreenState();
}

class _HelpCenterScreenState extends State<HelpCenterScreen> {
  late Future<List<dynamic>> _videosFuture;

  @override
  void initState() {
    super.initState();
    _videosFuture = _fetchVideos();
  }

  Future<List<dynamic>> _fetchVideos() async {
    try {
      const String baseUrl = String.fromEnvironment('API_URL', defaultValue: 'http://127.0.0.1:8080');
      final response = await http.get(Uri.parse('$baseUrl/api/help/videos'));
      if (response.statusCode == 200) {
        return jsonDecode(response.body);
      }
    } catch (e) {
      debugPrint('Error fetching videos: $e');
    }
    // Fallback static data if backend is unreachable
    return [
      {'id': '1', 'title': 'How to add your first product', 'duration': '1:20', 'description': 'Learn the basics of setting up your store.'},
      {'id': '2', 'title': 'Setting up automated support', 'duration': '0:55', 'description': 'Configure AI to answer common questions.'},
    ];
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      appBar: AppBar(
        title: const Text('Help Center', style: TextStyle(fontFamily: 'Outfit', color: Colors.white)),
        backgroundColor: Colors.transparent,
        elevation: 0,
        iconTheme: const IconThemeData(color: Colors.white),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(20),
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 600),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                const Text(
                  'How can we help?',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 28,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                  textAlign: TextAlign.center,
                ),
                const SizedBox(height: 20),
                GlassContainer(
                  child: TextField(
                    style: const TextStyle(color: Colors.white),
                    decoration: InputDecoration(
                      hintText: 'Search for articles...',
                      hintStyle: const TextStyle(color: Colors.white70),
                      prefixIcon: const Icon(Icons.search, color: Colors.white70),
                      border: OutlineInputBorder(
                        borderRadius: BorderRadius.circular(10),
                        borderSide: BorderSide.none,
                      ),
                      filled: true,
                      fillColor: Colors.white.withAlpha(20),
                    ),
                  ),
                ),
                const SizedBox(height: 30),
                const Text('Topics', style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white)),
                const SizedBox(height: 10),
                _buildTopicCard(context, 'Getting Started', 'Learn the basics of setting up your store.', Icons.rocket_launch),
                _buildTopicCard(context, 'My Store', 'Manage products, inventory, and storefront.', Icons.storefront),
                _buildTopicCard(context, 'Payments', 'Connect Stripe, get paid, and manage billing.', Icons.payment),
                _buildTopicCard(context, 'AI Agents', 'Configure your AI team to work for you.', Icons.smart_toy),
                const SizedBox(height: 30),
                const Text('Video Tutorials', style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white)),
                const SizedBox(height: 10),
                FutureBuilder<List<dynamic>>(
                  future: _videosFuture,
                  builder: (context, snapshot) {
                    if (snapshot.connectionState == ConnectionState.waiting) {
                      return const Center(child: CircularProgressIndicator());
                    }
                    if (snapshot.hasError || !snapshot.hasData || snapshot.data!.isEmpty) {
                      return const Text('No tutorials available.', style: TextStyle(color: Colors.white70));
                    }
                    return Column(
                      children: snapshot.data!.map((video) {
                        return _buildVideoCard(context, video);
                      }).toList(),
                    );
                  },
                ),
                const SizedBox(height: 30),
                const Text('Updates & Advanced', style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white)),
                const SizedBox(height: 10),
                ListTile(
                  title: const Text('Release Notes', style: TextStyle(color: Colors.white)),
                  trailing: const Icon(Icons.chevron_right, color: Colors.white),
                  onTap: () {
                    Navigator.push(context, MaterialPageRoute(builder: (context) => const ReleaseNotesScreen()));
                  },
                ),
                ListTile(
                  title: const Text('API Reference', style: TextStyle(color: Colors.white)),
                  trailing: const Icon(Icons.chevron_right, color: Colors.white),
                  onTap: () {
                    Navigator.push(context, MaterialPageRoute(builder: (context) => const ApiReferenceScreen()));
                  },
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildTopicCard(BuildContext context, String title, String subtitle, IconData icon) {
    return Card(
      color: Colors.white.withAlpha(15),
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: ListTile(
        leading: Icon(icon, color: const Color(0xFF6B4EFF)),
        title: Text(title, style: const TextStyle(fontWeight: FontWeight.bold, color: Colors.white)),
        subtitle: Text(subtitle, style: const TextStyle(color: Colors.white70)),
        trailing: const Icon(Icons.chevron_right, color: Colors.white54),
        onTap: () {
          // Stub for topic details
        },
      ),
    );
  }

  Widget _buildVideoCard(BuildContext context, dynamic videoData) {
    return Card(
      color: Colors.white.withAlpha(15),
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: ListTile(
        leading: const Icon(Icons.play_circle_fill, color: const Color(0xFF6B4EFF), size: 40),
        title: Text(videoData['title'] ?? '', style: const TextStyle(fontWeight: FontWeight.bold, color: Colors.white)),
        subtitle: Text(videoData['duration'] ?? '', style: const TextStyle(color: Colors.white70)),
        onTap: () {
          Navigator.push(
            context,
            MaterialPageRoute(
              builder: (context) => VideoTutorialScreen(videoData: videoData),
            ),
          );
        },
      ),
    );
  }
}
