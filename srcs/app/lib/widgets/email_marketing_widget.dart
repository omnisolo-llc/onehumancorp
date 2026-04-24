import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class EmailMarketingWidget extends ConsumerStatefulWidget {
  const EmailMarketingWidget({super.key});

  @override
  ConsumerState<EmailMarketingWidget> createState() => _EmailMarketingWidgetState();
}

class _EmailMarketingWidgetState extends ConsumerState<EmailMarketingWidget> {
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
                    Icons.email,
                    size: 48,
                    color: colorScheme.tertiary,
                  ),
                  const SizedBox(width: 24),
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          'Email Marketing',
                          style: TextStyle(
                            fontFamily: 'Outfit',
                            fontSize: 20,
                            fontWeight: FontWeight.bold,
                            color: colorScheme.onSurface,
                          ),
                        ),
                        const SizedBox(height: 8),
                        Text(
                          'Select contacts, pick an AI-generated template ("New arrivals", "Flash sale", "Thank you"), preview, and send.',
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
                                title: const Text('Email Campaign', style: TextStyle(fontFamily: 'Outfit')),
                                content: const Text('AI agent is drafting your new email campaign...'),
                                actions: [
                                  TextButton(
                                      onPressed: () => Navigator.of(context).pop(),
                                      child: const Text('Cancel')
                                  ),
                                  ElevatedButton(
                                      onPressed: () {
                                          Navigator.of(context).pop();
                                          ScaffoldMessenger.of(context).showSnackBar(
                                            SnackBar(
                                              content: const Text('Campaign sent successfully!'),
                                              backgroundColor: colorScheme.primary,
                                            ),
                                          );
                                      },
                                      child: const Text('Send')
                                  )
                                ],
                            )
                        );
                      }
                    },
                    icon: const Icon(Icons.send),
                    label: const Text('New Campaign', style: TextStyle(fontFamily: 'Outfit')),
                    style: ElevatedButton.styleFrom(
                        backgroundColor: colorScheme.tertiaryContainer,
                        foregroundColor: colorScheme.onTertiaryContainer,
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
