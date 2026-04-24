import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class WalkthroughOverlay {
  static OverlayEntry? _currentOverlay;

  static void show(
    BuildContext context, {
    required String title,
    required String content,
    required VoidCallback onDismiss,
    VoidCallback? onNext,
    String nextLabel = 'Next',
  }) {
    if (_currentOverlay != null) return;

    _currentOverlay = OverlayEntry(
      builder: (context) {
        return Positioned.fill(
          child: GestureDetector(
            onTap: () {
              remove();
              onDismiss();
            },
            child: Material(
              color: Colors.black.withOpacity(0.5),
              child: Center(
                child: GestureDetector(
                  onTap: () {}, // Prevent dismissing when clicking inside
                  child: SizedBox(
                    width: 350,
                    child: GlassCard(
                      child: Padding(
                        padding: const EdgeInsets.all(24.0),
                        child: Column(
                          mainAxisSize: MainAxisSize.min,
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text(
                              title,
                              style: const TextStyle(
                                fontFamily: 'Outfit',
                                fontSize: 20,
                                fontWeight: FontWeight.bold,
                                color: Colors.white,
                              ),
                            ),
                            const SizedBox(height: 12),
                            Text(
                              content,
                              style: const TextStyle(
                                fontFamily: 'Inter',
                                fontSize: 14,
                                color: Colors.white70,
                              ),
                            ),
                            const SizedBox(height: 24),
                            Row(
                              mainAxisAlignment: MainAxisAlignment.end,
                              children: [
                                TextButton(
                                  onPressed: () {
                                    remove();
                                    onDismiss();
                                  },
                                  child: const Text('Skip', style: TextStyle(color: Colors.white54)),
                                ),
                                const SizedBox(width: 8),
                                if (onNext != null)
                                  ElevatedButton(
                                    onPressed: () {
                                      remove();
                                      onNext();
                                    },
                                    style: ElevatedButton.styleFrom(
                                      backgroundColor: Colors.indigoAccent,
                                      foregroundColor: Colors.white,
                                    ),
                                    child: Text(nextLabel),
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
            ),
          ),
        );
      },
    );

    Overlay.of(context).insert(_currentOverlay!);
  }

  static void remove() {
    _currentOverlay?.remove();
    _currentOverlay = null;
  }
}
