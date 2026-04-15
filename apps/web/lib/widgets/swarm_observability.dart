import 'dart:convert';
import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

class SwarmObservabilityWidget extends StatefulWidget {
  final String wsUrl;
  final WebSocketChannel? channel;

  const SwarmObservabilityWidget({
    Key? key,
    this.wsUrl = 'ws://localhost:8080/ws',
    this.channel,
  }) : super(key: key);

  @override
  _SwarmObservabilityWidgetState createState() => _SwarmObservabilityWidgetState();
}

class _SwarmObservabilityWidgetState extends State<SwarmObservabilityWidget> {
  WebSocketChannel? _channel;
  List<Map<String, String>> _activeAgents = [];

  @override
  void initState() {
    super.initState();
    _connectWebSocket();
  }

  void _connectWebSocket() {
    try {
      _channel = widget.channel ?? WebSocketChannel.connect(Uri.parse(widget.wsUrl));
      _channel?.stream.listen((message) {
        try {
          final data = jsonDecode(message);
          if (data is List) {
            if (mounted) {
              setState(() {
                _activeAgents = List<Map<String, String>>.from(
                  data.map((item) => Map<String, String>.from(item)),
                );
              });
            }
          }
        } catch (e) {
          // ignore parsing error
        }
      }, onError: (error) {
        // handle error
      }, onDone: () {
        // handle disconnect
      });
    } catch (e) {
      // ignore connection error
    }
  }

  @override
  void dispose() {
    _channel?.sink.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          padding: const EdgeInsets.all(16.0),
          decoration: BoxDecoration(
            color: const Color.fromRGBO(255, 255, 255, 0.03),
            borderRadius: BorderRadius.circular(16),
            border: Border.all(color: const Color.fromRGBO(255, 255, 255, 0.1)),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              const Text(
                'Swarm Intelligence',
                style: TextStyle(
                  color: Colors.white,
                  fontFamily: 'Outfit',
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                ),
              ),
              const SizedBox(height: 16),
              if (_activeAgents.isEmpty)
                const Text(
                  'No agents active',
                  style: TextStyle(color: Colors.white70, fontFamily: 'Outfit'),
                )
              else
                ..._activeAgents.map((agent) {
                  return Padding(
                    padding: const EdgeInsets.only(bottom: 8.0),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Text(
                          agent['name'] ?? 'Unknown Agent',
                          style: const TextStyle(
                            color: Colors.white,
                            fontFamily: 'Outfit',
                            fontSize: 16,
                          ),
                        ),
                        Row(
                          children: [
                            Container(
                              width: 8,
                              height: 8,
                              decoration: const BoxDecoration(
                                color: Colors.greenAccent,
                                shape: BoxShape.circle,
                              ),
                            ),
                            const SizedBox(width: 8),
                            Text(
                              agent['status'] ?? 'IDLE',
                              style: const TextStyle(
                                color: Colors.white70,
                                fontFamily: 'Outfit',
                                fontSize: 14,
                              ),
                            ),
                          ],
                        ),
                      ],
                    ),
                  );
                }).toList(),
            ],
          ),
        ),
      ),
    );
  }
}
