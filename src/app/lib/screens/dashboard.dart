import 'package:flutter/material.dart';
import 'dart:ui';
import 'package:http/http.dart' as http;
import 'dart:convert';

class DashboardScreen extends StatefulWidget {
  @override
  _DashboardScreenState createState() => _DashboardScreenState();
}

class _DashboardScreenState extends State<DashboardScreen> {
  String summary = 'Loading your weekly snapshot...';
  String suggestion = '';
  bool isLoading = true;

  @override
  void initState() {
    super.initState();
    _fetchHealthReport();
  }

  Future<void> _fetchHealthReport() async {
    try {
      final response = await http.get(Uri.parse(const String.fromEnvironment('API_URL', defaultValue: 'http://10.0.2.2:8080') + '/api/v1/dashboard/health_report'));
      if (response.statusCode == 200) {
        final data = jsonDecode(response.body);
        setState(() {
          summary = data['summary'] ?? 'No summary available.';
          suggestion = data['actionable_suggestion'] ?? '';
          isLoading = false;
        });
      } else {
        setState(() {
          summary = 'Could not load report.';
          isLoading = false;
        });
      }
    } catch (e) {
      setState(() {
        summary = 'Failed to connect to server.';
        isLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Color(0xFFF3F4F6),
      appBar: AppBar(
        title: Text('My Dashboard'),
        elevation: 0,
        backgroundColor: Colors.white,
        foregroundColor: Colors.black,
      ),
      body: Center(
        child: Container(
          width: 375,
          child: ListView(
            padding: EdgeInsets.all(16),
            children: [
              Text(
                'Weekly Business Snapshot',
                style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
              ),
              SizedBox(height: 16),
              ClipRRect(
                borderRadius: BorderRadius.circular(20),
                child: BackdropFilter(
                  filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
                  child: Container(
                    decoration: BoxDecoration(
                      color: Colors.white.withOpacity(0.6),
                      borderRadius: BorderRadius.circular(20),
                      border: Border.all(color: Colors.white.withOpacity(0.8)),
                      boxShadow: [
                        BoxShadow(
                          color: Colors.black.withOpacity(0.05),
                          blurRadius: 10,
                          offset: Offset(0, 5),
                        )
                      ],
                    ),
                    padding: EdgeInsets.all(24),
                    child: isLoading ? Center(child: CircularProgressIndicator()) : Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          '📈 Your Weekly Health Report',
                          style: TextStyle(
                            fontFamily: 'Outfit',
                            fontSize: 18,
                            fontWeight: FontWeight.bold,
                            color: Colors.black87,
                          ),
                        ),
                        SizedBox(height: 16),
                        Text(
                          summary,
                          style: TextStyle(
                            fontFamily: 'Outfit',
                            fontSize: 16,
                            color: Colors.black87,
                          ),
                        ),
                        if (suggestion.isNotEmpty) ...[
                          SizedBox(height: 16),
                          Container(
                            padding: EdgeInsets.all(12),
                            decoration: BoxDecoration(
                              color: Colors.blue.withOpacity(0.1),
                              borderRadius: BorderRadius.circular(12),
                            ),
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Text(
                                  '💡 Actionable Suggestion',
                                  style: TextStyle(
                                    fontFamily: 'Outfit',
                                    fontWeight: FontWeight.bold,
                                    color: Colors.blue[800],
                                  ),
                                ),
                                SizedBox(height: 8),
                                Text(
                                  suggestion,
                                  style: TextStyle(
                                    fontFamily: 'Outfit',
                                    fontSize: 14,
                                    color: Colors.black87,
                                  ),
                                ),
                                SizedBox(height: 12),
                                Row(
                                  children: [
                                    ElevatedButton(
                                      onPressed: () {},
                                      style: ElevatedButton.styleFrom(
                                        backgroundColor: Colors.blue,
                                        foregroundColor: Colors.white,
                                        shape: RoundedRectangleBorder(
                                          borderRadius: BorderRadius.circular(8),
                                        ),
                                      ),
                                      child: Text('Yes, draft it'),
                                    ),
                                    SizedBox(width: 8),
                                    TextButton(
                                      onPressed: () {},
                                      child: Text('No thanks', style: TextStyle(color: Colors.grey[700])),
                                    ),
                                  ],
                                ),
                              ],
                            ),
                          ),
                        ],
                      ],
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
