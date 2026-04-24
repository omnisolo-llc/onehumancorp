import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'dart:ui';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class WelcomeChecklistScreen extends ConsumerStatefulWidget {
  const WelcomeChecklistScreen({super.key});

  @override
  ConsumerState<WelcomeChecklistScreen> createState() =>
      _WelcomeChecklistScreenState();
}

class _WelcomeChecklistScreenState
    extends ConsumerState<WelcomeChecklistScreen> {
  final Map<int, bool> _tasks = {
    0: true, // Business Live
    1: false, // Add 3 more products
    2: false, // Connect Instagram
    3: false, // Share your link with a friend
  };

  void _toggleTask(int index) {
    if (index == 0) return; // Cannot untoggle first task
    setState(() {
      _tasks[index] = !(_tasks[index] ?? false);
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Color(0xFF0D0D1A), Color(0xFF1A1A33)],
          ),
        ),
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 600),
            child: ClipRRect(
              borderRadius: BorderRadius.circular(24),
              child: BackdropFilter(
                filter: ImageFilter.compose(
                  outer: ColorFilter.matrix(const <double>[
                    1.787,
                    -0.715,
                    -0.072,
                    0,
                    0,
                    -0.213,
                    1.285,
                    -0.072,
                    0,
                    0,
                    -0.213,
                    -0.715,
                    1.928,
                    0,
                    0,
                    0,
                    0,
                    0,
                    1,
                    0,
                  ]),
                  inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                ),
                child: Container(
                  decoration: BoxDecoration(
                    color: Theme.of(
                      context,
                    ).colorScheme.surface.withOpacity(0.6),
                    borderRadius: BorderRadius.circular(24),
                    border: Border.all(
                      color: Theme.of(
                        context,
                      ).colorScheme.outlineVariant.withOpacity(0.3),
                    ),
                  ),
                  child: Padding(
                    padding: const EdgeInsets.all(32),
                    child: SingleChildScrollView(
                      child: Column(
                      mainAxisSize: MainAxisSize.min,
                      crossAxisAlignment: CrossAxisAlignment.stretch,
                      children: [
                        const Icon(
                          Icons.rocket_launch,
                          size: 64,
                          color: Colors.blueAccent,
                        ),
                        const SizedBox(height: 24),
                        const Text(
                          "You're set up! Here's what to do next:",
                          textAlign: TextAlign.center,
                          style: TextStyle(
                            fontSize: 24,
                            fontWeight: FontWeight.bold,
                            fontFamily: 'Outfit',
                            color: Colors.white,
                          ),
                        ),
                        const SizedBox(height: 32),
                        _buildTaskTile(
                          0,
                          'Business Live',
                          Icons.check_circle,
                          true,
                        ),
                        _buildTaskTile(
                          1,
                          'Add 3 more products',
                          Icons.add_box_outlined,
                          false,
                        ),
                        _buildTaskTile(
                          2,
                          'Connect Instagram',
                          Icons.camera_alt_outlined,
                          false,
                        ),
                        _buildTaskTile(
                          3,
                          'Share your link with a friend',
                          Icons.share_outlined,
                          false,
                        ),
                        const SizedBox(height: 32),
                        ElevatedButton(
                          onPressed: () {
                            context.go('/dashboard');
                          },
                          child: const Text(
                            'Go to my Dashboard ->',
                            style: TextStyle(fontFamily: 'Inter'),
                          ),
                        ),
                      ],
                    ),
                    ),
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildTaskTile(
    int index,
    String title,
    IconData icon,
    bool isCompletedAlways,
  ) {
    final bool isCompleted = isCompletedAlways || (_tasks[index] ?? false);
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0),
      child: GlassCard(
        color:
            isCompleted
                ? Colors.blue.withOpacity(0.1)
                : Colors.white.withOpacity(0.05),
        child: ListTile(
          leading: Icon(
            isCompleted ? Icons.check_circle : icon,
            color: isCompleted ? Colors.blueAccent : Colors.white70,
          ),
          title: Text(
            title,
            style: TextStyle(
              fontFamily: 'Inter',
              color: isCompleted ? Colors.white : Colors.white70,
              decoration:
                  isCompleted && !isCompletedAlways
                      ? TextDecoration.lineThrough
                      : null,
            ),
          ),
          onTap: () => _toggleTask(index),
        ),
      ),
    );
  }
}
