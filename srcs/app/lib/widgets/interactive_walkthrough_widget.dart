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

  @override
  State<InteractiveWalkthrough> createState() => _InteractiveWalkthroughState();
}

class _InteractiveWalkthroughState extends State<InteractiveWalkthrough> {
  int _currentStepIndex = -1;

  void startWalkthrough() {
    if (widget.steps.isNotEmpty) {
      setState(() {
        _currentStepIndex = 0;
      });
      _showOverlay();
    }
  }

  void _nextStep() {
    setState(() {
      if (_currentStepIndex < widget.steps.length - 1) {
        _currentStepIndex++;
        _showOverlay();
      } else {
        _endWalkthrough();
      }
    });
  }

  void _endWalkthrough() {
    setState(() {
      _currentStepIndex = -1;
    });
    // Remove overlay if needed
  }

  void _showOverlay() {
    // In a real implementation, you would use an OverlayEntry to draw
    // a highlight around the widget and show a speech bubble.
    // This is a simplified placeholder.
    if (_currentStepIndex >= 0 && _currentStepIndex < widget.steps.length) {
      final step = widget.steps[_currentStepIndex];
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(step.title, style: const TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Outfit')),
              Text(step.description, style: const TextStyle(fontFamily: 'Inter')),
            ],
          ),
          action: SnackBarAction(
            label: _currentStepIndex < widget.steps.length - 1 ? 'Next' : 'Finish',
            onPressed: _nextStep,
          ),
          duration: const Duration(days: 1), // Stay until dismissed
        ),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    // We could provide a way to start it via context, but for now it's internal logic.
    return widget.child;
  }
}
