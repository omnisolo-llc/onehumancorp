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
  OverlayEntry? _overlayEntry;

  @override
  void initState() {
    super.initState();
    // Simulate auto-start for demonstration purposes
    WidgetsBinding.instance.addPostFrameCallback((_) {
      // In a real app, check if this walkthrough has been completed before
      // startWalkthrough();
    });
  }

  @override
  void dispose() {
    _removeOverlay();
    super.dispose();
  }

  void startWalkthrough() {
    if (widget.steps.isNotEmpty) {
      setState(() {
        _currentStepIndex = 0;
      });
      _showOverlay();
    }
  }

  void _nextStep() {
    if (_currentStepIndex < widget.steps.length - 1) {
      setState(() {
        _currentStepIndex++;
      });
      _showOverlay();
    } else {
      _endWalkthrough();
    }
  }

  void _endWalkthrough() {
    setState(() {
      _currentStepIndex = -1;
    });
    _removeOverlay();
  }

  void _removeOverlay() {
    _overlayEntry?.remove();
    _overlayEntry = null;
  }

  void _showOverlay() {
    _removeOverlay();

    if (_currentStepIndex < 0 || _currentStepIndex >= widget.steps.length) return;

    final step = widget.steps[_currentStepIndex];
    final RenderBox? targetBox = step.key.currentContext?.findRenderObject() as RenderBox?;

    if (targetBox == null) return;

    final targetSize = targetBox.size;
    final targetPosition = targetBox.localToGlobal(Offset.zero);

    _overlayEntry = OverlayEntry(
      builder: (context) => Stack(
        children: [
          // Dark background with cutout
          Positioned.fill(
            child: GestureDetector(
              onTap: _endWalkthrough, // or do nothing
              child: Container(
                color: Colors.black.withOpacity(0.6),
              ),
            ),
          ),
          // Cutout (pseudo) - we just draw a box with border
          Positioned(
            left: targetPosition.dx - 8,
            top: targetPosition.dy - 8,
            width: targetSize.width + 16,
            height: targetSize.height + 16,
            child: IgnorePointer(
              child: Container(
                decoration: BoxDecoration(
                  border: Border.all(color: Colors.blue, width: 2),
                  borderRadius: BorderRadius.circular(8),
                ),
              ),
            ),
          ),
          // Speech Bubble
          Positioned(
            left: targetPosition.dx,
            top: targetPosition.dy + targetSize.height + 16,
            child: Material(
              color: Colors.transparent,
              child: Container(
                width: 250,
                padding: const EdgeInsets.all(16),
                decoration: BoxDecoration(
                  color: Colors.white,
                  borderRadius: BorderRadius.circular(12),
                  boxShadow: const [
                    BoxShadow(color: Colors.black26, blurRadius: 10, offset: Offset(0, 4)),
                  ],
                ),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Text(
                      step.title,
                      style: const TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Outfit', fontSize: 16, color: Colors.black),
                    ),
                    const SizedBox(height: 8),
                    Text(
                      step.description,
                      style: const TextStyle(fontFamily: 'Inter', fontSize: 14, color: Colors.black87),
                    ),
                    const SizedBox(height: 16),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.end,
                      children: [
                        TextButton(
                          onPressed: _endWalkthrough,
                          child: const Text('Skip'),
                        ),
                        const SizedBox(width: 8),
                        ElevatedButton(
                          onPressed: _nextStep,
                          child: Text(_currentStepIndex < widget.steps.length - 1 ? 'Next' : 'Finish'),
                        ),
                      ],
                    ),
                  ],
                ),
              ),
            ),
          ),
        ],
      ),
    );

    Overlay.of(context).insert(_overlayEntry!);
  }

  @override
  Widget build(BuildContext context) {
    return widget.child;
  }
}
