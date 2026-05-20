import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';

enum OnboardingState { welcome, input, generating, launching, dashboard, draft, live }

class OnboardingScreen extends StatefulWidget {
  @override
  _OnboardingScreenState createState() => _OnboardingScreenState();
}

class _OnboardingScreenState extends State<OnboardingScreen> {
  final _formKey = GlobalKey<FormState>();
  String businessIntent = '';
  OnboardingState _state = OnboardingState.welcome;

  Future<void> submit() async {
    if (_formKey.currentState!.validate()) {
      _formKey.currentState!.save();
      setState(() => _state = OnboardingState.generating);

      try {
        final response = await http.post(
          Uri.parse('http://localhost:8080/api/onboarding/start'),
          headers: {'Content-Type': 'application/json'},
          body: jsonEncode({
            'business_intent': businessIntent,
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
          // If error occurs, go back to input.
           setState(() => _state = OnboardingState.input);
        }
      } catch (e) {
        print('Error: \$e');
         setState(() => _state = OnboardingState.input);
      }
    }
  }

  Future<void> launchStore() async {
    setState(() => _state = OnboardingState.launching); // Show loading state during launch

    try {
      final response = await http.post(
        Uri.parse('http://localhost:8080/api/onboarding/launch'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({'status': 'live'}),
      );

      if (response.statusCode == 200) {
        setState(() => _state = OnboardingState.live);
      } else {
        setState(() => _state = OnboardingState.draft);
      }
    } catch (e) {
      print('Error launching store: $e');
      setState(() => _state = OnboardingState.draft);
    }
  }

  @override
  Widget build(BuildContext context) {
    if (_state == OnboardingState.live) {
      return StoreLiveScreen(onDashboardPressed: () {
        setState(() => _state = OnboardingState.dashboard);
      });
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
      case OnboardingState.welcome:
        return _buildWelcomeState();
      case OnboardingState.input:
        return _buildInputState();
      case OnboardingState.generating:
        return _buildGeneratingState(isLaunching: false);
      case OnboardingState.launching:
        return _buildGeneratingState(isLaunching: true);
      case OnboardingState.dashboard:
        return _buildDashboardState();
      case OnboardingState.draft:
        return _buildDraftState();
      default:
        return SizedBox.shrink();
    }
  }

  Widget _buildWelcomeState() {
    return Padding(
      padding: EdgeInsets.all(24),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Icon(Icons.storefront, size: 80, color: Color(0xFF0066FF)),
          SizedBox(height: 32),
          Text(
            'OneHumanCorp',
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
            'The universal operating system for small business.',
            style: TextStyle(
              fontFamily: 'Inter',
              fontSize: 16,
              color: Colors.grey[600],
            ),
            textAlign: TextAlign.center,
          ),
          SizedBox(height: 48),
          ElevatedButton(
            onPressed: () {
              setState(() => _state = OnboardingState.input);
            },
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
              'Start a Business',
              style: TextStyle(
                fontFamily: 'Inter',
                fontSize: 16,
                fontWeight: FontWeight.w600,
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildInputState() {
    return Padding(
      padding: EdgeInsets.all(24),
      child: Form(
        key: _formKey,
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text(
              'What do you want to build today?',
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
              'Describe your business and AI will do the rest.',
              style: TextStyle(
                fontFamily: 'Inter',
                fontSize: 16,
                color: Colors.grey[600],
              ),
              textAlign: TextAlign.center,
            ),
            SizedBox(height: 32),
            TextFormField(
              key: Key('bio-input'),
              maxLines: 4,
              decoration: InputDecoration(
                labelText: 'Business Idea',
                hintText: 'e.g., A custom cake shop',
                filled: true,
                fillColor: Colors.white,
                border: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(16),
                  borderSide: BorderSide.none,
                ),
                contentPadding: EdgeInsets.all(20),
              ),
              style: TextStyle(fontFamily: 'Inter', fontSize: 16),
              validator: (value) => value == null || value.isEmpty ? 'Required' : null,
              onSaved: (value) => businessIntent = value!,
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

  Widget _buildGeneratingState({required bool isLaunching}) {
    return Container(
      width: double.infinity,
      height: double.infinity,
      decoration: BoxDecoration(
        color: Colors.white.withOpacity(0.1),
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(16),
        child: BackdropFilter(
          filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
          child: Container(
            color: Colors.white.withOpacity(0.2),
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
                  isLaunching ? 'Launching your business...' : 'Agents are working...',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 24,
                    fontWeight: FontWeight.bold,
                    color: Color(0xFF1D1D1F),
                  ),
                  textAlign: TextAlign.center,
                ),
                SizedBox(height: 16),
                Text(
                  isLaunching ? 'Provisioning infrastructure.' : 'Designing storefront and writing policies.',
                  style: TextStyle(
                    fontFamily: 'Inter',
                    fontSize: 14,
                    color: Colors.grey[700],
                  ),
                  textAlign: TextAlign.center,
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildDashboardState() {
    return Column(
      children: [
        // Top banner
        Container(
          width: double.infinity,
          color: Colors.white,
          padding: EdgeInsets.symmetric(vertical: 16, horizontal: 24),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              SizedBox(height: 32),
              Text(
                'Dashboard',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 32,
                  fontWeight: FontWeight.bold,
                  color: Color(0xFF1D1D1F),
                  letterSpacing: -0.5,
                ),
              ),
              SizedBox(height: 8),
              Text(
                'Welcome to your new business',
                style: TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 16,
                  color: Colors.grey[600],
                ),
              ),
            ],
          ),
        ),
        // Main Content Area
        Expanded(
          child: Container(
            color: Color(0xFFF5F5F7),
            width: double.infinity,
            padding: EdgeInsets.all(16),
            child: SingleChildScrollView(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  // Revenue Section
                  Container(
                    padding: EdgeInsets.all(24),
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
                      children: [
                        Text(
                          'Revenue',
                          style: TextStyle(color: Colors.grey[600], fontSize: 14),
                        ),
                        SizedBox(height: 8),
                        Text(
                          '\$0.00',
                          style: TextStyle(
                            fontFamily: 'Outfit',
                            fontSize: 40,
                            fontWeight: FontWeight.bold,
                            color: Color(0xFF1D1D1F),
                          ),
                        ),
                        Text(
                          'Today',
                          style: TextStyle(color: Colors.grey[500], fontSize: 12),
                        ),
                      ],
                    ),
                  ),
                  SizedBox(height: 16),
                  // Pending Agent Approvals
                  Container(
                    padding: EdgeInsets.all(24),
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
                            Icon(Icons.smart_toy, color: Color(0xFF0066FF), size: 20),
                            SizedBox(width: 8),
                            Text(
                              'Pending Agent Approvals',
                              style: TextStyle(
                                fontFamily: 'Outfit',
                                fontSize: 18,
                                fontWeight: FontWeight.bold,
                              ),
                            ),
                          ],
                        ),
                        SizedBox(height: 16),
                        Text(
                          'No pending approvals.',
                          style: TextStyle(color: Colors.grey[600], fontSize: 14),
                        ),
                      ],
                    ),
                  ),
                  SizedBox(height: 16),
                  // Recent Orders
                  Container(
                    padding: EdgeInsets.all(24),
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
                            Icon(Icons.receipt_long, color: Color(0xFF34C759), size: 20),
                            SizedBox(width: 8),
                            Text(
                              'Recent Orders',
                              style: TextStyle(
                                fontFamily: 'Outfit',
                                fontSize: 18,
                                fontWeight: FontWeight.bold,
                              ),
                            ),
                          ],
                        ),
                        SizedBox(height: 16),
                        Text(
                          'No orders yet.',
                          style: TextStyle(color: Colors.grey[600], fontSize: 14),
                        ),
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ],
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
            child: SingleChildScrollView(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  SizedBox(height: 48),
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
                  SizedBox(height: 32),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      SizedBox(
                        width: 44,
                        height: 44,
                        child: IconButton(
                          icon: Icon(Icons.edit, color: Color(0xFF0066FF)),
                          onPressed: () {},
                        ),
                      ),
                      SizedBox(width: 16),
                      SizedBox(
                        width: 44,
                        height: 44,
                        child: IconButton(
                          icon: Icon(Icons.tune, color: Color(0xFF0066FF)),
                          onPressed: () {},
                        ),
                      ),
                    ],
                  ),
                  SizedBox(height: 48),
                ],
              ),
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
                  'Launch Business',
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
  final VoidCallback onDashboardPressed;

  const StoreLiveScreen({Key? key, required this.onDashboardPressed}) : super(key: key);

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
                        onPressed: onDashboardPressed,
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
