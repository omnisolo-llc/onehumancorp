import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:ohc_app/services/settings_service.dart';

// Represents an API endpoint documented via OpenAPI
class ApiEndpoint {
  final String method;
  final String path;
  final String description;
  final String? parameters;
  final String? requestBody;
  final String? responses;

  ApiEndpoint({
    required this.method,
    required this.path,
    required this.description,
    this.parameters,
    this.requestBody,
    this.responses,
  });

  factory ApiEndpoint.fromJson(String path, String method, Map<String, dynamic> json) {
    return ApiEndpoint(
      method: method.toUpperCase(),
      path: path,
      description: json['summary'] ?? json['description'] ?? 'No description available',
      parameters: json['parameters'] != null ? const JsonEncoder.withIndent('  ').convert(json['parameters']) : null,
      requestBody: json['requestBody'] != null ? const JsonEncoder.withIndent('  ').convert(json['requestBody']) : null,
      responses: json['responses'] != null ? const JsonEncoder.withIndent('  ').convert(json['responses']) : null,
    );
  }
}

final openApiDocProvider = FutureProvider.autoDispose<List<ApiEndpoint>>((ref) async {
  final clientSettings = ref.watch(clientSettingsProvider).valueOrNull;
  final baseUrl = clientSettings?.backendUrl ?? 'http://localhost:8080';

  try {
    // Attempt to load live swagger from backend
    final response = await http.get(Uri.parse('$baseUrl/openapi.json'));
    if (response.statusCode == 200) {
      final data = jsonDecode(response.body);
      final paths = data['paths'] as Map<String, dynamic>? ?? {};
      final endpoints = <ApiEndpoint>[];

      paths.forEach((path, methods) {
        (methods as Map<String, dynamic>).forEach((method, details) {
          endpoints.add(ApiEndpoint.fromJson(path, method, details as Map<String, dynamic>));
        });
      });
      return endpoints;
    }
  } catch (e) {
    // Fallback to static if backend isn't serving swagger yet, to avoid breaking the UI
  }

  // Fallback to hardcoded mock data per previous instruction
  return [
    ApiEndpoint(
      method: 'GET',
      path: '/api/v1/agents',
      description: 'List all active agents for the tenant.',
      responses: '{\n  "200": {\n    "description": "A list of agents"\n  }\n}',
    ),
    ApiEndpoint(
      method: 'POST',
      path: '/api/v1/agents/hire',
      description: 'Hire a new agent with a specific role.',
      requestBody: '{\n  "role": "string"\n}',
    ),
    ApiEndpoint(
      method: 'GET',
      path: '/api/v1/store/products',
      description: 'Retrieve the storefront product catalog.',
    ),
  ];
});

class ApiDocsScreen extends ConsumerWidget {
  const ApiDocsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final docsAsyncValue = ref.watch(openApiDocProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('API Documentation (Swagger UI)', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () {
            if (context.canPop()) {
              context.pop();
            } else {
              context.go('/help');
            }
          },
        ),
      ),
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Color(0xFF1E1E28), Color(0xFF0F0F14)],
          ),
        ),
        child: docsAsyncValue.when(
          loading: () => const Center(child: CircularProgressIndicator()),
          error: (err, stack) => Center(child: Text('Error loading docs: $err', style: const TextStyle(color: Colors.red))),
          data: (endpoints) => ListView.builder(
            padding: const EdgeInsets.all(16.0),
            itemCount: endpoints.length,
            itemBuilder: (context, index) {
              final endpoint = endpoints[index];
              return Padding(
                padding: const EdgeInsets.only(bottom: 16.0),
                child: GlassCard(
                  child: ExpansionTile(
                    title: Row(
                      children: [
                        Container(
                          padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                          decoration: BoxDecoration(
                            color: endpoint.method == 'GET' ? Colors.green.withOpacity(0.3) :
                                   endpoint.method == 'POST' ? Colors.blue.withOpacity(0.3) :
                                   endpoint.method == 'PUT' ? Colors.orange.withOpacity(0.3) :
                                   endpoint.method == 'DELETE' ? Colors.red.withOpacity(0.3) :
                                   Colors.grey.withOpacity(0.3),
                            borderRadius: BorderRadius.circular(4),
                          ),
                          child: Text(
                            endpoint.method,
                            style: TextStyle(
                              color: endpoint.method == 'GET' ? Colors.greenAccent :
                                     endpoint.method == 'POST' ? Colors.blueAccent :
                                     endpoint.method == 'PUT' ? Colors.orangeAccent :
                                     endpoint.method == 'DELETE' ? Colors.redAccent :
                                     Colors.white,
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                        ),
                        const SizedBox(width: 12),
                        Expanded(
                          child: Text(
                            endpoint.path,
                            style: const TextStyle(
                              color: Colors.white,
                              fontFamily: 'monospace',
                              fontSize: 16,
                            ),
                          ),
                        ),
                      ],
                    ),
                    subtitle: Padding(
                      padding: const EdgeInsets.only(top: 8.0),
                      child: Text(
                        endpoint.description,
                        style: const TextStyle(color: Colors.white70),
                      ),
                    ),
                    children: [
                      Padding(
                        padding: const EdgeInsets.all(16.0),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            if (endpoint.parameters != null) ...[
                              const Text('Parameters', style: TextStyle(color: Colors.white, fontWeight: FontWeight.bold)),
                              const SizedBox(height: 8),
                              Container(
                                width: double.infinity,
                                padding: const EdgeInsets.all(12),
                                decoration: BoxDecoration(color: Colors.black45, borderRadius: BorderRadius.circular(8)),
                                child: Text(endpoint.parameters!, style: const TextStyle(color: Colors.white70, fontFamily: 'monospace', fontSize: 12)),
                              ),
                              const SizedBox(height: 16),
                            ],
                            if (endpoint.requestBody != null) ...[
                              const Text('Request Body', style: TextStyle(color: Colors.white, fontWeight: FontWeight.bold)),
                              const SizedBox(height: 8),
                              Container(
                                width: double.infinity,
                                padding: const EdgeInsets.all(12),
                                decoration: BoxDecoration(color: Colors.black45, borderRadius: BorderRadius.circular(8)),
                                child: Text(endpoint.requestBody!, style: const TextStyle(color: Colors.white70, fontFamily: 'monospace', fontSize: 12)),
                              ),
                              const SizedBox(height: 16),
                            ],
                            if (endpoint.responses != null) ...[
                              const Text('Responses', style: TextStyle(color: Colors.white, fontWeight: FontWeight.bold)),
                              const SizedBox(height: 8),
                              Container(
                                width: double.infinity,
                                padding: const EdgeInsets.all(12),
                                decoration: BoxDecoration(color: Colors.black45, borderRadius: BorderRadius.circular(8)),
                                child: Text(endpoint.responses!, style: const TextStyle(color: Colors.white70, fontFamily: 'monospace', fontSize: 12)),
                              ),
                            ],
                          ],
                        ),
                      )
                    ],
                  ),
                ),
              );
            },
          ),
        ),
      ),
    );
  }
}
