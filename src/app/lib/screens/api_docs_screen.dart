import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class ApiDocsScreen extends StatelessWidget {
  const ApiDocsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('API Documentation', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          const Text(
            'One Human Corp API (v1)',
            style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
          ),
          const SizedBox(height: 8),
          const Text('Base URL: https://api.onehumancorp.io/v1'),
          const SizedBox(height: 24),
          _ApiEndpoint(
            method: 'GET',
            path: '/api/dashboard',
            description: 'Returns the current dashboard snapshot including active agents, tasks, and recent handoffs.',
            color: Colors.blue,
          ),
          const SizedBox(height: 16),
          _ApiEndpoint(
            method: 'POST',
            path: '/api/agents/hire',
            description: 'Provisions a new AI agent with the specified role and capabilities.',
            color: Colors.green,
          ),
          const SizedBox(height: 16),
          _ApiEndpoint(
            method: 'POST',
            path: '/api/help/chat',
            description: 'Sends a query to the AI Help Agent and returns an answer along with relevant help center article links.',
            color: Colors.green,
          ),
        ],
      ),
    );
  }
}

class _ApiEndpoint extends StatelessWidget {
  final String method;
  final String path;
  final String description;
  final Color color;

  const _ApiEndpoint({
    required this.method,
    required this.path,
    required this.description,
    required this.color,
  });

  @override
  Widget build(BuildContext context) {
    return GlassCard(
      child: ExpansionTile(
        title: Row(
          children: [
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
              decoration: BoxDecoration(
                color: color.withOpacity(0.2),
                borderRadius: BorderRadius.circular(4),
                border: Border.all(color: color.withOpacity(0.5)),
              ),
              child: Text(method, style: TextStyle(color: color, fontWeight: FontWeight.bold)),
            ),
            const SizedBox(width: 12),
            Text(path, style: const TextStyle(fontWeight: FontWeight.bold, fontFamily: 'monospace')),
          ],
        ),
        children: [
          Padding(
            padding: const EdgeInsets.all(16.0),
            child: Text(description),
          ),
        ],
      ),
    );
  }
}
