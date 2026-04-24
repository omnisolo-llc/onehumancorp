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
        padding: const EdgeInsets.all(24),
        children: [
          const Text(
            'Advanced: OHC Developer API',
            style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
          ),
          const SizedBox(height: 8),
          const Text(
            'Use our REST API to integrate OHC with your existing tools. All requests require an OHC-Token header.',
            style: TextStyle(fontFamily: 'Inter', color: Colors.white70),
          ),
          const SizedBox(height: 24),
          _ApiEndpoint(
            method: 'GET',
            path: '/api/dashboard',
            description: 'Fetch the latest business snapshot, including active agents and metrics.',
          ),
          _ApiEndpoint(
            method: 'GET',
            path: '/api/agents',
            description: 'List all AI agents currently hired for your organization.',
          ),
          _ApiEndpoint(
            method: 'POST',
            path: '/api/agents/scale',
            description: 'Scale the number of agents for a specific role.',
            payload: '{"role": "marketing", "count": 2}',
          ),
          _ApiEndpoint(
            method: 'GET',
            path: '/api/help/articles',
            description: 'Fetch help center content programmatically.',
          ),
          _ApiEndpoint(
            method: 'POST',
            path: '/api/help/chat',
            description: 'Send a message to the AI Help Assistant.',
            payload: '{"message": "How do I set up Stripe?"}',
          ),
          const SizedBox(height: 40),
          const GlassCard(
            child: Padding(
              padding: const EdgeInsets.all(16.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text('Authentication', style: TextStyle(fontWeight: FontWeight.bold, fontSize: 18)),
                  SizedBox(height: 8),
                  Text(
                    'OHC uses Bearer tokens for authentication. Include your token in the "Authorization" header or "OHC-Token" custom header.',
                    style: TextStyle(fontSize: 14, color: Colors.white70),
                  ),
                ],
              ),
            ),
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
  final String? payload;

  const _ApiEndpoint({
    required this.method,
    required this.path,
    required this.description,
    this.payload,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(bottom: 16),
      child: GlassCard(
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                children: [
                  Container(
                    padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                    decoration: BoxDecoration(
                      color: _getMethodColor(method).withValues(alpha: 0.2),
                      borderRadius: BorderRadius.circular(4),
                      border: Border.all(color: _getMethodColor(method)),
                    ),
                    child: Text(
                      method,
                      style: TextStyle(color: _getMethodColor(method), fontWeight: FontWeight.bold, fontSize: 12),
                    ),
                  ),
                  const SizedBox(width: 12),
                  Text(path, style: const TextStyle(fontFamily: 'monospace', fontWeight: FontWeight.bold)),
                ],
              ),
              const SizedBox(height: 12),
              Text(description, style: const TextStyle(fontSize: 14, color: Colors.white70)),
              if (payload != null) ...[
                const SizedBox(height: 12),
                const Text('Example Payload:', style: TextStyle(fontSize: 12, fontWeight: FontWeight.bold)),
                const SizedBox(height: 4),
                Container(
                  width: double.infinity,
                  padding: const EdgeInsets.all(8),
                  decoration: BoxDecoration(
                    color: Colors.black26,
                    borderRadius: BorderRadius.circular(4),
                  ),
                  child: Text(
                    payload!,
                    style: const TextStyle(fontFamily: 'monospace', fontSize: 12, color: Colors.cyanAccent),
                  ),
                ),
              ],
            ],
          ),
        ),
      ),
    );
  }

  Color _getMethodColor(String method) {
    switch (method) {
      case 'GET': return Colors.greenAccent;
      case 'POST': return Colors.blueAccent;
      case 'PUT': return Colors.orangeAccent;
      case 'DELETE': return Colors.redAccent;
      default: return Colors.grey;
    }
  }
}
