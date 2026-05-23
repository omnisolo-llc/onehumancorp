import 'package:flutter/material.dart';

class ApiDocsScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF1E293B), // Dark background for "advanced" tech feel
      appBar: AppBar(
        title: const Text('API Reference (Advanced)', style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontWeight: FontWeight.bold)),
        backgroundColor: const Color(0xFF0F172A),
        elevation: 0,
        iconTheme: const IconThemeData(color: Colors.white),
      ),
      body: Center(
        child: Container(
          maxWidth: 800,
          padding: const EdgeInsets.all(24),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text(
                'OHC API Reference',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 28,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const SizedBox(height: 8),
              const Text(
                'For advanced users who want to connect custom tools or checkout systems directly to their OHC account.',
                style: TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 16,
                  color: Color(0xFF94A3B8),
                ),
              ),
              const SizedBox(height: 32),
              Expanded(
                child: ListView(
                  children: [
                    _buildEndpointCard(
                      method: 'POST',
                      endpoint: '/api/orgs/register',
                      description: 'Registers a new tenant organization in the multi-tenant OHC environment.',
                    ),
                    _buildEndpointCard(
                      method: 'POST',
                      endpoint: '/api/agents/task',
                      description: 'Dispatches a new task to the AI Swarm Orchestrator.',
                    ),
                    _buildEndpointCard(
                      method: 'GET',
                      endpoint: '/api/agents/status',
                      description: 'Retrieves the current status of the agent swarm workforce.',
                    ),
                  ],
                ),
              ),
              const SizedBox(height: 16),
              Center(
                child: ElevatedButton.icon(
                  onPressed: () {
                    ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Opening full OpenAPI Spec in browser...')));
                  },
                  icon: const Icon(Icons.open_in_new),
                  label: const Text('View Full Swagger UI'),
                  style: ElevatedButton.styleFrom(
                    backgroundColor: const Color(0xFF0EA5E9),
                    foregroundColor: Colors.white,
                    padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                  ),
                ),
              )
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildEndpointCard({required String method, required String endpoint, required String description}) {
    Color methodColor;
    switch (method) {
      case 'GET':
        methodColor = Colors.green;
        break;
      case 'POST':
        methodColor = Colors.blue;
        break;
      default:
        methodColor = Colors.grey;
    }

    return Container(
      margin: const EdgeInsets.only(bottom: 16),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: const Color(0xFF334155),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: const Color(0xFF475569)),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                decoration: BoxDecoration(
                  color: methodColor.withOpacity(0.2),
                  borderRadius: BorderRadius.circular(4),
                ),
                child: Text(
                  method,
                  style: TextStyle(color: methodColor, fontWeight: FontWeight.bold, fontFamily: 'monospace'),
                ),
              ),
              const SizedBox(width: 12),
              Text(
                endpoint,
                style: const TextStyle(color: Colors.white, fontFamily: 'monospace', fontSize: 16),
              ),
            ],
          ),
          const SizedBox(height: 12),
          Text(
            description,
            style: const TextStyle(color: Color(0xFFCBD5E1), fontFamily: 'Inter', fontSize: 14),
          )
        ],
      ),
    );
  }
}

extension on Container {
  get maxWidth => 800.0;
}
