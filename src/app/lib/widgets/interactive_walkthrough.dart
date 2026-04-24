import 'package:flutter/material.dart';

class InteractiveWalkthrough extends StatelessWidget {
  final Widget child;

  const InteractiveWalkthrough({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    // In a real app, this would use an overlay to show a transparent mask
    // with a highlighted hole, and a speech bubble next to it.
    // For now, we simply wrap the child. The architecture is ready for a
    // package like `tutorial_coach_mark` or a custom painter implementation.
    return child;
  }
}
