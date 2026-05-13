import 'package:flutter/material.dart';

class WalkthroughHighlight extends StatelessWidget {
  final Widget child;
  final bool showHighlight;
  final String speechBubbleText;
  final VoidCallback onDismiss;

  const WalkthroughHighlight({
    super.key,
    required this.child,
    required this.showHighlight,
    required this.speechBubbleText,
    required this.onDismiss,
  });

  @override
  Widget build(BuildContext context) {
    if (!showHighlight) return child;

    return Stack(
      clipBehavior: Clip.none,
      children: [
        Container(
          decoration: BoxDecoration(
            border: Border.all(color: const Color(0xFF6B4EFF), width: 3),
            borderRadius: BorderRadius.circular(8),
            boxShadow: [
              BoxShadow(
                color: const Color(0xFF6B4EFF).withAlpha(128),
                blurRadius: 10,
                spreadRadius: 2,
              ),
            ],
          ),
          child: child,
        ),
        Positioned(
          top: -45,
          left: 0,
          child: Material(
            color: Colors.transparent,
            child: Container(
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
              decoration: const BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.only(
                  topLeft: Radius.circular(16),
                  topRight: Radius.circular(16),
                  bottomRight: Radius.circular(16),
                  bottomLeft: Radius.circular(0), // Speech bubble tail
                ),
                boxShadow: [
                  BoxShadow(color: Colors.black26, blurRadius: 4, offset: Offset(2, 2)),
                ],
              ),
              child: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Text(
                    speechBubbleText,
                    style: const TextStyle(color: Colors.black, fontWeight: FontWeight.bold, fontSize: 14),
                  ),
                  const SizedBox(width: 10),
                  GestureDetector(
                    onTap: onDismiss,
                    child: const Icon(Icons.close, size: 16, color: Colors.black54),
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
