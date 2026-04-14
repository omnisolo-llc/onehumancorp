import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:intl/intl.dart';
import 'package:ohc_app/models/pipeline.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/widgets/slide_to_approve.dart';

/// Screen for monitoring SDLC pipelines and promoting releases.
class PipelinesScreen extends ConsumerStatefulWidget {
  const PipelinesScreen({super.key});

  @override
  ConsumerState<PipelinesScreen> createState() => _PipelinesScreenState();
}

class _PipelinesScreenState extends ConsumerState<PipelinesScreen> {
  late Future<List<Pipeline>> _pipelinesFuture;
  final Set<String> _processingIds = {};

  @override
  void initState() {
    super.initState();
    _refresh();
  }

  void _refresh() {
    setState(() {
      _pipelinesFuture = ref.read(apiServiceProvider)!.listPipelines();
    });
  }

  Future<void> _handlePromote(String id) async {
    setState(() => _processingIds.add(id));
    try {
      await ref.read(apiServiceProvider)!.promotePipeline(id);
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Pipeline promoted to Production')),
        );
        _refresh();
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Promotion failed: $e'),
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
    final colors = Theme.of(context).colorScheme;

    return Scaffold(
      appBar: AppBar(
        title: const Text('SDLC Pipelines'),
        actions: [
          IconButton(
            onPressed: _refresh,
            icon: const Icon(Icons.refresh),
            tooltip: 'Refresh pipelines',
          ),
        ],
      ),
      body: FutureBuilder<List<Pipeline>>(
        future: _pipelinesFuture,
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            return Center(
              child: CircularProgressIndicator(
                color: Theme.of(context).colorScheme.primary,
              ),
            );
          }

          if (snapshot.hasError) {
            return Center(child: Text('Error: ${snapshot.error}'));
          }

          final pipelines = snapshot.data ?? [];
          if (pipelines.isEmpty) {
            return const Center(child: Text('No active pipelines found.'));
          }

          return ListView.builder(
            padding: const EdgeInsets.all(24),
            itemCount: pipelines.length,
            itemBuilder: (context, index) {
              final pipeline = pipelines[index];
              final isProcessing = _processingIds.contains(pipeline.id);

              return _PipelineCard(
                pipeline: pipeline,
                isProcessing: isProcessing,
                onPromote: () => _handlePromote(pipeline.id),
              );
            },
          );
        },
      ),
    );
  }
}

class _PipelineCard extends StatefulWidget {
  final Pipeline pipeline;
  final bool isProcessing;
  final VoidCallback onPromote;

  const _PipelineCard({
    required this.pipeline,
    required this.isProcessing,
    required this.onPromote,
  });

  @override
  State<_PipelineCard> createState() => _PipelineCardState();
}

class _PipelineCardState extends State<_PipelineCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final pipeline = widget.pipeline;

    return Semantics(
      label: 'Pipeline: ${pipeline.name}, Status: ${pipeline.status}',
      excludeSemantics: true,
      child: Padding(
        padding: const EdgeInsets.only(bottom: 24),
        child: GlassCard(
          padding: const EdgeInsets.all(24),
          child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                pipeline.name,
                                style: const TextStyle(
                                  fontSize: 18,
                                  fontWeight: FontWeight.bold,
                                ),
                              ),
                              const SizedBox(height: 4),
                              Row(
                                children: [
                                  const Icon(
                                    Icons.account_tree_outlined,
                                    size: 14,
                                  ),
                                  const SizedBox(width: 4),
                                  Text(
                                    pipeline.branch,
                                    style: TextStyle(
                                      color: colors.onSurfaceVariant,
                                      fontSize: 12,
                                    ),
                                  ),
                                ],
                              ),
                            ],
                          ),
                          _StatusBadge(status: pipeline.status),
                        ],
                      ),
                      const SizedBox(height: 24),
                      Row(
                        children: [
                          const Icon(Icons.person_outline, size: 16),
                          const SizedBox(width: 8),
                          Text(
                            'Initiated by: ${pipeline.initiatedBy ?? "System"}',
                            style: const TextStyle(fontSize: 12),
                          ),
                          const Spacer(),
                          Text(
                            'Updated: ${DateFormat.MMMd().add_jm().format(pipeline.updatedAt)}',
                            style: TextStyle(
                              fontSize: 12,
                              color: colors.onSurfaceVariant,
                            ),
                          ),
                        ],
                      ),
                      if (pipeline.stagingUrl != null) ...[
                        const SizedBox(height: 16),
                        Container(
                          padding: const EdgeInsets.all(12),
                          decoration: BoxDecoration(
                            color: colors.secondaryContainer.withValues(alpha: 0.3),
                            borderRadius: BorderRadius.circular(8),
                          ),
                          child: Row(
                            children: [
                              const Icon(Icons.link, size: 16),
                              const SizedBox(width: 8),
                              Expanded(
                                child: Text(
                                  pipeline.stagingUrl!,
                                  style: const TextStyle(
                                    fontSize: 12,
                                    fontFamily: 'monospace',
                                  ),
                                  overflow: TextOverflow.ellipsis,
                                ),
                              ),
                              Semantics(
                                button: true,
                                label: 'Open staging URL',
                                child: IconButton(
                                  icon: const Icon(
                                    Icons.open_in_new,
                                    size: 16,
                                  ),
                                  tooltip: 'Open staging URL',
                                  onPressed: () {}, // Link preview
                                ),
                              ),
                            ],
                          ),
                        ),
                      ],
                      const SizedBox(height: 24),
                      if (pipeline.status == 'staging') ...[
                        const Text(
                          'Promote to Production',
                          style: TextStyle(
                            fontWeight: FontWeight.bold,
                            fontSize: 14,
                          ),
                        ),
                        const SizedBox(height: 12),
                        SlideToApprove(
                          disabled: widget.isProcessing,
                          onApprove: widget.onPromote,
                          onReject: () {}, // Optional cancel
                        ),
                      ],
                    ],
                  ),
      ),
      ),
    );
  }
}

class _StatusBadge extends StatelessWidget {
  final String status;

  const _StatusBadge({required this.status});

  @override
  Widget build(BuildContext context) {
    Color color;
    switch (status.toLowerCase()) {
      case 'running':
      case 'active':
        color = Theme.of(context).colorScheme.primary;
        break;
      case 'staging':
        color = Theme.of(context).colorScheme.tertiary;
        break;
      case 'completed':
      case 'merged':
      case 'success':
        color = Theme.of(context).colorScheme.secondary;
        break;
      case 'failed':
        color = Theme.of(context).colorScheme.error;
        break;
      default:
        color = Theme.of(context).colorScheme.onSurfaceVariant;
    }

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
      decoration: BoxDecoration(
        color: color.withValues(alpha: 0.1),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: color.withValues(alpha: 0.5)),
      ),
      child: Text(
        status.toUpperCase(),
        style: TextStyle(
          fontSize: 10,
          fontWeight: FontWeight.bold,
          color: color,
        ),
      ),
    );
  }
}
