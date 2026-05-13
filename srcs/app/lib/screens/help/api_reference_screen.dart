import 'package:flutter/material.dart';
import '../../main.dart'; // For GlassContainer

class ApiReferenceScreen extends StatelessWidget {
  const ApiReferenceScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      appBar: AppBar(
        title: const Text('API Reference', style: TextStyle(fontFamily: 'Outfit', color: Colors.white)),
        backgroundColor: Colors.transparent,
        elevation: 0,
        iconTheme: const IconThemeData(color: Colors.white),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(20),
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 800),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                const Text(
                  'Developer API (Advanced)',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 28,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                ),
                const SizedBox(height: 10),
                const Text(
                  'Use our APIs to connect custom apps or workflows. This section is intended for developers.',
                  style: TextStyle(fontSize: 16, color: Colors.white70),
                ),
                const SizedBox(height: 30),
                GlassContainer(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: const [
                      Text('Authentication', style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white)),
                      SizedBox(height: 10),
                      Text(
                        'All API requests require a Bearer token in the Authorization header.',
                        style: TextStyle(color: Colors.white70),
                      ),
                      SizedBox(height: 10),
                      Text(
                        'Authorization: Bearer YOUR_API_KEY',
                        style: TextStyle(fontFamily: 'monospace', color: Color(0xFF6B4EFF), backgroundColor: Colors.black26),
                      ),
                    ],
                  ),
                ),
                const SizedBox(height: 20),
                _buildEndpointCard('GET', '/v1/products', 'List all products', 'Returns a paginated list of your products.'),
                const SizedBox(height: 10),
                _buildEndpointCard('POST', '/v1/orders', 'Create order', 'Creates a new order in your store.'),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildEndpointCard(String method, String path, String title, String description) {
    Color methodColor = method == 'GET' ? Colors.green : (method == 'POST' ? Colors.blue : Colors.orange);

    return Card(
      color: Colors.white.withAlpha(15),
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: Padding(
        padding: const EdgeInsets.all(15.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                  decoration: BoxDecoration(
                    color: methodColor.withAlpha(50),
                    borderRadius: BorderRadius.circular(4),
                    border: Border.all(color: methodColor),
                  ),
                  child: Text(method, style: TextStyle(color: methodColor, fontWeight: FontWeight.bold)),
                ),
                const SizedBox(width: 10),
                Text(path, style: const TextStyle(fontFamily: 'monospace', color: Colors.white, fontSize: 16)),
              ],
            ),
            const SizedBox(height: 10),
            Text(title, style: const TextStyle(fontWeight: FontWeight.bold, color: Colors.white, fontSize: 18)),
            const SizedBox(height: 5),
            Text(description, style: const TextStyle(color: Colors.white70)),
          ],
        ),
      ),
    );
  }
}
