import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/api_service.dart';

class HelpChatScreen extends ConsumerStatefulWidget {
  const HelpChatScreen({super.key});

  @override
  ConsumerState<HelpChatScreen> createState() => _HelpChatScreenState();
}

class _HelpChatScreenState extends ConsumerState<HelpChatScreen> {
  final TextEditingController _controller = TextEditingController();
  final List<Map<String, String>> _messages = [
    {'role': 'ai', 'content': 'Hi there! I am the OHC Help Agent. Ask me anything about setting up your business, managing your store, or using AI agents.'}
  ];
  bool _isLoading = false;

  Future<void> _sendMessage() async {
    if (_controller.text.trim().isEmpty) return;

    final query = _controller.text;
    setState(() {
      _messages.add({'role': 'user', 'content': query});
      _isLoading = true;
    });
    _controller.clear();

    try {
      final api = ref.read(apiServiceProvider);
      if (api != null) {
        final res = await api.client.post(
          Uri.parse('${api.baseUrl}/api/help/chat'),
          headers: {'Content-Type': 'application/json'},
          body: jsonEncode({'query': query}),
        );
        if (res.statusCode == 200) {
          final data = jsonDecode(res.body);
          setState(() {
            _messages.add({'role': 'ai', 'content': data['reply'] ?? 'Sorry, I do not understand.'});
            if (data['article_link'] != null) {
              _messages.add({'role': 'link', 'content': data['article_link']});
            }
          });
        } else {
          throw Exception('Failed to get answer');
        }
      }
    } catch (e) {
      setState(() {
        _messages.add({'role': 'ai', 'content': 'Sorry, I encountered an error connecting to the Help Center.'});
      });
    } finally {
      setState(() {
        _isLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('AI Help Chat', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold))),
      body: Column(
        children: [
          Expanded(
            child: ListView.builder(
              padding: const EdgeInsets.all(16),
              itemCount: _messages.length,
              itemBuilder: (context, index) {
                final msg = _messages[index];
                if (msg['role'] == 'link') {
                  return Align(
                    alignment: Alignment.centerLeft,
                    child: Padding(
                      padding: const EdgeInsets.only(left: 48, top: 4, bottom: 16),
                      child: TextButton.icon(
                        onPressed: () {}, // Future implementation
                        icon: const Icon(Icons.article),
                        label: const Text('Read the full article →'),
                      ),
                    ),
                  );
                }

                final isUser = msg['role'] == 'user';
                return Align(
                  alignment: isUser ? Alignment.centerRight : Alignment.centerLeft,
                  child: Container(
                    margin: const EdgeInsets.only(bottom: 16),
                    constraints: BoxConstraints(maxWidth: MediaQuery.of(context).size.width * 0.75),
                    child: GlassCard(
                      child: Padding(
                        padding: const EdgeInsets.all(12),
                        child: Text(msg['content'] ?? ''),
                      ),
                    ),
                  ),
                );
              },
            ),
          ),
          if (_isLoading) const Padding(padding: EdgeInsets.all(8.0), child: CircularProgressIndicator()),
          Padding(
            padding: const EdgeInsets.all(16.0),
            child: Row(
              children: [
                Expanded(
                  child: TextField(
                    controller: _controller,
                    decoration: InputDecoration(
                      hintText: 'Type your question...',
                      border: OutlineInputBorder(borderRadius: BorderRadius.circular(24)),
                      contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                    ),
                    onSubmitted: (_) => _sendMessage(),
                  ),
                ),
                const SizedBox(width: 8),
                IconButton.filled(
                  icon: const Icon(Icons.send),
                  onPressed: _sendMessage,
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
