import 'dart:convert';
import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:shared_preferences/shared_preferences.dart';

class AgentAuditDashboard extends StatefulWidget {
  @override
  _AgentAuditDashboardState createState() => _AgentAuditDashboardState();
}

class _AgentAuditDashboardState extends State<AgentAuditDashboard> {
  bool _isLoading = true;
  String _errorMessage = '';

  String _cost = '\$0.00';
  List<dynamic> _approvals = [];

  @override
  void initState() {
    super.initState();
    _fetchData();
  }

  Future<void> _fetchData() async {
    try {
      final prefs = await SharedPreferences.getInstance();
      final token = prefs.getString('auth_token') ?? '';
      final baseUrl = const String.fromEnvironment('API_BASE_URL', defaultValue: 'http://localhost:18789');

      final headers = {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer $token'
      };

      // Fetch approvals (which we use for "violations" or alerts)
      final approvalsResponse = await http.get(Uri.parse('$baseUrl/api/agents/approvals'), headers: headers);
      if (approvalsResponse.statusCode == 200) {
        final data = json.decode(approvalsResponse.body);
        _approvals = data['pending_approvals'] ?? [];
      } else {
         _errorMessage = 'Failed to load approvals: ${approvalsResponse.statusCode}';
      }

      // Fetch cost dashboard
      final costResponse = await http.get(Uri.parse('$baseUrl/api/billing/cost-dashboard'), headers: headers);
      if (costResponse.statusCode == 200) {
        final data = json.decode(costResponse.body);
        final totalCostsCents = data['total_costs'] ?? 0;
        _cost = '\$${(totalCostsCents / 100.0).toStringAsFixed(2)}';
      } else {
        _errorMessage = 'Failed to load costs: ${costResponse.statusCode}';
      }

      setState(() {
        _isLoading = false;
      });

    } catch (e) {
      setState(() {
        _errorMessage = 'Error fetching data: $e';
        _isLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Color(0xFFF5F5F7), // Background color matching the aesthetic
      appBar: AppBar(
        title: Text(
          'Agent Audit Dashboard',
          style: TextStyle(fontFamily: 'Outfit', color: Colors.black87, fontWeight: FontWeight.bold),
        ),
        backgroundColor: Colors.white.withOpacity(0.8),
        elevation: 0,
        flexibleSpace: ClipRect(
          child: BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0), // Mandated 20px blur
            child: Container(color: Colors.transparent),
          ),
        ),
      ),
      body: _isLoading
          ? Center(child: CircularProgressIndicator())
          : _errorMessage.isNotEmpty
            ? Center(child: Text(_errorMessage, style: TextStyle(color: Colors.red)))
            : LayoutBuilder(
        builder: (context, constraints) {
          final isMobile = constraints.maxWidth < 768;

          if (isMobile) {
            return SingleChildScrollView(
              child: Column(
                children: [
                  _buildCostTracker(),
                  _buildMobileAuditGrid(),
                  _buildViolationFeed(isMobile: true),
                ],
              ),
            );
          }

          return Row(
            children: [
              Expanded(
                flex: 3,
                child: GridView.count(
                  crossAxisCount: 2,
                  padding: EdgeInsets.all(16),
                  children: _buildAuditCards(),
                ),
              ),
              Expanded(
                flex: 1,
                child: Column(
                  children: [
                    _buildCostTracker(),
                    Expanded(child: _buildViolationFeed()),
                  ],
                ),
              ),
            ],
          );
        },
      ),
    );
  }

  List<Widget> _buildAuditCards() {
    return [
      _buildAuditCard('Operations', 'The Manager - Health: OK', Colors.green),
      _buildAuditCard('Marketing & Advertising', 'The Promoter - High Load', Colors.orange),
      _buildAuditCard('Sales & Acquisition', 'The Salesperson - Health: OK', Colors.green),
      _buildAuditCard('Customer Success', 'The Ambassador - Health: OK', Colors.green),
      _buildAuditCard('Finance & Payments', 'The Accountant - Health: OK', Colors.green),
      _buildAuditCard('Legal & Compliance', 'The Protector - Health: OK', Colors.green),
      _buildAuditCard('Business Advisory', 'The Advisor - Processing', Colors.blue),
    ];
  }

  Widget _buildMobileAuditGrid() {
    return Padding(
      padding: EdgeInsets.symmetric(horizontal: 16),
      child: Column(
        children: _buildAuditCards(),
      ),
    );
  }

  Widget _buildAuditCard(String name, String status, Color statusColor) {
    return Container(
      margin: EdgeInsets.symmetric(vertical: 8, horizontal: 4),
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
            padding: EdgeInsets.all(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Text(
                  name,
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 18,
                    fontWeight: FontWeight.bold,
                    color: Colors.black87,
                  ),
                ),
                SizedBox(height: 8),
                Row(
                  children: [
                    Container(
                      width: 12,
                      height: 12,
                      decoration: BoxDecoration(
                        color: statusColor,
                        shape: BoxShape.circle,
                      ),
                    ),
                    SizedBox(width: 8),
                    Expanded(
                      child: Text(
                        status,
                        style: TextStyle(
                          fontFamily: 'Inter',
                          fontSize: 14,
                          color: Colors.black87,
                        ),
                        overflow: TextOverflow.ellipsis,
                      ),
                    ),
                  ],
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildCostTracker() {
    return Container(
      margin: EdgeInsets.all(16),
      width: double.infinity,
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
            padding: EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.center,
              children: [
                Text(
                  'Cost Tracker',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 16,
                    fontWeight: FontWeight.w600,
                    color: Colors.black54,
                  ),
                ),
                SizedBox(height: 8),
                Text(
                  _cost,
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 32,
                    fontWeight: FontWeight.bold,
                    color: Colors.black87,
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildViolationFeed({bool isMobile = false}) {
    List<Widget> feedItems = _approvals.map((approval) {
      final risk = approval['action_risk'] ?? 'Unknown';
      final desc = approval['description'] ?? 'No description';
      final dept = approval['department'] ?? 'System';
      return _buildViolationItem('[$risk Risk] $desc', dept, 'Just now');
    }).toList();

    if (feedItems.isEmpty) {
      feedItems = [
        Padding(
          padding: EdgeInsets.symmetric(vertical: 8),
          child: Text('No recent violations or alerts.', style: TextStyle(color: Colors.black54)),
        )
      ];
    }

    final innerContent = Padding(
      padding: EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: isMobile ? MainAxisSize.min : MainAxisSize.max,
        children: [
          Text(
            'Violation Feed',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 16,
              fontWeight: FontWeight.bold,
              color: Colors.red[800],
            ),
          ),
          Divider(color: Colors.red.withOpacity(0.2)),
          if (isMobile)
            Column(children: feedItems)
          else
            Expanded(
              child: ListView(children: feedItems),
            ),
        ],
      ),
    );

    return Container(
      margin: EdgeInsets.only(left: 16, right: 16, bottom: 16),
      width: double.infinity,
      decoration: BoxDecoration(
        color: Colors.red.withOpacity(0.05),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: Colors.red.withOpacity(0.2), width: 1),
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(16),
        child: BackdropFilter(
          filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
          child: innerContent,
        ),
      ),
    );
  }

  Widget _buildViolationItem(String message, String agent, String time) {
    return Padding(
      padding: EdgeInsets.symmetric(vertical: 8),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            message,
            style: TextStyle(fontFamily: 'Inter', fontSize: 14, color: Colors.black87),
          ),
          SizedBox(height: 4),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Expanded(
                child: Text(
                  agent,
                  style: TextStyle(fontFamily: 'Inter', fontSize: 12, color: Colors.black54),
                  overflow: TextOverflow.ellipsis,
                ),
              ),
              Text(
                time,
                style: TextStyle(fontFamily: 'Inter', fontSize: 12, color: Colors.black54),
              ),
            ],
          ),
        ],
      ),
    );
  }
}
