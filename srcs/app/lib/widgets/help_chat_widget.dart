import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:ohc_app/services/help_service.dart';
import 'package:go_router/go_router.dart';

class HelpChatWidget extends ConsumerStatefulWidget {
  const HelpChatWidget({super.key});

  @override
  ConsumerState<HelpChatWidget> createState() => _HelpChatWidgetState();
}

class _HelpChatWidgetState extends ConsumerState<HelpChatWidget> {
  bool _isOpen = false;
  final TextEditingController _controller = TextEditingController();
  final List<Map<String, String>> _messages = [];
  bool _isTyping = false;

  @override
  void initState() {
    super.initState();
    _messages.add({'role': 'assistant', 'content': 'Hi! How can I help you with OneHumanCorp today?'});
  }

  void _sendMessage() async {
    if (_controller.text.isEmpty) return;
    final userMsg = _controller.text;
    setState(() {
      _messages.add({'role': 'user', 'content': userMsg});
      _controller.clear();
      _isTyping = true;
    });

    final replyData = await ref.read(helpServiceProvider).getHelpChatReply(userMsg);

    if (mounted) {
      setState(() {
        _isTyping = false;
        _messages.add({
          'role': 'assistant',
          'content': replyData['reply'] ?? 'I am sorry, I am having trouble connecting.',
          'link': replyData['link'] ?? '',
        });
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        AnimatedPositioned(
          duration: const Duration(milliseconds: 400),
          curve: Curves.easeOutQuart,
          right: 16,
          bottom: _isOpen ? 80 : -500,
          child: AnimatedOpacity(
            duration: const Duration(milliseconds: 300),
            opacity: _isOpen ? 1.0 : 0.0,
            child: SizedBox(
              width: 320,
              height: 450,
              child: GlassCard(
                child: Column(
                  children: [
                    Container(
                      padding: const EdgeInsets.all(16),
                      decoration: BoxDecoration(
                        border: Border(bottom: BorderSide(color: Theme.of(context).dividerColor)),
                      ),
                      child: Row(
                        children: [
                          const Icon(Icons.psychology, color: Colors.cyanAccent),
                          const SizedBox(width: 8),
                          const Text('OHC Help Assistant', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
                          const Spacer(),
                          IconButton(icon: const Icon(Icons.close), onPressed: () => setState(() => _isOpen = false)),
                        ],
                      ),
                    ),
                    Expanded(
                      child: ListView.builder(
                        padding: const EdgeInsets.all(16),
                        itemCount: _messages.length,
                        itemBuilder: (context, index) {
                          final m = _messages[index];
                          final isUser = m['role'] == 'user';
                          return Align(
                            alignment: isUser ? Alignment.centerRight : Alignment.centerLeft,
                            child: Container(
                              margin: const EdgeInsets.only(bottom: 12),
                              padding: const EdgeInsets.all(12),
                              decoration: BoxDecoration(
                                color: isUser ? Theme.of(context).colorScheme.primary.withValues(alpha: 0.2) : Colors.white10,
                                borderRadius: BorderRadius.circular(12),
                              ),
                              child: Column(
                                crossAxisAlignment: CrossAxisAlignment.start,
                                children: [
                                  Text(m['content']!, style: const TextStyle(fontFamily: 'Inter', fontSize: 13)),
                                  if (m['link'] != null && m['link']!.isNotEmpty)
                                    Padding(
                                      padding: const EdgeInsets.only(top: 8.0),
                                      child: TextButton(
                                        onPressed: () => context.push(m['link']!),
                                        child: const Text('Read Article →', style: TextStyle(fontSize: 12, color: Colors.cyanAccent)),
                                      ),
                                    ),
                                ],
                              ),
                            ),
                          );
                        },
                      ),
                    ),
                    if (_isTyping)
                      const Padding(
                        padding: EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                        child: Align(alignment: Alignment.centerLeft, child: Text('Assistant is typing...', style: TextStyle(fontSize: 11, fontStyle: FontStyle.italic))),
                      ),
                    Padding(
                      padding: const EdgeInsets.all(16.0),
                      child: Row(
                        children: [
                          Expanded(
                            child: TextField(
                              controller: _controller,
                              decoration: const InputDecoration(hintText: 'Ask anything...', border: InputBorder.none),
                              onSubmitted: (_) => _sendMessage(),
                            ),
                          ),
                          IconButton(icon: const Icon(Icons.send), onPressed: _sendMessage),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
        Positioned(
          right: 16,
          bottom: 16,
          child: FloatingActionButton(
            key: const ValueKey('help_fab'),
            onPressed: () => setState(() => _isOpen = !_isOpen),
            backgroundColor: Theme.of(context).colorScheme.primary,
            child: Icon(_isOpen ? Icons.close : Icons.help_outline),
          ),
        ),
      ],
    );
  }
}
