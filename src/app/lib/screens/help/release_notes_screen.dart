import 'dart:ui';
import 'package:flutter/material.dart';

class ReleaseNotesScreen extends StatelessWidget {
  const ReleaseNotesScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final releases = [
      {
        'version': 'v1.2.0',
        'date': 'October 24, 2026',
        'notes': [
          'Added a new Help Center with Video Tutorials and API Documentation.',
          'Improved AI Agent responsiveness.',
          'Fixed an issue with Stripe payments on mobile.',
        ],
      },
      {
        'version': 'v1.1.0',
        'date': 'October 10, 2026',
        'notes': [
          'Introduced the Marketing AI Agent.',
          'Added support for custom domains.',
          'UI performance improvements.',
        ],
      },
    ];

    return Scaffold(
      appBar: AppBar(
        title: const Text("What's New", style: TextStyle(fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      extendBodyBehindAppBar: true,
      body: Container(
        decoration: BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [
              Theme.of(context).colorScheme.surface,
              Theme.of(context).colorScheme.surfaceContainerHighest.withOpacity(0.5),
            ],
          ),
        ),
        child: SafeArea(
          child: ListView.builder(
            padding: const EdgeInsets.all(24.0),
            itemCount: releases.length,
            itemBuilder: (context, index) {
              final release = releases[index];
              return Padding(
                padding: const EdgeInsets.only(bottom: 24.0),
                child: ClipRRect(
                  borderRadius: BorderRadius.circular(24),
                  child: BackdropFilter(
                    filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
                    child: Container(
                      padding: const EdgeInsets.all(32),
                      decoration: BoxDecoration(
                        color: Theme.of(context).colorScheme.surface.withOpacity(0.5),
                        borderRadius: BorderRadius.circular(24),
                        border: Border.all(color: Colors.white.withOpacity(0.2)),
                      ),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Row(
                            mainAxisAlignment: MainAxisAlignment.spaceBetween,
                            children: [
                              Text(
                                release['version'] as String,
                                style: const TextStyle(
                                  fontSize: 24,
                                  fontWeight: FontWeight.bold,
                                  fontFamily: 'Outfit',
                                ),
                              ),
                              Text(
                                release['date'] as String,
                                style: TextStyle(
                                  fontSize: 14,
                                  color: Theme.of(context).colorScheme.onSurfaceVariant,
                                  fontFamily: 'Inter',
                                ),
                              ),
                            ],
                          ),
                          const SizedBox(height: 16),
                          ...(release['notes'] as List<String>).map((note) {
                            return Padding(
                              padding: const EdgeInsets.only(bottom: 8.0),
                              child: Row(
                                crossAxisAlignment: CrossAxisAlignment.start,
                                children: [
                                  const Text('• ', style: TextStyle(fontSize: 16)),
                                  Expanded(
                                    child: Text(
                                      note,
                                      style: const TextStyle(
                                        fontSize: 16,
                                        height: 1.5,
                                        fontFamily: 'Inter',
                                      ),
                                    ),
                                  ),
                                ],
                              ),
                            );
                          }).toList(),
                        ],
                      ),
                    ),
                  ),
                ),
              );
            },
          ),
        ),
      ),
    );
  }
}
