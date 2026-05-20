import 'package:flutter/material.dart';

class DashboardScreen extends StatefulWidget {
  @override
  _DashboardScreenState createState() => _DashboardScreenState();
}

class _DashboardScreenState extends State<DashboardScreen> {
  // Mock Data
  List<Map<String, String>> agentActivities = [
    {"agent": "The Promoter", "action": "generated 3 quotes", "time": "2h ago"},
    {"agent": "The Manager", "action": "processed 2 orders", "time": "5h ago"},
    {
      "agent": "The Advisor",
      "action": "updated weekly report",
      "time": "1d ago",
    },
  ];

  int productCount = 95;
  bool firstOrderReceived = false;

  @override
  void initState() {
    super.initState();
    // Simulate checking for milestones shortly after dashboard loads
    Future.delayed(Duration(seconds: 2), () {
      _checkMilestones();
    });
  }

  void _checkMilestones() {
    if (!mounted) return;
    if (!firstOrderReceived) {
      _triggerMilestone(
        "First Order Received! The Manager is processing it.",
        false,
      );
      setState(() {
        firstOrderReceived = true;
      });
    }

    Future.delayed(Duration(seconds: 4), () {
      if (!mounted) return;
      if (productCount >= 95) {
        _triggerMilestone(
          "Product count approaching 100 limit. Upgrade to Starter for unlimited products.",
          true,
        );
      }
    });
  }

  void _triggerMilestone(String message, bool isUpgrade) {
    if (!mounted) return;

    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(
          message,
          style: TextStyle(fontFamily: 'Inter', color: Colors.white),
        ),
        backgroundColor: isUpgrade ? Color(0xFFFF9500) : Color(0xFF34C759),
        behavior: SnackBarBehavior.floating,
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
        margin: EdgeInsets.all(16),
        action:
            isUpgrade
                ? SnackBarAction(
                  label: 'Upgrade',
                  textColor: Colors.white,
                  onPressed: () {
                    // Navigate to upgrade screen
                  },
                )
                : null,
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Color(0xFFF5F5F7), // Light background
      appBar: AppBar(
        title: Text(
          'Dashboard',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontWeight: FontWeight.bold,
            color: Color(0xFF1D1D1F),
          ),
        ),
        backgroundColor: Colors.white,
        elevation: 0,
        iconTheme: IconThemeData(color: Color(0xFF1D1D1F)),
      ),
      body: Center(
        child: Container(
          width: 375, // 375px mobile target
          child: ListView(
            padding: EdgeInsets.all(16),
            children: [
              _buildActionableInsights(),
              SizedBox(height: 24),
              _buildAgentActivityFeed(),
            ],
          ),
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          // Quick actions logic
        },
        backgroundColor: Color(0xFF0066FF),
        child: Icon(Icons.add, color: Colors.white),
      ),
    );
  }

  Widget _buildActionableInsights() {
    return Container(
      padding: EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(16),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.05),
            blurRadius: 10,
            offset: Offset(0, 5),
          ),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(Icons.insights, color: Color(0xFF0066FF)),
              SizedBox(width: 8),
              Text(
                'Actionable Insights',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ],
          ),
          SizedBox(height: 12),
          Text(
            'You had a busy weekend! See your revenue report.',
            style: TextStyle(
              fontFamily: 'Inter',
              fontSize: 14,
              color: Colors.grey[600],
            ),
          ),
          SizedBox(height: 12),
          ElevatedButton(
            onPressed: () {},
            style: ElevatedButton.styleFrom(
              backgroundColor: Color(0xFF0066FF).withOpacity(0.1),
              foregroundColor: Color(0xFF0066FF),
              elevation: 0,
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(8),
              ),
            ),
            child: Text('View Report'),
          ),
        ],
      ),
    );
  }

  Widget _buildAgentActivityFeed() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'Agent Actions Today',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 20,
            fontWeight: FontWeight.bold,
            color: Color(0xFF1D1D1F),
          ),
        ),
        SizedBox(height: 16),
        ...agentActivities.map((activity) {
          return Container(
            margin: EdgeInsets.only(bottom: 12),
            padding: EdgeInsets.all(16),
            decoration: BoxDecoration(
              color: Colors.white,
              borderRadius: BorderRadius.circular(12),
              border: Border.all(color: Colors.grey[200]!),
            ),
            child: Row(
              children: [
                CircleAvatar(
                  backgroundColor: Color(0xFF34C759).withOpacity(0.1),
                  child: Icon(
                    Icons.smart_toy,
                    color: Color(0xFF34C759),
                    size: 20,
                  ),
                ),
                SizedBox(width: 12),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        activity["agent"]!,
                        style: TextStyle(
                          fontFamily: 'Inter',
                          fontWeight: FontWeight.bold,
                          fontSize: 14,
                        ),
                      ),
                      Text(
                        '${activity["agent"]} ${activity["action"]}',
                        style: TextStyle(
                          fontFamily: 'Inter',
                          color: Colors.grey[600],
                          fontSize: 14,
                        ),
                      ),
                    ],
                  ),
                ),
                Text(
                  activity["time"]!,
                  style: TextStyle(
                    fontFamily: 'Inter',
                    color: Colors.grey[400],
                    fontSize: 12,
                  ),
                ),
              ],
            ),
          );
        }).toList(),
      ],
    );
  }
}
