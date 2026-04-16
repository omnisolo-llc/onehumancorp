import 'dart:convert';
import 'dart:ui';
import 'package:flutter/material.dart';
import 'dart:async';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'agent_task_progress.dart';
import 'agent_mesh_message_tile.dart';
import 'autodream_pipeline_widget.dart';
import 'vector_memory_visualizer.dart';

class SwarmObservabilityDashboard extends StatefulWidget {
  final WebSocketChannel channel;

  const SwarmObservabilityDashboard({
    Key? key,
    required this.channel,
  }) : super(key: key);

  @override
  State<SwarmObservabilityDashboard> createState() => _SwarmObservabilityDashboardState();
}

class _SwarmObservabilityDashboardState extends State<SwarmObservabilityDashboard> {
  List<dynamic> _tasks = [];
  List<dynamic> _messages = [];
  double _embeddingActivity = 0.5;
  late final StreamSubscription<dynamic> _subscription;

  @override
  void initState() {
    super.initState();
    _subscription = widget.channel.stream.listen((message) {
      if (message is String) {
        final data = jsonDecode(message);
        if (data['tasks'] != null) {
          setState(() {
            _tasks = data['tasks'];
          });
        }
        if (data['embeddingActivity'] != null) {
          setState(() {
            _embeddingActivity = (data['embeddingActivity'] as num).toDouble();
          });
        }
        if (data['messages'] != null) {
          setState(() {
            _messages = data['messages'];
          });
        }
      }
    });
  }


  @override
  void dispose() {
    _subscription.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          padding: const EdgeInsets.all(24.0),
          decoration: BoxDecoration(
            color: const Color.fromRGBO(255, 255, 255, 0.03),
            borderRadius: BorderRadius.circular(16),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const AutoDreamPipelineWidget(),
              const SizedBox(height: 16),
              VectorMemoryVisualizerWidget(embeddingActivity: _embeddingActivity),
              const SizedBox(height: 24),
              const Text(
                'Swarm Observability',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 24,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const SizedBox(height: 24),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Expanded(
                      flex: 1,
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          const Text(
                            'Tasks',
                            style: TextStyle(
                              fontFamily: 'Outfit',
                              fontSize: 18,
                              fontWeight: FontWeight.bold,
                              color: Colors.white70,
                            ),
                          ),
                          const SizedBox(height: 12),
                          Expanded(
                            child: ListView.builder(
                              itemCount: _tasks.length,
                              itemBuilder: (context, index) {
                                final task = _tasks[index];
                                return Padding(
                                  padding: const EdgeInsets.only(bottom: 16.0),
                                  child: AgentTaskProgressWidget(
                                    taskName: task['name'] ?? 'Unknown Task',
                                    progress: (task['progress'] ?? 0.0).toDouble(),
                                    isWorking: task['isWorking'] ?? false,
                                  ),
                                );
                              },
                            ),
                          ),
                        ],
                      ),
                    ),
                    const SizedBox(height: 16),
                    Expanded(
                      flex: 1,
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          const Text(
                            'Agent Mesh',
                            style: TextStyle(
                              fontFamily: 'Outfit',
                              fontSize: 18,
                              fontWeight: FontWeight.bold,
                              color: Colors.white70,
                            ),
                          ),
                          const SizedBox(height: 12),
                          Expanded(
                            child: ListView.builder(
                              itemCount: _messages.length,
                              itemBuilder: (context, index) {
                                final msg = _messages[index];
                                return Padding(
                                  padding: const EdgeInsets.only(bottom: 16.0),
                                  child: AgentMeshMessageTile(
                                    sender: msg['sender'] ?? 'Unknown',
                                    message: msg['message'] ?? '',
                                    timestamp: msg['timestamp'] != null
                                        ? DateTime.tryParse(msg['timestamp']) ?? DateTime.now()
                                        : DateTime.now(),
                                  ),
                                );
                              },
                            ),
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
