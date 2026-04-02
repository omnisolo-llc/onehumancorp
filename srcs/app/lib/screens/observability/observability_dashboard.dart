import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../services/teammate_mesh_service.dart';
import '../../widgets/glass_container.dart';

class ObservabilityDashboard extends ConsumerStatefulWidget {
  final String roomId;

  const ObservabilityDashboard({super.key, required this.roomId});

  @override
  ConsumerState<ObservabilityDashboard> createState() => _ObservabilityDashboardState();
}

class _ObservabilityDashboardState extends ConsumerState<ObservabilityDashboard> {
  final List<MeshMessage> _messages = [];

  @override
  Widget build(BuildContext context) {
    // Listen to the stream and accumulate messages
    ref.listen<AsyncValue<MeshMessage>>(
      teammateMeshProvider(widget.roomId),
      (previous, next) {
        if (next.hasValue && next.value != null) {
          setState(() {
            _messages.insert(0, next.value!);
            if (_messages.length > 50) {
              _messages.removeLast(); // Keep only last 50 messages
            }
          });
        }
      },
    );

    return Scaffold(
      backgroundColor: const Color(0xFF0F172A), // Slate 900 background
      appBar: AppBar(
        title: const Text('Swarm Observability', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.w600, color: Colors.white)),
        backgroundColor: Colors.transparent,
        elevation: 0,
        iconTheme: const IconThemeData(color: Colors.white),
      ),
      body: Stack(
        children: [
          // Background subtle gradient/shapes for glassmorphism pop
          Positioned(
            top: 100,
            left: -50,
            child: Container(
              width: 400,
              height: 400,
              decoration: BoxDecoration(
                shape: BoxShape.circle,
                color: const Color(0xFF3B82F6).withOpacity(0.15), // Blue 500
              ),
            ),
          ),
          Positioned(
            bottom: -50,
            right: -50,
            child: Container(
              width: 350,
              height: 350,
              decoration: BoxDecoration(
                shape: BoxShape.circle,
                color: const Color(0xFF8B5CF6).withOpacity(0.15), // Violet 500
              ),
            ),
          ),

          Padding(
            padding: const EdgeInsets.all(24.0),
            child: GlassContainer(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text(
                    'Realtime Teammate Mesh',
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 24,
                      fontWeight: FontWeight.bold,
                      color: Colors.white,
                    ),
                  ),
                  const SizedBox(height: 16),
                  Expanded(
                    child: _messages.isEmpty
                        ? const Center(child: Text("Waiting for messages...", style: TextStyle(color: Colors.white54, fontFamily: 'Inter')))
                        : ListView.builder(
                            itemCount: _messages.length,
                            itemBuilder: (context, index) {
                              return _buildMessageTile(_messages[index]);
                            },
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

  Widget _buildMessageTile(MeshMessage message) {
    return Container(
      margin: const EdgeInsets.only(bottom: 12.0),
      padding: const EdgeInsets.all(12.0),
      decoration: BoxDecoration(
        color: Colors.white.withOpacity(0.05),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: Colors.white.withOpacity(0.1)),
      ),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          CircleAvatar(
            backgroundColor: const Color(0xFF6366F1), // Indigo 500
            child: Text(message.role.substring(0, 1), style: const TextStyle(color: Colors.white, fontFamily: 'Inter', fontWeight: FontWeight.bold)),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Text(
                      message.role,
                      style: const TextStyle(
                        fontFamily: 'Outfit',
                        fontWeight: FontWeight.w600,
                        color: Colors.white70,
                      ),
                    ),
                    Text(
                      '${message.timestamp.hour.toString().padLeft(2, '0')}:${message.timestamp.minute.toString().padLeft(2, '0')}',
                      style: const TextStyle(
                        fontFamily: 'Inter',
                        fontSize: 12,
                        color: Colors.white38,
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 6),
                Text(
                  message.content,
                  style: const TextStyle(
                    fontFamily: 'Inter',
                    color: Colors.white,
                    height: 1.4,
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
