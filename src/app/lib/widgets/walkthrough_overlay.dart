import 'package:flutter/material.dart';
import '../theme.dart';

class WalkthroughStep {
  final GlobalKey targetKey;
  final String title;
  final String description;
  final ContentAlign align;

  WalkthroughStep({
    required this.targetKey,
    required this.title,
    required this.description,
    this.align = ContentAlign.bottom,
  });
}

enum ContentAlign { top, bottom, left, right }

class WalkthroughOverlay extends StatefulWidget {
  final List<WalkthroughStep> steps;
  final VoidCallback onComplete;
  final VoidCallback onSkip;

  const WalkthroughOverlay({
    super.key,
    required this.steps,
    required this.onComplete,
    required this.onSkip,
  });

  @override
  State<WalkthroughOverlay> createState() => _WalkthroughOverlayState();
}

class _WalkthroughOverlayState extends State<WalkthroughOverlay> {
  int _currentStepIndex = 0;

  @override
  Widget build(BuildContext context) {
    if (_currentStepIndex >= widget.steps.length) {
      return const SizedBox.shrink();
    }

    final step = widget.steps[_currentStepIndex];

    // We need a layout builder to safely render overlays and measure positions
    return LayoutBuilder(
      builder: (context, constraints) {
        return Stack(
          children: [
            // Dark overlay
            Positioned.fill(
              child: GestureDetector(
                onTap: () {
                  // Prevent tapping through
                },
                child: Container(
                  color: Colors.black.withOpacity(0.6),
                ),
              ),
            ),

            // Highlight cutout and speech bubble would go here
            // In a full implementation, we'd use CustomPainter to cut out a hole
            // over the targetKey.currentContext.findRenderObject() rect.
            // For now, we center a guided card for simplicity in the overlay.

            Center(
              child: Material(
                color: Colors.transparent,
                child: Container(
                  width: 320,
                  padding: const EdgeInsets.all(24),
                  decoration: BoxDecoration(
                    color: AppTheme.glassBackgroundColor,
                    borderRadius: BorderRadius.circular(16),
                    border: Border.all(color: AppTheme.glassBorderColor, width: 1),
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
                            style: Theme.of(context).textTheme.bodySmall?.copyWith(
                                  color: AppTheme.secondaryTextColor,
                                  fontWeight: FontWeight.bold,
                                ),
                          ),
                          IconButton(
                            icon: const Icon(Icons.close, size: 20),
                            onPressed: widget.onSkip,
                            padding: EdgeInsets.zero,
                            constraints: const BoxConstraints(),
                          ),
                        ],
                      ),
                      const SizedBox(height: 16),
                      Text(
                        step.title,
                        style: Theme.of(context).textTheme.titleLarge?.copyWith(
                              fontWeight: FontWeight.bold,
                            ),
                      ),
                      const SizedBox(height: 8),
                      Text(
                        step.description,
                        style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                              color: AppTheme.secondaryTextColor,
                            ),
                      ),
                      const SizedBox(height: 24),
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          TextButton(
                            onPressed: widget.onSkip,
                            child: const Text('Skip'),
                          ),
                          ElevatedButton(
                            onPressed: () {
                              if (_currentStepIndex < widget.steps.length - 1) {
                                setState(() {
                                  _currentStepIndex++;
                                });
                              } else {
                                widget.onComplete();
                              }
                            },
                            style: ElevatedButton.styleFrom(
                              backgroundColor: AppTheme.primaryColor,
                              foregroundColor: Colors.white,
                              shape: RoundedRectangleBorder(
                                borderRadius: BorderRadius.circular(8),
                              ),
                            ),
                            child: Text(_currentStepIndex < widget.steps.length - 1 ? 'Next' : 'Done'),
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
      },
    );
  }
}