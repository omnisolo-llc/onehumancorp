import 'package:flutter/material.dart';
import 'dart:convert';
import 'package:web_socket_channel/web_socket_channel.dart';
import '../widgets/glass_card.dart';

class KairosDashboardScreen extends StatefulWidget {
  final WebSocketChannel channel;

  const KairosDashboardScreen({super.key, required this.channel});

  @override
  State<KairosDashboardScreen> createState() => _KairosDashboardScreenState();
}

class _KairosDashboardScreenState extends State<KairosDashboardScreen> {
  final List<String> _sharedTasks = [];
  final List<String> _meshStream = [];
  final List<String> _autoDreamMemory = [];

  @override
  void initState() {
    super.initState();
    widget.channel.stream.listen((message) {
      final decoded = jsonDecode(message);
      setState(() {
        if (decoded['type'] == 'mesh:tasks') {
          _sharedTasks.insert(0, decoded['payload']);
        } else if (decoded['type'] == 'mesh:coordination') {
          _meshStream.insert(0, decoded['payload']);
        } else if (decoded['type'] == 'autodream') {
          _autoDreamMemory.insert(0, decoded['payload']);
        }
      });
    });
  }

  @override
  void dispose() {
    widget.channel.sink.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.transparent,
      body: Row(
        children: [
          Expanded(
            child: GlassCard(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text('Shared Task Queue', style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 20)),
                  Expanded(
                    child: ListView.builder(
                      itemCount: _sharedTasks.length,
                      itemBuilder: (context, index) => Text(_sharedTasks[index], style: const TextStyle(fontFamily: 'Inter', color: Colors.white70)),
                    ),
                  ),
                ],
              ),
            ),
          ),
          Expanded(
            child: GlassCard(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text('Teammate Mesh Stream', style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 20)),
                  Expanded(
                    child: ListView.builder(
                      itemCount: _meshStream.length,
                      itemBuilder: (context, index) => Text(_meshStream[index], style: const TextStyle(fontFamily: 'Inter', color: Colors.white70)),
                    ),
                  ),
                ],
              ),
            ),
          ),
          Expanded(
            child: GlassCard(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text('AutoDream Memory', style: TextStyle(fontFamily: 'Outfit', color: Colors.white, fontSize: 20)),
                  Expanded(
                    child: ListView.builder(
                      itemCount: _autoDreamMemory.length,
                      itemBuilder: (context, index) => Text(_autoDreamMemory[index], style: const TextStyle(fontFamily: 'Inter', color: Colors.white70)),
                    ),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
