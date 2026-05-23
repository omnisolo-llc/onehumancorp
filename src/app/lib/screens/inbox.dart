import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'agent_audit_dashboard.dart';

class InboxMessage {
  final String id;
  final String tenantId;
  final String source;
  final String content;
  final String draftReply;
  final String status;
  final String createdAt;

  InboxMessage({
    required this.id,
    required this.tenantId,
    required this.source,
    required this.content,
    required this.draftReply,
    required this.status,
    required this.createdAt,
  });

  factory InboxMessage.fromJson(Map<String, dynamic> json) {
    return InboxMessage(
      id: json['id'] ?? '',
      tenantId: json['tenant_id'] ?? '',
      source: json['source'] ?? '',
      content: json['content'] ?? '',
      draftReply: json['draft_reply'] ?? '',
      status: json['status'] ?? '',
      createdAt: json['created_at'] ?? '',
    );
  }
}

class InboxScreen extends StatefulWidget {
  @override
  _InboxScreenState createState() => _InboxScreenState();
}

class _InboxScreenState extends State<InboxScreen> {
  List<InboxMessage> _messages = [];
  InboxMessage? _selectedMessage;
  bool _isLoading = true;
  TextEditingController _replyController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _fetchMessages();
  }

  Future<void> _fetchMessages() async {
    setState(() {
      _isLoading = true;
    });
    try {
      final response = await http.get(Uri.parse('http://localhost:8080/api/inbox/messages'));
      if (response.statusCode == 200) {
        final List<dynamic> data = jsonDecode(response.body);
        setState(() {
          _messages = data.map((item) => InboxMessage.fromJson(item)).toList();
          _isLoading = false;
        });
      } else {
        setState(() {
          _isLoading = false;
        });
      }
    } catch (e) {
      setState(() {
        _isLoading = false;
      });
    }
  }

  void _selectMessage(InboxMessage msg) {
    setState(() {
      _selectedMessage = msg;
      _replyController.text = msg.draftReply;
    });
  }

  void _sendMessage() {
    setState(() {
      _selectedMessage = null;
      _replyController.clear();
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Color(0xFFF5F5F7), // Light translucent glass background
      appBar: AppBar(
        title: Text(
          'Unified Inbox',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontWeight: FontWeight.bold,
            color: Color(0xFF1D1D1F),
          ),
        ),
        actions: [
          IconButton(
            icon: Icon(Icons.admin_panel_settings, color: Colors.black87),
            onPressed: () {
              Navigator.push(
                context,
                MaterialPageRoute(builder: (context) => AgentAuditDashboard()),
              );
            },
            tooltip: 'Agent Audit Dashboard',
          )
        ],
        backgroundColor: Colors.white.withOpacity(0.65),
        elevation: 0,
        flexibleSpace: ClipRect(
          child: BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 30, sigmaY: 30),
            child: Container(
              color: Colors.transparent,
            ),
          ),
        ),
      ),
      body: LayoutBuilder(
        builder: (context, constraints) {
          bool isWide = constraints.maxWidth > 600;
          return Row(
            children: [
              // Message List
              if (_selectedMessage == null || isWide)
                Expanded(
                  flex: 1,
                  child: _isLoading
                      ? Center(child: CircularProgressIndicator())
                      : _messages.isEmpty
                          ? Center(child: Text("No messages.", style: TextStyle(fontFamily: 'Inter', color: Colors.grey[600])))
                          : ListView.builder(
                              itemCount: _messages.length,
                              itemBuilder: (context, index) {
                                final msg = _messages[index];
                                return _buildMessageCard(msg);
                              },
                            ),
                ),

              // Detail View (Thread + Draft)
              if (_selectedMessage != null)
                Expanded(
                  flex: 2,
                  child: _buildThreadView(),
                ),
            ],
          );
        },
      ),
    );
  }

  Widget _buildMessageCard(InboxMessage msg) {
    return GestureDetector(
      onTap: () => _selectMessage(msg),
      child: Container(
        margin: EdgeInsets.symmetric(horizontal: 16, vertical: 8),
        decoration: BoxDecoration(
          color: Colors.white.withOpacity(0.65),
          borderRadius: BorderRadius.circular(16),
          border: Border.all(color: Colors.white.withOpacity(0.4), width: 1),
          boxShadow: [
            BoxShadow(
              color: Colors.black.withOpacity(0.05),
              blurRadius: 10,
              offset: Offset(0, 5),
            ),
          ],
        ),
        child: ClipRRect(
          borderRadius: BorderRadius.circular(16),
          child: BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 30, sigmaY: 30),
            child: Padding(
              padding: EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      Text(
                        msg.source.toUpperCase(),
                        style: TextStyle(
                          fontFamily: 'Inter',
                          fontSize: 12,
                          fontWeight: FontWeight.bold,
                          color: Color(0xFF0066FF), // UniFi Accent Blue
                        ),
                      ),
                      Text(
                        msg.createdAt,
                        style: TextStyle(
                          fontFamily: 'Inter',
                          fontSize: 10,
                          color: Colors.grey[500],
                        ),
                      ),
                    ],
                  ),
                  SizedBox(height: 8),
                  Text(
                    msg.content,
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                    style: TextStyle(
                      fontFamily: 'Inter',
                      fontSize: 14,
                      color: Color(0xFF1D1D1F),
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildThreadView() {
    return Container(
      color: Colors.white,
      child: Column(
        children: [
          // Thread Header
          Container(
            padding: EdgeInsets.all(16),
            decoration: BoxDecoration(
              border: Border(bottom: BorderSide(color: Colors.grey[200]!)),
            ),
            child: Row(
              children: [
                IconButton(
                  icon: Icon(Icons.arrow_back),
                  onPressed: () {
                    setState(() {
                      _selectedMessage = null;
                    });
                  },
                ),
                SizedBox(width: 8),
                Text(
                  'Message from ${_selectedMessage!.source}',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 18,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ],
            ),
          ),

          // Chat History
          Expanded(
            child: ListView(
              padding: EdgeInsets.all(16),
              children: [
                _buildChatBubble(_selectedMessage!.content, false),
              ],
            ),
          ),

          // Input Box with Draft
          Container(
            padding: EdgeInsets.all(16),
            decoration: BoxDecoration(
              color: Colors.white.withOpacity(0.65),
              border: Border(top: BorderSide(color: Colors.grey[200]!)),
            ),
            child: ClipRRect(
              borderRadius: BorderRadius.circular(8),
              child: BackdropFilter(
                filter: ImageFilter.blur(sigmaX: 30, sigmaY: 30),
                child: Container(
                  decoration: BoxDecoration(
                    color: Colors.white,
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(color: Color(0xFF0066FF).withOpacity(0.5), width: 1.5),
                    boxShadow: [
                      BoxShadow(
                        color: Color(0xFF0066FF).withOpacity(0.15),
                        blurRadius: 10,
                        spreadRadius: 2,
                      ),
                    ],
                  ),
                  child: Row(
                    children: [
                      Expanded(
                        child: TextField(
                          controller: _replyController,
                          maxLines: null,
                          decoration: InputDecoration(
                            hintText: 'Type your reply...',
                            contentPadding: EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                            border: InputBorder.none,
                          ),
                          style: TextStyle(fontFamily: 'Inter', fontSize: 14),
                        ),
                      ),
                      IconButton(
                        icon: Icon(Icons.send, color: Color(0xFF0066FF)),
                        onPressed: _sendMessage,
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildChatBubble(String text, bool isMe) {
    return Align(
      alignment: isMe ? Alignment.centerRight : Alignment.centerLeft,
      child: Container(
        margin: EdgeInsets.only(bottom: 16),
        padding: EdgeInsets.all(16),
        decoration: BoxDecoration(
          color: isMe ? Color(0xFF0066FF) : Colors.grey[100],
          borderRadius: BorderRadius.circular(16),
        ),
        child: Text(
          text,
          style: TextStyle(
            fontFamily: 'Inter',
            color: isMe ? Colors.white : Color(0xFF1D1D1F),
            fontSize: 14,
          ),
        ),
      ),
    );
  }
}
