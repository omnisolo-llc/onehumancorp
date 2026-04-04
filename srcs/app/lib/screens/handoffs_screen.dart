import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:intl/intl.dart';
import 'dart:ui' as dart_ui;
import 'package:ohc_app/models/handoff.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/widgets/slide_to_approve.dart';

/// Screen for managing agent-to-human escalation handoffs.
class HandoffsScreen extends ConsumerStatefulWidget {
  const HandoffsScreen({super.key});

  @override
  ConsumerState<HandoffsScreen> createState() => _HandoffsScreenState();
}

class _HandoffsScreenState extends ConsumerState<HandoffsScreen> {
  late Future<List<HandoffPackage>> _handoffsFuture;
  final Set<String> _processingIds = {};

  @override
  void initState() {
    super.initState();
    _refresh();
  }

  void _refresh() {
    setState(() {
      _handoffsFuture = ref.read(apiServiceProvider)!.listHandoffs();
    });
  }

  Future<void> _handleApprove(String id) async {
    setState(() => _processingIds.add(id));
    try {
      await ref.read(apiServiceProvider)!.resolveHandoff(id, 'approved');
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Handoff approved successfully')),
        );
        _refresh();
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Error: $e'),
            backgroundColor: Theme.of(context).colorScheme.error,
          ),
        );
      }
    } finally {
      if (mounted) setState(() => _processingIds.remove(id));
    }
  }

  Future<void> _handleReject(String id) async {
    setState(() => _processingIds.add(id));
    try {
      await ref.read(apiServiceProvider)!.resolveHandoff(id, 'rejected');
      if (mounted) {
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(const SnackBar(content: Text('Handoff rejected')));
        _refresh();
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Error: $e'),
            backgroundColor: Theme.of(context).colorScheme.error,
          ),
        );
      }
    } finally {
      if (mounted) setState(() => _processingIds.remove(id));
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Handoffs & Escalations'),
        actions: [
          IconButton(
            tooltip: 'Refresh',
            onPressed: _refresh,
            icon: const Icon(Icons.refresh),
          ),
        ],
      ),
      body: FutureBuilder<List<HandoffPackage>>(
        future: _handoffsFuture,
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            return Center(
              child: CircularProgressIndicator(
                color: Theme.of(context).colorScheme.primary,
              ),
            );
          }

          if (snapshot.hasError) {
            return Center(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Icon(
                    Icons.error_outline,
                    size: 60,
                    color: Theme.of(context).colorScheme.error,
                  ),
                  const SizedBox(height: 16),
                  Text(
                    'Failed to load handoffs',
                    style: Theme.of(context).textTheme.titleLarge,
                  ),
                  TextButton(
                    onPressed: _refresh,
                    child: const Text('Try Again'),
                  ),
                ],
              ),
            );
          }

          final handoffs = snapshot.data ?? [];
          if (handoffs.isEmpty) {
            return Center(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Icon(
                    Icons.check_circle_outline,
                    size: 64,
                    color: Theme.of(context).colorScheme.primary.withValues(alpha: 0.5),
                  ),
                  const SizedBox(height: 16),
                  const Text(
                    'No pending handoffs',
                    style: TextStyle(fontSize: 18, fontWeight: FontWeight.w500),
                  ),
                  Text(
                    'Your agents are operating autonomously.',
                    style: TextStyle(
                      color: Theme.of(context).colorScheme.onSurfaceVariant.withValues(alpha: 0.7),
                    ),
                  ),
                ],
              ),
            );
          }

          return ListView.builder(
            padding: const EdgeInsets.all(16),
            itemCount: handoffs.length,
            itemBuilder: (context, index) {
              final handoff = handoffs[index];
              final isProcessing = _processingIds.contains(handoff.id);

              return _AnimatedHandoffCard(
                key: ValueKey(handoff.id),
                handoff: handoff,
                isProcessing: isProcessing,
                index: index,
                onApprove: () => _handleApprove(handoff.id),
                onReject: () => _handleReject(handoff.id),
              );
            },
          );
        },
      ),
    );
  }
}

class _AnimatedHandoffCard extends StatefulWidget {
  final HandoffPackage handoff;
  final bool isProcessing;
  final int index;
  final VoidCallback onApprove;
  final VoidCallback onReject;

  const _AnimatedHandoffCard({
    super.key,
    required this.handoff,
    required this.isProcessing,
    required this.index,
    required this.onApprove,
    required this.onReject,
  });

  @override
  State<_AnimatedHandoffCard> createState() => _AnimatedHandoffCardState();
}

class _AnimatedHandoffCardState extends State<_AnimatedHandoffCard> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<Offset> _slideAnimation;
  late Animation<double> _fadeAnimation;
  bool _isHovered = false;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 600),
    );
    _slideAnimation = Tween<Offset>(
      begin: const Offset(0, 0.2),
      end: Offset.zero,
    ).animate(CurvedAnimation(parent: _controller, curve: Curves.easeOutQuart));
    _fadeAnimation = Tween<double>(begin: 0.0, end: 1.0)
        .animate(CurvedAnimation(parent: _controller, curve: Curves.easeOut));

    Future.delayed(Duration(milliseconds: 100 * widget.index), () {
      if (mounted) {
        _controller.forward();
      }
    });
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return Semantics(
      label: 'Handoff from agent to human. Intent: ${widget.handoff.intent}',
      excludeSemantics: true,
      child: SlideTransition(
        position: _slideAnimation,
        child: FadeTransition(
          opacity: _fadeAnimation,
          child: Padding(
            padding: const EdgeInsets.only(bottom: 16),
            child: MouseRegion(
              onEnter: (_) => setState(() => _isHovered = true),
              onExit: (_) => setState(() => _isHovered = false),
              child: AnimatedScale(
                scale: _isHovered ? 1.01 : 1.0,
                duration: const Duration(milliseconds: 200),
                curve: Curves.easeOutCubic,
                child: ClipRRect(
                  borderRadius: BorderRadius.circular(24),
                  child: BackdropFilter(
                    filter: _getGlassmorphismFilter(),
                    child: AnimatedContainer(
                      duration: const Duration(milliseconds: 300),
                      decoration: BoxDecoration(
                        color: _isHovered
                            ? const Color.fromRGBO(255, 255, 255, 0.05)
                            : const Color.fromRGBO(255, 255, 255, 0.03),
                        borderRadius: BorderRadius.circular(24),
                        border: Border.all(
                          color: _isHovered
                              ? Colors.white.withValues(alpha: 0.2)
                              : Colors.white.withValues(alpha: 0.1),
                        ),
                      ),
                      child: Padding(
                        padding: const EdgeInsets.all(24),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Row(
                              mainAxisAlignment: MainAxisAlignment.spaceBetween,
                              children: [
                                Container(
                                  padding: const EdgeInsets.symmetric(
                                    horizontal: 10,
                                    vertical: 6,
                                  ),
                                  decoration: BoxDecoration(
                                    color: colors.primary.withValues(alpha: 0.15),
                                    borderRadius: BorderRadius.circular(8),
                                    border: Border.all(color: colors.primary.withValues(alpha: 0.3)),
                                  ),
                                  child: Text(
                                    'Intent: ${widget.handoff.intent.toUpperCase()}',
                                    style: TextStyle(
                                      fontSize: 11,
                                      fontWeight: FontWeight.bold,
                                      color: colors.primary,
                                      fontFamily: 'Outfit',
                                    ),
                                  ),
                                ),
                                Text(
                                  DateFormat.yMMMd().add_jm().format(
                                    widget.handoff.createdAt,
                                  ),
                                  style: TextStyle(
                                    fontSize: 13,
                                    color: colors.onSurfaceVariant,
                                    fontFamily: 'Inter',
                                  ),
                                ),
                              ],
                            ),
                            const SizedBox(height: 20),
                            Text(
                              'Escalated by Agent: ${widget.handoff.fromAgentId}',
                              style: const TextStyle(
                                fontWeight: FontWeight.bold,
                                fontSize: 16,
                                fontFamily: 'Outfit',
                              ),
                            ),
                            const SizedBox(height: 8),
                            Text(
                              widget.handoff.currentState,
                              style: const TextStyle(
                                fontSize: 15,
                                fontFamily: 'Inter',
                              ),
                            ),
                            if (widget.handoff.visualGroundTruth != null) ...[
                              const SizedBox(height: 20),
                              Container(
                                height: 200,
                                width: double.infinity,
                                decoration: BoxDecoration(
                                  color: colors.surfaceContainerHighest.withValues(alpha: 0.5),
                                  borderRadius: BorderRadius.circular(12),
                                  border: Border.all(color: colors.outlineVariant.withValues(alpha: 0.5)),
                                ),
                                child: Center(
                                  child: Icon(
                                    Icons.image_outlined,
                                    size: 48,
                                    color: colors.onSurfaceVariant.withValues(alpha: 0.7),
                                  ),
                                ),
                              ),
                            ],
                            const SizedBox(height: 28),
                            SlideToApprove(
                              disabled: widget.isProcessing,
                              onApprove: widget.onApprove,
                              onReject: widget.onReject,
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
        ),
      ),
    );
  }

  // Same matrix as _ObservabilityWidget in Dashboard for consistent style
  dart_ui.ImageFilter _getGlassmorphismFilter() {
    return dart_ui.ImageFilter.compose(
      outer: const dart_ui.ColorFilter.matrix(<double>[
        1.168, -0.153, -0.015, 0, 0,
        -0.046, 1.061, -0.015, 0, 0,
        -0.046, -0.152, 1.198, 0, 0,
        0, 0, 0, 1, 0,
      ]),
      inner: dart_ui.ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
    );
  }
}
