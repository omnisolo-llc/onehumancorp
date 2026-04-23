import 'dart:ui';
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
  final Widget child;
  final bool startAutomatically;
  final VoidCallback? onComplete;

  const InteractiveWalkthrough({
    super.key,
    required this.steps,
    required this.child,
    this.startAutomatically = false,
    this.onComplete,
  });

  @override
  State<InteractiveWalkthrough> createState() => InteractiveWalkthroughState();
}

class InteractiveWalkthroughState extends State<InteractiveWalkthrough> {
  bool _isActive = false;
  int _currentStepIndex = 0;
  OverlayEntry? _overlayEntry;

  @override
  void initState() {
    super.initState();
    if (widget.startAutomatically) {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        startWalkthrough();
      });
    }
  }

  @override
  void dispose() {
    _removeOverlay();
    super.dispose();
  }

  void startWalkthrough() {
    if (widget.steps.isEmpty) return;
    setState(() {
      _isActive = true;
      _currentStepIndex = 0;
    });
    _showOverlay();
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
      _isActive = false;
    });
    _removeOverlay();
    if (widget.onComplete != null) {
      widget.onComplete!();
    }
  }

  void _removeOverlay() {
    _overlayEntry?.remove();
    _overlayEntry = null;
  }

  void _showOverlay() {
    _removeOverlay();

    if (!_isActive || _currentStepIndex >= widget.steps.length) return;

    final step = widget.steps[_currentStepIndex];
    final targetContext = step.targetKey.currentContext;

    if (targetContext == null) {
      _overlayEntry = OverlayEntry(
        builder: (context) => _buildCenteredOverlay(step),
      );
    } else {
      final renderBox = targetContext.findRenderObject() as RenderBox;
      final size = renderBox.size;
      final offset = renderBox.localToGlobal(Offset.zero);

      _overlayEntry = OverlayEntry(
        builder: (context) => _buildTargetedOverlay(step, size, offset),
      );
    }

    Overlay.of(context).insert(_overlayEntry!);
  }

  Widget _buildCenteredOverlay(WalkthroughStep step) {
    return Material(
      color: Colors.black.withValues(alpha: 0.6),
      child: Center(
        child: SizedBox(
          width: 350,
          child: _buildSpeechBubble(step),
        ),
      ),
    );
  }

  Widget _buildTargetedOverlay(WalkthroughStep step, Size size, Offset offset) {
    final highlightRect = Rect.fromLTWH(
      offset.dx - 8,
      offset.dy - 8,
      size.width + 16,
      size.height + 16,
    );

    return Material(
      color: Colors.transparent,
      child: Stack(
        children: [
          ColorFiltered(
            colorFilter: ColorFilter.mode(
              Colors.black.withValues(alpha: 0.7),
              BlendMode.srcOut,
            ),
            child: Stack(
              children: [
                Container(
                  decoration: const BoxDecoration(
                    color: Colors.black,
                    backgroundBlendMode: BlendMode.dstOut,
                  ),
                ),
                Positioned.fromRect(
                  rect: highlightRect,
                  child: Container(
                    decoration: BoxDecoration(
                      color: Colors.white,
                      borderRadius: BorderRadius.circular(16),
                    ),
                  ),
                ),
              ],
            ),
          ),

          Positioned(
            top: highlightRect.bottom + 16,
            left: MediaQuery.of(context).size.width / 2 > offset.dx ? offset.dx : null,
            right: MediaQuery.of(context).size.width / 2 <= offset.dx ? MediaQuery.of(context).size.width - offset.dx - size.width : null,
            child: SizedBox(
              width: 320,
              child: _buildSpeechBubble(step),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSpeechBubble(WalkthroughStep step) {
    final isLastStep = _currentStepIndex == widget.steps.length - 1;

    return GlassCard(
      padding: const EdgeInsets.all(20),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(Icons.tips_and_updates, color: Theme.of(context).colorScheme.primary),
              const SizedBox(width: 8),
              Text(
                'Step ${_currentStepIndex + 1} of ${widget.steps.length}',
                style: const TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 12,
                  color: Colors.grey,
                ),
              ),
            ],
          ),
          const SizedBox(height: 12),
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
            style: const TextStyle(
              fontFamily: 'Outfit',
              fontSize: 15,
            ),
          ),
          const SizedBox(height: 20),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              TextButton(
                onPressed: _endWalkthrough,
                child: const Text('Skip', style: TextStyle(fontFamily: 'Outfit')),
              ),
              ElevatedButton(
                onPressed: _nextStep,
                style: ElevatedButton.styleFrom(
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(20),
                  ),
                ),
                child: Text(isLastStep ? 'Finish' : 'Next', style: const TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
              ),
            ],
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return widget.child;
  }
}
