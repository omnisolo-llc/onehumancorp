import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import '../config/api.dart';

class AgentFeedScreen extends StatefulWidget {
  @override
  _AgentFeedScreenState createState() => _AgentFeedScreenState();
}

class _AgentFeedScreenState extends State<AgentFeedScreen> {
  List<dynamic> approvals = [];
  bool loading = true;

  @override
  void initState() {
    super.initState();
    fetchApprovals();
  }

  Future<void> fetchApprovals() async {
    try {
      final response = await http.get(Uri.parse('${ApiConfig.baseUrl}/api/agents/approvals'));
      if (response.statusCode == 200) {
        setState(() {
          approvals = jsonDecode(response.body)['pending_approvals'] ?? [];
          loading = false;
        });
      }
    } catch (e) {
      setState(() => loading = false);
    }
  }

  Future<void> handleApproval(String id, bool approve) async {
    try {
      await http.post(
        Uri.parse('${ApiConfig.baseUrl}/api/agents/approvals/$id'),
        body: jsonEncode({'approved': approve}),
        headers: {'Content-Type': 'application/json'}
      );
      fetchApprovals();
    } catch (e) {
      print('Error: $e');
    }
  }

  @override
  Widget build(BuildContext context) {
    if (loading) {
      return Scaffold(
        body: Center(child: CircularProgressIndicator()),
      );
    }

    return Scaffold(
      backgroundColor: Color(0xFFF5F5F7),
      appBar: AppBar(
        title: Text('Agent Actions Today', style: TextStyle(color: Colors.black)),
        backgroundColor: Colors.white,
        elevation: 0,
        iconTheme: IconThemeData(color: Colors.black),
        actions: [
          IconButton(
            icon: Icon(Icons.settings),
            onPressed: () {
               Navigator.push(context, MaterialPageRoute(builder: (context) => SettingsScreen()));
            },
          )
        ],
      ),
      body: ListView.builder(
        padding: EdgeInsets.all(16),
        itemCount: approvals.length,
        itemBuilder: (context, index) {
          final item = approvals[index];
          return Card(
            margin: EdgeInsets.only(bottom: 16),
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
            child: Padding(
              padding: EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Container(
                        padding: EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                        decoration: BoxDecoration(
                          color: item['department'] == 'CustomerSuccess' ? Colors.blue.shade100 : Colors.green.shade100,
                          borderRadius: BorderRadius.circular(4),
                        ),
                        child: Text(
                          item['department'],
                          style: TextStyle(fontSize: 12, fontWeight: FontWeight.bold),
                        ),
                      ),
                      Spacer(),
                      Text(item['action_risk'] == 'HIGH' ? 'Needs Review' : 'Auto', style: TextStyle(color: Colors.grey, fontSize: 12)),
                    ],
                  ),
                  SizedBox(height: 12),
                  Text(item['description'], style: TextStyle(fontSize: 16)),
                  SizedBox(height: 16),
                  if (item['action_risk'] == 'HIGH' || item['action_risk'] == 'High' || item['status'] == 'Pending')
                  Row(
                    mainAxisAlignment: MainAxisAlignment.end,
                    children: [
                      TextButton(
                        onPressed: () {
                           Navigator.push(context, MaterialPageRoute(builder: (context) => ActionDetailScreen(item: item, onApprove: () {
                             handleApproval(item['id'], true);
                             Navigator.pop(context);
                           }, onEdit: () {
                             // Edit logic
                           })));
                        },
                        child: Text('View Details', style: TextStyle(color: Colors.blue)),
                      ),
                      Spacer(),
                      TextButton(
                        onPressed: () => handleApproval(item['id'], false),
                        child: Text('Reject', style: TextStyle(color: Colors.red)),
                      ),
                      SizedBox(width: 8),
                      ElevatedButton(
                        onPressed: () => handleApproval(item['id'], true),
                        style: ElevatedButton.styleFrom(backgroundColor: Colors.blue),
                        child: Text('Approve & Send', style: TextStyle(color: Colors.white)),
                      ),
                    ],
                  )
                ],
              ),
            ),
          );
        },
      ),
    );
  }
}

class ActionDetailScreen extends StatelessWidget {
  final dynamic item;
  final VoidCallback onApprove;
  final VoidCallback onEdit;

  ActionDetailScreen({required this.item, required this.onApprove, required this.onEdit});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Action Details', style: TextStyle(color: Colors.black)),
        backgroundColor: Colors.white,
        elevation: 0,
        iconTheme: IconThemeData(color: Colors.black),
      ),
      body: Padding(
        padding: EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Department: ${item['department']}', style: TextStyle(fontWeight: FontWeight.bold, fontSize: 18)),
            SizedBox(height: 16),
            Text('Risk Level: ${item['action_risk']}', style: TextStyle(color: Colors.grey)),
            SizedBox(height: 16),
            Text('Description:', style: TextStyle(fontWeight: FontWeight.bold)),
            SizedBox(height: 8),
            Text(item['description'], style: TextStyle(fontSize: 16)),
            Spacer(),
            Row(
              children: [
                Expanded(
                  child: OutlinedButton(
                    onPressed: onEdit,
                    child: Text('Edit'),
                  ),
                ),
                SizedBox(width: 16),
                Expanded(
                  child: ElevatedButton(
                    onPressed: onApprove,
                    style: ElevatedButton.styleFrom(backgroundColor: Colors.blue),
                    child: Text('Approve & Send', style: TextStyle(color: Colors.white)),
                  ),
                ),
              ],
            )
          ],
        ),
      ),
    );
  }
}

class SettingsScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Agent Settings', style: TextStyle(color: Colors.black)),
        backgroundColor: Colors.white,
        elevation: 0,
        iconTheme: IconThemeData(color: Colors.black),
      ),
      body: ListView(
        padding: EdgeInsets.all(16),
        children: [
          SwitchListTile(
            title: Text('Auto-reply to common questions'),
            value: true,
            onChanged: (val) {},
          ),
          SwitchListTile(
            title: Text('Auto-draft social posts'),
            value: true,
            onChanged: (val) {},
          ),
        ],
      ),
    );
  }
}
