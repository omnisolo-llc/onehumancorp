import 'package:flutter/material.dart';

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

class InteractiveWalkthroughOverlay extends StatefulWidget {
  final List<WalkthroughStep> steps;
  final VoidCallback onComplete;
  final VoidCallback onSkip;

  const InteractiveWalkthroughOverlay({
    super.key,
    required this.steps,
    required this.onComplete,
    required this.onSkip,
  });

  @override
  State<InteractiveWalkthroughOverlay> createState() => _InteractiveWalkthroughOverlayState();
}

class _InteractiveWalkthroughOverlayState extends State<InteractiveWalkthroughOverlay> {
  int _currentStep = 0;

  @override
  Widget build(BuildContext context) {
    if (widget.steps.isEmpty || _currentStep >= widget.steps.length) {
      return const SizedBox.shrink();
    }

    final step = widget.steps[_currentStep];
    final renderBox = step.targetKey.currentContext?.findRenderObject() as RenderBox?;

    // Fallback if target is not laid out yet
    if (renderBox == null) {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (mounted) setState(() {});
      });
      return Container(color: Colors.black54);
    }

    final size = renderBox.size;
    final position = renderBox.localToGlobal(Offset.zero);
    final screenSize = MediaQuery.of(context).size;

    // Calculate popup position
    final popupTop = position.dy + size.height + 16;
    final isBottom = popupTop + 200 > screenSize.height;

    return Stack(
      children: [
        // Darkened background with a hole for the target
        ColorFiltered(
          colorFilter: const ColorFilter.mode(
            Colors.black54,
            BlendMode.srcOut,
          ),
          child: Stack(
            children: [
              Container(
                decoration: const BoxDecoration(
                  color: Colors.transparent,
                ),
                child: Container(
                  decoration: const BoxDecoration(
                    color: Colors.black,
                  ),
                ),
              ),
              Positioned(
                left: position.dx - 4,
                top: position.dy - 4,
                width: size.width + 8,
                height: size.height + 8,
                child: Container(
                  decoration: BoxDecoration(
                    color: Colors.white,
                    borderRadius: BorderRadius.circular(8),
                  ),
                ),
              ),
            ],
          ),
        ),

        // Highlight border
        Positioned(
          left: position.dx - 4,
          top: position.dy - 4,
          width: size.width + 8,
          height: size.height + 8,
          child: IgnorePointer(
            child: Container(
              decoration: BoxDecoration(
                border: Border.all(color: Colors.blueAccent, width: 2),
                borderRadius: BorderRadius.circular(8),
              ),
            ),
          ),
        ),

        // Speech bubble
        Positioned(
          left: 24,
          right: 24,
          top: isBottom ? position.dy - 200 : popupTop,
          child: Material(
            color: Colors.transparent,
            child: Container(
              padding: const EdgeInsets.all(20),
              decoration: BoxDecoration(
                color: Theme.of(context).colorScheme.surface,
                borderRadius: BorderRadius.circular(16),
                boxShadow: const [
                  BoxShadow(
                    color: Colors.black26,
                    blurRadius: 12,
                    offset: Offset(0, 4),
                  ),
                ],
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.min,
                children: [
                  Text(
                    step.title,
                    style: const TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 18,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  const SizedBox(height: 8),
                  Text(
                    step.description,
                    style: TextStyle(
                      fontSize: 14,
                      color: Theme.of(context).colorScheme.onSurfaceVariant,
                    ),
                  ),
                  const SizedBox(height: 24),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      TextButton(
                        onPressed: widget.onSkip,
                        child: const Text('Skip tutorial'),
                      ),
                      Row(
                        children: [
                          Text(
                            '${_currentStep + 1} of ${widget.steps.length}',
                            style: TextStyle(
                              color: Theme.of(context).colorScheme.onSurfaceVariant,
                              fontSize: 12,
                            ),
                          ),
                          const SizedBox(width: 16),
                          ElevatedButton(
                            onPressed: () {
                              if (_currentStep < widget.steps.length - 1) {
                                setState(() {
                                  _currentStep++;
                                });
                              } else {
                                widget.onComplete();
                              }
                            },
                            child: Text(_currentStep < widget.steps.length - 1 ? 'Next' : 'Finish'),
                          ),
                        ],
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
