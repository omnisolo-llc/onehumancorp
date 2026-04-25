import 'package:flutter/material.dart';

class WalkthroughStep {
  final GlobalKey key;
  final String title;
  final String description;

  WalkthroughStep({
    required this.key,
    required this.title,
    required this.description,
  });
}

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
    setState(() {
      if (_currentStepIndex < widget.steps.length - 1) {
        _currentStepIndex++;
      } else {
        widget.onComplete();
      }
    });
  }

  void _skipWalkthrough() {
    widget.onComplete();
  }

  @override
  Widget build(BuildContext context) {
    if (widget.steps.isEmpty) return const SizedBox.shrink();

    final step = widget.steps[_currentStepIndex];

    // Find the target element's render box
    final renderObject = step.key.currentContext?.findRenderObject();

    // If the target isn't found (e.g., scrolled off-screen or not built yet),
    // default to showing the bubble in the center of the screen
    Rect targetRect;
    if (renderObject != null && renderObject is RenderBox) {
      final size = renderObject.size;
      final offset = renderObject.localToGlobal(Offset.zero);
      targetRect = offset & size;
    } else {
      // Fallback
      final screenSize = MediaQuery.of(context).size;
      targetRect = Rect.fromCenter(
        center: Offset(screenSize.width / 2, screenSize.height / 2),
        width: 100,
        height: 100,
      );
    }

    // Determine bubble placement relative to the target
    final screenSize = MediaQuery.of(context).size;
    final bool placeBelow = targetRect.bottom + 200 < screenSize.height;

    double top;
    if (placeBelow) {
      top = targetRect.bottom + 16;
    } else {
      top = targetRect.top - 200; // rough height estimate
    }

    // Keep bubble within horizontal screen bounds
    double left = targetRect.center.dx - 160; // 320 width / 2
    if (left < 16) left = 16;
    if (left + 320 > screenSize.width - 16) left = screenSize.width - 336;

    return Positioned.fill(
      child: Stack(
        children: [
          // Overlay background with highlight cutout
          CustomPaint(
            size: Size.infinite,
            painter: _HighlightPainter(targetRect: targetRect),
          ),

          // Speech Bubble
          Positioned(
            top: top,
            left: left,
            child: Material(
              color: Colors.transparent,
              child: Container(
                width: 320,
                padding: const EdgeInsets.all(20),
                decoration: BoxDecoration(
                  color: Theme.of(context).colorScheme.surface,
                  borderRadius: BorderRadius.circular(16),
                  boxShadow: [
                    BoxShadow(
                      color: Colors.black.withValues(alpha: 0.2),
                      blurRadius: 16,
                      offset: const Offset(0, 8),
                    ),
                  ],
                ),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Text(
                          'Step ${_currentStepIndex + 1} of ${widget.steps.length}',
                          style: const TextStyle(
                            color: Colors.grey,
                            fontWeight: FontWeight.bold,
                            fontFamily: 'Inter',
                            fontSize: 12,
                          ),
                        ),
                        IconButton(
                          icon: const Icon(Icons.close, size: 20),
                          onPressed: _skipWalkthrough,
                          padding: EdgeInsets.zero,
                          constraints: const BoxConstraints(),
                          tooltip: 'Skip walkthrough',
                        ),
                      ],
                    ),
                    const SizedBox(height: 12),
                    Text(
                      step.title,
                      style: const TextStyle(
                        fontSize: 18,
                        fontWeight: FontWeight.bold,
                        fontFamily: 'Outfit',
                      ),
                    ),
                    const SizedBox(height: 8),
                    Text(
                      step.description,
                      style: const TextStyle(
                        fontSize: 14,
                        fontFamily: 'Inter',
                      ),
                    ),
                    const SizedBox(height: 20),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.end,
                      children: [
                        if (_currentStepIndex < widget.steps.length - 1)
                          TextButton(
                            onPressed: _skipWalkthrough,
                            child: const Text('Skip'),
                          ),
                        const SizedBox(width: 8),
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
      ),
    );
  }
}

class _HighlightPainter extends CustomPainter {
  final Rect targetRect;

  _HighlightPainter({required this.targetRect});

  @override
  void paint(Canvas canvas, Size size) {
    // Fill screen with semi-transparent dark overlay
    final bgPaint = Paint()
      ..color = Colors.black54
      ..style = PaintingStyle.fill;

    // Create a path for the whole screen
    final bgPath = Path()..addRect(Rect.fromLTWH(0, 0, size.width, size.height));

    // Create a path for the cutout (with some padding around the target)
    final cutoutRect = targetRect.inflate(8);
    final cutoutPath = Path()..addRRect(RRect.fromRectAndRadius(cutoutRect, const Radius.circular(8)));

    // Subtract the cutout from the background
    final combinedPath = Path.combine(PathOperation.difference, bgPath, cutoutPath);

    canvas.drawPath(combinedPath, bgPaint);
  }

  @override
  bool shouldRepaint(covariant _HighlightPainter oldDelegate) {
    return oldDelegate.targetRect != targetRect;
  }
}
