import 'dart:ui';
import 'package:flutter/material.dart';

class TokenSavingsWidget extends StatelessWidget {
  final double savingsAmount;

  const TokenSavingsWidget({Key? key, required this.savingsAmount}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          padding: const EdgeInsets.all(20),
          decoration: BoxDecoration(
            color: const Color.fromRGBO(255, 255, 255, 0.03),
            border: Border.all(color: const Color.fromRGBO(255, 255, 255, 0.1)),
          ),
          child: Text(
            'Token Savings: \$${savingsAmount.toStringAsFixed(2)}',
            style: const TextStyle(
              fontFamily: 'Outfit',
              color: Colors.white,
              fontSize: 18,
            ),
          ),
        ),
      ),
    );
  }
}
