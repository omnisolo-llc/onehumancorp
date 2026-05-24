import 'dart:convert';
import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import '../widgets/tooltip_wrapper.dart';

class AgentDashboard extends StatefulWidget {
  @override
  _AgentDashboardState createState() => _AgentDashboardState();
}

class _AgentDashboardState extends State<AgentDashboard> {
  List<dynamic> _approvals = [];
  bool _isLoading = true;
  String _errorMessage = '';

  @override
  void initState() {
    super.initState();
    _fetchApprovals();
  }

  Future<void> _fetchApprovals() async {
    try {
      final baseUrl = const String.fromEnvironment('API_BASE_URL', defaultValue: 'http://localhost:18789');
      final response = await http.get(Uri.parse('$baseUrl/api/agents/approvals'));
      if (response.statusCode == 200) {
        final data = json.decode(response.body);
        setState(() {
          _approvals = data['pending_approvals'] ?? [];
          _isLoading = false;
        });
      } else {
        setState(() {
          _errorMessage = 'Failed to load approvals: ${response.statusCode}';
          _isLoading = false;
        });
      }
    } catch (e) {
      setState(() {
        _errorMessage = 'Error fetching approvals: $e';
        _isLoading = false;
      });
    }
  }

  Future<void> _approveAction(String id) async {
    try {
      final baseUrl = const String.fromEnvironment('API_BASE_URL', defaultValue: 'http://localhost:18789');
      final response = await http.post(
        Uri.parse('$baseUrl/api/agents/approvals/$id'),
        headers: {'Content-Type': 'application/json'},
        body: json.encode({'approved': true}),
      );
      if (response.statusCode == 200) {
        setState(() {
          _approvals.removeWhere((item) => item['id'] == id);
        });
      } else {
        // Handle error appropriately
        print('Failed to approve action');
      }
    } catch (e) {
      print('Error approving action: $e');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Color(0xFFF5F5F7),
      appBar: AppBar(
        title: TooltipWrapper(
          tooltipId: 'agent_updates',
          child: Text('Agent Updates', style: TextStyle(fontFamily: 'Outfit', color: Colors.black87, fontWeight: FontWeight.bold)),
        ),
        backgroundColor: Colors.white.withOpacity(0.8),
        elevation: 0,
        flexibleSpace: ClipRect(
          child: BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 10.0, sigmaY: 10.0),
            child: Container(color: Colors.transparent),
          ),
        ),
      ),
      body: _isLoading
          ? Center(child: CircularProgressIndicator())
          : _errorMessage.isNotEmpty
              ? Center(child: Text(_errorMessage))
              : _approvals.isEmpty
                  ? Center(child: Text('All caught up!', style: TextStyle(color: Colors.grey)))
                  : ListView.builder(
                      padding: EdgeInsets.all(16),
                      itemCount: _approvals.length,
                      itemBuilder: (context, index) {
                        final item = _approvals[index];
                        return _buildGlassmorphismCard(item);
                      },
                    ),
    );
  }

  Widget _buildGlassmorphismCard(dynamic item) {
    return Container(
      margin: EdgeInsets.only(bottom: 16),
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
          filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
          child: Padding(
            padding: EdgeInsets.all(20),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Text(
                      item['department'] ?? 'Department',
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        fontWeight: FontWeight.bold,
                        color: Colors.blue[800],
                      ),
                    ),
                    Container(
                      padding: EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                      decoration: BoxDecoration(
                        color: item['action_risk'] == 'High' ? Colors.orange[100] : Colors.green[100],
                        borderRadius: BorderRadius.circular(8),
                      ),
                      child: Text(
                        '${item['action_risk'] ?? 'Low'} Risk',
                        style: TextStyle(
                          fontSize: 10,
                          fontWeight: FontWeight.bold,
                          color: item['action_risk'] == 'High' ? Colors.orange[800] : Colors.green[800],
                        ),
                      ),
                    ),
                  ],
                ),
                SizedBox(height: 12),
                Text(
                  item['description'] ?? 'No description provided.',
                  style: TextStyle(
                    fontFamily: 'Inter',
                    fontSize: 14,
                    color: Colors.black87,
                  ),
                ),
                SizedBox(height: 16),
                Row(
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: [
                    TextButton(
                      onPressed: () {
                        // Implement edit/reject if needed
                      },
                      child: Text('Edit', style: TextStyle(color: Colors.grey[600])),
                    ),
                    SizedBox(width: 8),
                    ElevatedButton(
                      onPressed: () => _approveAction(item['id']),
                      style: ElevatedButton.styleFrom(
                        backgroundColor: Color(0xFF0066FF),
                        foregroundColor: Colors.white,
                        shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(8),
                        ),
                      ),
                      child: Text('Approve & Send'),
                    ),
                  ],
                )
              ],
            ),
          ),
        ),
      ),
    );
  }
}
