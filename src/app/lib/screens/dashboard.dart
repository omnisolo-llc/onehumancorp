import 'package:flutter/material.dart';

class DashboardScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Color(0xFFF5F5F7),
      appBar: AppBar(
        title: Text(
          'Dashboard',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontWeight: FontWeight.bold,
            color: Color(0xFF1D1D1F),
          ),
        ),
        backgroundColor: Colors.white.withOpacity(0.65),
        elevation: 0,
        centerTitle: false,
      ),
      body: Center(
        child: Container(
          width: 375, // Enforce mobile-first constraint
          padding: EdgeInsets.all(24),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Text(
                'Business Snapshot',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 24,
                  fontWeight: FontWeight.bold,
                  color: Color(0xFF1D1D1F),
                ),
              ),
              SizedBox(height: 24),
              _buildMetricCard('Today\\'s Sales', '\$0.00'),
              SizedBox(height: 16),
              _buildMetricCard('Active Customers', '12'),
              SizedBox(height: 16),
              _buildMetricCard('Pending Orders', '3'),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildMetricCard(String title, String value) {
    return Container(
      padding: EdgeInsets.all(20),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: Colors.grey.withOpacity(0.2)),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.02),
            blurRadius: 10,
            offset: Offset(0, 4),
          ),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            title,
            style: TextStyle(
              fontFamily: 'Inter',
              fontSize: 14,
              fontWeight: FontWeight.w500,
              color: Color(0xFF86868B),
            ),
          ),
          SizedBox(height: 8),
          Text(
            value,
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 32,
              fontWeight: FontWeight.bold,
              color: Color(0xFF1D1D1F),
            ),
          ),
        ],
      ),
    );
  }
}
