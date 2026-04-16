import 'package:flutter/material.dart';
import 'dart:ui';
import 'package:http/http.dart' as http;
import 'dart:convert';

class ABTestWidget extends StatefulWidget {
  final String experimentId;
  final String variant;
  final Widget child;

  const ABTestWidget({
    Key? key,
    required this.experimentId,
    required this.variant,
    required this.child,
  }) : super(key: key);

  @override
  _ABTestWidgetState createState() => _ABTestWidgetState();
}

class _ABTestWidgetState extends State<ABTestWidget> {
  @override
  void initState() {
    super.initState();
    _recordImpression();
  }

  Future<void> _recordImpression() async {
    try {
      await http.post(
        Uri.parse('http://localhost:8080/api/v1/growth/ab_test/impression'),
        headers: {
          'Content-Type': 'application/json',
          'X-Spiffe-Id': 'spiffe://example.org/frontend',
        },
        body: jsonEncode({
          'experiment_id': widget.experimentId,
          'variant': widget.variant,
        }),
      );
    } catch (e) {
      print('Failed to record impression: $e');
    }
  }

  Future<void> _recordConversion() async {
    try {
      await http.post(
        Uri.parse('http://localhost:8080/api/v1/growth/ab_test/conversion'),
        headers: {
          'Content-Type': 'application/json',
          'X-Spiffe-Id': 'spiffe://example.org/frontend',
        },
        body: jsonEncode({
          'experiment_id': widget.experimentId,
          'variant': widget.variant,
        }),
      );
    } catch (e) {
      print('Failed to record conversion: $e');
    }
  }

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: () {
        _recordConversion();
      },
      child: ClipRRect(
        borderRadius: BorderRadius.circular(12),
        child: BackdropFilter(
          filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
          child: Container(
            padding: const EdgeInsets.all(20),
            decoration: BoxDecoration(
              color: const Color.fromRGBO(255, 255, 255, 0.03),
              borderRadius: BorderRadius.circular(12),
              border: Border.all(
                color: const Color.fromRGBO(255, 255, 255, 0.1),
              ),
            ),
            child: widget.child,
          ),
        ),
      ),
    );
  }
}
