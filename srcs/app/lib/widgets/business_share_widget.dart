import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:ohc_app/models/dashboard.dart';

class BusinessShareWidget extends StatefulWidget {
  final Organization organization;

  const BusinessShareWidget({super.key, required this.organization});

  @override
  State<BusinessShareWidget> createState() => _BusinessShareWidgetState();
}

class _BusinessShareWidgetState extends State<BusinessShareWidget> {
  bool _isHovered = false;

  void _shareBusiness() async {
    final domain = widget.organization.domain.isNotEmpty ? widget.organization.domain : 'my-business';
    final link = 'https://$domain.ohc.app';
    final message = 'Check out my business: ${widget.organization.name} at $link';

    await Clipboard.setData(ClipboardData(text: message));

    if (mounted) {
      final colorScheme = Theme.of(context).colorScheme;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(
            'Business link copied to clipboard!',
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
  }

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    final companyName = widget.organization.name.isNotEmpty ? widget.organization.name : 'Your Business';
    final domain = widget.organization.domain.isNotEmpty ? widget.organization.domain : 'your-business';

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
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Icon(Icons.storefront, color: colorScheme.primary, size: 28),
                      const SizedBox(width: 12),
                      Text(
                        'Share Your Business',
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                          color: colorScheme.onSurface,
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 16),

                  // OpenGraph-like Preview Card
                  Container(
                    decoration: BoxDecoration(
                      color: Colors.black.withOpacity(0.3),
                      borderRadius: BorderRadius.circular(12),
                      border: Border.all(color: colorScheme.outline.withOpacity(0.3)),
                    ),
                    clipBehavior: Clip.antiAlias,
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.stretch,
                      children: [
                        // Fake Image area
                        Container(
                          height: 120,
                          color: colorScheme.primary.withOpacity(0.2),
                          child: Center(
                            child: Icon(Icons.image, size: 48, color: colorScheme.primary.withOpacity(0.5)),
                          ),
                        ),
                        Padding(
                          padding: const EdgeInsets.all(16.0),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                companyName,
                                style: const TextStyle(
                                  fontFamily: 'Outfit',
                                  fontSize: 18,
                                  fontWeight: FontWeight.bold,
                                  color: Colors.white,
                                ),
                              ),
                              const SizedBox(height: 4),
                              Text(
                                'Built with OHC — Start your free business today.',
                                style: TextStyle(
                                  fontFamily: 'Inter',
                                  fontSize: 14,
                                  color: Colors.white.withOpacity(0.7),
                                ),
                              ),
                              const SizedBox(height: 8),
                              Text(
                                '$domain.ohc.app',
                                style: TextStyle(
                                  fontFamily: 'Inter',
                                  fontSize: 12,
                                  color: colorScheme.primary,
                                ),
                              ),
                            ],
                          ),
                        ),
                      ],
                    ),
                  ),
                  const SizedBox(height: 20),

                  // Buttons
                  Row(
                    children: [
                      Expanded(
                        child: ElevatedButton.icon(
                          onPressed: _shareBusiness,
                          icon: const Icon(Icons.share),
                          label: const Text('Share my business', style: TextStyle(fontFamily: 'Outfit')),
                          style: ElevatedButton.styleFrom(
                            padding: const EdgeInsets.symmetric(vertical: 16),
                          ),
                        ),
                      ),
                      const SizedBox(width: 12),
                      IconButton(
                        onPressed: _shareBusiness,
                        icon: const Icon(Icons.camera_alt),
                        tooltip: 'Post to Instagram',
                        style: IconButton.styleFrom(
                          backgroundColor: Colors.white.withOpacity(0.1),
                        ),
                      ),
                      const SizedBox(width: 8),
                      IconButton(
                        onPressed: _shareBusiness,
                        icon: const Icon(Icons.chat),
                        tooltip: 'Share on WhatsApp',
                        style: IconButton.styleFrom(
                          backgroundColor: Colors.white.withOpacity(0.1),
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
