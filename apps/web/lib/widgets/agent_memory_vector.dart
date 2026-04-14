import 'dart:ui';
import 'package:flutter/material.dart';

class AgentMemoryVectorWidget extends StatelessWidget {
  final List<double> vectorData;

  const AgentMemoryVectorWidget({Key? key, required this.vectorData}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.compose(
          outer: const ColorFilter.matrix(<double>[
            1.168, -0.153, -0.015, 0, 0,
            -0.046, 1.061, -0.015, 0, 0,
            -0.046, -0.152, 1.198, 0, 0,
            0, 0, 0, 1, 0,
          ]),
          inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        ),
        child: Container(
          padding: const EdgeInsets.all(16.0),
          decoration: BoxDecoration(
            color: const Color.fromRGBO(255, 255, 255, 0.05),
            borderRadius: BorderRadius.circular(16),
            border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text(
                'AutoDream Memory Vector',
                style: TextStyle(
                  color: Colors.white,
                  fontWeight: FontWeight.bold,
                  fontFamily: 'Outfit',
                  fontSize: 16,
                ),
              ),
              const SizedBox(height: 12),
              Wrap(
                spacing: 4,
                runSpacing: 4,
                children: vectorData.map((val) {
                  final int intensity = (val.abs() * 255).clamp(0, 255).toInt();
                  final Color blockColor = val >= 0
                      ? Color.fromRGBO(0, intensity, 0, 0.8)
                      : Color.fromRGBO(intensity, 0, 0, 0.8);
                  return Tooltip(
                    message: val.toStringAsFixed(4),
                    child: Container(
                      width: 12,
                      height: 12,
                      decoration: BoxDecoration(
                        color: blockColor,
                        borderRadius: BorderRadius.circular(2),
                      ),
                    ),
                  );
                }).toList(),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
