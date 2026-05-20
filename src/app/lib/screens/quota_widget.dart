import 'dart:ui';
import 'package:flutter/material.dart';

class FreeTierQuotaWidget extends StatelessWidget {
  final int currentReferrals;
  final int maxReferrals;
  final VoidCallback onInvitePressed;

  const FreeTierQuotaWidget({
    Key? key,
    this.currentReferrals = 0,
    this.maxReferrals = 5,
    required this.onInvitePressed,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 30, sigmaY: 30),
        child: Container(
          padding: EdgeInsets.all(24),
          decoration: BoxDecoration(
            color: Colors.white.withOpacity(0.65), // Light Mode Translucent Glass
            borderRadius: BorderRadius.circular(16),
            border: Border.all(color: Colors.white.withOpacity(0.4), width: 1),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              Wrap(
                alignment: WrapAlignment.spaceBetween,
                crossAxisAlignment: WrapCrossAlignment.center,
                children: [
                  Text(
                    'Free-Tier Quota',
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 20,
                      fontWeight: FontWeight.bold,
                      color: Color(0xFF1D1D1F), // Core Light text
                      letterSpacing: -0.5,
                    ),
                  ),
                  Container(
                    padding: EdgeInsets.symmetric(horizontal: 12, vertical: 4),
                    decoration: BoxDecoration(
                      color: Colors.black.withOpacity(0.05),
                      borderRadius: BorderRadius.circular(99),
                    ),
                    child: Text(
                      '$currentReferrals / $maxReferrals Referrals',
                      style: TextStyle(
                        fontFamily: 'Inter',
                        fontSize: 14,
                        fontWeight: FontWeight.w600,
                        color: Colors.grey[700],
                      ),
                    ),
                  ),
                ],
              ),
              SizedBox(height: 16),
              // Progress Bar
              Container(
                width: double.infinity,
                height: 12,
                decoration: BoxDecoration(
                  color: Color(0xFFE2E8F0),
                  borderRadius: BorderRadius.circular(99),
                ),
                child: FractionallySizedBox(
                  alignment: Alignment.centerLeft,
                  widthFactor: maxReferrals > 0 ? (currentReferrals / maxReferrals).clamp(0.0, 1.0) : 0,
                  child: Container(
                    decoration: BoxDecoration(
                      color: Color(0xFF0066FF), // OHC Accent Blue
                      borderRadius: BorderRadius.circular(99),
                      boxShadow: [
                        BoxShadow(
                          color: Color(0xFF0066FF).withOpacity(0.5),
                          blurRadius: 10,
                          offset: Offset(0, 0),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
              SizedBox(height: 12),
              Text(
                'Expand your quota by inviting more businesses.',
                style: TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 14,
                  color: Colors.grey[600],
                ),
              ),
              SizedBox(height: 24),
              ElevatedButton(
                onPressed: onInvitePressed,
                style: ElevatedButton.styleFrom(
                  backgroundColor: Color(0xFF0066FF), // Apple/UniFi Blue
                  foregroundColor: Colors.white,
                  padding: EdgeInsets.symmetric(vertical: 16),
                  minimumSize: Size(double.infinity, 50),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(8),
                  ),
                  elevation: 0,
                ),
                child: Text(
                  'Invite Team to Expand Quota',
                  style: TextStyle(
                    fontFamily: 'Inter',
                    fontSize: 16,
                    fontWeight: FontWeight.w600,
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
