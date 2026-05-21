import 'package:flutter/material.dart';
import 'dart:ui';
import 'package:http/http.dart' as http;
import 'dart:convert';

class GrowthReferralWidget extends StatefulWidget {
  @override
  _GrowthReferralWidgetState createState() => _GrowthReferralWidgetState();
}

class _GrowthReferralWidgetState extends State<GrowthReferralWidget> {
  String _referralLink = '';
  bool _isLoading = false;

  Future<void> _generateReferralLink() async {
    setState(() {
      _isLoading = true;
    });

    try {
      final response = await http.post(
        Uri.parse('http://localhost:8080/api/v1/growth/referrals/generate'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({'user_id': 'local-user-id', 'context_type': 'standalone-bridge'}),
      );

      if (response.statusCode == 200) {
        final data = jsonDecode(response.body);
        setState(() {
          _referralLink = data['referral_link'];
        });
      } else {
        setState(() {
          _referralLink = 'Error: Failed to generate link';
        });
      }
    } catch (e) {
      setState(() {
        _referralLink = 'Error: $e';
      });
    } finally {
      setState(() {
        _isLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
        child: Container(
          padding: const EdgeInsets.all(24),
          decoration: BoxDecoration(
            color: Colors.white.withOpacity(0.05),
            border: Border.all(color: Colors.white.withOpacity(0.1), width: 1),
            borderRadius: BorderRadius.circular(12),
          ),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                'Invite a Collaborator',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const SizedBox(height: 8),
              Text(
                'Bridge your standalone context with the Cloud securely.',
                style: TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 14,
                  color: Colors.white70,
                ),
              ),
              const SizedBox(height: 24),
              ElevatedButton(
                onPressed: _isLoading ? null : _generateReferralLink,
                style: ElevatedButton.styleFrom(
                  backgroundColor: Colors.blueAccent,
                  foregroundColor: Colors.white,
                  padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(8),
                  ),
                ),
                child: _isLoading
                  ? const SizedBox(
                      width: 20,
                      height: 20,
                      child: CircularProgressIndicator(color: Colors.white, strokeWidth: 2)
                    )
                  : const Text('Generate Referral Link'),
              ),
              if (_referralLink.isNotEmpty) ...[
                const SizedBox(height: 16),
                Container(
                  padding: const EdgeInsets.all(12),
                  decoration: BoxDecoration(
                    color: Colors.black.withOpacity(0.2),
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(color: Colors.white.withOpacity(0.05)),
                  ),
                  child: Row(
                    children: [
                      Expanded(
                        child: Text(
                          _referralLink,
                          style: TextStyle(
                            fontFamily: 'Inter',
                            color: Colors.blue[200],
                            fontSize: 14,
                          ),
                          overflow: TextOverflow.ellipsis,
                        ),
                      ),
                      Icon(Icons.copy, color: Colors.white54, size: 18),
                    ],
                  ),
                ),
              ],
            ],
          ),
        ),
      ),
    );
  }
}
