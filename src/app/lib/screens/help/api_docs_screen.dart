import 'dart:ui';
import 'package:flutter/material.dart';

class ApiDocsScreen extends StatelessWidget {
  const ApiDocsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('API Reference', style: TextStyle(fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      extendBodyBehindAppBar: true,
      body: Container(
        decoration: BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [
              Theme.of(context).colorScheme.surface,
              Theme.of(context).colorScheme.surfaceContainerHighest.withOpacity(0.5),
            ],
          ),
        ),
        child: SafeArea(
          child: SingleChildScrollView(
            padding: const EdgeInsets.all(24.0),
            child: ClipRRect(
              borderRadius: BorderRadius.circular(24),
              child: BackdropFilter(
                filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
                child: Container(
                  padding: const EdgeInsets.all(32),
                  decoration: BoxDecoration(
                    color: Theme.of(context).colorScheme.surface.withOpacity(0.5),
                    borderRadius: BorderRadius.circular(24),
                    border: Border.all(color: Colors.white.withOpacity(0.2)),
                  ),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text(
                        'OHC API Documentation',
                        style: TextStyle(
                          fontSize: 28,
                          fontWeight: FontWeight.bold,
                          fontFamily: 'Outfit',
                        ),
                      ),
                      const SizedBox(height: 16),
                      const Text(
                        'This section is for advanced users who want to connect custom tools or checkout flows to OneHumanCorp. Warning: Using APIs requires technical knowledge.',
                        style: TextStyle(
                          fontSize: 16,
                          color: Colors.redAccent,
                          fontFamily: 'Inter',
                        ),
                      ),
                      const SizedBox(height: 32),
                      _buildApiSection(context, 'GET /api/v1/products', 'List all your active products and services.', '''
{
  "products": [
    {
      "id": "prod_123",
      "name": "Custom Cake",
      "price": 4500
    }
  ]
}'''),
                      const SizedBox(height: 16),
                      _buildApiSection(context, 'POST /api/v1/orders', 'Create a new custom order.', '''
{
  "product_id": "prod_123",
  "quantity": 1,
  "customer_email": "test@example.com"
}'''),
                      const SizedBox(height: 16),
                      _buildApiSection(context, 'GET /api/v1/agents/status', 'Check the status of your AI agents.', '''
{
  "agents": [
    {
      "id": "agent_1",
      "status": "WORKING"
    }
  ]
}'''),
                    ],
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildApiSection(BuildContext context, String endpoint, String description, String mockResponse) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surfaceContainerHighest.withOpacity(0.3),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: Theme.of(context).colorScheme.outline.withOpacity(0.2)),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            endpoint,
            style: const TextStyle(
              fontFamily: 'monospace',
              fontWeight: FontWeight.bold,
              fontSize: 16,
            ),
          ),
          const SizedBox(height: 8),
          Text(
            description,
            style: const TextStyle(fontFamily: 'Inter'),
          ),
          const SizedBox(height: 12),
          Container(
            width: double.infinity,
            padding: const EdgeInsets.all(12),
            decoration: BoxDecoration(
              color: Colors.black.withOpacity(0.5),
              borderRadius: BorderRadius.circular(8),
            ),
            child: Text(
              mockResponse,
              style: const TextStyle(
                fontFamily: 'monospace',
                color: Colors.greenAccent,
                fontSize: 12,
              ),
            ),
          )
        ],
      ),
    );
  }
}
