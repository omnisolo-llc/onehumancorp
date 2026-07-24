import 'dart:async';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:http/http.dart' as http;

/// OHC Unified Inbox Mobile Triage Screen Stub
/// Designed with a modern, mobile-first translucent glass visual styling
/// and optimal touch targets (minimum 44x44px).
class OhcChatTriageScreen extends StatefulWidget {
  final String tenantId;
  final String authToken;
  final String apiBaseUrl; // e.g. "https://api.onehumancorp.com"
  final String wsBaseUrl;  // e.g. "wss://api.onehumancorp.com"

  const OhcChatTriageScreen({
    Key? key,
    required this.tenantId,
    required this.authToken,
    this.apiBaseUrl = 'http://localhost:18789',
    this.wsBaseUrl = 'ws://localhost:18789',
  }) : super(key: key);

  @override
  _OhcChatTriageScreenState createState() => _OhcChatTriageScreenState();
}

class _OhcChatTriageScreenState extends State<OhcChatTriageScreen> {
  WebSocketChannel? _channel;
  final List<Map<String, dynamic>> _messages = [];
  final List<Map<String, dynamic>> _conversations = [];
  bool _isLoading = true;
  String? _selectedConversationId;

  // Local Offline DB Cache Simulation for Offline-tolerant reads
  final Map<String, List<Map<String, dynamic>>> _offlineLocalDbCache = {};

  @override
  void initState() {
    super.override.initState();
    _fetchConversationsAndMessages();
    _connectWebSocket();
  }

  @override
  void dispose() {
    _channel?.sink.close();
    super.dispose();
  }

  /// REST fetch for conversations and messages (with local fallback)
  Future<void> _fetchConversationsAndMessages() async {
    try {
      final response = await http.get(
        Uri.parse('${widget.apiBaseUrl}/api/v1/chat/conversations'),
        headers: {
          'Authorization': 'Bearer ${widget.authToken}',
          'Content-Type': 'application/json',
        },
      );

      if (response.statusCode == 200) {
        final List<dynamic> data = json.decode(response.body);
        setState(() {
          _conversations.clear();
          for (var item in data) {
            _conversations.add(item as Map<String, dynamic>);
          }
          // Cache to local DB offline storage
          _offlineLocalDbCache['conversations'] = List.from(_conversations);
          _isLoading = false;
        });
      } else {
        throw Exception('Failed to load online data');
      }
    } catch (e) {
      // Offline-tolerant fallback to local DB cache
      if (_offlineLocalDbCache.containsKey('conversations')) {
        setState(() {
          _conversations.clear();
          _conversations.addAll(_offlineLocalDbCache['conversations']!);
          _isLoading = false;
        });
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Offline mode: loaded data from local cache.')),
        );
      } else {
        setState(() {
          _isLoading = false;
        });
      }
    }
  }

  /// Ingest message REST API
  Future<void> _sendMessage(String content) async {
    if (_selectedConversationId == null) return;
    try {
      final response = await http.post(
        Uri.parse('${widget.apiBaseUrl}/api/v1/chat/conversations/$_selectedConversationId/messages'),
        headers: {
          'Authorization': 'Bearer ${widget.authToken}',
          'Content-Type': 'application/json',
        },
        body: json.encode({
          'sender_id': 'agent_1',
          'sender_type': 'agent',
          'content': content,
        }),
      );

      if (response.statusCode != 201) {
        throw Exception('Failed to send message');
      }
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Network error: message will retry once online.')),
      );
    }
  }

  /// Establishes the token-authenticated WebSocket stream connection
  void _connectWebSocket() {
    final wsUri = Uri.parse('${widget.wsBaseUrl}/api/v1/chat/ws');
    try {
      _channel = WebSocketChannel.connect(wsUri);

      // Send auth token as the first message if needed, or query params
      _channel!.sink.add(json.encode({
        'action': 'subscribe',
        'tenant_id': widget.tenantId,
        'token': widget.authToken,
      }));

      _channel!.stream.listen(
        (message) {
          final decoded = json.decode(message);
          if (decoded['event'] == 'message_created') {
            final msg = decoded['message'];
            setState(() {
              if (msg['conversation_id'] == _selectedConversationId) {
                _messages.add(msg);
              }
            });
          }
        },
        onError: (err) {
          debugPrint('WebSocket error: $err');
          Future.delayed(const Duration(seconds: 5), _connectWebSocket);
        },
        onDone: () {
          debugPrint('WebSocket connection closed');
        },
      );
    } catch (e) {
      debugPrint('Failed to connect to WS: $e');
    }
  }

  @override
  Widget build(BuildContext context) {
    // 375px mobile breakpoint check
    final double screenWidth = MediaQuery.of(context).size.width;
    final bool isMobile = screenWidth <= 375;

    return Scaffold(
      backgroundColor: const Color(0xFF0F0F14),
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
        title: const Text(
          'OHC Work Triage',
          style: TextStyle(
            color: Colors.white,
            fontWeight: FontWeight.bold,
            letterSpacing: -0.5,
          ),
        ),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh, color: Colors.white),
            onPressed: _fetchConversationsAndMessages,
          ),
        ],
      ),
      body: Container(
        width: isMobile ? 375 : screenWidth,
        margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
        child: Column(
          children: [
            // Translucent glass visual styling for today's summary card
            _buildGlassCard(
              child: Padding(
                padding: const EdgeInsets.all(16.0),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: const [
                        Text(
                          'Today\'s Priorities',
                          style: TextStyle(
                            color: Colors.white70,
                            fontSize: 14,
                            fontWeight: FontWeight.w600,
                          ),
                        ),
                        SizedBox(height: 4),
                        Text(
                          '3 Actions Need Attention',
                          style: TextStyle(
                            color: Colors.white,
                            fontSize: 18,
                            fontWeight: FontWeight.bold,
                            letterSpacing: -0.3,
                          ),
                        ),
                      ],
                    ),
                    const Icon(Icons.bolt, color: Colors.amberAccent, size: 28),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 16),
            Expanded(
              child: _isLoading
                  ? const Center(child: CircularProgressIndicator(color: Colors.white))
                  : _conversations.isEmpty
                      ? const Center(
                          child: Text(
                            'All caught up today!',
                            style: TextStyle(color: Colors.white70),
                          ),
                        )
                      : ListView.separated(
                          itemCount: _conversations.length,
                          separatorBuilder: (_, __) => const SizedBox(height: 12),
                          itemBuilder: (context, index) {
                            final convo = _conversations[index];
                            return _buildConversationItem(convo);
                          },
                        ),
            ),
          ],
        ),
      ),
    );
  }

  /// Builds conversational list item with translucent frosted glass look
  Widget _buildConversationItem(Map<String, dynamic> convo) {
    return _buildGlassCard(
      child: InkWell(
        onTap: () {
          setState(() {
            _selectedConversationId = convo['id'];
          });
        },
        borderRadius: BorderRadius.circular(16),
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(
                    'Channel: ${convo['inbox_id'] ?? 'Web Widget'}',
                    style: const TextStyle(
                      color: Colors.amberAccent,
                      fontSize: 12,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  Container(
                    padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                    decoration: BoxDecoration(
                      color: Colors.white10,
                      borderRadius: BorderRadius.circular(8),
                    ),
                    child: Text(
                      convo['status'].toUpperCase(),
                      style: const TextStyle(
                        color: Colors.white70,
                        fontSize: 10,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 8),
              const Text(
                'New Custom Request',
                style: TextStyle(
                  color: Colors.white,
                  fontSize: 16,
                  fontWeight: FontWeight.bold,
                ),
              ),
              const SizedBox(height: 12),
              // Mobile-first Quick Actions with at least 44x44px touch targets
              Row(
                mainAxisAlignment: MainAxisAlignment.end,
                children: [
                  _buildQuickAction(
                    icon: Icons.auto_awesome,
                    label: 'Draft AI',
                    onPressed: () => _sendMessage('Draft Reply by AI...'),
                  ),
                  const SizedBox(width: 8),
                  _buildQuickAction(
                    icon: Icons.snooze,
                    label: 'Snooze',
                    onPressed: () {},
                  ),
                  const SizedBox(width: 8),
                  _buildQuickAction(
                    icon: Icons.check,
                    label: 'Resolve',
                    onPressed: () {},
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }

  /// UI Container with Restrained Translucent Materials (frosted glass)
  Widget _buildGlassCard({required Widget child}) {
    return Container(
      decoration: BoxDecoration(
        color: Colors.white.withOpacity(0.06),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(
          color: Colors.white.withOpacity(0.08),
          width: 1,
        ),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.2),
            blurRadius: 20,
            offset: const Offset(0, 8),
          ),
        ],
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(16),
        child: child,
      ),
    );
  }

  /// Optimal touch target (at least 44x44px) button wrapper
  Widget _buildQuickAction({
    required IconData icon,
    required String label,
    required VoidCallback onPressed,
  }) {
    return Container(
      constraints: const BoxConstraints(minWidth: 44, minHeight: 44),
      child: TextButton.icon(
        style: TextButton.styleFrom(
          foregroundColor: Colors.white,
          backgroundColor: Colors.white12,
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
          ),
        ),
        onPressed: onPressed,
        icon: Icon(icon, size: 16, color: Colors.white),
        label: Text(
          label,
          style: const TextStyle(fontSize: 12, fontWeight: FontWeight.bold),
        ),
      ),
    );
  }
}
