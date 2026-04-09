import 'package:flutter/material.dart';
import 'dart:ui';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'dart:convert';

class GlassCard extends StatelessWidget {
  final Widget child;
  const GlassCard({Key? key, required this.child}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(8.0),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(16),
        child: BackdropFilter(
          filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
          child: Container(
            color: const Color.fromRGBO(255, 255, 255, 0.03),
            child: Material(
              type: MaterialType.transparency,
              child: child,
            ),
          ),
        ),
      ),
    );
  }
}

class SwarmMonitorWidget extends StatefulWidget {
  final String wsUrl;
  const SwarmMonitorWidget({Key? key, this.wsUrl = 'ws://localhost:8080/ws/swarm'}) : super(key: key);

  @override
  State<SwarmMonitorWidget> createState() => _SwarmMonitorWidgetState();
}

class _SwarmMonitorWidgetState extends State<SwarmMonitorWidget> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _animation;
  late WebSocketChannel _channel;
  String _statusMessage = 'Connecting...';
  bool _isConnected = false;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(duration: const Duration(seconds: 2), vsync: this)..repeat(reverse: true);
    _animation = Tween<double>(begin: 0.8, end: 1.0).animate(CurvedAnimation(parent: _controller, curve: Curves.easeInOut));
    _connectWebSocket();
  }

  void _connectWebSocket() {
    try {
      _channel = WebSocketChannel.connect(Uri.parse(widget.wsUrl));
      _channel.stream.listen((message) {
        if (!mounted) return;
        setState(() {
          try {
            final data = jsonDecode(message);
            _statusMessage = data['status'] ?? 'Active and observing';
            _isConnected = true;
          } catch (e) {
            _statusMessage = 'Active and observing';
            _isConnected = true;
          }
        });
      }, onError: (error) {
        if (!mounted) return;
        setState(() {
          _statusMessage = 'Connection error';
          _isConnected = false;
        });
      }, onDone: () {
        if (!mounted) return;
        setState(() {
          _statusMessage = 'Disconnected';
          _isConnected = false;
        });
      });
    } catch (e) {
      if (!mounted) return;
      setState(() {
        _statusMessage = 'Failed to connect';
        _isConnected = false;
      });
    }
  }

  @override
  void dispose() {
    _controller.dispose();
    _channel.sink.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return GlassCard(
      child: ListTile(
        leading: ScaleTransition(
          scale: _animation,
          child: Icon(Icons.memory, color: _isConnected ? Colors.blueAccent : Colors.grey),
        ),
        title: const Text('Swarm Agent Status', style: TextStyle(fontFamily: 'Outfit', color: Colors.white)),
        subtitle: Text(_statusMessage, style: const TextStyle(fontFamily: 'Inter', color: Colors.white70)),
      ),
    );
  }
}
