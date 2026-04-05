import 'dart:async';

import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/centrifuge_service.dart';
import 'package:uuid/uuid.dart';

/// Default room used when no specific room is selected.
const _kDefaultRoom = 'general';

/// Real-time messages accumulated from the Centrifuge subscription.
final _messagesProvider = StateProvider<List<CentrifugeMessage>>(
  (ref) => const [],
);

/// Active room ID.
final _roomProvider = StateProvider<String>((ref) => _kDefaultRoom);

class ChatScreen extends ConsumerStatefulWidget {
  const ChatScreen({super.key});

  @override
  ConsumerState<ChatScreen> createState() => _ChatScreenState();
}

class _ChatScreenState extends ConsumerState<ChatScreen> {
  final _ctrl = TextEditingController();
  final _scrollCtrl = ScrollController();
  final Set<String> _seenMessageIds = {};
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
      final room = ref.read(_roomProvider);
      _sub = svc.subscribe(room).listen(_onMessage);
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(SnackBar(content: Text('Chat connection failed: $e')));
      }
    }
  }

  void _onMessage(CentrifugeMessage msg) {
    ref.read(_messagesProvider.notifier).update((msgs) => [...msgs, msg]);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (_scrollCtrl.hasClients) {
        _scrollCtrl.animateTo(
          _scrollCtrl.position.maxScrollExtent,
          duration: const Duration(milliseconds: 200),
          curve: Curves.easeOut,
        );
      }
    });
  }

  Future<void> _send() async {
    final text = _ctrl.text.trim();
    if (text.isEmpty) return;
    setState(() => _sending = true);
    try {
      final room = ref.read(_roomProvider);
      await _service?.publish(room, text);
      final user = ref.read(authStateProvider).valueOrNull;
      // Optimistically add the local message so the sender sees it immediately.
      final msg = CentrifugeMessage(
        id: const Uuid().v4(),
        channelId: room,
        authorId: user?.id ?? '',
        authorName: user?.name ?? 'You',
        body: text,
        sentAt: DateTime.now(),
      );
      _onMessage(msg);
      _ctrl.clear();
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(SnackBar(content: Text('Send failed: $e')));
      }
    } finally {
      if (mounted) setState(() => _sending = false);
    }
  }

  @override
  void dispose() {
    _sub?.cancel();
    _service?.disconnect();
    _ctrl.dispose();
    _scrollCtrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final messages = ref.watch(_messagesProvider);
    final room = ref.watch(_roomProvider);
    final user = ref.watch(authStateProvider).valueOrNull;

    return Scaffold(
      appBar: AppBar(
        title: Text('Chat — #$room'),
        actions: [
          Semantics(
            label: 'Switch to a different chat room',
            child: IconButton(
              icon: const Icon(Icons.meeting_room),
              tooltip: 'Switch room',
              onPressed: () => _showRoomPicker(context),
            ),
          ),
        ],
      ),
      body: Column(
        children: [
          Expanded(
            child: messages.isEmpty
                ? const Center(child: Text('No messages yet. Say hello!'))
                : ListView.builder(
                    controller: _scrollCtrl,
                    padding: const EdgeInsets.all(16),
                    itemCount: messages.length,
                    itemBuilder: (_, i) {
                      final m = messages[i];
                      final isMe = m.authorId == user?.id;
                      final isNew = !_seenMessageIds.contains(m.id);
                      if (isNew) {
                        _seenMessageIds.add(m.id);
                      }
                      return _AnimatedMessageBubble(
                        message: m,
                        isMe: isMe,
                        shouldAnimate: isNew,
                      );
                    },
                  ),
          ),
          _InputBar(controller: _ctrl, sending: _sending, onSend: _send),
        ],
      ),
    );
  }

  void _showRoomPicker(BuildContext context) {
    showDialog<String>(
      context: context,
      builder: (_) => _RoomPickerDialog(current: ref.read(_roomProvider)),
    ).then((room) async {
      if (room == null || room == ref.read(_roomProvider)) return;
      // Unsubscribe from old room, subscribe to new one.
      final oldRoom = ref.read(_roomProvider);
      await _sub?.cancel();
      await _service?.unsubscribe(oldRoom);
      ref.read(_roomProvider.notifier).state = room;
      ref.read(_messagesProvider.notifier).state = const [];
      _sub = _service?.subscribe(room).listen(_onMessage);
    });
  }
}

// ── Widgets ────────────────────────────────────────────────────────────────

class _AnimatedMessageBubble extends StatefulWidget {
  final CentrifugeMessage message;
  final bool isMe;
  final bool shouldAnimate;

  const _AnimatedMessageBubble({
    required this.message,
    required this.isMe,
    required this.shouldAnimate,
  });

  @override
  State<_AnimatedMessageBubble> createState() => _AnimatedMessageBubbleState();
}

class _AnimatedMessageBubbleState extends State<_AnimatedMessageBubble>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<Offset> _slideAnimation;
  late Animation<double> _fadeAnimation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 400),
    );
    _slideAnimation = Tween<Offset>(
      begin: widget.isMe ? const Offset(0.2, 0) : const Offset(-0.2, 0),
      end: Offset.zero,
    ).animate(CurvedAnimation(parent: _controller, curve: Curves.easeOutQuart));
    _fadeAnimation = Tween<double>(
      begin: 0.0,
      end: 1.0,
    ).animate(CurvedAnimation(parent: _controller, curve: Curves.easeOut));

    if (widget.shouldAnimate) {
      _controller.forward();
    } else {
      _controller.value = 1.0;
    }
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final bgColor = isMe
        ? cs.primaryContainer.withValues(alpha: 0.8)
        : cs.surfaceContainerHighest.withValues(alpha: 0.8);

    final borderRadius = BorderRadius.only(
      topLeft: const Radius.circular(16),
      topRight: const Radius.circular(16),
      bottomLeft: Radius.circular(isMe ? 16 : 0),
      bottomRight: Radius.circular(isMe ? 0 : 16),
    );

    return Align(
      alignment: widget.isMe ? Alignment.centerRight : Alignment.centerLeft,
      child: SlideTransition(
        position: _slideAnimation,
        child: FadeTransition(
          opacity: _fadeAnimation,
          child: Container(
            margin: const EdgeInsets.only(bottom: 8),
            constraints: BoxConstraints(
              maxWidth: MediaQuery.of(context).size.width * 0.75,
            ),
            child: ClipRRect(
              borderRadius: borderRadius,
              child: BackdropFilter(
                filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                child: Container(
                  padding: const EdgeInsets.symmetric(
                    horizontal: 14,
                    vertical: 10,
                  ),
                  decoration: BoxDecoration(
                    color: bgColor,
                    border: Border.all(
                      color: Colors.white.withValues(alpha: 0.1),
                    ),
                  ),
                  child: Column(
                    crossAxisAlignment: widget.isMe
                        ? CrossAxisAlignment.end
                        : CrossAxisAlignment.start,
                    children: [
                      if (!widget.isMe)
                        Text(
                          widget.message.authorName,
                          style: TextStyle(
                            fontWeight: FontWeight.bold,
                            fontSize: 12,
                            color: cs.onSurfaceVariant,
                            fontFamily: 'Outfit',
                          ),
                        ),
                      Text(
                        widget.message.body,
                        style: TextStyle(
                          color: cs.onSurface,
                          fontFamily: 'Inter',
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }

  bool get isMe => widget.isMe;
}

class _InputBar extends StatelessWidget {
  final TextEditingController controller;
  final bool sending;
  final VoidCallback onSend;

  const _InputBar({
    required this.controller,
    required this.sending,
    required this.onSend,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(12),
      child: Row(
        children: [
          Expanded(
            child: TextField(
              controller: controller,
              decoration: const InputDecoration(
                hintText: 'Type a message…',
                border: OutlineInputBorder(),
                isDense: true,
              ),
              onSubmitted: (_) => onSend(),
            ),
          ),
          const SizedBox(width: 8),
          Semantics(
            label: 'Send chat message',
            child: IconButton.filled(
              tooltip: 'Send message',
              icon: sending
                  ? const SizedBox(
                      width: 20,
                      height: 20,
                      child: CircularProgressIndicator(strokeWidth: 2),
                    )
                  : const Icon(Icons.send),
              onPressed: sending ? null : onSend,
            ),
          ),
        ],
      ),
    );
  }
}

class _RoomPickerDialog extends StatefulWidget {
  final String current;
  const _RoomPickerDialog({required this.current});

  @override
  State<_RoomPickerDialog> createState() => _RoomPickerDialogState();
}

class _RoomPickerDialogState extends State<_RoomPickerDialog> {
  late final TextEditingController _ctrl;

  @override
  void initState() {
    super.initState();
    _ctrl = TextEditingController(text: widget.current);
  }

  @override
  void dispose() {
    _ctrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Switch Chat Room'),
      content: TextField(
        controller: _ctrl,
        decoration: const InputDecoration(
          labelText: 'Room ID',
          hintText: 'e.g. general, support, sales',
          border: OutlineInputBorder(),
        ),
        autofocus: true,
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: const Text('Cancel'),
        ),
        FilledButton(
          onPressed: () {
            final val = _ctrl.text.trim();
            if (val.isNotEmpty) Navigator.pop(context, val);
          },
          child: const Text('Switch'),
        ),
      ],
    );
  }
}
