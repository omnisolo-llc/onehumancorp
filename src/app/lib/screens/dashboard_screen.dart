import 'dart:async';
import 'dart:math';
import 'package:flutter/material.dart';
import '../widgets/swarm_velocity_widget.dart';
import 'swarm_memory_screen.dart';

class DashboardScreen extends StatefulWidget {
  const DashboardScreen({super.key});

  @override
  State<DashboardScreen> createState() => _DashboardScreenState();
}

class _DashboardScreenState extends State<DashboardScreen> {
  late Timer _timer;
  double _taskCompletionRate = 142.5;
  double _latencyMs = 85.0;
  final Random _random = Random();

  @override
  void initState() {
    super.initState();
    _startSimulatingRealTimeMetrics();
  }

  void _startSimulatingRealTimeMetrics() {
    _timer = Timer.periodic(const Duration(seconds: 2), (timer) {
      if (mounted) {
        setState(() {
          // Simulate some natural fluctuation
          _taskCompletionRate = 140.0 + _random.nextDouble() * 10;
          _latencyMs = 80.0 + _random.nextDouble() * 20;
        });
      }
    });
  }

  @override
  void dispose() {
    _timer.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF121212), // Premium dark theme
      appBar: AppBar(
        title: const Text(
          'OHC Business Dashboard',
          style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold),
        ),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            SwarmVelocityWidget(
              taskCompletionRate: _taskCompletionRate,
              latencyMs: _latencyMs,
            ),
            const SizedBox(height: 24),
            ElevatedButton(
              key: const Key('view_memory_btn'),
              style: ElevatedButton.styleFrom(
                backgroundColor: Colors.blueAccent,
                padding: const EdgeInsets.symmetric(vertical: 16),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(12),
                ),
              ),
              onPressed: () {
                Navigator.push(
                  context,
                  MaterialPageRoute(builder: (context) => const SwarmMemoryScreen()),
                );
              },
              child: const Text(
                'View Swarm Memory State',
                style: TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 16,
                  fontWeight: FontWeight.w600,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
