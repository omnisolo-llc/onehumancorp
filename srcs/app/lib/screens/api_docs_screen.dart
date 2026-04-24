import 'package:flutter/material.dart';

class ApiDocsScreen extends StatelessWidget {
  const ApiDocsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('API Documentation')),
      body: const Center(child: Text('API Documentation Content')),
    );
  }
}
