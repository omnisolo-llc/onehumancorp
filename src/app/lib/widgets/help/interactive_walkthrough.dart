import 'package:flutter/material.dart';

class WalkthroughStep {
  final GlobalKey key;
  final String title;
  final String description;

  WalkthroughStep({required this.key, required this.title, required this.description});
}

class InteractiveWalkthrough extends StatefulWidget {
  final List<WalkthroughStep> steps;
  final Widget child;

  const InteractiveWalkthrough({super.key, required this.steps, required this.child});

  static InteractiveWalkthroughState? of(BuildContext context) {
    return context.findAncestorStateOfType<InteractiveWalkthroughState>();
  }

  @override
  State<InteractiveWalkthrough> createState() => InteractiveWalkthroughState();
}

class InteractiveWalkthroughState extends State<InteractiveWalkthrough> {
  int _currentStepIndex = -1;
  OverlayEntry? _overlayEntry;

  void startWalkthrough() {
    if (widget.steps.isEmpty) return;
    setState(() {
      _currentStepIndex = 0;
    });
    _showOverlay();
  }

  void _nextStep() {
    if (_currentStepIndex < widget.steps.length - 1) {
      setState(() {
        _currentStepIndex++;
      });
      _overlayEntry?.markNeedsBuild();
    } else {
      _endWalkthrough();
    }
  }

  void _endWalkthrough() {
    setState(() {
      _currentStepIndex = -1;
    });
    _overlayEntry?.remove();
    _overlayEntry = null;
  }

  void _showOverlay() {
    _overlayEntry?.remove();
    _overlayEntry = OverlayEntry(
      builder: (context) => _WalkthroughOverlay(
        step: widget.steps[_currentStepIndex],
        onNext: _nextStep,
        onSkip: _endWalkthrough,
        isLastStep: _currentStepIndex == widget.steps.length - 1,
      ),
    );
    Overlay.of(context).insert(_overlayEntry!);
  }

  @override
  Widget build(BuildContext context) {
    return widget.child;
  }
}

class _WalkthroughOverlay extends StatelessWidget {
  final WalkthroughStep step;
  final VoidCallback onNext;
  final VoidCallback onSkip;
  final bool isLastStep;

  const _WalkthroughOverlay({
    required this.step,
    required this.onNext,
    required this.onSkip,
    required this.isLastStep,
  });

  @override
  Widget build(BuildContext context) {
    // Basic implementation: an overlay that obscures background and shows a bubble.
    // In a real implementation, this would use the global key to highlight the target element.
    return Material(
      color: Colors.black54,
      child: Center(
        child: Container(
          width: 300,
          padding: const EdgeInsets.all(24),
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.surface,
            borderRadius: BorderRadius.circular(16),
          ),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                step.title,
                style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
              ),
              const SizedBox(height: 12),
              Text(
                step.description,
                style: const TextStyle(fontSize: 14, fontFamily: 'Inter'),
              ),
              const SizedBox(height: 24),
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  TextButton(
                    onPressed: onSkip,
                    child: const Text('Skip'),
                  ),
                  ElevatedButton(
                    onPressed: onNext,
                    child: Text(isLastStep ? 'Finish' : 'Next'),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}
