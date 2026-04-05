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
          duration: const Duration(milliseconds: 300),
          curve: Curves.easeOutCubic,
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
    final colors = Theme.of(context).colorScheme;

    return Scaffold(
      appBar: AppBar(
        title: Text(
          'Chat — #$room',
          style: const TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold),
        ),
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
                ? Center(
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        Icon(Icons.chat_bubble_outline, size: 64, color: colors.outline),
                        const SizedBox(height: 16),
                        Text(
                          'No messages yet. Say hello!',
                          style: TextStyle(
                            fontFamily: 'Inter',
                            color: colors.onSurfaceVariant,
                            fontSize: 16,
                          ),
                        ),
                      ],
                    ),
                  )
                : ListView.builder(
                    controller: _scrollCtrl,
                    padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 24),
                    itemCount: messages.length,
                    itemBuilder: (_, i) {
                      final m = messages[i];
                      final isMe = m.authorId == user?.id;
                      return _AnimatedMessageBubble(
                        key: ValueKey(m.id),
                        message: m,
                        isMe: isMe,
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

  const _AnimatedMessageBubble({super.key, required this.message, required this.isMe});

  @override
  State<_AnimatedMessageBubble> createState() => _AnimatedMessageBubbleState();
}

class _AnimatedMessageBubbleState extends State<_AnimatedMessageBubble> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<Offset> _slideAnimation;
  late Animation<double> _fadeAnimation;
  late Animation<double> _scaleAnimation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 400),
    );
    _slideAnimation = Tween<Offset>(
      begin: Offset(widget.isMe ? 0.2 : -0.2, 0),
      end: Offset.zero,
    ).animate(CurvedAnimation(parent: _controller, curve: Curves.easeOutCubic));
    _fadeAnimation = Tween<double>(begin: 0.0, end: 1.0)
        .animate(CurvedAnimation(parent: _controller, curve: Curves.easeOut));
    _scaleAnimation = Tween<double>(begin: 0.9, end: 1.0)
        .animate(CurvedAnimation(parent: _controller, curve: Curves.easeOutBack));

    _controller.forward();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;

    // Glassmorphism colors
    final bgColor = widget.isMe
        ? cs.primaryContainer.withValues(alpha: 0.4)
        : cs.surfaceContainerHighest.withValues(alpha: 0.4);

    final borderColor = widget.isMe
        ? cs.primary.withValues(alpha: 0.2)
        : cs.outlineVariant.withValues(alpha: 0.2);

    return SlideTransition(
      position: _slideAnimation,
      child: FadeTransition(
        opacity: _fadeAnimation,
        child: ScaleTransition(
          scale: _scaleAnimation,
          child: Align(
            alignment: widget.isMe ? Alignment.centerRight : Alignment.centerLeft,
            child: Container(
              margin: const EdgeInsets.only(bottom: 12),
              constraints: BoxConstraints(
                maxWidth: MediaQuery.of(context).size.width * 0.75,
              ),
              child: ClipRRect(
                borderRadius: BorderRadius.only(
                  topLeft: const Radius.circular(20),
                  topRight: const Radius.circular(20),
                  bottomLeft: Radius.circular(widget.isMe ? 20 : 4),
                  bottomRight: Radius.circular(widget.isMe ? 4 : 20),
                ),
                child: BackdropFilter(
                  filter: ImageFilter.compose(
                    outer: const ColorFilter.matrix(<double>[
                      1.168, -0.153, -0.015, 0, 0,
                      -0.046, 1.061, -0.015, 0, 0,
                      -0.046, -0.152, 1.198, 0, 0,
                      0, 0, 0, 1, 0,
                    ]),
                    inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                  ),
                  child: Container(
                    padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                    decoration: BoxDecoration(
                      color: bgColor,
                      border: Border.all(color: borderColor),
                    ),
                    child: Column(
                      crossAxisAlignment:
                          widget.isMe ? CrossAxisAlignment.end : CrossAxisAlignment.start,
                      children: [
                        if (!widget.isMe) ...[
                          Text(
                            widget.message.authorName,
                            style: TextStyle(
                              fontFamily: 'Outfit',
                              fontWeight: FontWeight.bold,
                              fontSize: 13,
                              color: cs.primary,
                            ),
                          ),
                          const SizedBox(height: 4),
                        ],
                        Text(
                          widget.message.body,
                          style: TextStyle(
                            fontFamily: 'Inter',
                            fontSize: 15,
                            color: cs.onSurface,
                            height: 1.4,
                          ),
                        ),
                        const SizedBox(height: 4),
                        Text(
                          '${widget.message.sentAt.hour.toString().padLeft(2, '0')}:${widget.message.sentAt.minute.toString().padLeft(2, '0')}',
                          style: TextStyle(
                            fontFamily: 'Inter',
                            fontSize: 10,
                            color: cs.onSurfaceVariant.withValues(alpha: 0.7),
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
      ),
    );
  }
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
    final cs = Theme.of(context).colorScheme;

    return Container(
      padding: EdgeInsets.only(
        left: 16,
        right: 16,
        top: 16,
        bottom: MediaQuery.of(context).padding.bottom + 16,
      ),
      decoration: BoxDecoration(
        color: cs.surface.withValues(alpha: 0.8),
        border: Border(
          top: BorderSide(
            color: cs.outlineVariant.withValues(alpha: 0.2),
          ),
        ),
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(24),
        child: BackdropFilter(
          filter: ImageFilter.compose(
            outer: const ColorFilter.matrix(<double>[
              1.168, -0.153, -0.015, 0, 0,
              -0.046, 1.061, -0.015, 0, 0,
              -0.046, -0.152, 1.198, 0, 0,
              0, 0, 0, 1, 0,
            ]),
            inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
          ),
          child: Container(
            decoration: BoxDecoration(
              color: cs.surfaceContainerHighest.withValues(alpha: 0.3),
              borderRadius: BorderRadius.circular(24),
              border: Border.all(color: cs.outlineVariant.withValues(alpha: 0.5)),
            ),
            child: Row(
              children: [
                Expanded(
                  child: TextField(
                    controller: controller,
                    style: const TextStyle(fontFamily: 'Inter'),
                    decoration: InputDecoration(
                      hintText: 'Type a message…',
                      hintStyle: TextStyle(
                        fontFamily: 'Inter',
                        color: cs.onSurfaceVariant.withValues(alpha: 0.7),
                      ),
                      border: InputBorder.none,
                      contentPadding: const EdgeInsets.symmetric(horizontal: 20, vertical: 14),
                      isDense: true,
                    ),
                    onSubmitted: (_) => onSend(),
                  ),
                ),
                Padding(
                  padding: const EdgeInsets.only(right: 8.0),
                  child: Semantics(
                    label: 'Send chat message',
                    child: IconButton.filled(
                      tooltip: 'Send message',
                      style: IconButton.styleFrom(
                        backgroundColor: cs.primary,
                        foregroundColor: cs.onPrimary,
                        shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(16),
                        ),
                      ),
                      icon: sending
                          ? SizedBox(
                              width: 20,
                              height: 20,
                              child: CircularProgressIndicator(
                                strokeWidth: 2,
                                color: cs.onPrimary,
                              ),
                            )
                          : const Icon(Icons.send_rounded, size: 20),
                      onPressed: sending ? null : onSend,
                    ),
                  ),
                ),
              ],
            ),
          ),
        ),
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
    final cs = Theme.of(context).colorScheme;

    return Dialog(
      backgroundColor: Colors.transparent,
      elevation: 0,
      child: ClipRRect(
        borderRadius: BorderRadius.circular(28),
        child: BackdropFilter(
          filter: ImageFilter.compose(
            outer: const ColorFilter.matrix(<double>[
              1.168, -0.153, -0.015, 0, 0,
              -0.046, 1.061, -0.015, 0, 0,
              -0.046, -0.152, 1.198, 0, 0,
              0, 0, 0, 1, 0,
            ]),
            inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
          ),
          child: Container(
            padding: const EdgeInsets.all(24),
            decoration: BoxDecoration(
              color: cs.surface.withValues(alpha: 0.7),
              borderRadius: BorderRadius.circular(28),
              border: Border.all(color: cs.outlineVariant.withValues(alpha: 0.5)),
            ),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Row(
                  children: [
                    Container(
                      padding: const EdgeInsets.all(10),
                      decoration: BoxDecoration(
                        color: cs.primaryContainer,
                        borderRadius: BorderRadius.circular(12),
                      ),
                      child: Icon(Icons.meeting_room, color: cs.onPrimaryContainer),
                    ),
                    const SizedBox(width: 16),
                    Expanded(
                      child: Text(
                        'Switch Chat Room',
                        style: TextStyle(
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                          fontFamily: 'Outfit',
                          color: cs.onSurface,
                        ),
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 24),
                TextField(
                  controller: _ctrl,
                  style: const TextStyle(fontFamily: 'Inter'),
                  decoration: InputDecoration(
                    labelText: 'Room ID',
                    labelStyle: TextStyle(fontFamily: 'Inter', color: cs.primary),
                    hintText: 'e.g. general, support, sales',
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(16),
                      borderSide: BorderSide(color: cs.outlineVariant),
                    ),
                    focusedBorder: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(16),
                      borderSide: BorderSide(color: cs.primary, width: 2),
                    ),
                    filled: true,
                    fillColor: cs.surfaceContainerHighest.withValues(alpha: 0.3),
                  ),
                  autofocus: true,
                ),
                const SizedBox(height: 32),
                Row(
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: [
                    TextButton(
                      onPressed: () => Navigator.pop(context),
                      style: TextButton.styleFrom(
                        padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
                        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
                      ),
                      child: const Text('Cancel', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.w600)),
                    ),
                    const SizedBox(width: 12),
                    FilledButton(
                      onPressed: () {
                        final val = _ctrl.text.trim();
                        if (val.isNotEmpty) Navigator.pop(context, val);
                      },
                      style: FilledButton.styleFrom(
                        padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
                        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
                      ),
                      child: const Text('Switch Room', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.w600)),
                    ),
                  ],
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
