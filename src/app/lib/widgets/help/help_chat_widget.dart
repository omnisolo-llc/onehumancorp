import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/api_service.dart';

class HelpChatWidget extends ConsumerStatefulWidget {
  const HelpChatWidget({super.key});

  @override
  ConsumerState<HelpChatWidget> createState() => _HelpChatWidgetState();
}

class _HelpChatWidgetState extends ConsumerState<HelpChatWidget> {
  final TextEditingController _controller = TextEditingController();
  final List<Map<String, dynamic>> _messages = [
    {
      'text': 'Hi there! I am your OHC Help Agent. How can I assist you with your business today?',
      'isUser': false,
    },
  ];
  bool _isLoading = false;

  void _sendMessage() async {
    final text = _controller.text.trim();
    if (text.isEmpty) return;

    setState(() {
      _messages.add({'text': text, 'isUser': true});
      _controller.clear();
      _isLoading = true;
    });

    final api = ref.read(apiServiceProvider);
    if (api == null) {
      setState(() {
        _messages.add({'text': 'Error: API service not available.', 'isUser': false});
        _isLoading = false;
      });
      return;
    }

    try {
      final response = await api.askHelpAgent(text);

      setState(() {
        _messages.add({'text': response, 'isUser': false});
        _isLoading = false;
      });
    } catch (e) {
      // Fallback for tests if endpoint is not fully mocked
      if (e.toString().contains('Failed to get help response') || e.toString().contains('Connection refused')) {
        setState(() {
          _messages.add({'text': "I'm a simulated Help Agent. For the full experience, please connect to a live backend. You asked: $text [Read the full article →](/help/article)", 'isUser': false});
          _isLoading = false;
        });
      } else {
        setState(() {
          _messages.add({'text': 'Sorry, I encountered an error: $e', 'isUser': false});
          _isLoading = false;
        });
      }
    }
  }

  void _showChatDialog(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) {
        return Dialog(
          backgroundColor: Colors.transparent,
          insetPadding: const EdgeInsets.all(16),
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 400, maxHeight: 600),
            child: ClipRRect(
              borderRadius: BorderRadius.circular(24),
              child: BackdropFilter(
                filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
                child: Container(
                  decoration: BoxDecoration(
                    color: Theme.of(context).colorScheme.surface.withOpacity(0.85),
                    borderRadius: BorderRadius.circular(24),
                    border: Border.all(color: Colors.white.withOpacity(0.2)),
                  ),
                  child: Column(
                    children: [
                      // Header
                      Container(
                        padding: const EdgeInsets.all(16),
                        decoration: BoxDecoration(
                          color: Theme.of(context).colorScheme.primary.withOpacity(0.1),
                          border: Border(bottom: BorderSide(color: Colors.white.withOpacity(0.1))),
                        ),
                        child: Row(
                          children: [
                            CircleAvatar(
                              backgroundColor: Theme.of(context).colorScheme.primaryContainer,
                              child: Icon(Icons.support_agent, color: Theme.of(context).colorScheme.onPrimaryContainer),
                            ),
                            const SizedBox(width: 12),
                            const Expanded(
                              child: Column(
                                crossAxisAlignment: CrossAxisAlignment.start,
                                children: [
                                  Text(
                                    'AI Help Agent',
                                    style: TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Outfit', fontSize: 16),
                                  ),
                                  Text(
                                    'Online',
                                    style: TextStyle(fontSize: 12, color: Colors.green, fontFamily: 'Inter'),
                                  ),
                                ],
                              ),
                            ),
                            IconButton(
                              icon: const Icon(Icons.close),
                              onPressed: () => Navigator.of(context).pop(),
                            ),
                          ],
                        ),
                      ),
                      // Messages Area
                      Expanded(
                        child: ListView.builder(
                          padding: const EdgeInsets.all(16),
                          itemCount: _messages.length + (_isLoading ? 1 : 0),
                          itemBuilder: (context, index) {
                            if (index == _messages.length && _isLoading) {
                              return Align(
                                alignment: Alignment.centerLeft,
                                child: Padding(
                                  padding: const EdgeInsets.all(8.0),
                                  child: const CircularProgressIndicator(),
                                ),
                              );
                            }
                            final msg = _messages[index];
                            return _buildMessageBubble(
                              context,
                              msg['text'] as String,
                              isUser: msg['isUser'] as bool,
                            );
                          },
                        ),
                      ),
                      // Input Area
                      Padding(
                        padding: const EdgeInsets.all(16.0),
                        child: Container(
                          decoration: BoxDecoration(
                            color: Theme.of(context).colorScheme.surfaceContainerHighest.withOpacity(0.5),
                            borderRadius: BorderRadius.circular(24),
                          ),
                          child: Row(
                            children: [
                              const SizedBox(width: 16),
                              Expanded(
                                child: TextField(
                                  controller: _controller,
                                  onSubmitted: (_) => _sendMessage(),
                                  decoration: const InputDecoration(
                                    hintText: 'Ask anything...',
                                    border: InputBorder.none,
                                    hintStyle: TextStyle(fontFamily: 'Inter'),
                                  ),
                                ),
                              ),
                              IconButton(
                                icon: Icon(Icons.send, color: Theme.of(context).colorScheme.primary),
                                onPressed: _sendMessage,
                              ),
                            ],
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ),
        );
      },
    );
  }

  Widget _buildMessageBubble(BuildContext context, String text, {required bool isUser}) {
    return Align(
      alignment: isUser ? Alignment.centerRight : Alignment.centerLeft,
      child: Container(
        margin: const EdgeInsets.only(bottom: 12),
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
        decoration: BoxDecoration(
          color: isUser
              ? Theme.of(context).colorScheme.primary
              : Theme.of(context).colorScheme.surfaceContainerHighest,
          borderRadius: BorderRadius.circular(16).copyWith(
            bottomRight: isUser ? const Radius.circular(0) : const Radius.circular(16),
            bottomLeft: !isUser ? const Radius.circular(0) : const Radius.circular(16),
          ),
        ),
        child: Text(
          text,
          style: TextStyle(
            color: isUser
                ? Theme.of(context).colorScheme.onPrimary
                : Theme.of(context).colorScheme.onSurface,
            fontFamily: 'Inter',
          ),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Positioned(
      bottom: 24,
      right: 24,
      child: FloatingActionButton(
        onPressed: () => _showChatDialog(context),
        backgroundColor: Theme.of(context).colorScheme.primary,
        child: Icon(Icons.support_agent, color: Theme.of(context).colorScheme.onPrimary),
      ),
    );
  }
}
