import 'dart:ui';
import 'package:flutter/material.dart';

class GrowthReferralWidget extends StatefulWidget {
  const GrowthReferralWidget({super.key});

  @override
  State<GrowthReferralWidget> createState() => _GrowthReferralWidgetState();
}

class _GrowthReferralWidgetState extends State<GrowthReferralWidget> {
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
            filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
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
                    Icons.group_add,
                    size: 48,
                    color: colorScheme.primary,
                  ),
                  const SizedBox(width: 24),
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          'Grow Your Swarm. Maintain Sovereignty.',
                          style: TextStyle(
                            fontFamily: 'Outfit',
                            fontSize: 20,
                            fontWeight: FontWeight.bold,
                            color: colorScheme.onSurface,
                          ),
                        ),
                        const SizedBox(height: 8),
                        Text(
                          'Bridge your Standalone Mode to the Cloud. Invite team members securely with zero data leakage.',
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
                  Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      const Text(
                        'Free Tier Quota',
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          color: Colors.white,
                          fontSize: 18,
                        ),
                      ),
                      const SizedBox(height: 10),
                      const Text(
                        '10 / 100 missions used',
                        style: TextStyle(
                          fontFamily: 'Inter',
                          color: Colors.white70,
                          fontSize: 14,
                        ),
                      ),
                      const SizedBox(height: 20),
                      ElevatedButton(
                        onPressed: () {},
                        child: const Text('Invite Team to Expand Quota', style: TextStyle(fontFamily: 'Outfit')),
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
