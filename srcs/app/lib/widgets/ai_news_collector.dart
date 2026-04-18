import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'dart:ui';

class AiNewsCollectorWidget extends StatelessWidget {
  const AiNewsCollectorWidget({super.key});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 300,
      child: ClipRRect(
        borderRadius: BorderRadius.circular(24),
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
          child: Container(
            padding: const EdgeInsets.all(16),
            decoration: BoxDecoration(
              color: const Color.fromRGBO(255, 255, 255, 0.03),
              borderRadius: BorderRadius.circular(24),
              border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text(
                  'AI News Collector',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 20,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                ),
                const SizedBox(height: 16),
                Expanded(
                  child: ListView(
                    children: const [
                      _NewsItem(
                        title: 'GPT-5 Release Rumors',
                        source: 'TechCrunch',
                        time: '2h ago',
                      ),
                      _NewsItem(
                        title: 'New Claude Model Achieves SOTA',
                        source: 'Anthropic Blog',
                        time: '4h ago',
                      ),
                      _NewsItem(
                        title: 'Open Source LLMs Catching Up',
                        source: 'HuggingFace',
                        time: '1d ago',
                      ),
                    ],
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

class _NewsItem extends StatelessWidget {
  final String title;
  final String source;
  final String time;

  const _NewsItem({
    required this.title,
    required this.source,
    required this.time,
  });

  @override
  Widget build(BuildContext context) {
    return GlassCard(
      margin: const EdgeInsets.only(bottom: 8),
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              title,
              style: const TextStyle(
                fontFamily: 'Inter',
                fontWeight: FontWeight.bold,
                fontSize: 16,
              ),
            ),
            const SizedBox(height: 4),
            Row(
              children: [
                Text(
                  source,
                  style: TextStyle(
                    fontFamily: 'Inter',
                    fontSize: 12,
                    color: Theme.of(context).colorScheme.primary,
                  ),
                ),
                const Spacer(),
                Text(
                  time,
                  style: const TextStyle(
                    fontFamily: 'Inter',
                    fontSize: 12,
                    color: Colors.grey,
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
