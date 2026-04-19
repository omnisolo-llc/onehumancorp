import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/api_service.dart';

class GrowthReferralWidget extends ConsumerStatefulWidget {
  const GrowthReferralWidget({super.key});

  @override
  ConsumerState<GrowthReferralWidget> createState() => _GrowthReferralWidgetState();
}

class _GrowthReferralWidgetState extends ConsumerState<GrowthReferralWidget> {
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
                      FutureBuilder<Map<String, dynamic>>(
                        future: ref.read(apiServiceProvider)!.getQuota(),
                        builder: (context, snapshot) {
                          if (snapshot.connectionState == ConnectionState.waiting) {
                            return const CircularProgressIndicator();
                          }
                          if (snapshot.hasError || !snapshot.hasData) {
                            return const Text('Error loading quota', style: TextStyle(color: Colors.red));
                          }
                          final data = snapshot.data!;
                          return Text(
                            '${data['used']} / ${data['max']} missions used',
                            style: const TextStyle(
                              fontFamily: 'Inter',
                              color: Colors.white70,
                              fontSize: 14,
                            ),
                          );
                        },
                      ),
                      const SizedBox(height: 20),
                      ElevatedButton(
                        onPressed: () async {
                          try {
                            await ref.read(apiServiceProvider)!.createReferral(
                              "anonymous",
                              "xYz8vQ_local_sovereign",
                            );
                            if (context.mounted) {
                              final snackBar = SnackBar(
                                content: Text(
                                    'Cloud-Bridge invite link copied: https://cloud.ohc.io/invite?token=xYz8vQ_local_sovereign',
                                    style: TextStyle(
                                      color: colorScheme.onPrimaryContainer,
                                      fontFamily: 'Inter',
                                    ),
                                ),
                                behavior: SnackBarBehavior.floating,
                                backgroundColor: colorScheme.primaryContainer,
                              );
                              ScaffoldMessenger.of(context).showSnackBar(snackBar);
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
