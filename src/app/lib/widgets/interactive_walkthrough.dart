import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class InteractiveWalkthrough extends StatefulWidget {
  final Widget child;
  final List<WalkthroughStep> steps;

  const InteractiveWalkthrough({
    super.key,
    required this.child,
    required this.steps,
  });

  @override
  State<InteractiveWalkthrough> createState() => _InteractiveWalkthroughState();
}

class WalkthroughStep {
  final String title;
  final String description;

  WalkthroughStep({required this.title, required this.description});
}

class _InteractiveWalkthroughState extends State<InteractiveWalkthrough> {
  int _currentStepIndex = 0;
  bool _isActive = true;

  void _nextStep() {
    if (_currentStepIndex < widget.steps.length - 1) {
      setState(() {
        _currentStepIndex++;
      });
    } else {
      setState(() {
        _isActive = false;
      });
    }
  }

  void _skipWalkthrough() {
    setState(() {
      _isActive = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    if (!_isActive || widget.steps.isEmpty) {
      return widget.child;
    }

    final currentStep = widget.steps[_currentStepIndex];

    return Stack(
      children: [
        widget.child,
        Positioned.fill(
          child: Container(
            color: Colors.black.withValues(alpha: 0.5),
          ),
        ),
        Positioned(
          top: 100,
          left: 24,
          right: 24,
          child: Center(
            child: SizedBox(
              width: 350,
              child: GlassCard(
                child: Padding(
                  padding: const EdgeInsets.all(24.0),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Expanded(
                            child: Text(
                              currentStep.title,
                              style: const TextStyle(
                                fontFamily: 'Outfit',
                                fontSize: 20,
                                fontWeight: FontWeight.bold,
                                color: Colors.white,
                              ),
                            ),
                          ),
                          Text(
                            '${_currentStepIndex + 1} of ${widget.steps.length}',
                            style: const TextStyle(
                              fontFamily: 'Inter',
                              fontSize: 14,
                              color: Colors.white54,
                            ),
                          ),
                        ],
                      ),
                      const SizedBox(height: 12),
                      Text(
                        currentStep.description,
                        style: const TextStyle(
                          fontFamily: 'Inter',
                          fontSize: 16,
                          color: Colors.white70,
                        ),
                      ),
                      const SizedBox(height: 24),
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          TextButton(
                            onPressed: _skipWalkthrough,
                            child: const Text('Skip Tour', style: TextStyle(color: Colors.white54)),
                          ),
                          ElevatedButton(
                            onPressed: _nextStep,
                            child: Text(_currentStepIndex == widget.steps.length - 1 ? 'Finish' : 'Next'),
                          ),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ),
        ),
      ],
    );
  }
}
