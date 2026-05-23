import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:shared_preferences/shared_preferences.dart';
import 'package:flutter/material.dart';
import 'dart:ui';

class GrowthReferralWidget extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
        child: Container(
          decoration: BoxDecoration(
            color: Colors.white.withOpacity(0.05),
            borderRadius: BorderRadius.circular(12),
            border: Border.all(color: Colors.white.withOpacity(0.1)),
          ),
          padding: EdgeInsets.all(24),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                'Invite Collaborator',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              SizedBox(height: 8),
              Text(
                'Invite a partner or employee to help manage your business.',
                style: TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 14,
                  color: Colors.white70,
                ),
              ),
              SizedBox(height: 16),
              ElevatedButton(
                onPressed: () async {
                  try {
                    final prefs = await SharedPreferences.getInstance();
                    final token = prefs.getString('auth_token') ?? '';
                    final baseUrl = const String.fromEnvironment('API_BASE_URL', defaultValue: 'http://localhost:18789');

                    final response = await http.post(
                      Uri.parse('$baseUrl/referrals/generate'),
                      headers: {
                        'Content-Type': 'application/json',
                        'Authorization': 'Bearer $token'
                      },
                    );
                    if (response.statusCode == 200) {
                      final data = jsonDecode(response.body);
                      ScaffoldMessenger.of(context).showSnackBar(
                        SnackBar(content: Text('Link: ${data["referral_link"]}')),
                      );
                    }
                  } catch (e) {
                    ScaffoldMessenger.of(context).showSnackBar(
                      SnackBar(content: Text('Error generating link')),
                    );
                  }
                },
                style: ElevatedButton.styleFrom(
                  backgroundColor: Color(0xFF0066FF),
                  foregroundColor: Colors.white,
                  padding: EdgeInsets.symmetric(vertical: 12, horizontal: 24),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(8),
                  ),
                ),
                child: Text('Get Invite Link'),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class UserManagementScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Color(0xFF1A1A1A), // Dark theme matching growth strategy aesthetic
      appBar: AppBar(
        title: Text('User Management'),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      body: SingleChildScrollView(
        padding: EdgeInsets.all(24),
        child: Column(
          children: [
            GrowthReferralWidget(),
          ],
        ),
      ),
    );
  }
}
