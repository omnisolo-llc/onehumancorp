import 'dart:math';
import 'package:flutter/material.dart';
import '../widgets/vector_memory_visualizer.dart';

class SwarmMemoryScreen extends StatefulWidget {
  const SwarmMemoryScreen({super.key});

  @override
  State<SwarmMemoryScreen> createState() => _SwarmMemoryScreenState();
}

class _SwarmMemoryScreenState extends State<SwarmMemoryScreen> {
  late List<double> _mockVectorState;
  bool _isPulsing = false;

  @override
  void initState() {
    super.initState();
    _generateMockState();
  }

  void _generateMockState() {
    final random = Random();
    // Simulate a 1536d pgvector state using a subset for visualization
    _mockVectorState = List.generate(150, (_) => random.nextDouble() * 2 - 1);
  }

  void _simulateTeammateEvent() {
    setState(() {
      _generateMockState();
      _isPulsing = true;
    });
    Future.delayed(const Duration(milliseconds: 1500), () {
      if (mounted) {
        setState(() {
          _isPulsing = false;
        });
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF1A1A2E), // Deep space theme for memory
      appBar: AppBar(
        title: const Text(
          'Swarm Memory State',
          style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold),
        ),
        backgroundColor: Colors.transparent,
        elevation: 0,
        iconTheme: const IconThemeData(color: Colors.white),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            const Text(
              'Vector Space 1536-D',
              style: TextStyle(
                fontFamily: 'Inter',
                fontSize: 18,
                color: Colors.white70,
              ),
            ),
            const SizedBox(height: 16),
            VectorMemoryVisualizerWidget(
              vectorState: _mockVectorState,
              isPulsing: _isPulsing,
            ),
            const Spacer(),
            ElevatedButton.icon(
              key: const Key('broadcast_event_btn'),
              icon: const Icon(Icons.hub),
              label: const Text(
                'Broadcast High-Priority Event',
                style: TextStyle(fontFamily: 'Inter', fontSize: 16),
              ),
              style: ElevatedButton.styleFrom(
                backgroundColor: Colors.pinkAccent,
                padding: const EdgeInsets.symmetric(vertical: 16),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(12),
                ),
              ),
              onPressed: _simulateTeammateEvent,
            ),
            const SizedBox(height: 32),
          ],
        ),
      ),
    );
  }
}
