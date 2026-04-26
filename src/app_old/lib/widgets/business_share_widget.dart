import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:flutter/foundation.dart' show kIsWeb;
// Note: We use a placeholder for url_launcher functionality as the package cannot be added
// without modifying pubspec.lock which violates forbidden paths rules in this repository.

class BusinessShareWidget extends ConsumerStatefulWidget {
  final DashboardSnapshot data;

  const BusinessShareWidget({super.key, required this.data});

  @override
  ConsumerState<BusinessShareWidget> createState() => _BusinessShareWidgetState();
}

class _BusinessShareWidgetState extends ConsumerState<BusinessShareWidget> {
  bool _isHovered = false;

  void _copyLink() {
    final domain = widget.data.organization.domain.isNotEmpty
        ? widget.data.organization.domain
        : '${widget.data.organization.id}.ohc.io';
    final url = 'https://$domain';
    Clipboard.setData(ClipboardData(text: url));

    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('Business link copied to clipboard: $url'),
          behavior: SnackBarBehavior.floating,
        ),
      );
    }
  }

  Future<void> _shareToX() async {
    // In a real implementation this would use url_launcher:
    // final tweetText = Uri.encodeComponent('Check out my new business built with OHC! $url');
    // launchUrl(Uri.parse('https://twitter.com/intent/tweet?text=$tweetText'));
    _copyLink();
  }

  Future<void> _shareToWhatsApp() async {
    // In a real implementation this would use url_launcher:
    // final text = Uri.encodeComponent('Check out my new business built with OHC! $url');
    // launchUrl(Uri.parse('https://wa.me/?text=$text'));
    _copyLink();
  }

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final orgName = widget.data.organization.name.isNotEmpty
        ? widget.data.organization.name
        : 'Your Business';

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
                border: Border.all(color: colors.outline.withValues(alpha: 0.2)),
                borderRadius: BorderRadius.circular(16),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Icon(Icons.business, color: colors.primary, size: 28),
                      const SizedBox(width: 12),
                      Text(
                        'Share my business',
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                          color: colors.onSurface,
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 16),
                  Container(
                    padding: const EdgeInsets.all(16),
                    decoration: BoxDecoration(
                      color: colors.surfaceContainerHighest.withValues(alpha: 0.5),
                      borderRadius: BorderRadius.circular(12),
                      border: Border.all(color: colors.outlineVariant),
                    ),
                    child: Row(
                      children: [
                        Container(
                          width: 48,
                          height: 48,
                          decoration: BoxDecoration(
                            color: colors.primaryContainer,
                            shape: BoxShape.circle,
                          ),
                          child: Center(
                            child: Text(
                              orgName.substring(0, 1).toUpperCase(),
                              style: TextStyle(
                                color: colors.onPrimaryContainer,
                                fontSize: 24,
                                fontWeight: FontWeight.bold,
                                fontFamily: 'Outfit',
                              ),
                            ),
                          ),
                        ),
                        const SizedBox(width: 16),
                        Expanded(
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                orgName,
                                style: const TextStyle(
                                  fontWeight: FontWeight.bold,
                                  fontSize: 16,
                                  fontFamily: 'Outfit',
                                ),
                              ),
                              const SizedBox(height: 4),
                              Text(
                                'Check out my new business built with OHC!',
                                style: TextStyle(
                                  color: colors.onSurfaceVariant,
                                  fontSize: 14,
                                  fontFamily: 'Inter',
                                ),
                              ),
                            ],
                          ),
                        ),
                      ],
                    ),
                  ),
                  const SizedBox(height: 16),
                  Row(
                    children: [
                      Expanded(
                        child: ElevatedButton.icon(
                          onPressed: _copyLink,
                          icon: const Icon(Icons.link),
                          label: const Text('Copy Link', style: TextStyle(fontFamily: 'Inter')),
                          style: ElevatedButton.styleFrom(
                            padding: const EdgeInsets.symmetric(vertical: 16),
                          ),
                        ),
                      ),
                      const SizedBox(width: 12),
                      Semantics(
                        label: 'Share to Instagram',
                        child: OutlinedButton(
                          onPressed: _copyLink, // Instagram doesn't have a reliable share intent with prefilled text
                          style: OutlinedButton.styleFrom(
                            padding: const EdgeInsets.all(16),
                            shape: const CircleBorder(),
                          ),
                          child: const Icon(Icons.camera_alt_outlined),
                        ),
                      ),
                      const SizedBox(width: 8),
                      Semantics(
                        label: 'Share to X',
                        child: OutlinedButton(
                          onPressed: _shareToX,
                          style: OutlinedButton.styleFrom(
                            padding: const EdgeInsets.all(16),
                            shape: const CircleBorder(),
                          ),
                          child: const Icon(Icons.close),
                        ),
                      ),
                      const SizedBox(width: 8),
                      Semantics(
                        label: 'Share to WhatsApp',
                        child: OutlinedButton(
                          onPressed: _shareToWhatsApp,
                          style: OutlinedButton.styleFrom(
                            padding: const EdgeInsets.all(16),
                            shape: const CircleBorder(),
                          ),
                          child: const Icon(Icons.chat_bubble_outline),
                        ),
                      ),
                    ],
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
