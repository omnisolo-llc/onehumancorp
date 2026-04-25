import 'package:flutter/material.dart';

/// Defines a single step in a walkthrough.
class WalkthroughStep {
  final GlobalKey targetKey;
  final String title;
  final String description;

  WalkthroughStep({
    required this.targetKey,
    required this.title,
    required this.description,
  });
}

/// A widget that manages an interactive walkthrough over the UI.
class WalkthroughOverlay extends StatefulWidget {
  final List<WalkthroughStep> steps;
  final VoidCallback onComplete;

  const WalkthroughOverlay({
    super.key,
    required this.steps,
    required this.onComplete,
  });

  @override
  State<WalkthroughOverlay> createState() => _WalkthroughOverlayState();
}

class _WalkthroughOverlayState extends State<WalkthroughOverlay> {
  int _currentStepIndex = 0;

  void _nextStep() {
    if (_currentStepIndex < widget.steps.length - 1) {
      setState(() {
        _currentStepIndex++;
      });
    } else {
      widget.onComplete();
    }
  }

  void _skip() {
    widget.onComplete();
  }

  @override
  Widget build(BuildContext context) {
    if (widget.steps.isEmpty) return const SizedBox.shrink();

    final currentStep = widget.steps[_currentStepIndex];
    final targetContext = currentStep.targetKey.currentContext;

    Rect? targetRect;
    if (targetContext != null) {
      final RenderBox renderBox = targetContext.findRenderObject() as RenderBox;
      final size = renderBox.size;
      final offset = renderBox.localToGlobal(Offset.zero);
      targetRect = offset & size;
    }

    return Stack(
      children: [
        // Semi-transparent background
        Positioned.fill(
          child: GestureDetector(
            onTap: _nextStep,
            child: Container(color: Colors.black54),
          ),
        ),

        // Highlight box (if target found)
        if (targetRect != null)
          Positioned(
            left: targetRect.left - 8,
            top: targetRect.top - 8,
            width: targetRect.width + 16,
            height: targetRect.height + 16,
            child: Container(
              decoration: BoxDecoration(
                border: Border.all(color: Colors.white, width: 2),
                borderRadius: BorderRadius.circular(8),
              ),
            ),
          ),

        // Speech bubble
        Positioned(
          left: (targetRect?.left ?? MediaQuery.of(context).size.width / 2) - 150,
          top: (targetRect?.bottom ?? MediaQuery.of(context).size.height / 2) + 20,
          child: Material(
            color: Colors.transparent,
            child: Container(
              width: 300,
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Theme.of(context).colorScheme.surface,
                borderRadius: BorderRadius.circular(12),
                boxShadow: const [
                  BoxShadow(color: Colors.black26, blurRadius: 10, offset: Offset(0, 4))
                ],
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.min,
                children: [
                  Text(
                    currentStep.title,
                    style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 18, fontFamily: 'Outfit'),
                  ),
                  const SizedBox(height: 8),
                  Text(
                    currentStep.description,
                    style: const TextStyle(fontFamily: 'Inter'),
                  ),
                  const SizedBox(height: 16),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      TextButton(
                        onPressed: _skip,
                        child: const Text('Skip Tour'),
                      ),
                      ElevatedButton(
                        onPressed: _nextStep,
                        child: Text(
                          _currentStepIndex < widget.steps.length - 1 ? 'Next' : 'Finish',
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ),
          ),
        ),
      ],
    );
  }
}
