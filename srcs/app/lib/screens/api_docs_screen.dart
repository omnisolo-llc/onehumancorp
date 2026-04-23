import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class ApiDocsScreen extends StatelessWidget {
  const ApiDocsScreen({super.key});

  final List<Map<String, dynamic>> _endpoints = const [
    {
      'method': 'GET',
      'path': '/api/v1/store/products',
      'description': 'Retrieve a list of all products in your store.',
      'color': Colors.green,
    },
    {
      'method': 'POST',
      'path': '/api/v1/store/orders',
      'description': 'Create a new order programmatically.',
      'color': Colors.orange,
    },
    {
      'method': 'GET',
      'path': '/api/v1/agents/status',
      'description': 'Get the current health and status of all AI agents.',
      'color': Colors.blue,
    },
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Developer API Docs', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      extendBodyBehindAppBar: true,
      body: SafeArea(
        child: LayoutBuilder(
          builder: (context, constraints) {
            return Center(
              child: ConstrainedBox(
                constraints: const BoxConstraints(maxWidth: 800),
                child: Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Container(
                        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                        decoration: BoxDecoration(
                          color: Colors.red.withValues(alpha: 0.2),
                          borderRadius: BorderRadius.circular(16),
                          border: Border.all(color: Colors.red.withValues(alpha: 0.5)),
                        ),
                        child: const Text(
                          'Advanced Feature',
                          style: TextStyle(
                            fontFamily: 'Outfit',
                            fontWeight: FontWeight.bold,
                            color: Colors.red,
                            fontSize: 12,
                          ),
                        ),
                      ),
                      const SizedBox(height: 16),
                      const Text(
                        'API Reference',
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 28,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      const SizedBox(height: 8),
                      const Text(
                        'Connect your own custom applications to your OHC store. These APIs are for developers building advanced integrations.',
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 16,
                          color: Colors.grey,
                        ),
                      ),
                      const SizedBox(height: 32),
                      Expanded(
                        child: ListView.builder(
                          itemCount: _endpoints.length,
                          itemBuilder: (context, index) {
                            return _buildEndpointCard(context, _endpoints[index]);
                          },
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            );
          },
        ),
      ),
    );
  }

  Widget _buildEndpointCard(BuildContext context, Map<String, dynamic> endpoint) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 16.0),
      child: GlassCard(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Container(
                  width: 60,
                  padding: const EdgeInsets.symmetric(vertical: 4),
                  alignment: Alignment.center,
                  decoration: BoxDecoration(
                    color: (endpoint['color'] as Color).withValues(alpha: 0.2),
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(color: (endpoint['color'] as Color).withValues(alpha: 0.5)),
                  ),
                  child: Text(
                    endpoint['method'],
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontWeight: FontWeight.bold,
                      color: endpoint['color'],
                    ),
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: Text(
                    endpoint['path'],
                    style: const TextStyle(
                      fontFamily: 'monospace',
                      fontSize: 16,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
                IconButton(
                  icon: const Icon(Icons.copy, size: 20),
                  onPressed: () {
                    ScaffoldMessenger.of(context).showSnackBar(
                      const SnackBar(content: Text('Endpoint path copied to clipboard')),
                    );
                  },
                ),
              ],
            ),
            const SizedBox(height: 12),
            Text(
              endpoint['description'],
              style: const TextStyle(
                fontFamily: 'Outfit',
                fontSize: 16,
                color: Colors.grey,
              ),
            ),
            const SizedBox(height: 16),
            Align(
              alignment: Alignment.centerRight,
              child: TextButton(
                onPressed: () {
                  ScaffoldMessenger.of(context).showSnackBar(
                    const SnackBar(content: Text('Opening interactive API explorer...')),
                  );
                },
                child: const Text('Try it out →', style: TextStyle(fontFamily: 'Outfit')),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
