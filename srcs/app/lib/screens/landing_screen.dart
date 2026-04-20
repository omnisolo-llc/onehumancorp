import 'package:ohc_app/widgets/glass_card.dart';

import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'dart:ui';
import 'package:ohc_app/services/api_service.dart';

class LandingScreen extends ConsumerStatefulWidget {
  const LandingScreen({super.key});

  @override
  ConsumerState<LandingScreen> createState() => _LandingScreenState();
}

class _LandingScreenState extends ConsumerState<LandingScreen> {

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
        child: Stack(
          children: [
            Center(
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
                      Wrap(
                        spacing: 16,
                        runSpacing: 16,
                        alignment: WrapAlignment.center,
                        children: [
                          _DownloadButton(os: 'Mac', icon: Icons.apple),
                          _DownloadButton(os: 'Windows', icon: Icons.window),
                          _DownloadButton(os: 'Linux', icon: Icons.laptop_chromebook),
                        ],
                      ),
                      const SizedBox(height: 16),
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
          ],
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
          package: 'ohc_app',
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
          'The Ultimate Business OS',
          style: Theme.of(context).textTheme.headlineMedium?.copyWith(
            color: Theme.of(context).colorScheme.primary,
          ),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 16),
        Text(
          'Build your company. Scale your vision. The gold standard for private business automation and secure workflow management.',
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
          icon: Icons.business,
          title: 'Automated Operations',
          description:
              'Set up professional workflows that run your business on autopilot. No technical skills required.',
        ),
        _GlassCard(
          icon: Icons.security,
          title: 'Total Data Privacy',
          description:
              'Your business data belongs to you. We provide a secure, private environment for all your documents.',
        ),
        _GlassCard(
          icon: Icons.trending_up,
          title: 'Unlimited Growth',
          description:
              'Scale from a single person to a full organization with our integrated business management tools.',
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

class _DownloadButton extends ConsumerWidget { // Tracking intent metrics
  final String os;
  final IconData icon;

  const _DownloadButton({required this.os, required this.icon});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Container(
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: Colors.white.withValues(alpha: 0.2)),
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(12),
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
          child: TextButton.icon(
            onPressed: () async {
              try {
                await ref.read(apiServiceProvider)!.trackDownload(os, '1.0.0');
                if (context.mounted) {
                  ScaffoldMessenger.of(context).showSnackBar(
                    SnackBar(
                      content: Text('Downloading OHC Desktop for $os...'),
                      behavior: SnackBarBehavior.floating,
                    ),
                  );
                }
              } catch (e) {
                if (context.mounted) {
                  ScaffoldMessenger.of(context).showSnackBar(
                    SnackBar(
                      content: Text('Error: $e'),
                      backgroundColor: Theme.of(context).colorScheme.error,
                    ),
                  );
                }
              }
            },
            icon: Icon(icon, color: Theme.of(context).colorScheme.primary),
            label: Text('Download for $os', style: TextStyle(color: Theme.of(context).colorScheme.primary)),
            style: TextButton.styleFrom(
              padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
              backgroundColor: Theme.of(context).colorScheme.primary.withValues(alpha: 0.1),
              textStyle: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
            ),
          ),
        ),
      ),
    );
  }
}
