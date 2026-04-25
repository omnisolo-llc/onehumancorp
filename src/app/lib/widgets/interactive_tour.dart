import 'package:flutter/material.dart';

class InteractiveTour extends StatelessWidget {
  final Widget child;
  final bool isRunning;
  final String currentStepMessage;
  final GlobalKey? currentTargetKey;
  final VoidCallback onNext;
  final VoidCallback onSkip;

  const InteractiveTour({
    super.key,
    required this.child,
    this.isRunning = false,
    this.currentStepMessage = '',
    this.currentTargetKey,
    required this.onNext,
    required this.onSkip,
  });

  @override
  Widget build(BuildContext context) {
    if (!isRunning) return child;

    return Stack(
      children: [
        child,
        if (currentTargetKey != null) ...[
          // We would add overlay logic here using the GlobalKey's RenderBox to position a highlight.
          // For simplicity in the initial architecture, we'll just show a bottom sheet like banner.
        ],
        Positioned(
          bottom: 0,
          left: 0,
          right: 0,
          child: Material(
            color: Theme.of(context).colorScheme.primaryContainer,
            child: SafeArea(
              child: Padding(
                padding: const EdgeInsets.all(16.0),
                child: Row(
                  children: [
                    Expanded(
                      child: Text(
                        currentStepMessage,
                        style: Theme.of(context).textTheme.bodyLarge?.copyWith(
                          color: Theme.of(context).colorScheme.onPrimaryContainer,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                    ),
                    TextButton(
                      onPressed: onSkip,
                      child: const Text('Skip Tour'),
                    ),
                    ElevatedButton(
                      onPressed: onNext,
                      child: const Text('Next'),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ],
    );
  }
}
