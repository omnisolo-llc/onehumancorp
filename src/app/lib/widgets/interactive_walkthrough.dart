import 'package:flutter/material.dart';

class InteractiveWalkthrough extends StatefulWidget {
  final Widget child;
  const InteractiveWalkthrough({super.key, required this.child});
  void startWalkthrough() {}
  @override
  State<InteractiveWalkthrough> createState() => _InteractiveWalkthroughState();
}

class _InteractiveWalkthroughState extends State<InteractiveWalkthrough> {
  @override
  Widget build(BuildContext context) {
    return widget.child;
  }
}

void showDashboardWalkthrough(BuildContext context) {
  OverlayState? overlayState = Overlay.of(context);
  if (overlayState == null) return;
  OverlayEntry? overlayEntry;

  overlayEntry = OverlayEntry(
    builder: (context) {
      return Stack(
        children: [
          Positioned(
            top: kToolbarHeight + 20,
            right: 80,
            child: Material(
              color: Colors.transparent,
              child: CustomPaint(
                painter: _BubblePainter(),
                child: Container(
                  padding: const EdgeInsets.all(16),
                  width: 250,
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text('Step 1 of 3', style: TextStyle(color: Colors.grey, fontSize: 12)),
                      const SizedBox(height: 8),
                      const Text('The System Observability Panel shows you how healthy your operations are.', style: TextStyle(fontFamily: 'Inter')),
                      const SizedBox(height: 12),
                      ElevatedButton(
                        onPressed: () => overlayEntry?.remove(),
                        child: const Text('Got it!'),
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ),
        ],
      );
    },
  );
  overlayState.insert(overlayEntry);
}

class _BubblePainter extends CustomPainter {
  @override
  void paint(Canvas canvas, Size size) {
    final paint = Paint()
      ..color = Colors.black87
      ..style = PaintingStyle.fill;

    final rrect = RRect.fromRectAndRadius(Rect.fromLTWH(0, 0, size.width, size.height), const Radius.circular(12));
    canvas.drawRRect(rrect, paint);
  }
  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) => false;
}
