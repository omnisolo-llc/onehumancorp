import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../router.dart';
import '../theme.dart';

class ApiDocumentationScreen extends ConsumerWidget {
  const ApiDocumentationScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return AppShell(
      title: 'API Documentation',
      child: Center(
        child: Container(
          constraints: const BoxConstraints(maxWidth: 800),
          padding: const EdgeInsets.all(24.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                'OneHumanCorp API',
                style: Theme.of(context).textTheme.headlineMedium?.copyWith(
                      fontWeight: FontWeight.bold,
                      color: AppTheme.primaryTextColor,
                    ),
              ),
              const SizedBox(height: 16),
              Text(
                'Build custom integrations and automate your workflow using our REST API.',
                style: Theme.of(context).textTheme.bodyLarge?.copyWith(
                      color: AppTheme.secondaryTextColor,
                    ),
              ),
              const SizedBox(height: 32),
              _buildEndpointCard(
                context,
                'GET',
                '/api/v1/products',
                'List all products',
                'Returns a paginated list of products for the authenticated store.',
              ),
              const SizedBox(height: 16),
              _buildEndpointCard(
                context,
                'POST',
                '/api/v1/orders',
                'Create a new order',
                'Creates a new order in the system. Requires a valid customer and product payload.',
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildEndpointCard(BuildContext context, String method, String path, String title, String description) {
    Color methodColor;
    switch (method) {
      case 'GET':
        methodColor = Colors.blue;
        break;
      case 'POST':
        methodColor = Colors.green;
        break;
      default:
        methodColor = Colors.grey;
    }

    return Card(
      elevation: 0,
      color: AppTheme.glassBackgroundColor,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
        side: BorderSide(color: AppTheme.glassBorderColor, width: 1),
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
                    borderRadius: BorderRadius.circular(4),
                    border: BorderSide(color: methodColor.withOpacity(0.5)),
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
                  path,
                  style: const TextStyle(
                    fontFamily: 'monospace',
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 12),
            Text(
              title,
              style: Theme.of(context).textTheme.titleMedium?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
            ),
            const SizedBox(height: 4),
            Text(
              description,
              style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                    color: AppTheme.secondaryTextColor,
                  ),
            ),
          ],
        ),
      ),
    );
  }
}