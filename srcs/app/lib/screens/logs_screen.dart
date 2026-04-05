import 'dart:async';
import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/api_service.dart';

final _logsProvider = FutureProvider.family<List<String>, int>((
  ref,
  lines,
) async {
  final api = ref.watch(apiServiceProvider);
  if (api == null) return [];
  return api.getLogs(lines: lines);
});

class LogsScreen extends ConsumerStatefulWidget {
  const LogsScreen({super.key});

  @override
  ConsumerState<LogsScreen> createState() => _LogsScreenState();
}

class _LogsScreenState extends ConsumerState<LogsScreen> {
  int _lines = 100;
  Timer? _timer;
  final _scrollCtrl = ScrollController();

  @override
  void initState() {
    super.initState();
    // Auto-refresh logs every 3 seconds.
    _timer = Timer.periodic(const Duration(seconds: 3), (_) {
      ref.invalidate(_logsProvider(_lines));
    });
  }

  @override
  void dispose() {
    _timer?.cancel();
    _scrollCtrl.dispose();
    super.dispose();
  }

  void _scrollToBottom() {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (_scrollCtrl.hasClients) {
        _scrollCtrl.jumpTo(_scrollCtrl.position.maxScrollExtent);
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    final snapshot = ref.watch(_logsProvider(_lines));
    return Scaffold(
      appBar: AppBar(
        title: const Text(
          'Service Logs',
          style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold),
        ),
        centerTitle: false,
        actions: [
          // Line count selector
          Tooltip(
            message: 'Select number of log lines to show',
            child: DropdownButton<int>(
              value: _lines,
              underline: const SizedBox(),
              items: const [
                DropdownMenuItem(value: 50, child: Text('50 lines', style: TextStyle(fontFamily: 'Inter'))),
                DropdownMenuItem(value: 100, child: Text('100 lines', style: TextStyle(fontFamily: 'Inter'))),
                DropdownMenuItem(value: 500, child: Text('500 lines', style: TextStyle(fontFamily: 'Inter'))),
              ],
              onChanged: (v) {
                if (v != null) setState(() => _lines = v);
              },
            ),
          ),
          const SizedBox(width: 8),
          IconButton(
            icon: const Icon(Icons.refresh),
            tooltip: 'Refresh',
            onPressed: () => ref.invalidate(_logsProvider(_lines)),
          ),
          const SizedBox(width: 8),
        ],
      ),
      body: snapshot.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error: (e, _) => Center(child: Text('Error: $e')),
        data: (lines) {
          _scrollToBottom();
          if (lines.isEmpty) {
            return const Center(child: Text('No logs yet.'));
          }
          return Container(
            color: const Color(0xFF0F0F1A),
            child: Stack(
              children: [
                Positioned.fill(
                  child: ClipRect(
                    child: BackdropFilter(
                      filter: ImageFilter.compose(
                        outer: ColorFilter.matrix(const <double>[
                          1.168, -0.153, -0.015, 0, 0,
                          -0.046, 1.061, -0.015, 0, 0,
                          -0.046, -0.152, 1.198, 0, 0,
                          0, 0, 0, 1, 0,
                        ]),
                        inner: ImageFilter.blur(sigmaX: 10.0, sigmaY: 10.0),
                      ),
                      child: Container(color: Colors.transparent),
                    ),
                  ),
                ),
                ListView.builder(
                  controller: _scrollCtrl,
                  padding: const EdgeInsets.all(16),
                  itemCount: lines.length,
                  itemBuilder: (_, i) => _LogLine(line: lines[i], index: i),
                ),
              ],
            ),
          );
        },
      ),
    );
  }
}

class _LogLine extends StatefulWidget {
  final String line;
  final int index;

  const _LogLine({required this.line, required this.index});

  @override
  State<_LogLine> createState() => _LogLineState();
}

class _LogLineState extends State<_LogLine> with SingleTickerProviderStateMixin {
  bool _isHovered = false;
  late AnimationController _controller;
  late Animation<double> _fadeAnimation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 300),
    );
    _fadeAnimation = Tween<double>(begin: 0.0, end: 1.0).animate(CurvedAnimation(parent: _controller, curve: Curves.easeOut));

    Future.delayed(Duration(milliseconds: (widget.index % 50) * 10), () {
      if (mounted) _controller.forward();
    });
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  Color _color(BuildContext context) {
    final lower = widget.line.toLowerCase();
    if (lower.contains('error') || lower.contains('fatal'))
      return Theme.of(context).colorScheme.error;
    if (lower.contains('warn')) return Theme.of(context).colorScheme.tertiary;
    if (lower.contains('info')) return Theme.of(context).colorScheme.secondary;
    if (lower.contains('debug'))
      return Theme.of(context).colorScheme.onSurfaceVariant.withValues(alpha: 0.5);
    return Theme.of(context).colorScheme.onSurfaceVariant;
  }

  @override
  Widget build(BuildContext context) {
    return FadeTransition(
      opacity: _fadeAnimation,
      child: MouseRegion(
        onEnter: (_) => setState(() => _isHovered = true),
        onExit: (_) => setState(() => _isHovered = false),
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 200),
          margin: const EdgeInsets.symmetric(vertical: 2),
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
          decoration: BoxDecoration(
            color: _isHovered
                ? const Color.fromRGBO(255, 255, 255, 0.08)
                : const Color.fromRGBO(255, 255, 255, 0.03),
            borderRadius: BorderRadius.circular(8),
            border: Border.all(
              color: _isHovered
                  ? Colors.white.withValues(alpha: 0.2)
                  : const Color.fromRGBO(255, 255, 255, 0.05),
            ),
          ),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              SizedBox(
                width: 40,
                child: Text(
                  '${widget.index + 1}',
                  style: TextStyle(
                    color: Theme.of(
                      context,
                    ).colorScheme.onSurfaceVariant.withValues(alpha: 0.6),
                    fontFamily: 'monospace',
                    fontSize: 12,
                  ),
                ),
              ),
              Expanded(
                child: SelectableText(
                  widget.line,
                  style: TextStyle(
                    color: _color(context),
                    fontFamily: 'monospace',
                    fontSize: 12,
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
