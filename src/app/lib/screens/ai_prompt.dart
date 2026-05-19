import 'package:flutter/material.dart';
import 'generating_state.dart';

class AIPromptScreen extends StatefulWidget {
  const AIPromptScreen({super.key});

  @override
  State<AIPromptScreen> createState() => _AIPromptScreenState();
}

class _AIPromptScreenState extends State<AIPromptScreen> {
  final TextEditingController _controller = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('New Storefront')),
      body: Padding(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Describe your business in one sentence',
              style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 8),
            const Text(
              'e.g., "I bake custom vegan cakes in Austin"',
              style: TextStyle(color: Colors.grey),
            ),
            const SizedBox(height: 24),
            TextField(
              controller: _controller,
              maxLines: 3,
              decoration: const InputDecoration(
                border: OutlineInputBorder(),
                hintText: 'Enter your business description...',
              ),
            ),
            const Spacer(),
            SizedBox(
              width: double.infinity,
              height: 50,
              child: ElevatedButton(
                onPressed: () {
                  Navigator.push(
                    context,
                    MaterialPageRoute(
                      builder: (context) => GeneratingStateScreen(
                        description: _controller.text,
                      ),
                    ),
                  );
                },
                child: const Text('Generate with AI'),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
