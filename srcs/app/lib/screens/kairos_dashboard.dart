import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

class KairosDashboardScreen extends ConsumerStatefulWidget {
  const KairosDashboardScreen({super.key});
  @override
  ConsumerState<KairosDashboardScreen> createState() => _KairosDashboardScreenState();
}

class _KairosDashboardScreenState extends ConsumerState<KairosDashboardScreen> {
  late WebSocketChannel _channel;
  final List<String> _meshMessages = [];

  @override
  void initState() {
    super.initState();
    _channel = WebSocketChannel.connect(Uri.parse('ws://localhost:8000/connection/kairos_stream'));
    _channel.stream.listen((message) {
      if (mounted) {
        setState(() {
          _meshMessages.insert(0, message.toString());
          if (_meshMessages.length > 50) _meshMessages.removeLast();
        });
      }
    });
  }

  @override
  void dispose() {
    _channel.sink.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Swarm Analytics Dashboard', style: TextStyle(fontFamily: 'Outfit'))),
      body: Row(
        children: [
          Expanded(child: GlassCard(child: Column(children: [const Text('Shared Task Queue', style: TextStyle(fontFamily: 'Inter')), Expanded(child: ListView())]))),
          Expanded(child: GlassCard(child: Column(children: [const Text('Teammate Mesh Stream', style: TextStyle(fontFamily: 'Inter')), Expanded(child: ListView.builder(itemCount: _meshMessages.length, itemBuilder: (context, index) => Text(_meshMessages[index])))]))),
          Expanded(child: GlassCard(child: Column(children: [const Text('AutoDream Memory', style: TextStyle(fontFamily: 'Inter')), Expanded(child: ListView())]))),
        ],
      ),
    );
  }
}
