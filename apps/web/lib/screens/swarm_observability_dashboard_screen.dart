import 'dart:async';
import 'dart:convert';
import 'dart:ui';
import 'package:flutter/material.dart';

// Mock Web Socket for testing
abstract class IMeshBroker {
  Stream<dynamic> get stream;
  void close();
}

class MockMeshBroker implements IMeshBroker {
  final _controller = StreamController<dynamic>.broadcast();

  @override
  Stream<dynamic> get stream => _controller.stream;

  @override
  void close() {
    _controller.close();
  }
}

class SwarmObservabilityDashboardScreen extends StatefulWidget {
  final IMeshBroker? broker;

  const SwarmObservabilityDashboardScreen({Key? key, this.broker}) : super(key: key);

  @override
  State<SwarmObservabilityDashboardScreen> createState() =>
      _SwarmObservabilityDashboardScreenState();
}

class _SwarmObservabilityDashboardScreenState
    extends State<SwarmObservabilityDashboardScreen> {
  final List<Map<String, String>> _activeAgents = [];
  late IMeshBroker _broker;
  StreamSubscription? _subscription;

  @override
  void initState() {
    super.initState();
    _broker = widget.broker ?? MockMeshBroker();
    _connectWebSocket();
  }

  void _connectWebSocket() {
    try {
      _subscription = _broker.stream.listen((message) {
        try {
          final decoded = jsonDecode(message as String);
          setState(() {
            // Update or add agent
            final existingIndex = _activeAgents.indexWhere((a) => a['name'] == decoded['agent_id']);
            final newAgentData = {
              'name': decoded['agent_id']?.toString() ?? 'Unknown',
              'task': decoded['action']?.toString() ?? 'Idle',
              'status': decoded['status']?.toString() ?? 'IDLE',
            };

            if (existingIndex >= 0) {
              _activeAgents[existingIndex] = newAgentData;
            } else {
              _activeAgents.add(newAgentData);
            }
          });
        } catch (e) {
          debugPrint('Error parsing mesh message: $e');
        }
      }, onError: (error) {
        debugPrint('WebSocket Error: $error');
      });
    } catch (e) {
       debugPrint('WebSocket Connection Error: $e');
    }
  }

  @override
  void dispose() {
    _subscription?.cancel();
    _broker.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.black,
      appBar: AppBar(
        title: const Text(
          'Swarm Observability',
          style: TextStyle(
            fontFamily: 'Outfit',
            color: Colors.white,
          ),
        ),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      body: Center(
        child: Container(
          width: 800,
          padding: const EdgeInsets.all(24.0),
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(16),
            color: const Color.fromRGBO(255, 255, 255, 0.03),
            border: Border.all(color: const Color.fromRGBO(255, 255, 255, 0.1)),
          ),
          child: ClipRRect(
            borderRadius: BorderRadius.circular(16),
            child: BackdropFilter(
              filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
              child: Padding(
                padding: const EdgeInsets.all(24.0),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text(
                      'Active Swarm Mesh',
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 24,
                        fontWeight: FontWeight.bold,
                        color: Colors.white,
                      ),
                    ),
                    const SizedBox(height: 16),
                    _activeAgents.isEmpty ?
                     const Padding(
                       padding: EdgeInsets.all(16.0),
                       child: Text(
                          'Waiting for mesh telemetry...',
                          style: TextStyle(
                            fontFamily: 'Inter',
                            color: Colors.white54,
                          ),
                        ),
                     )
                    : ListView.builder(
                      shrinkWrap: true,
                      itemCount: _activeAgents.length,
                      itemBuilder: (context, index) {
                        final agent = _activeAgents[index];
                        return Container(
                          margin: const EdgeInsets.only(bottom: 12),
                          padding: const EdgeInsets.all(16),
                          decoration: BoxDecoration(
                            color: const Color.fromRGBO(255, 255, 255, 0.05),
                            borderRadius: BorderRadius.circular(8),
                          ),
                          child: Row(
                            mainAxisAlignment: MainAxisAlignment.spaceBetween,
                            children: [
                              Column(
                                crossAxisAlignment: CrossAxisAlignment.start,
                                children: [
                                  Text(
                                    agent['name']!,
                                    style: const TextStyle(
                                      fontFamily: 'Outfit',
                                      fontSize: 18,
                                      fontWeight: FontWeight.bold,
                                      color: Colors.white,
                                    ),
                                  ),
                                  const SizedBox(height: 4),
                                  Text(
                                    agent['task']!,
                                    style: const TextStyle(
                                      fontFamily: 'Inter',
                                      fontSize: 14,
                                      color: Colors.white70,
                                    ),
                                  ),
                                ],
                              ),
                              Container(
                                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                                decoration: BoxDecoration(
                                  color: agent['status'] == 'IN_PROGRESS'
                                      ? Colors.green.withOpacity(0.2)
                                      : Colors.grey.withOpacity(0.2),
                                  borderRadius: BorderRadius.circular(16),
                                  border: Border.all(
                                    color: agent['status'] == 'IN_PROGRESS'
                                        ? Colors.green
                                        : Colors.grey,
                                  ),
                                ),
                                child: Text(
                                  agent['status']!,
                                  style: TextStyle(
                                    fontFamily: 'Inter',
                                    fontSize: 12,
                                    fontWeight: FontWeight.bold,
                                    color: agent['status'] == 'IN_PROGRESS'
                                        ? Colors.green
                                        : Colors.grey,
                                  ),
                                ),
                              ),
                            ],
                          ),
                        );
                      },
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
