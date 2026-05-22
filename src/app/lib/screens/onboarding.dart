import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'agent_dashboard.dart';

enum OnboardingState { welcome, step1, step2, generating, dashboard, live }

class OnboardingScreen extends StatefulWidget {
  final http.Client? httpClient;

  const OnboardingScreen({Key? key, this.httpClient}) : super(key: key);

  @override
  _OnboardingScreenState createState() => _OnboardingScreenState();
}

class _OnboardingScreenState extends State<OnboardingScreen> {
  final _formKey = GlobalKey<FormState>();
  String businessName = '';
  String niche = '';
  OnboardingState _state = OnboardingState.welcome;
  late final http.Client _client;

  @override
  void initState() {
    super.initState();
    _client = widget.httpClient ?? http.Client();
  }

  Future<void> handleNext() async {
    if (_formKey.currentState!.validate()) {
      _formKey.currentState!.save();
      setState(() => _state = OnboardingState.step2);
    }
  }

  Future<void> handleIntakeSubmit() async {
    if (_formKey.currentState!.validate()) {
      _formKey.currentState!.save();
      setState(() => _state = OnboardingState.generating);

      try {
        final baseUrl = const String.fromEnvironment('API_BASE_URL', defaultValue: 'http://localhost:18789');
        final response = await _client.post(
          Uri.parse('$baseUrl/api/onboarding/intake'),
          headers: {'Content-Type': 'application/json'},
          body: jsonEncode({
            'description': 'Business Name: $businessName\nCategory/Products: $niche',
          }),
        );

        if (response.statusCode == 200) {
          setState(() => _state = OnboardingState.dashboard);
        } else {
          setState(() => _state = OnboardingState.step2);
        }
      } catch (e) {
        print('Error: $e');
        setState(() => _state = OnboardingState.step2);
      }
    }
  }

  Future<void> handleStartOnboarding() async {
    setState(() => _state = OnboardingState.generating);
    try {
      final baseUrl = const String.fromEnvironment('API_BASE_URL', defaultValue: 'http://localhost:18789');
      final response = await _client.post(
        Uri.parse('$baseUrl/api/onboarding/start'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({
          'business_type': 'Retail',
          'company_name': businessName,
          'company_description': '',
          'selling_categories': [],
          'payment_pref': 'stripe',
          'admin_email': 'admin@example.com',
          'admin_name': 'Admin',
          'admin_password': 'password123',
          'website_template': 'modern',
          'first_product_name': 'Sample Product',
          'first_product_price': '10.00',
          'domain_choice': 'subdomain',
          'price_type': 'fixed',
        }),
      );
      if (response.statusCode == 200) {
        setState(() => _state = OnboardingState.live);
      } else {
        setState(() => _state = OnboardingState.dashboard);
      }
    } catch (e) {
      print('Error launching: $e');
      setState(() => _state = OnboardingState.dashboard);
    }
  }

  @override
  Widget build(BuildContext context) {
    if (_state == OnboardingState.live) {
      return StoreLiveScreen();
    }

    return Scaffold(
      backgroundColor: Color(0xFFF5F5F7), // Light background
      body: Center(
        child: ConstrainedBox(
          constraints: BoxConstraints(maxWidth: 500),
          child: Container(
            height: MediaQuery.of(
              context,
            ).size.height, // Takes up screen height gracefully
            padding: EdgeInsets.symmetric(vertical: 20),
            child: ClipRRect(
              borderRadius: BorderRadius.circular(16),
              child: BackdropFilter(
                filter: ImageFilter.blur(sigmaX: 30, sigmaY: 30),
                child: Container(
                  decoration: BoxDecoration(
                    color: Colors.white.withOpacity(0.65),
                    borderRadius: BorderRadius.circular(16),
                    border: Border.all(
                      color: Colors.white.withOpacity(0.4),
                      width: 1,
                    ),
                  ),
                  child: _buildContent(),
                ),
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
      case OnboardingState.step1:
        return _buildStep1State();
      case OnboardingState.step2:
        return _buildStep2State();
      case OnboardingState.generating:
        return _buildGeneratingState();
      case OnboardingState.dashboard:
        return _buildDashboardState();
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
              setState(() => _state = OnboardingState.step1);
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

  Widget _buildStep1State() {
    return Padding(
      padding: EdgeInsets.all(24),
      child: Form(
        key: _formKey,
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text(
              'What\'s the name of your business?',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 24,
                fontWeight: FontWeight.bold,
                color: Color(0xFF1D1D1F),
              ),
            ),
            SizedBox(height: 8),
            Text(
              'Don\'t worry, you can change this later.',
              style: TextStyle(
                fontFamily: 'Inter',
                fontSize: 14,
                color: Colors.grey[600],
              ),
            ),
            SizedBox(height: 24),
            TextFormField(
              key: Key('bio-input'),
              initialValue: businessName,
              decoration: InputDecoration(
                hintText: 'e.g. Maya\'s Cakes',
                filled: true,
                fillColor: Colors.white,
                border: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(12),
                  borderSide: BorderSide(color: Colors.grey[200]!),
                ),
                focusedBorder: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(12),
                  borderSide: BorderSide(color: Color(0xFF0066FF)),
                ),
                contentPadding: EdgeInsets.all(16),
              ),
              style: TextStyle(fontFamily: 'Inter', fontSize: 18),
              validator: (value) =>
                  value == null || value.isEmpty ? 'Required' : null,
              onSaved: (value) => businessName = value!,
            ),
            SizedBox(height: 16),
            ElevatedButton(
              onPressed: handleNext,
              style: ElevatedButton.styleFrom(
                backgroundColor: Color(0xFF0066FF),
                foregroundColor: Colors.white,
                padding: EdgeInsets.symmetric(vertical: 16),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(12),
                ),
                elevation: 0,
              ),
              child: Text(
                'Next',
                style: TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 16,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildStep2State() {
    return Padding(
      padding: EdgeInsets.all(24),
      child: Form(
        key: _formKey,
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text(
              'What\'s your niche?',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 24,
                fontWeight: FontWeight.bold,
                color: Color(0xFF1D1D1F),
              ),
            ),
            SizedBox(height: 8),
            Text(
              'Products, services, or bookings.',
              style: TextStyle(
                fontFamily: 'Inter',
                fontSize: 14,
                color: Colors.grey[600],
              ),
            ),
            SizedBox(height: 24),
            TextFormField(
              key: Key('niche-input'),
              initialValue: niche,
              decoration: InputDecoration(
                hintText: 'e.g. I bake custom wedding cakes',
                filled: true,
                fillColor: Colors.white,
                border: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(12),
                  borderSide: BorderSide(color: Colors.grey[200]!),
                ),
                focusedBorder: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(12),
                  borderSide: BorderSide(color: Color(0xFF0066FF)),
                ),
                contentPadding: EdgeInsets.all(16),
              ),
              style: TextStyle(fontFamily: 'Inter', fontSize: 18),
              validator: (value) =>
                  value == null || value.isEmpty ? 'Required' : null,
              onSaved: (value) => niche = value!,
            ),
            SizedBox(height: 16),
            Row(
              children: [
                Expanded(
                  flex: 1,
                  child: ElevatedButton(
                    onPressed: () => setState(() => _state = OnboardingState.step1),
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.grey[100],
                      foregroundColor: Colors.grey[600],
                      padding: EdgeInsets.symmetric(vertical: 16),
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(12),
                      ),
                      elevation: 0,
                    ),
                    child: Text(
                      'Back',
                      style: TextStyle(fontWeight: FontWeight.bold),
                    ),
                  ),
                ),
                SizedBox(width: 12),
                Expanded(
                  flex: 2,
                  child: ElevatedButton(
                    onPressed: handleIntakeSubmit,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Color(0xFF0066FF),
                      foregroundColor: Colors.white,
                      padding: EdgeInsets.symmetric(vertical: 16),
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(12),
                      ),
                      elevation: 0,
                    ),
                    child: Text(
                      'Generate Draft',
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
            'AI is building your storefront...',
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

  Widget _buildDashboardState() {
    return Padding(
      padding: EdgeInsets.all(24),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          Container(
            width: 64,
            height: 64,
            decoration: BoxDecoration(
              color: Color(0xFFEEF2FF),
              shape: BoxShape.circle,
            ),
            child: Center(
              child: Text(
                '✨',
                style: TextStyle(fontSize: 32),
              ),
            ),
          ),
          SizedBox(height: 24),
          Text(
            'Looks Great!',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 24,
              fontWeight: FontWeight.bold,
              color: Color(0xFF1D1D1F),
            ),
            textAlign: TextAlign.center,
          ),
          SizedBox(height: 8),
          Text(
            'Here is what our AI extracted. Ready to publish?',
            style: TextStyle(
              fontFamily: 'Inter',
              fontSize: 14,
              color: Colors.grey[500],
            ),
            textAlign: TextAlign.center,
          ),
          SizedBox(height: 32),
          Container(
            padding: EdgeInsets.all(20),
            decoration: BoxDecoration(
              color: Colors.white.withOpacity(0.8),
              borderRadius: BorderRadius.circular(12),
              border: Border.all(color: Colors.grey[100]!),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text('Business Name', style: TextStyle(fontSize: 12, color: Colors.grey[400], fontWeight: FontWeight.bold)),
                SizedBox(height: 4),
                Text(businessName, style: TextStyle(fontWeight: FontWeight.w500)),
                SizedBox(height: 12),
                Text('Type', style: TextStyle(fontSize: 12, color: Colors.grey[400], fontWeight: FontWeight.bold)),
                SizedBox(height: 4),
                Text('Retail', style: TextStyle(fontWeight: FontWeight.w500)),
              ],
            ),
          ),
          SizedBox(height: 32),
          Row(
            children: [
              Expanded(
                flex: 1,
                child: ElevatedButton(
                  onPressed: () => setState(() => _state = OnboardingState.step2),
                  style: ElevatedButton.styleFrom(
                    backgroundColor: Colors.grey[100],
                    foregroundColor: Colors.grey[600],
                    padding: EdgeInsets.symmetric(vertical: 16),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(12),
                    ),
                    elevation: 0,
                  ),
                  child: Text(
                    'Edit',
                    style: TextStyle(fontWeight: FontWeight.bold),
                  ),
                ),
              ),
              SizedBox(width: 12),
              Expanded(
                flex: 2,
                child: ElevatedButton(
                  onPressed: handleStartOnboarding,
                  style: ElevatedButton.styleFrom(
                    backgroundColor: Color(0xFF34C759),
                    foregroundColor: Colors.white,
                    padding: EdgeInsets.symmetric(vertical: 16),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(12),
                    ),
                    elevation: 0,
                  ),
                  child: Text(
                    'Publish Now',
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
        ],
      ),
    );
  }
}

class StoreLiveScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Color(0xFFF5F5F7),
      body: Center(
        child: ConstrainedBox(
          constraints: BoxConstraints(maxWidth: 500),
          child: Container(
            height: MediaQuery.of(context).size.height,
            padding: EdgeInsets.symmetric(vertical: 20, horizontal: 24),
            child: ClipRRect(
              borderRadius: BorderRadius.circular(16),
              child: BackdropFilter(
                filter: ImageFilter.blur(sigmaX: 30, sigmaY: 30),
                child: Container(
                  decoration: BoxDecoration(
                    color: Colors.white.withOpacity(0.65),
                    borderRadius: BorderRadius.circular(16),
                    border: Border.all(
                      color: Colors.white.withOpacity(0.4),
                      width: 1,
                    ),
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
                        child: Icon(
                          Icons.check_circle,
                          size: 64,
                          color: Color(0xFF34C759),
                        ),
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
                          onPressed: () {
                            Navigator.of(context).pushReplacement(
                              MaterialPageRoute(builder: (_) => AgentDashboard()),
                            );
                          },
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
      ),
    );
  }
}
