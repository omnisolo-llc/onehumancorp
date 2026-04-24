import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

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

class InteractiveWalkthrough extends StatefulWidget {
  final List<WalkthroughStep> steps;
  final VoidCallback onComplete;

  const InteractiveWalkthrough({
    super.key,
    required this.steps,
    required this.onComplete,
  });

  @override
  State<InteractiveWalkthrough> createState() => _InteractiveWalkthroughState();
}

class _InteractiveWalkthroughState extends State<InteractiveWalkthrough> {
  int _currentStepIndex = 0;

  @override
  Widget build(BuildContext context) {
    if (widget.steps.isEmpty) return const SizedBox.shrink();

    final step = widget.steps[_currentStepIndex];
    final RenderBox? renderBox = step.targetKey.currentContext?.findRenderObject() as RenderBox?;

    if (renderBox == null) {
      return const SizedBox.shrink();
    }

    final position = renderBox.localToGlobal(Offset.zero);
    final size = renderBox.size;

    return Stack(
      children: [
        GestureDetector(
          onTap: widget.onComplete,
          child: Container(color: Colors.black.withValues(alpha: 0.5)),
        ),
        Positioned(
          left: position.dx - 4,
          top: position.dy - 4,
          child: Container(
            width: size.width + 8,
            height: size.height + 8,
            decoration: BoxDecoration(
              border: Border.all(color: Colors.cyanAccent, width: 2),
              borderRadius: BorderRadius.circular(8),
            ),
          ),
        ),
        Positioned(
          left: position.dx,
          top: position.dy + size.height + 12,
          child: SizedBox(
            width: 280,
            child: GlassCard(
              child: Padding(
                padding: const EdgeInsets.all(16.0),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(step.title, style: const TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold, fontSize: 16)),
                    const SizedBox(height: 8),
                    Text(step.description, style: const TextStyle(fontFamily: 'Inter', fontSize: 14)),
                    const SizedBox(height: 16),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Text('${_currentStepIndex + 1}/${widget.steps.length}', style: const TextStyle(fontSize: 12)),
                        TextButton(
                          onPressed: () {
                            if (_currentStepIndex < widget.steps.length - 1) {
                              setState(() => _currentStepIndex++);
                            } else {
                              widget.onComplete();
                            }
                          },
                          child: Text(_currentStepIndex < widget.steps.length - 1 ? 'Next' : 'Finish'),
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
    );
  }
}
