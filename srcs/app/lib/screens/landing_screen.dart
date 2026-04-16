import 'package:ohc_app/widgets/glass_card.dart';

import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'dart:ui';

class LandingScreen extends StatelessWidget {
  const LandingScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        decoration: BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [
              Theme.of(context).colorScheme.surface,
              Theme.of(context).colorScheme.surfaceContainerHighest,
            ],
          ),
        ),
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 800),
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(32),
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  const _HeaderSection(),
                  const SizedBox(height: 48),
                  const _ValuePropGrid(),
                  const SizedBox(height: 48),
                  ElevatedButton(
                    onPressed: () => context.go('/business_setup'),
                    style: ElevatedButton.styleFrom(
                      padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 16),
                    ),
                    child: const Text('Start Business Setup', style: TextStyle(fontFamily: 'Inter', fontSize: 16)),
                  ),
                  const SizedBox(height: 8),
                  TextButton(
                    onPressed: () => context.go('/login'),
                    child: const Text('Or continue to Cloud Dashboard'),
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

class _HeaderSection extends StatelessWidget {
  const _HeaderSection();

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Image.asset(
          'assets/logo.png',
          height: 80,
          errorBuilder: (context, error, stackTrace) {
            return Icon(
              Icons.blur_on,
              size: 80,
              color: Theme.of(context).colorScheme.primary,
            );
          },
        ),
        const SizedBox(height: 24),
        Text(
          'One Human Corp',
          style: Theme.of(context).textTheme.displaySmall?.copyWith(
            fontWeight: FontWeight.bold,
          ),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 16),
        Text(
          'The Hybrid Agentic OS',
          style: Theme.of(context).textTheme.headlineMedium?.copyWith(
            color: Theme.of(context).colorScheme.primary,
          ),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 16),
        Text(
          'Scale your intelligence. Retain your sovereignty. Experience the gold standard for private LLM usage with our Local-First Standalone Mode.',
          style: Theme.of(context).textTheme.bodyLarge,
          textAlign: TextAlign.center,
        ),
      ],
    );
  }
}

class _ValuePropGrid extends StatelessWidget {
  const _ValuePropGrid();

  @override
  Widget build(BuildContext context) {
    return Wrap(
      spacing: 24,
      runSpacing: 24,
      alignment: WrapAlignment.center,
      children: const [
        _GlassCard(
          icon: Icons.shield,
          title: 'Zero Data Leakage',
          description:
              'All intelligence operations run completely local via SQLite. Absolute sovereignty over your IP.',
        ),
        _GlassCard(
          icon: Icons.cloud_off,
          title: 'Air-Gapped Autonomy',
          description:
              'Operate entirely offline. OHC degrades gracefully without heavy cloud dependencies.',
        ),
        _GlassCard(
          icon: Icons.group_add,
          title: 'Viral Referral Loop',
          description:
              'Seamlessly bridge to the Cloud to collaborate with human team members when ready.',
        ),
      ],
    );
  }
}

class _GlassCard extends StatelessWidget {
  final IconData icon;
  final String title;
  final String description;

  const _GlassCard({
    required this.icon,
    required this.title,
    required this.description,
  });

  @override
  Widget build(BuildContext context) {
    final color = Theme.of(context).colorScheme.primary;

    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 350),
      child: GlassCard(
        padding: const EdgeInsets.all(24),
        color: color.withValues(alpha: 0.05),
        child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Icon(icon, size: 40, color: color),
                const SizedBox(height: 16),
                Text(
                  title,
                  style: Theme.of(context).textTheme.titleLarge?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const SizedBox(height: 8),
                Text(
                  description,
                  style: Theme.of(context).textTheme.bodyMedium,
                ),
              ],
            ),
      ),
    );
  }
}
