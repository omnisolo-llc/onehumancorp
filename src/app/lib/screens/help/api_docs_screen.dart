import 'package:flutter/material.dart';

class ApiDocsScreen extends StatelessWidget {
  const ApiDocsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('API Reference', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: const Center(
        child: Text(
          'Advanced API Documentation (OpenAPI / Swagger UI placeholder)',
          style: TextStyle(fontSize: 18, fontFamily: 'Inter'),
        ),
      ),
    );
  }
}
