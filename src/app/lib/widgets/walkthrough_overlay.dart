import 'package:flutter/material.dart';

class WalkthroughStep {
  final GlobalKey targetKey;
  final String text;
  final String title;

  WalkthroughStep({required this.targetKey, required this.title, required this.text});
}

class WalkthroughService {
  static final WalkthroughService _instance = WalkthroughService._internal();

  factory WalkthroughService() {
    return _instance;
  }

  WalkthroughService._internal();

  OverlayEntry? _overlayEntry;
  List<WalkthroughStep> _steps = [];
  int _currentStepIndex = 0;

  void startWalkthrough(BuildContext context, List<WalkthroughStep> steps) {
    if (steps.isEmpty) return;
    _steps = steps;
    _currentStepIndex = 0;
    _showOverlay(context);
  }

  void _showOverlay(BuildContext context) {
    _overlayEntry?.remove();
    _overlayEntry = _createOverlayEntry(context);
    Overlay.of(context).insert(_overlayEntry!);
  }

  void _nextStep(BuildContext context) {
    if (_currentStepIndex < _steps.length - 1) {
      _currentStepIndex++;
      _showOverlay(context);
    } else {
      stopWalkthrough();
    }
  }

  void stopWalkthrough() {
    _overlayEntry?.remove();
    _overlayEntry = null;
    _steps = [];
    _currentStepIndex = 0;
  }

  OverlayEntry _createOverlayEntry(BuildContext context) {
    final step = _steps[_currentStepIndex];
    final RenderBox? renderBox = step.targetKey.currentContext?.findRenderObject() as RenderBox?;

    Offset targetPosition = Offset.zero;
    Size targetSize = Size.zero;

    if (renderBox != null) {
      targetPosition = renderBox.localToGlobal(Offset.zero);
      targetSize = renderBox.size;
    }

    return OverlayEntry(
      builder: (context) {
        return Stack(
          children: [
            // Darken background
            Positioned.fill(
              child: GestureDetector(
                onTap: () => stopWalkthrough(),
                child: Container(
                  color: Colors.black.withOpacity(0.5),
                ),
              ),
            ),

            // Highlight target (approximate with a transparent hole if possible, or just draw near it)
            if (renderBox != null)
              Positioned(
                left: targetPosition.dx - 4,
                top: targetPosition.dy - 4,
                width: targetSize.width + 8,
                height: targetSize.height + 8,
                child: IgnorePointer(
                  child: Container(
                    decoration: BoxDecoration(
                      border: Border.all(color: Colors.blueAccent, width: 4),
                      borderRadius: BorderRadius.circular(8),
                    ),
                  ),
                ),
              ),

            // Speech bubble
            Positioned(
              left: (targetPosition.dx + targetSize.width / 2) - 125, // Center approximately
              top: targetPosition.dy + targetSize.height + 16,
              width: 250,
              child: Material(
                color: Colors.transparent,
                child: Container(
                  padding: EdgeInsets.all(16),
                  decoration: BoxDecoration(
                    color: Colors.white,
                    borderRadius: BorderRadius.circular(12),
                    boxShadow: [
                      BoxShadow(color: Colors.black26, blurRadius: 10, offset: Offset(0, 4)),
                    ],
                  ),
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        step.title,
                        style: TextStyle(fontFamily: 'Outfit', fontSize: 16, fontWeight: FontWeight.bold, color: Colors.black87),
                      ),
                      SizedBox(height: 8),
                      Text(
                        step.text,
                        style: TextStyle(fontFamily: 'Inter', fontSize: 14, color: Colors.black54),
                      ),
                      SizedBox(height: 16),
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Text(
                            '${_currentStepIndex + 1} of ${_steps.length}',
                            style: TextStyle(fontSize: 12, color: Colors.grey),
                          ),
                          Row(
                            children: [
                              TextButton(
                                onPressed: () => stopWalkthrough(),
                                child: Text('Skip', style: TextStyle(color: Colors.grey)),
                              ),
                              ElevatedButton(
                                onPressed: () => _nextStep(context),
                                style: ElevatedButton.styleFrom(
                                  backgroundColor: Colors.blueAccent,
                                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                                ),
                                child: Text(_currentStepIndex < _steps.length - 1 ? 'Next' : 'Done'),
                              ),
                            ],
                          )
                        ],
                      )
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
