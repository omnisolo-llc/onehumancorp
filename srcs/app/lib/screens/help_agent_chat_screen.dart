import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/centrifuge_service.dart';
import 'package:uuid/uuid.dart';

const _kHelpDeskRoom = 'help_desk_ai';

class HelpAgentChatScreen extends ConsumerStatefulWidget {
  const HelpAgentChatScreen({super.key});

  @override
  ConsumerState<HelpAgentChatScreen> createState() => _HelpAgentChatScreenState();
}

class _HelpAgentChatScreenState extends ConsumerState<HelpAgentChatScreen> {
  final TextEditingController _messageController = TextEditingController();
  final ScrollController _scrollController = ScrollController();

  final List<CentrifugeMessage> _messages = [
    CentrifugeMessage(
      id: 'init-msg',
      channelId: _kHelpDeskRoom,
      authorId: 'ai-help-agent',
      authorName: 'Help Agent',
      body: 'Hi there! I am your OHC Help Agent. How can I assist you with your business today?',
      sentAt: DateTime.now(),
    )
  ];

  bool _sending = false;
  StreamSubscription<CentrifugeMessage>? _sub;
  CentrifugeService? _service;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) => _connect());
  }

  Future<void> _connect() async {
    final svc = ref.read(centrifugeServiceProvider);
    if (svc == null) return;
    _service = svc;
    try {
      await svc.connect();
      _sub = svc.subscribe(_kHelpDeskRoom).listen(_onMessage);
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Help chat connection failed: $e')),
        );
      }
    }
  }

  void _onMessage(CentrifugeMessage msg) {
    if (!mounted) return;
    // Skip if we already optimistically added it
    if (_messages.any((m) => m.id == msg.id)) return;

    setState(() {
      _messages.add(msg);
    });

    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (_scrollController.hasClients) {
        _scrollController.animateTo(
          _scrollController.position.maxScrollExtent,
          duration: const Duration(milliseconds: 200),
          curve: Curves.easeOut,
        );
      }
    });
  }

  Future<void> _sendMessage() async {
    if (_messageController.text.trim().isEmpty) return;

    final text = _messageController.text.trim();
    setState(() => _sending = true);

    try {
      final user = ref.read(authStateProvider).valueOrNull;
      final msgId = const Uuid().v4();

      // Optimistically add message
      final msg = CentrifugeMessage(
        id: msgId,
        channelId: _kHelpDeskRoom,
        authorId: user?.id ?? 'local-user',
        authorName: user?.name ?? 'You',
        body: text,
        sentAt: DateTime.now(),
      );

      setState(() {
        _messages.add(msg);
        _messageController.clear();
      });

      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (_scrollController.hasClients) {
          _scrollController.animateTo(
            _scrollController.position.maxScrollExtent,
            duration: const Duration(milliseconds: 200),
            curve: Curves.easeOut,
          );
        }
      });

      await _service?.publish(_kHelpDeskRoom, text);
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Failed to send message: $e')),
        );
      }
    } finally {
      if (mounted) setState(() => _sending = false);
    }
  }

  @override
  void dispose() {
    _sub?.cancel();
    _service?.unsubscribe(_kHelpDeskRoom);
    _messageController.dispose();
    _scrollController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final user = ref.watch(authStateProvider).valueOrNull;

    return Scaffold(
      appBar: AppBar(
        title: const Text('Help Agent'),
      ),
      body: Column(
        children: [
          Expanded(
            child: ListView.builder(
              controller: _scrollController,
              padding: const EdgeInsets.all(16),
              itemCount: _messages.length,
              itemBuilder: (context, index) {
                final message = _messages[index];
                final isUser = message.authorId == user?.id || message.authorId == 'local-user';

                return Align(
                  alignment: isUser ? Alignment.centerRight : Alignment.centerLeft,
                  child: Container(
                    margin: const EdgeInsets.only(bottom: 16),
                    padding: const EdgeInsets.all(16),
                    decoration: BoxDecoration(
                      color: isUser ? Theme.of(context).primaryColor : Colors.grey[200],
                      borderRadius: BorderRadius.circular(16).copyWith(
                        bottomRight: isUser ? const Radius.circular(0) : const Radius.circular(16),
                        bottomLeft: !isUser ? const Radius.circular(0) : const Radius.circular(16),
                      ),
                    ),
                    constraints: BoxConstraints(
                      maxWidth: MediaQuery.of(context).size.width * 0.75,
                    ),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        if (!isUser && message.authorId != 'init-msg')
                           Text(
                            message.authorName,
                            style: const TextStyle(
                              color: Colors.black54,
                              fontSize: 12,
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                        Text(
                          message.body,
                          style: TextStyle(
                            color: isUser ? Colors.white : Colors.black87,
                            fontSize: 16,
                          ),
                        ),
                        // Link extraction simulation if needed could be done here,
                        // but normally backend sends formatted text.
                        if (!isUser && message.body.contains('http'))
                           InkWell(
                                onTap: () {
                                  showDialog(
                                    context: context,
                                    builder: (context) => AlertDialog(
                                      title: const Text('Article link'),
                                      content: const Text('Navigating to article...'),
                                      actions: [
                                        TextButton(onPressed: () => Navigator.pop(context), child: const Text('Close'))
                                      ],
                                    ),
                                  );
                                },
                                child: Text(
                                  'Read the full article \u2192',
                                  style: TextStyle(
                                    color: Theme.of(context).primaryColor,
                                    fontWeight: FontWeight.bold,
                                  ),
                                ),
                              ),
                      ],
                    ),
                  ),
                );
              },
            ),
          ),
          Container(
            padding: const EdgeInsets.all(16),
            decoration: BoxDecoration(
              color: Theme.of(context).cardColor,
              boxShadow: [
                BoxShadow(
                  color: Colors.black.withValues(alpha: 0.05),
                  offset: const Offset(0, -4),
                  blurRadius: 16,
                ),
              ],
            ),
            child: Row(
              children: [
                Expanded(
                  child: TextField(
                    controller: _messageController,
                    decoration: InputDecoration(
                      hintText: 'Type your question...',
                      border: OutlineInputBorder(
                        borderRadius: BorderRadius.circular(24),
                        borderSide: BorderSide.none,
                      ),
                      filled: true,
                      fillColor: Colors.grey[100],
                      contentPadding: const EdgeInsets.symmetric(horizontal: 20, vertical: 10),
                    ),
                    onSubmitted: (_) => _sendMessage(),
                  ),
                ),
                const SizedBox(width: 8),
                CircleAvatar(
                  backgroundColor: Theme.of(context).primaryColor,
                  child: IconButton(
                    icon: _sending
                      ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white))
                      : const Icon(Icons.send, color: Colors.white),
                    onPressed: _sending ? null : _sendMessage,
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
