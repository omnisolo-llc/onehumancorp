import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/auth_service.dart';

class ShareBusinessWidget extends ConsumerStatefulWidget {
  const ShareBusinessWidget({super.key});

  @override
  ConsumerState<ShareBusinessWidget> createState() => _ShareBusinessWidgetState();
}

class _ShareBusinessWidgetState extends ConsumerState<ShareBusinessWidget> {
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
                    Icons.share,
                    size: 48,
                    color: colorScheme.secondary,
                  ),
                  const SizedBox(width: 24),
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          'Share your business',
                          style: TextStyle(
                            fontFamily: 'Outfit',
                            fontSize: 20,
                            fontWeight: FontWeight.bold,
                            color: colorScheme.onSurface,
                          ),
                        ),
                        const SizedBox(height: 8),
                        Text(
                          'Copy the link to your beautifully designed shareable storefront and optionally post to Instagram, WhatsApp, or X.',
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
                    onPressed: () async {
                      try {
                        final authUser = ref.read(authStateProvider).valueOrNull;
                        final userId = authUser?.id ?? "anonymous";
                        final link = "https://ohc.app/$userId/storefront";
                        await Clipboard.setData(ClipboardData(text: link));

                        if (context.mounted) {
                          ScaffoldMessenger.of(context).showSnackBar(
                            SnackBar(
                              content: Text(
                                'Storefront link copied to clipboard!',
                                style: TextStyle(
                                  color: colorScheme.onPrimaryContainer,
                                  fontFamily: 'Inter',
                                ),
                              ),
                              behavior: SnackBarBehavior.floating,
                              backgroundColor: colorScheme.primaryContainer,
                            ),
                          );
                        }
                      } catch (e) {
                        if (context.mounted) {
                          ScaffoldMessenger.of(context).showSnackBar(
                            SnackBar(
                              content: Text('Error: $e'),
                              backgroundColor: colorScheme.error,
                            ),
                          );
                        }
                      }
                    },
                    icon: const Icon(Icons.copy),
                    label: const Text('Share my business', style: TextStyle(fontFamily: 'Outfit')),
                    style: ElevatedButton.styleFrom(
                        backgroundColor: colorScheme.secondaryContainer,
                        foregroundColor: colorScheme.onSecondaryContainer,
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
