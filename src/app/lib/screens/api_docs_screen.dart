import 'package:flutter/material.dart';

class ApiDocsScreen extends StatelessWidget {
  const ApiDocsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('OneHumanCorp API'),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'OneHumanCorp API',
              style: TextStyle(
                fontSize: 28,
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 16),
            const Text(
              'Welcome to the OneHumanCorp API documentation. Here you can find '
              'information on how to integrate with our platform programmatically.',
              style: TextStyle(fontSize: 16),
            ),
            const SizedBox(height: 32),
            _buildEndpointCard(
              context,
              'GET',
              '/v1/business',
              'Retrieve details about your business.',
            ),
            const SizedBox(height: 16),
            _buildEndpointCard(
              context,
              'POST',
              '/v1/products',
              'Create a new product.',
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildEndpointCard(
      BuildContext context, String method, String endpoint, String description) {
    Color methodColor = method == 'GET' ? Colors.blue : Colors.green;
    return Card(
      elevation: 0,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
        side: BorderSide(color: Colors.grey.withOpacity(0.3)),
      ),
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
                    color: methodColor.withOpacity(0.1),
                    borderRadius: BorderRadius.circular(6),
                  ),
                  child: Text(
                    method,
                    style: TextStyle(
                      color: methodColor,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
                const SizedBox(width: 12),
                Text(
                  endpoint,
                  style: const TextStyle(
                    fontFamily: 'monospace',
                    fontWeight: FontWeight.w600,
                    fontSize: 16,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 12),
            Text(description),
          ],
        ),
      ),
    );
  }
}
