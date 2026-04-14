import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/swarm/swarm_observability_dashboard.dart';

class SwarmObservabilityScreen extends StatelessWidget {
  const SwarmObservabilityScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Swarm Observability')),
      body: const Padding(
        padding: EdgeInsets.all(16.0),
        child: SwarmObservabilityDashboard(),
      ),
    );
  }
}
