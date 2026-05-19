import 'package:flutter/material.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
import 'live_storefront.dart';

class PreviewEditScreen extends StatefulWidget {
  final Map<String, dynamic> draft;
  const PreviewEditScreen({super.key, required this.draft});

  @override
  State<PreviewEditScreen> createState() => _PreviewEditScreenState();
}

class _PreviewEditScreenState extends State<PreviewEditScreen> {
  late List<dynamic> _blocks;
  bool _isPublishing = false;

  @override
  void initState() {
    super.initState();
    _blocks = widget.draft['pages'][0]['blocks'];
  }

  void _reorder(int oldIndex, int newIndex) {
    setState(() {
      if (newIndex > oldIndex) {
        newIndex -= 1;
      }
      final item = _blocks.removeAt(oldIndex);
      _blocks.insert(newIndex, item);
    });
  }

  Future<void> _publish() async {
    setState(() => _isPublishing = true);
    try {
      final response = await http.post(
        Uri.parse('/api/v1/builder/publish_draft'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({
          'domain': null,
          'draft': widget.draft,
        }),
      );

      if (response.statusCode == 200) {
        final site = jsonDecode(response.body);
        if (mounted) {
          Navigator.pushReplacement(
            context,
            MaterialPageRoute(
              builder: (context) => LiveStorefrontScreen(site: site),
            ),
          );
        }
      } else {
        throw Exception('Failed to publish');
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Published (Simulated)')),
        );
        Navigator.pushReplacement(
          context,
          MaterialPageRoute(
            builder: (context) => LiveStorefrontScreen(site: {'domain': 'test-store.ohc.io'}),
          ),
        );
      }
    } finally {
      setState(() => _isPublishing = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Preview & Edit'),
        actions: [
          _isPublishing
              ? const Center(child: CircularProgressIndicator(color: Colors.white))
              : TextButton(
                  onPressed: _publish,
                  child: const Text('Publish', style: TextStyle(color: Colors.blue, fontWeight: FontWeight.bold)),
                ),
        ],
      ),
      body: Column(
        children: [
          const Padding(
            padding: EdgeInsets.all(16.0),
            child: Text('Tap and hold to reorder blocks', style: TextStyle(color: Colors.grey)),
          ),
          Expanded(
            child: ReorderableListView(
              onReorder: _reorder,
              children: [
                for (int i = 0; i < _blocks.length; i++)
                  Card(
                    key: ValueKey(i),
                    margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                    child: ListTile(
                      title: Text(_blocks[i]['block_type']),
                      subtitle: Text(jsonEncode(_blocks[i]['content'])),
                      trailing: const Icon(Icons.drag_handle),
                      leading: const Icon(Icons.view_quilt),
                    ),
                  ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
