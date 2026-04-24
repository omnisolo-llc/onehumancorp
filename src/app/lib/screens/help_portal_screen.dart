import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class HelpPortalScreen extends StatefulWidget {
  const HelpPortalScreen({super.key});

  @override
  State<HelpPortalScreen> createState() => _HelpPortalScreenState();
}

class _HelpPortalScreenState extends State<HelpPortalScreen> {
  final _searchController = TextEditingController();

  @override
  void dispose() {
    _searchController.dispose();
    super.dispose();
  }

  Widget _buildCategoryCard(String title, IconData icon, String description) {
    return GlassCard(
      child: InkWell(
        onTap: () {
          // Future: Navigate to specific category view
        },
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Icon(icon, size: 32, color: Colors.blueAccent),
              const SizedBox(height: 12),
              Text(
                title,
                style: const TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const SizedBox(height: 8),
              Text(
                description,
                style: const TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 14,
                  color: Colors.white70,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildVideoTutorial(String title, String duration) {
    return Container(
      margin: const EdgeInsets.only(bottom: 12),
      decoration: BoxDecoration(
        color: Colors.black26,
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: Colors.white10),
      ),
      child: ListTile(
        leading: const Icon(Icons.play_circle_fill, color: Colors.blueAccent, size: 36),
        title: Text(title, style: const TextStyle(color: Colors.white, fontFamily: 'Outfit')),
        subtitle: Text(duration, style: const TextStyle(color: Colors.white60, fontFamily: 'Inter')),
        trailing: const Icon(Icons.chevron_right, color: Colors.white54),
        onTap: () {
          // Future: launch video player overlay
        },
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      appBar: AppBar(
        title: const Text('Help Portal', style: TextStyle(fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            GlassCard(
              child: TextField(
                controller: _searchController,
                style: const TextStyle(color: Colors.white),
                decoration: const InputDecoration(
                  hintText: 'Search for articles, guides, and videos...',
                  hintStyle: TextStyle(color: Colors.white54),
                  prefixIcon: Icon(Icons.search, color: Colors.white54),
                  border: InputBorder.none,
                  contentPadding: EdgeInsets.symmetric(horizontal: 16, vertical: 14),
                ),
              ),
            ),
            const SizedBox(height: 32),
            const Text(
              'Topics',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 24,
                fontWeight: FontWeight.bold,
                color: Colors.white,
              ),
            ),
            const SizedBox(height: 16),
            GridView.count(
              crossAxisCount: MediaQuery.of(context).size.width > 768 ? 3 : 2,
              crossAxisSpacing: 16,
              mainAxisSpacing: 16,
              shrinkWrap: true,
              physics: const NeverScrollableScrollPhysics(),
              children: [
                _buildCategoryCard('Getting Started', Icons.rocket_launch, 'Learn the basics of setting up your OHC app.'),
                _buildCategoryCard('My Store', Icons.storefront, 'Manage products, inventory, and your storefront.'),
                _buildCategoryCard('Payments', Icons.payment, 'Set up Stripe and manage deposits and payouts.'),
                _buildCategoryCard('AI Agents', Icons.smart_toy, 'Hire and configure your AI Swarm.'),
                _buildCategoryCard('Marketing', Icons.campaign, 'SEO, social media, and email campaigns.'),
                _buildCategoryCard('Account & Billing', Icons.account_circle, 'Manage your subscription and profile.'),
              ],
            ),
            const SizedBox(height: 32),
            const Text(
              'Video Tutorials',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 24,
                fontWeight: FontWeight.bold,
                color: Colors.white,
              ),
            ),
            const SizedBox(height: 16),
            _buildVideoTutorial('How to accept your first payment', '1:20'),
            _buildVideoTutorial('Setting up your AI Support Agent', '2:05'),
            _buildVideoTutorial('Customizing your storefront', '1:45'),
          ],
        ),
      ),
    );
  }
}
