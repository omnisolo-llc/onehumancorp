import re

with open('srcs/app/lib/screens/handoffs_screen.dart', 'r') as f:
    content = f.read()

# Make sure dart:ui is imported
if "import 'dart:ui';" not in content:
    content = content.replace("import 'package:flutter/material.dart';", "import 'package:flutter/material.dart';\nimport 'dart:ui';")

search_card = """              return Semantics(
                label: 'Handoff from agent to human. Intent: ${handoff.intent}',
                excludeSemantics: true,
                child: Card(
                  margin: const EdgeInsets.only(bottom: 16),
                  child: Padding(
                    padding: const EdgeInsets.all(20),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: [
                            Container(
                              padding: const EdgeInsets.symmetric(
                                horizontal: 8,
                                vertical: 4,
                              ),
                              decoration: BoxDecoration(
                                color: Theme.of(context).colorScheme.primaryContainer,
                                borderRadius: BorderRadius.circular(4),
                              ),
                              child: Text(
                                'Intent: ${handoff.intent.toUpperCase()}',
                                style: TextStyle(
                                  fontSize: 10,
                                  fontWeight: FontWeight.bold,
                                  color: Theme.of(context).colorScheme.onPrimaryContainer,
                                ),
                              ),
                            ),
                            Text(
                              DateFormat.yMMMd().add_jm().format(
                                handoff.createdAt,
                              ),
                              style: TextStyle(
                                fontSize: 12,
                                color: Theme.of(context).colorScheme.onSurfaceVariant,
                              ),
                            ),
                          ],
                        ),
                        const SizedBox(height: 16),
                        Text(
                          'Escalated by Agent: ${handoff.fromAgentId}',
                          style: const TextStyle(fontWeight: FontWeight.bold),
                        ),
                        const SizedBox(height: 8),
                        Text(
                          handoff.currentState,
                          style: const TextStyle(fontSize: 16),
                        ),
                        if (handoff.visualGroundTruth != null) ...[
                          const SizedBox(height: 16),
                          Container(
                            height: 200,
                            width: double.infinity,
                            decoration: BoxDecoration(
                                color: Theme.of(context).colorScheme.surfaceContainerHighest,
                              borderRadius: BorderRadius.circular(8),
                            ),
                            child: Center(
                              child: Icon(
                                Icons.image_outlined,
                                size: 48,
                                  color: Theme.of(context).colorScheme.onSurfaceVariant.withValues(alpha: 0.7),
                              ),
                            ),
                          ),
                        ],
                        const SizedBox(height: 24),
                        SlideToApprove(
                          disabled: isProcessing,
                          onApprove: () => _handleApprove(handoff.id),
                          onReject: () => _handleReject(handoff.id),
                        ),
                      ],
                    ),
                  ),
                ),
              );"""

replace_card = """              return _HandoffCard(
                handoff: handoff,
                isProcessing: isProcessing,
                onApprove: () => _handleApprove(handoff.id),
                onReject: () => _handleReject(handoff.id),
              );"""

content = content.replace(search_card, replace_card)

append_class = """

class _HandoffCard extends StatefulWidget {
  final HandoffPackage handoff;
  final bool isProcessing;
  final VoidCallback onApprove;
  final VoidCallback onReject;

  const _HandoffCard({
    required this.handoff,
    required this.isProcessing,
    required this.onApprove,
    required this.onReject,
  });

  @override
  State<_HandoffCard> createState() => _HandoffCardState();
}

class _HandoffCardState extends State<_HandoffCard> {
  bool _hovering = false;

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return Semantics(
      label: 'Handoff from agent to human. Intent: ${widget.handoff.intent}',
      excludeSemantics: true,
      child: MouseRegion(
        onEnter: (_) => setState(() => _hovering = true),
        onExit: (_) => setState(() => _hovering = false),
        child: AnimatedScale(
          scale: _hovering ? 1.02 : 1.0,
          duration: const Duration(milliseconds: 200),
          child: Padding(
            padding: const EdgeInsets.only(bottom: 16),
            child: ClipRRect(
              borderRadius: BorderRadius.circular(16),
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
                child: AnimatedContainer(
                  duration: const Duration(milliseconds: 200),
                  decoration: BoxDecoration(
                    color: _hovering
                        ? colors.surfaceContainerHighest.withValues(alpha: 0.3)
                        : colors.surface.withValues(alpha: 0.1),
                    borderRadius: BorderRadius.circular(16),
                    border: Border.all(
                      color: _hovering
                          ? colors.outlineVariant
                          : colors.outlineVariant.withValues(alpha: 0.5),
                    ),
                  ),
                  child: Padding(
                    padding: const EdgeInsets.all(20),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: [
                            Container(
                              padding: const EdgeInsets.symmetric(
                                horizontal: 8,
                                vertical: 4,
                              ),
                              decoration: BoxDecoration(
                                color: colors.primaryContainer,
                                borderRadius: BorderRadius.circular(4),
                              ),
                              child: Text(
                                'Intent: ${widget.handoff.intent.toUpperCase()}',
                                style: TextStyle(
                                  fontSize: 10,
                                  fontWeight: FontWeight.bold,
                                  color: colors.onPrimaryContainer,
                                ),
                              ),
                            ),
                            Text(
                              DateFormat.yMMMd().add_jm().format(
                                widget.handoff.createdAt,
                              ),
                              style: TextStyle(
                                fontSize: 12,
                                color: colors.onSurfaceVariant,
                              ),
                            ),
                          ],
                        ),
                        const SizedBox(height: 16),
                        Text(
                          'Escalated by Agent: ${widget.handoff.fromAgentId}',
                          style: const TextStyle(fontWeight: FontWeight.bold),
                        ),
                        const SizedBox(height: 8),
                        Text(
                          widget.handoff.currentState,
                          style: const TextStyle(fontSize: 16),
                        ),
                        if (widget.handoff.visualGroundTruth != null) ...[
                          const SizedBox(height: 16),
                          Container(
                            height: 200,
                            width: double.infinity,
                            decoration: BoxDecoration(
                                color: colors.surfaceContainerHighest,
                              borderRadius: BorderRadius.circular(8),
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
                        const SizedBox(height: 24),
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
    );
  }
}
"""

content += append_class

with open('srcs/app/lib/screens/handoffs_screen.dart', 'w') as f:
    f.write(content)
