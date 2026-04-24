import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class SocialMediaWidget extends ConsumerStatefulWidget {
  const SocialMediaWidget({super.key});

  @override
  ConsumerState<SocialMediaWidget> createState() => _SocialMediaWidgetState();
}

class _SocialMediaWidgetState extends ConsumerState<SocialMediaWidget> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedScale(
        scale: _isHovered ? 1.02 : 1.0,
        duration: const Duration(milliseconds: 200),
        curve: Curves.easeOut,
        child: ClipRRect(
          borderRadius: BorderRadius.circular(16),
          child: BackdropFilter(
            filter: ImageFilter.compose(
              outer: ColorFilter.matrix(const <double>[
                1.787, -0.715, -0.072, 0, 0,
                -0.213, 1.285, -0.072, 0, 0,
                -0.213, -0.715, 1.928, 0, 0,
                0, 0, 0, 1, 0,
              ]),
              inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            ),
            child: AnimatedContainer(
              duration: const Duration(milliseconds: 200),
              padding: const EdgeInsets.all(24),
              decoration: BoxDecoration(
                color: const Color.fromRGBO(255, 255, 255, 0.03),
                border: Border.all(color: colorScheme.outline.withValues(alpha: 0.2)),
                borderRadius: BorderRadius.circular(16),
              ),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Icon(
                    Icons.campaign,
                    size: 48,
                    color: colorScheme.primary,
                  ),
                  const SizedBox(width: 24),
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          'Social Media Auto-Posting',
                          style: TextStyle(
                            fontFamily: 'Outfit',
                            fontSize: 20,
                            fontWeight: FontWeight.bold,
                            color: colorScheme.onSurface,
                          ),
                        ),
                        const SizedBox(height: 8),
                        Text(
                          'AI agent auto-generates and schedules posts for new products, sales, or milestones on connected accounts.',
                          style: TextStyle(
                            fontFamily: 'Inter',
                            fontSize: 14,
                            color: colorScheme.onSurfaceVariant,
                          ),
                        ),
                      ],
                    ),
                  ),
                  const SizedBox(width: 24),
                  ElevatedButton.icon(
                    onPressed: () {
                      if (context.mounted) {
                        showDialog(
                            context: context,
                            builder: (context) => AlertDialog(
                                title: const Text('Connect Accounts', style: TextStyle(fontFamily: 'Outfit')),
                                content: const Text('Connect Instagram, Facebook, or X to enable auto-posting.'),
                                actions: [
                                  TextButton(
                                      onPressed: () => Navigator.of(context).pop(),
                                      child: const Text('Close')
                                  ),
                                ],
                            )
                        );
                      }
                    },
                    icon: const Icon(Icons.link),
                    label: const Text('Connect', style: TextStyle(fontFamily: 'Outfit')),
                    style: ElevatedButton.styleFrom(
                        backgroundColor: colorScheme.primaryContainer,
                        foregroundColor: colorScheme.onPrimaryContainer,
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
