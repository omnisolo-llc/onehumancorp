import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/ai_help_chat_widget.dart';

class HelpArticleScreen extends StatelessWidget {
  final String articleId;

  const HelpArticleScreen({super.key, required this.articleId});

  @override
  Widget build(BuildContext context) {
    // In a real app, you would fetch the article content based on articleId
    // and render it using a Markdown widget or similar.
    // For this implementation, we will use a placeholder.

    String title = 'Help Article';
    String content = 'Content for $articleId';

    if (articleId == 'getting-started') {
      title = 'Getting Started';
      content = 'Welcome to OneHumanCorp! Let\'s get your business up and running.';
    } else if (articleId == 'my-store') {
      title = 'My Store';
      content = 'Learn how to add products, manage inventory, and customize your storefront.';
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
              style: const TextStyle(
                fontFamily: 'Outfit',
                fontSize: 28,
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 16),
            Text(
              content,
              style: const TextStyle(
                fontFamily: 'Inter',
                fontSize: 16,
                height: 1.5,
              ),
            ),

            // Mocked Backend Metadata Fetch
            FutureBuilder<String?>(
              future: Future.value(articleId == 'getting-started' ? 'https://backend.example.com/videos/1.mp4' : null),
              builder: (context, snapshot) {
                if (snapshot.connectionState == ConnectionState.waiting || !snapshot.hasData || snapshot.data == null) {
                  return const SizedBox.shrink();
                }
                return Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const SizedBox(height: 24),
                    const Text(
                      'Video Tutorial',
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 20,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    const SizedBox(height: 16),
                    AspectRatio(
                      aspectRatio: 9 / 16, // Mobile portrait-optimized player
                      child: Container(
                        decoration: BoxDecoration(
                          color: Colors.black87,
                          borderRadius: BorderRadius.circular(16),
                        ),
                        child: const Center(
                          child: Icon(
                            Icons.play_circle_fill,
                            color: Colors.white,
                            size: 64,
                          ),
                        ),
                      ),
                    ),
                  ],
                );
              },
            ),
            const SizedBox(height: 32),
            const Text(
              'Need more help?',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 20,
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 16),
            ElevatedButton.icon(
              onPressed: () {
                showModalBottomSheet(
                  context: context,
                  isScrollControlled: true,
                  backgroundColor: Colors.transparent,
                  builder: (context) => const ChatBottomSheet(),
                );
              },
              icon: const Icon(Icons.chat),
              label: const Text('Ask our AI Support Agent'),
            ),
          ],
        ),
      ),
    );
  }
}
