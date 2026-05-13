import 'package:flutter/material.dart';
import '../../main.dart'; // For GlassContainer
import 'release_notes_screen.dart';
import 'video_tutorials_screen.dart';
import 'api_reference_screen.dart';


class HelpCenterScreen extends StatelessWidget {
  const HelpCenterScreen({super.key});

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
                _buildTopicCard(context, 'Marketing', 'Grow your audience and drive sales.', Icons.campaign),
                const SizedBox(height: 30),
                const Text('Video Tutorials', style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white)),
                const SizedBox(height: 10),
                InkWell(
                  onTap: () => Navigator.push(context, MaterialPageRoute(builder: (context) => const VideoTutorialsScreen())),
                  child: _buildVideoCard('How to add your first product', '1:20'),
                ),
                _buildVideoCard('Setting up automated support', '0:55'),
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

  Widget _buildVideoCard(String title, String duration) {
    return Card(
      color: Colors.white.withAlpha(15),
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: ListTile(
        leading: const Icon(Icons.play_circle_fill, color: const Color(0xFF6B4EFF), size: 40),
        title: Text(title, style: const TextStyle(fontWeight: FontWeight.bold, color: Colors.white)),
        subtitle: Text(duration, style: const TextStyle(color: Colors.white70)),
        onTap: () {
          // Stub for video player
        },
      ),
    );
  }
}
