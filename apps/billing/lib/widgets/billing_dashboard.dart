import 'dart:ui';
import 'package:flutter/material.dart';

class BillingDashboard extends StatelessWidget {
  const BillingDashboard({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(16.0),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          color: const Color.fromRGBO(255, 255, 255, 0.03),
          padding: const EdgeInsets.all(24.0),
          child: const Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                'Cost Engineering Dashboard',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 24.0,
                  color: Colors.white,
                ),
              ),
              SizedBox(height: 16.0),
              Text(
                'Storage Compression Active',
                style: TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 16.0,
                  color: Colors.greenAccent,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
