import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:ohc_app/services/api_service.dart';
import 'dart:ui';
import 'package:go_router/go_router.dart';

class SharedOutputScreen extends ConsumerStatefulWidget {
  final String token;
  const SharedOutputScreen({super.key, required this.token});

  @override
  ConsumerState<SharedOutputScreen> createState() => _SharedOutputScreenState();
}

class _SharedOutputScreenState extends ConsumerState<SharedOutputScreen> {
  late Future<Map<String, dynamic>> _outputFuture;

  @override
  void initState() {
    super.initState();
    _refresh();
  }

  void _refresh() {
    setState(() {
      _outputFuture = ref.read(apiServiceProvider)!.getSharedOutput(widget.token);
    });
  }

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return Scaffold(
      body: Container(
        decoration: BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [
              colors.surface,
              colors.surfaceContainerHighest,
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
                  child: FutureBuilder<Map<String, dynamic>>(
                    future: _outputFuture,
                    builder: (context, snapshot) {
                      if (snapshot.connectionState == ConnectionState.waiting) {
                        return const Center(child: CircularProgressIndicator());
                      }
                      if (snapshot.hasError) {
                        return _ErrorView(error: snapshot.error.toString());
                      }
                      final data = snapshot.data!;
                      return _SharedContentView(data: data);
                    },
                  ),
                ),
              ),
            ),
            Positioned(
              top: 48,
              left: 32,
              child: Image.asset(
                package: 'ohc_app',
                'assets/logo.png',
                height: 40,
                errorBuilder: (context, error, stackTrace) =>
                    Icon(Icons.blur_on, size: 40, color: colors.primary),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _SharedContentView extends StatelessWidget {
  final Map<String, dynamic> data;
  const _SharedContentView({required this.data});

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        Text(
          'Agentic Intelligence Shared',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 32,
            fontWeight: FontWeight.bold,
            color: colors.onSurface,
          ),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 16),
        Text(
          'A member of the OHC Swarm has shared this private intelligence output with you.',
          style: TextStyle(
            fontFamily: 'Inter',
            fontSize: 16,
            color: colors.onSurfaceVariant,
          ),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 48),
        GlassCard(
          padding: const EdgeInsets.all(32),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                children: [
                  CircleAvatar(
                    backgroundColor: colors.primary.withValues(alpha: 0.1),
                    child: Icon(Icons.smart_toy, color: colors.primary),
                  ),
                  const SizedBox(width: 16),
                  Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        'Author: ${data['author'] ?? 'Autonomous Agent'}',
                        style: const TextStyle(
                          fontFamily: 'Outfit',
                          fontWeight: FontWeight.bold,
                          color: Colors.white,
                        ),
                      ),
                      Text(
                        'Task ID: ${data['taskId']}',
                        style: const TextStyle(
                          fontFamily: 'Inter',
                          fontSize: 12,
                          color: Colors.white70,
                        ),
                      ),
                    ],
                  ),
                ],
              ),
              const Divider(height: 48, color: Colors.white10),
              Text(
                data['content'] ?? '',
                style: const TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 16,
                  height: 1.6,
                  color: Colors.white,
                ),
              ),
            ],
          ),
        ),
        const SizedBox(height: 48),
        ElevatedButton(
          onPressed: () => context.go('/landing'),
          style: ElevatedButton.styleFrom(
            padding: const EdgeInsets.symmetric(horizontal: 48, vertical: 20),
            backgroundColor: colors.primary,
            foregroundColor: colors.onPrimary,
          ),
          child: const Text(
            'Join the OHC Swarm',
            style: TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold),
          ),
        ),
        const SizedBox(height: 16),
        TextButton(
          onPressed: () => context.go('/login'),
          child: const Text('Already a member? Sign in'),
        ),
      ],
    );
  }
}

class _ErrorView extends StatelessWidget {
  final String error;
  const _ErrorView({required this.error});

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        const Icon(Icons.error_outline, size: 64, color: Colors.redAccent),
        const SizedBox(height: 24),
        Text(
          'Shared content not found or expired',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 24,
            fontWeight: FontWeight.bold,
            color: Theme.of(context).colorScheme.error,
          ),
        ),
        const SizedBox(height: 16),
        Text(
          'The Cloud-Bridge link may be invalid or the output has been pruned for security.',
          textAlign: TextAlign.center,
          style: TextStyle(fontFamily: 'Inter', color: Theme.of(context).colorScheme.onSurfaceVariant),
        ),
        const SizedBox(height: 32),
        FilledButton(
          onPressed: () => context.go('/landing'),
          child: const Text('Return to Safety'),
        ),
      ],
    );
  }
}
