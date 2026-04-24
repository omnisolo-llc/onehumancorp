import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class ApiDocsScreen extends StatelessWidget {
  const ApiDocsScreen({super.key});

  Widget _buildEndpoint(String method, String path, String description) {
    Color methodColor;
    switch (method) {
      case 'GET':
        methodColor = Colors.green;
        break;
      case 'POST':
        methodColor = Colors.blue;
        break;
      case 'PUT':
        methodColor = Colors.orange;
        break;
      case 'DELETE':
        methodColor = Colors.red;
        break;
      default:
        methodColor = Colors.grey;
    }

    return Container(
      margin: const EdgeInsets.only(bottom: 16),
      child: GlassCard(
        child: ExpansionTile(
          title: Row(
            children: [
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                decoration: BoxDecoration(
                  color: methodColor.withOpacity(0.2),
                  borderRadius: BorderRadius.circular(4),
                  border: Border.all(color: methodColor.withOpacity(0.5)),
                ),
                child: Text(
                  method,
                  style: TextStyle(
                    color: methodColor,
                    fontWeight: FontWeight.bold,
                    fontFamily: 'Inter',
                  ),
                ),
              ),
              const SizedBox(width: 12),
              Text(
                path,
                style: const TextStyle(
                  color: Colors.white,
                  fontFamily: 'monospace',
                  fontSize: 16,
                ),
              ),
            ],
          ),
          children: [
            Padding(
              padding: const EdgeInsets.all(16.0),
              child: Align(
                alignment: Alignment.centerLeft,
                child: Text(
                  description,
                  style: const TextStyle(
                    color: Colors.white70,
                    fontFamily: 'Inter',
                    fontSize: 15,
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      appBar: AppBar(
        title: const Text('API Reference (Advanced)', style: TextStyle(fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      body: ListView(
        padding: const EdgeInsets.all(24.0),
        children: [
          const Text(
            'Connect your custom apps',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 24,
              fontWeight: FontWeight.bold,
              color: Colors.white,
            ),
          ),
          const SizedBox(height: 16),
          const Text(
            'Use these APIs to integrate OHC with your own custom checkout flow, website, or mobile app. You will need your API key from the Settings page.',
            style: TextStyle(
              fontFamily: 'Inter',
              fontSize: 16,
              color: Colors.white70,
              height: 1.5,
            ),
          ),
          const SizedBox(height: 32),
          _buildEndpoint('GET', '/api/v1/products', 'Retrieve a list of your products or services.'),
          _buildEndpoint('POST', '/api/v1/orders', 'Create a new order from a custom checkout.'),
          _buildEndpoint('GET', '/api/v1/agents', 'List the AI agents currently active in your swarm.'),
          _buildEndpoint('POST', '/api/v1/tasks', 'Assign a custom task directly to an AI agent.'),
        ],
      ),
    );
  }
}
