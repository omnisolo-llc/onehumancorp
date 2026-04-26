import 'package:flutter/material.dart';

class ApiDocumentationScreen extends StatelessWidget {
  const ApiDocumentationScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('OneHumanCorp API', style: TextStyle(fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'API Reference',
              style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
            ),
            const SizedBox(height: 16),
            const Text('Welcome to the advanced API documentation. Use these endpoints to connect custom integrations.'),
            const SizedBox(height: 24),
            _buildEndpointTile(
              method: 'GET',
              methodColor: Colors.blue,
              path: '/api/v1/products',
              description: 'List all products',
            ),
             _buildEndpointTile(
              method: 'POST',
              methodColor: Colors.green,
              path: '/api/v1/orders',
              description: 'Create a new order',
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildEndpointTile({required String method, required Color methodColor, required String path, required String description}) {
    return Card(
      margin: const EdgeInsets.only(bottom: 12),
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
                    color: methodColor.withValues(alpha: 0.2),
                    borderRadius: BorderRadius.circular(4),
                  ),
                  child: Text(
                    method,
                    style: TextStyle(color: methodColor, fontWeight: FontWeight.bold),
                  ),
                ),
                const SizedBox(width: 12),
                Text(
                  path,
                  style: const TextStyle(fontFamily: 'monospace', fontSize: 16),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Text(description, style: const TextStyle(color: Colors.grey)),
          ],
        ),
      ),
    );
  }
}
