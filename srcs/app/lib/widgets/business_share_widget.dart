import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';

class BusinessShareWidget extends ConsumerStatefulWidget {
  const BusinessShareWidget({super.key});

  @override
  ConsumerState<BusinessShareWidget> createState() => _BusinessShareWidgetState();
}

class _BusinessShareWidgetState extends ConsumerState<BusinessShareWidget> {
  bool _isHovered = false;

  void _shareLink(BuildContext context, String platform) {
    // Generate the share link. In a real app, this would use the actual domain/id.
    const shareLink = 'https://demo.ohc.io/storefront';

    Clipboard.setData(const ClipboardData(text: shareLink));

    if (context.mounted) {
      final colorScheme = Theme.of(context).colorScheme;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(
            'Link copied for $platform: $shareLink',
            style: TextStyle(
              color: colorScheme.onPrimaryContainer,
              fontFamily: 'Inter',
            ),
          ),
          behavior: SnackBarBehavior.floating,
          backgroundColor: colorScheme.primaryContainer,
          duration: const Duration(seconds: 3),
        ),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    final dashboardSnapshot = ref.watch(dashboardProvider).valueOrNull;
    final businessName = dashboardSnapshot?.organization.name ?? 'My Business';

    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedScale(
        scale: _isHovered ? 1.01 : 1.0,
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
                color: const Color.fromRGBO(255, 255, 255, 0.05),
                border: Border.all(color: colorScheme.outline.withValues(alpha: 0.3)),
                borderRadius: BorderRadius.circular(16),
              ),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  // OpenGraph Preview Representation
                  Container(
                    width: 200,
                    height: 120,
                    decoration: BoxDecoration(
                      gradient: LinearGradient(
                        colors: [
                          colorScheme.primary.withValues(alpha: 0.8),
                          colorScheme.secondary.withValues(alpha: 0.8),
                        ],
                        begin: Alignment.topLeft,
                        end: Alignment.bottomRight,
                      ),
                      borderRadius: BorderRadius.circular(12),
                      boxShadow: [
                        BoxShadow(
                          color: Colors.black.withValues(alpha: 0.2),
                          blurRadius: 8,
                          offset: const Offset(0, 4),
                        ),
                      ],
                    ),
                    child: Center(
                      child: Column(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: [
                          Icon(Icons.storefront, size: 40, color: colorScheme.onPrimary),
                          const SizedBox(height: 8),
                          Text(
                            businessName,
                            style: TextStyle(
                              fontFamily: 'Outfit',
                              fontWeight: FontWeight.bold,
                              fontSize: 18,
                              color: colorScheme.onPrimary,
                            ),
                            textAlign: TextAlign.center,
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                          ),
                          Text(
                            'Built with OHC',
                            style: TextStyle(
                              fontFamily: 'Inter',
                              fontSize: 10,
                              color: colorScheme.onPrimary.withValues(alpha: 0.8),
                            ),
                          )
                        ],
                      ),
                    ),
                  ),
                  const SizedBox(width: 32),
                  // Controls
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          'Share My Business',
                          style: TextStyle(
                            fontFamily: 'Outfit',
                            fontSize: 22,
                            fontWeight: FontWeight.bold,
                            color: colorScheme.onSurface,
                          ),
                        ),
                        const SizedBox(height: 8),
                        Text(
                          'Get your business noticed. Share your beautifully designed storefront link on social media to attract more customers.',
                          style: TextStyle(
                            fontFamily: 'Inter',
                            fontSize: 14,
                            color: colorScheme.onSurfaceVariant,
                            height: 1.4,
                          ),
                        ),
                        const SizedBox(height: 24),
                        Wrap(
                          spacing: 12,
                          runSpacing: 12,
                          children: [
                            ElevatedButton.icon(
                              onPressed: () => _shareLink(context, 'Clipboard'),
                              icon: const Icon(Icons.link),
                              label: const Text('Copy Link', style: TextStyle(fontFamily: 'Inter')),
                              style: ElevatedButton.styleFrom(
                                padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12),
                              ),
                            ),
                            OutlinedButton.icon(
                              onPressed: () => _shareLink(context, 'Instagram'),
                              icon: const Icon(Icons.camera_alt_outlined),
                              label: const Text('Instagram', style: TextStyle(fontFamily: 'Inter')),
                              style: OutlinedButton.styleFrom(
                                padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12),
                              ),
                            ),
                            OutlinedButton.icon(
                              onPressed: () => _shareLink(context, 'WhatsApp'),
                              icon: const Icon(Icons.chat_bubble_outline),
                              label: const Text('WhatsApp', style: TextStyle(fontFamily: 'Inter')),
                              style: OutlinedButton.styleFrom(
                                padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12),
                              ),
                            ),
                            OutlinedButton.icon(
                              onPressed: () => _shareLink(context, 'X'),
                              icon: const Icon(Icons.share),
                              label: const Text('X (Twitter)', style: TextStyle(fontFamily: 'Inter')),
                              style: OutlinedButton.styleFrom(
                                padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12),
                              ),
                            ),
                          ],
                        ),
                      ],
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
