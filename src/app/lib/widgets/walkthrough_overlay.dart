import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class WalkthroughOverlay extends StatelessWidget {
  final String text;
  final VoidCallback onNext;
  final VoidCallback onDismiss;
  final GlobalKey targetKey;

  const WalkthroughOverlay({
    super.key,
    required this.text,
    required this.onNext,
    required this.onDismiss,
    required this.targetKey,
  });

  @override
  Widget build(BuildContext context) {
    RenderBox? targetBox;
    Offset targetPosition = Offset.zero;
    Size targetSize = Size.zero;

    if (targetKey.currentContext != null) {
      targetBox = targetKey.currentContext!.findRenderObject() as RenderBox?;
      if (targetBox != null) {
        targetPosition = targetBox.localToGlobal(Offset.zero);
        targetSize = targetBox.size;
      }
    }

    return Stack(
      children: [
        GestureDetector(
          onTap: onDismiss,
          child: Container(color: Colors.black.withOpacity(0.5)),
        ),
        if (targetBox != null)
          Positioned(
            left: targetPosition.dx - 4,
            top: targetPosition.dy - 4,
            width: targetSize.width + 8,
            height: targetSize.height + 8,
            child: Container(
              decoration: BoxDecoration(
                border: Border.all(color: Theme.of(context).colorScheme.primary, width: 3),
                borderRadius: BorderRadius.circular(8),
              ),
            ),
          ),
        if (targetBox != null)
          Positioned(
            left: targetPosition.dx,
            top: targetPosition.dy + targetSize.height + 16,
            child: Material(
              color: Colors.transparent,
              child: SizedBox(
                width: 280,
                child: GlassCard(
                  child: Padding(
                    padding: const EdgeInsets.all(16.0),
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(text, style: const TextStyle(fontSize: 16)),
                        const SizedBox(height: 16),
                        Row(
                          mainAxisAlignment: MainAxisAlignment.end,
                          children: [
                            TextButton(onPressed: onDismiss, child: const Text('Skip')),
                            const SizedBox(width: 8),
                            FilledButton(onPressed: onNext, child: const Text('Next')),
                          ],
                        )
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
