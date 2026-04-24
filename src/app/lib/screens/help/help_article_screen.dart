import 'package:flutter/material.dart';

class HelpArticleScreen extends StatelessWidget {
  final String articleId;

  const HelpArticleScreen({super.key, required this.articleId});

  @override
  Widget build(BuildContext context) {
    // Basic mock content based on articleId
    String title = 'Help Article';
    String content = 'Detailed information about $articleId will appear here.';

    if (articleId == 'getting_started') {
      title = 'Getting Started';
      content = 'Welcome to One Human Corp! Getting started is easy. First, set up your business profile, then hire your first AI agent.';
    } else if (articleId == 'ai_agents') {
      title = 'AI Agents';
      content = 'Your AI agents work for you 24/7. Navigate to the Agents tab to see what they are currently working on.';
    }

    return Scaffold(
      appBar: AppBar(
        title: Text(title, style: const TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              title,
              style: const TextStyle(fontSize: 28, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
            ),
            const SizedBox(height: 24),
            Text(
              content,
              style: const TextStyle(fontSize: 16, height: 1.6, fontFamily: 'Inter'),
            ),
          ],
        ),
      ),
    );
  }
}
