import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class HelpCenterScreen extends StatelessWidget {
  const HelpCenterScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Help Center', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text('Browse by Topic', style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold)),
            const SizedBox(height: 16),
            GlassCard(
              child: Column(
                children: [
                  ListTile(
                    title: const Text('Getting Started'),
                    leading: const Icon(Icons.rocket_launch),
                    onTap: () {},
                  ),
                  const Divider(),
                  ListTile(
                    title: const Text('My Store'),
                    leading: const Icon(Icons.store),
                    onTap: () {},
                  ),
                  const Divider(),
                  ListTile(
                    title: const Text('Payments'),
                    leading: const Icon(Icons.payment),
                    onTap: () {},
                  ),
                ],
              ),
            ),
            const SizedBox(height: 32),
            const Text('Quick Links', style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold)),
            const SizedBox(height: 16),
            GlassCard(
              child: Column(
                children: [
                  ListTile(
                    title: const Text('API Documentation'),
                    leading: const Icon(Icons.code),
                    onTap: () => context.push('/help/api'),
                  ),
                  const Divider(),
                  ListTile(
                    title: const Text('Release Notes'),
                    leading: const Icon(Icons.new_releases),
                    onTap: () => context.push('/help/release-notes'),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
