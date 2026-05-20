import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';

enum OnboardingState { idle, generating, draft, live }

class OnboardingScreen extends StatefulWidget {
  @override
  _OnboardingScreenState createState() => _OnboardingScreenState();
}

class _OnboardingScreenState extends State<OnboardingScreen> {
  final _formKey = GlobalKey<FormState>();
  String bio = '';
  OnboardingState _state = OnboardingState.idle;

  Future<void> submit() async {
    if (_formKey.currentState!.validate()) {
      _formKey.currentState!.save();
      setState(() => _state = OnboardingState.generating);

      try {
        final response = await http.post(
          Uri.parse('http://localhost:8080/api/onboarding/start'),
          headers: {'Content-Type': 'application/json'},
          body: jsonEncode({
            'company_name': bio, // use bio as company name for now based on previous behavior
            'business_type': 'Bakery',
            'selling_categories': ['food', 'physical'],
            'payment_pref': 'online',
            'admin_email': 'admin@test.com',
            'admin_name': 'Admin User',
            'admin_password': 'password123',
            'website_template': 'Modern',
            'first_product_name': 'Custom Cake Deposit',
            'first_product_price': '25.00',
            'domain_choice': 'subdomain',
            'price_type': 'fixed'
          }),
        );

        if (response.statusCode == 200) {
          setState(() => _state = OnboardingState.draft);
        } else {
          // If error occurs, go back to idle.
           setState(() => _state = OnboardingState.idle);
        }
      } catch (e) {
        print('Error: \$e');
         setState(() => _state = OnboardingState.idle);
      }
    }
  }

  void launchStore() {
    setState(() => _state = OnboardingState.live);
  }

  @override
  Widget build(BuildContext context) {
    if (_state == OnboardingState.live) {
      return StoreLiveScreen();
    }

    return Scaffold(
      backgroundColor: Color(0xFFF5F5F7), // Light background
      body: Center(
        child: Container(
          width: 375, // Mobile viewport constraint
          height: 812, // Standard mobile height
          child: ClipRRect(
            borderRadius: BorderRadius.circular(16),
            child: BackdropFilter(
              filter: ImageFilter.blur(sigmaX: 30, sigmaY: 30),
              child: Container(
                decoration: BoxDecoration(
                  color: Colors.white.withOpacity(0.65),
                  borderRadius: BorderRadius.circular(16),
                  border: Border.all(color: Colors.white.withOpacity(0.4), width: 1),
                ),
                child: _buildContent(),
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildContent() {
    switch (_state) {
      case OnboardingState.idle:
        return _buildIdleState();
      case OnboardingState.generating:
        return _buildGeneratingState();
      case OnboardingState.draft:
        return _buildDraftState();
      default:
        return SizedBox.shrink();
    }
  }

  Widget _buildIdleState() {
    return Padding(
      padding: EdgeInsets.all(24),
      child: Form(
        key: _formKey,
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text(
              'What do you do?',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 32,
                fontWeight: FontWeight.bold,
                color: Color(0xFF1D1D1F),
                letterSpacing: -0.5,
              ),
              textAlign: TextAlign.center,
            ),
            SizedBox(height: 16),
            Text(
              'Describe your business. We\'ll magically generate your storefront in seconds.',
              style: TextStyle(
                fontFamily: 'Inter',
                fontSize: 16,
                color: Colors.grey[600],
              ),
              textAlign: TextAlign.center,
            ),
            SizedBox(height: 32),
            TextFormField(
              decoration: InputDecoration(
                hintText: 'e.g., I bake custom vegan cakes in Seattle.',
                filled: true,
                fillColor: Colors.white,
                border: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(16),
                  borderSide: BorderSide.none,
                ),
                contentPadding: EdgeInsets.all(20),
              ),
              maxLines: 4,
              style: TextStyle(fontFamily: 'Inter', fontSize: 16),
              validator: (value) => value == null || value.isEmpty ? 'Required' : null,
              onSaved: (value) => bio = value!,
            ),
            SizedBox(height: 32),
            ElevatedButton(
              onPressed: submit,
              style: ElevatedButton.styleFrom(
                backgroundColor: Color(0xFF0066FF), // OHC Accent Blue
                foregroundColor: Colors.white,
                padding: EdgeInsets.symmetric(vertical: 18),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(16),
                ),
                elevation: 0,
              ),
              child: Text(
                'Build My Storefront',
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
    );
  }

  Widget _buildGeneratingState() {
    return Padding(
      padding: EdgeInsets.all(24),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          CircularProgressIndicator(
            valueColor: AlwaysStoppedAnimation<Color>(Color(0xFF0066FF)),
            strokeWidth: 3,
          ),
          SizedBox(height: 32),
          Text(
            'The Promoter is building your business...',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 24,
              fontWeight: FontWeight.bold,
              color: Color(0xFF1D1D1F),
            ),
            textAlign: TextAlign.center,
          ),
        ],
      ),
    );
  }

  Widget _buildDraftState() {
    return Column(
      children: [
        // Top banner
        Container(
          width: double.infinity,
          color: Colors.black87,
          padding: EdgeInsets.symmetric(vertical: 8, horizontal: 16),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text(
                'Preview Mode',
                style: TextStyle(color: Colors.white, fontSize: 12, fontWeight: FontWeight.bold),
              ),
              Container(
                padding: EdgeInsets.symmetric(horizontal: 8, vertical: 2),
                decoration: BoxDecoration(
                  color: Colors.white24,
                  borderRadius: BorderRadius.circular(4),
                ),
                child: Text(
                  '375px',
                  style: TextStyle(color: Colors.white, fontSize: 10),
                ),
              ),
            ],
          ),
        ),
        // Fake Store Preview
        Expanded(
          child: Container(
            color: Colors.white,
            width: double.infinity,
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Icon(Icons.storefront, size: 80, color: Colors.grey[300]),
                SizedBox(height: 16),
                Text(
                  'Your Beautiful Store',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 24,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                SizedBox(height: 8),
                Text(
                  'Generated based on your bio.',
                  style: TextStyle(color: Colors.grey[500]),
                ),
              ],
            ),
          ),
        ),
        // Bottom Action Bar
        Container(
          padding: EdgeInsets.all(16),
          decoration: BoxDecoration(
            color: Colors.white,
            border: Border(top: BorderSide(color: Colors.grey[200]!)),
          ),
          child: ElevatedButton(
            onPressed: launchStore,
            style: ElevatedButton.styleFrom(
              backgroundColor: Color(0xFF0066FF),
              foregroundColor: Colors.white,
              padding: EdgeInsets.symmetric(vertical: 18),
              minimumSize: Size(double.infinity, 50),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(16),
              ),
              elevation: 0,
            ),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Text(
                  '1-Tap Launch',
                  style: TextStyle(
                    fontFamily: 'Inter',
                    fontSize: 16,
                    fontWeight: FontWeight.w600,
                  ),
                ),
                SizedBox(width: 8),
                Icon(Icons.rocket_launch, size: 18),
              ],
            ),
          ),
        ),
      ],
    );
  }
}

class StoreLiveScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Color(0xFFF5F5F7),
      body: Center(
        child: Container(
          width: 375,
          height: 812,
          padding: EdgeInsets.all(24),
          child: ClipRRect(
            borderRadius: BorderRadius.circular(16),
            child: BackdropFilter(
              filter: ImageFilter.blur(sigmaX: 30, sigmaY: 30),
              child: Container(
                decoration: BoxDecoration(
                  color: Colors.white.withOpacity(0.65),
                  borderRadius: BorderRadius.circular(16),
                  border: Border.all(color: Colors.white.withOpacity(0.4), width: 1),
                ),
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Container(
                      padding: EdgeInsets.all(20),
                      decoration: BoxDecoration(
                        color: Color(0xFF34C759).withOpacity(0.1),
                        shape: BoxShape.circle,
                      ),
                      child: Icon(Icons.check_circle, size: 64, color: Color(0xFF34C759)),
                    ),
                    SizedBox(height: 32),
                    Text(
                      'You\'re Live!',
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 32,
                        fontWeight: FontWeight.bold,
                        color: Color(0xFF1D1D1F),
                      ),
                      textAlign: TextAlign.center,
                    ),
                    SizedBox(height: 16),
                    Text(
                      'Your automated storefront is successfully published.',
                      style: TextStyle(
                        fontFamily: 'Inter',
                        fontSize: 16,
                        color: Colors.grey[600],
                      ),
                      textAlign: TextAlign.center,
                    ),
                    SizedBox(height: 48),
                    Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 24.0),
                      child: ElevatedButton(
                        onPressed: () {},
                        style: ElevatedButton.styleFrom(
                          backgroundColor: Colors.grey[100],
                          foregroundColor: Color(0xFF1D1D1F),
                          padding: EdgeInsets.symmetric(vertical: 18),
                          minimumSize: Size(double.infinity, 50),
                          shape: RoundedRectangleBorder(
                            borderRadius: BorderRadius.circular(16),
                          ),
                          elevation: 0,
                        ),
                        child: Text(
                          'Go to Dashboard',
                          style: TextStyle(
                            fontFamily: 'Inter',
                            fontSize: 16,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
