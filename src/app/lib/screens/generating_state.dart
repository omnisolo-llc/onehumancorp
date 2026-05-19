import 'package:flutter/material.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
import 'preview_edit.dart';

class GeneratingStateScreen extends StatefulWidget {
  final String description;
  const GeneratingStateScreen({super.key, required this.description});

  @override
  State<GeneratingStateScreen> createState() => _GeneratingStateScreenState();
}

class _GeneratingStateScreenState extends State<GeneratingStateScreen> {
  @override
  void initState() {
    super.initState();
    _generate();
  }

  Future<void> _generate() async {
    // In a real app, we'd fetch the token from a secure store
    try {
      final response = await http.post(
        Uri.parse('/api/v1/builder/generate'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({'description': widget.description}),
      );

      if (response.statusCode == 200) {
        final draft = jsonDecode(response.body);
        if (mounted) {
          Navigator.pushReplacement(
            context,
            MaterialPageRoute(
              builder: (context) => PreviewEditScreen(draft: draft),
            ),
          );
        }
      } else {
         // Handle error
         if (mounted) {
           ScaffoldMessenger.of(context).showSnackBar(
             const SnackBar(content: Text('Failed to generate storefront')),
           );
           Navigator.pop(context);
         }
      }
    } catch (e) {
      if (mounted) {
        // Fallback for demo if API is not available
        await Future.delayed(const Duration(seconds: 2));
        Navigator.pushReplacement(
          context,
          MaterialPageRoute(
            builder: (context) => PreviewEditScreen(draft: {
              "pages": [
                {
                  "title": "Home",
                  "path": "/",
                  "blocks": [
                    {"block_type": "HeroBlock", "content": {"headline": "Welcome"}},
                    {"block_type": "ProductGridBlock", "content": {"items": []}}
                  ],
                  "seo_metadata": {}
                }
              ]
            }),
          ),
        );
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const CircularProgressIndicator(),
            const SizedBox(height: 24),
            const Text(
              'The Promoter AI is assembling your site...',
              style: TextStyle(fontSize: 16, fontWeight: FontWeight.w500),
            ),
            const SizedBox(height: 8),
            Text(
              'Analyzing: "${widget.description}"',
              style: const TextStyle(color: Colors.grey, fontSize: 12),
              textAlign: TextAlign.center,
            ),
          ],
        ),
      ),
    );
  }
}
