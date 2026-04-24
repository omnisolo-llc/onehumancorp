import 'package:flutter/material.dart';

class ApiDocsScreen extends StatelessWidget {
  const ApiDocsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('API Documentation (Advanced)', style: TextStyle(fontFamily: 'Outfit')),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 800),
          child: ListView(
            padding: const EdgeInsets.all(24),
            children: [
              const Text(
                'OneHumanCorp API',
                style: TextStyle(fontSize: 28, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
              ),
              const SizedBox(height: 16),
              const Text(
                'This section is for developers who want to integrate OHC with other tools directly. If you just want to run your business, you don\'t need to use this!',
                style: TextStyle(fontSize: 16, fontFamily: 'Inter'),
              ),
              const SizedBox(height: 32),
              _buildEndpointDoc(
                context,
                method: 'GET',
                path: '/v1/store/products',
                description: 'List all active products in your store.',
              ),
              const SizedBox(height: 16),
              _buildEndpointDoc(
                context,
                method: 'POST',
                path: '/v1/store/orders',
                description: 'Create a new order programmatically.',
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildEndpointDoc(BuildContext context, {required String method, required String path, required String description}) {
    final Color methodColor = method == 'GET' ? Colors.blue : Colors.green;

    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
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
                    style: TextStyle(color: methodColor, fontWeight: FontWeight.bold, fontFamily: 'monospace'),
                  ),
                ),
                const SizedBox(width: 12),
                Text(
                  path,
                  style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 16, fontFamily: 'monospace'),
                ),
              ],
            ),
            const SizedBox(height: 12),
            Text(description, style: const TextStyle(fontFamily: 'Inter')),
          ],
        ),
      ),
    );
  }
}
