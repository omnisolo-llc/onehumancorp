import 'dart:ui';
import 'package:flutter/material.dart';
import 'dart:async';
import 'package:ohc_web_app/widgets/agent_message_state_animation.dart';
import 'package:ohc_web_app/widgets/agent_avatar.dart';

abstract class MeshClient {
  Stream<String> get messageStream;
}

class SwarmObservabilityDashboard extends StatefulWidget {
  final MeshClient client;
  const SwarmObservabilityDashboard({Key? key, required this.client}) : super(key: key);
  @override
  State<SwarmObservabilityDashboard> createState() => _SwarmObservabilityDashboardState();
}

class _SwarmObservabilityDashboardState extends State<SwarmObservabilityDashboard> {
  List<String> _messages = [];
  late StreamSubscription<String> _subscription;
  final GlobalKey<AnimatedListState> _listKey = GlobalKey<AnimatedListState>();

  @override
  void initState() {
    super.initState();
    _subscription = widget.client.messageStream.listen((msg) {
      setState(() {
        _messages.insert(0, msg);
        _listKey.currentState?.insertItem(0);
      });
    });
  }

  @override
  void dispose() {
    _subscription.cancel();
    super.dispose();
  }

  Widget _buildItem(String message, Animation<double> animation) {
    return SizeTransition(
      sizeFactor: animation,
      child: FadeTransition(
        opacity: animation,
        child: Padding(
          padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 16.0),
          child: AgentMessageStateAnimation(
            state: MessageState.delivered, // Reusing existing animation widget
            child: Row(
              children: [
                AgentAvatar(
                  agentName: 'System',
                  isOnline: true,
                  isWorking: true,
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: Text(
                    message,
                    style: const TextStyle(
                      fontFamily: 'Outfit',
                      color: Colors.white,
                      fontSize: 14,
                    ),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: const Color.fromRGBO(255, 255, 255, 0.05),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: const Color.fromRGBO(255, 255, 255, 0.1)),
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(16),
        child: BackdropFilter(
          filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
          child: AnimatedList(
            key: _listKey,
            initialItemCount: _messages.length,
            itemBuilder: (context, index, animation) {
              return _buildItem(_messages[index], animation);
            },
          ),
        ),
      ),
    );
  }
}
