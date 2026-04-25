import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

final walkthroughProvider = StateProvider<WalkthroughState?>((ref) => null);

class WalkthroughState {
  final List<WalkthroughStep> steps;
  final int currentStep;

  WalkthroughState({required this.steps, this.currentStep = 0});

  WalkthroughState copyWith({int? currentStep}) {
    return WalkthroughState(
      steps: steps,
      currentStep: currentStep ?? this.currentStep,
    );
  }
}

class WalkthroughStep {
  final GlobalKey? targetKey;
  final String title;
  final String content;

  WalkthroughStep({
    this.targetKey,
    required this.title,
    required this.content,
  });
}

class WalkthroughOverlay extends ConsumerWidget {
  final Widget child;

  const WalkthroughOverlay({super.key, required this.child});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(walkthroughProvider);

    return Stack(
      children: [
        child,
        if (state != null && state.steps.isNotEmpty)
          _buildOverlay(context, ref, state),
      ],
    );
  }

  Widget _buildOverlay(BuildContext context, WidgetRef ref, WalkthroughState state) {
    final step = state.steps[state.currentStep];

    // In a real implementation with a library like tutorial_coach_mark, this would highlight
    // the specific widget using targetKey. For our implementation, we'll position
    // it in the center or relative to the screen to simulate the tour.

    return Positioned.fill(
      child: GestureDetector(
        onTap: () {
          // Prevent interactions with underlying widgets while walkthrough is active
        },
        child: Container(
          color: Colors.black.withOpacity(0.6),
          child: Stack(
            children: [
              // Simulated speech bubble
              Center(
                child: ClipRRect(
                  borderRadius: BorderRadius.circular(16),
                  child: BackdropFilter(
                    filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
                    child: Container(
                      width: 300,
                      padding: const EdgeInsets.all(24),
                      decoration: BoxDecoration(
                        color: Theme.of(context).colorScheme.surface.withOpacity(0.8),
                        borderRadius: BorderRadius.circular(16),
                        border: Border.all(color: Theme.of(context).colorScheme.primary.withOpacity(0.5), width: 2),
                      ),
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Row(
                            children: [
                              Icon(Icons.info_outline, color: Theme.of(context).colorScheme.primary),
                              const SizedBox(width: 8),
                              Expanded(
                                child: Text(
                                  step.title,
                                  style: const TextStyle(
                                    fontSize: 18,
                                    fontWeight: FontWeight.bold,
                                    fontFamily: 'Outfit',
                                  ),
                                ),
                              ),
                            ],
                          ),
                          const SizedBox(height: 12),
                          Text(
                            step.content,
                            style: const TextStyle(
                              fontSize: 14,
                              fontFamily: 'Inter',
                            ),
                          ),
                          const SizedBox(height: 24),
                          Row(
                            mainAxisAlignment: MainAxisAlignment.spaceBetween,
                            children: [
                              TextButton(
                                onPressed: () => ref.read(walkthroughProvider.notifier).state = null,
                                child: const Text('Skip Tour'),
                              ),
                              ElevatedButton(
                                onPressed: () {
                                  if (state.currentStep < state.steps.length - 1) {
                                    ref.read(walkthroughProvider.notifier).state =
                                        state.copyWith(currentStep: state.currentStep + 1);
                                  } else {
                                    ref.read(walkthroughProvider.notifier).state = null;
                                  }
                                },
                                child: Text(state.currentStep < state.steps.length - 1 ? 'Next' : 'Finish'),
                              ),
                            ],
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
