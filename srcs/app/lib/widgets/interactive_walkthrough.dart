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

class InteractiveWalkthrough {
  static OverlayEntry? _currentEntry;
  static int _currentIndex = 0;
  static List<WalkthroughStep> _steps = [];

  static void start(BuildContext context, List<WalkthroughStep> steps) {
    if (steps.isEmpty) return;
    _steps = steps;
    _currentIndex = 0;
    _showStep(context);
  }

  static void _showStep(BuildContext context) {
    _currentEntry?.remove();
    if (_currentIndex >= _steps.length) {
      _currentEntry = null;
      return;
    }

    final step = _steps[_currentIndex];
    final renderBox = step.targetKey.currentContext?.findRenderObject() as RenderBox?;
    if (renderBox == null) {
      // Key not found, move to next step or exit
      _currentIndex++;
      _showStep(context);
      return;
    }

    final offset = renderBox.localToGlobal(Offset.zero);
    final size = renderBox.size;

    _currentEntry = OverlayEntry(
      builder: (context) => Stack(
        children: [
          // Semi-transparent background
          Positioned.fill(
            child: GestureDetector(
              onTap: () {
                _currentIndex++;
                _showStep(context);
              },
              child: Container(
                color: Colors.black.withOpacity(0.5),
              ),
            ),
          ),
          // Highlight
          Positioned(
            left: offset.dx - 4,
            top: offset.dy - 4,
            width: size.width + 8,
            height: size.height + 8,
            child: IgnorePointer(
              child: Container(
                decoration: BoxDecoration(
                  border: Border.all(color: Theme.of(context).colorScheme.primary, width: 3),
                  borderRadius: BorderRadius.circular(8),
                ),
              ),
            ),
          ),
          // Speech bubble
          Positioned(
            left: offset.dx,
            top: offset.dy + size.height + 16,
            child: Material(
              color: Colors.transparent,
              child: Container(
                width: 250,
                padding: const EdgeInsets.all(16),
                decoration: BoxDecoration(
                  color: Theme.of(context).colorScheme.surface,
                  borderRadius: BorderRadius.circular(12),
                  boxShadow: [
                    BoxShadow(
                      color: Colors.black.withOpacity(0.1),
                      blurRadius: 10,
                      offset: const Offset(0, 5),
                    )
                  ],
                ),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      step.title,
                      style: const TextStyle(
                        fontFamily: 'Outfit',
                        fontWeight: FontWeight.bold,
                        fontSize: 16,
                      ),
                    ),
                    const SizedBox(height: 8),
                    Text(
                      step.description,
                      style: const TextStyle(
                        fontFamily: 'Inter',
                        fontSize: 14,
                      ),
                    ),
                    const SizedBox(height: 12),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Text(
                          '${_currentIndex + 1} of ${_steps.length}',
                          style: TextStyle(
                            color: Theme.of(context).colorScheme.onSurface.withOpacity(0.5),
                            fontSize: 12,
                          ),
                        ),
                        TextButton(
                          onPressed: () {
                            _currentIndex++;
                            _showStep(context);
                          },
                          child: Text(_currentIndex < _steps.length - 1 ? 'Next' : 'Done'),
                        )
                      ],
                    )
                  ],
                ),
              ),
            ),
          )
        ],
      ),
    );

    Overlay.of(context).insert(_currentEntry!);
  }

  static void stop() {
    _currentEntry?.remove();
    _currentEntry = null;
  }
}
