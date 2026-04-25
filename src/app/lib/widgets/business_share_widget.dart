import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

class BusinessShareWidget extends StatefulWidget {
  const BusinessShareWidget({super.key});

  @override
  State<BusinessShareWidget> createState() => _BusinessShareWidgetState();
}

class _BusinessShareWidgetState extends State<BusinessShareWidget> {
  bool _isHovered = false;

  void _copyLink() {
    Clipboard.setData(const ClipboardData(text: 'https://mybusiness.ohc.io'));
    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: const Text(
            'Storefront link copied! Share it to grow your business.',
            style: TextStyle(fontFamily: 'Inter'),
          ),
          behavior: SnackBarBehavior.floating,
          backgroundColor: Theme.of(context).colorScheme.primaryContainer,
        ),
      );
    }
  }

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
                color: const Color.fromRGBO(255, 255, 255, 0.05),
                border: Border.all(color: colorScheme.outline.withValues(alpha: 0.3)),
                borderRadius: BorderRadius.circular(16),
              ),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Container(
                    width: 64,
                    height: 64,
                    decoration: BoxDecoration(
                      color: colorScheme.secondaryContainer,
                      shape: BoxShape.circle,
                    ),
                    child: Center(
                      child: Icon(Icons.storefront, size: 32, color: colorScheme.onSecondaryContainer),
                    ),
                  ),
                  const SizedBox(width: 24),
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          'Your Storefront is Live',
                          style: TextStyle(
                            fontFamily: 'Outfit',
                            fontSize: 22,
                            fontWeight: FontWeight.bold,
                            color: colorScheme.onSurface,
                          ),
                        ),
                        const SizedBox(height: 8),
                        Text(
                          'Share your business link to attract new customers. Built with OHC.',
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
                    onPressed: _copyLink,
                    icon: const Icon(Icons.copy),
                    label: const Text('Copy Link', style: TextStyle(fontFamily: 'Outfit')),
                    style: ElevatedButton.styleFrom(
                      padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
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
