import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'dart:async';
import 'agent_dashboard.dart';
import '../widgets/walkthrough_overlay.dart';

enum OnboardingState { welcome, input, generating, dashboard, draft, live }

class OnboardingScreen extends StatefulWidget {
  final http.Client? httpClient;

  const OnboardingScreen({Key? key, this.httpClient}) : super(key: key);

  @override
  _OnboardingScreenState createState() => _OnboardingScreenState();
}

class _OnboardingScreenState extends State<OnboardingScreen> {
  final _formKey = GlobalKey<FormState>();
  String bio = '';
  int _currentInputStep = 0;
  String businessName = '';
  String selectedTemplate = 'Modern';
  OnboardingState _state = OnboardingState.welcome;
  bool isAdvancedMode = false;
  String domainChoice = 'subdomain';
  late final http.Client _client;
  late final TextEditingController _bioController;
  Timer? _debounce;

  @override
  void initState() {
    super.initState();
    _client = widget.httpClient ?? http.Client();
    _bioController = TextEditingController();
    _loadBio();
  }

  @override
  void dispose() {
    _debounce?.cancel();
    _bioController.dispose();
    super.dispose();
  }

  Future<void> _loadBio() async {
    try {
      final baseUrl = const String.fromEnvironment('API_BASE_URL', defaultValue: 'http://localhost:8080');
      final response = await _client.get(Uri.parse('$baseUrl/api/onboarding/draft'));
      if (response.statusCode == 200) {
        final data = jsonDecode(response.body);
        if (data['bio'] != null) {
          setState(() {
            bio = data['bio'];
            _bioController.text = bio;
          });
        }
        if (data['businessName'] != null) {
          setState(() => businessName = data['businessName']);
        }
        if (data['selectedTemplate'] != null) {
          setState(() => selectedTemplate = data['selectedTemplate']);
        }
      }
    } catch (e) {
      print('Failed to load draft: $e');
    }
  }

  Future<void> _saveDraft(String text) async {
    try {
      final baseUrl = const String.fromEnvironment('API_BASE_URL', defaultValue: 'http://localhost:8080');
      await _client.post(
        Uri.parse('$baseUrl/api/onboarding/draft'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({'bio': text, 'businessName': businessName, 'selectedTemplate': selectedTemplate}),
      );
    } catch (e) {
      print('Failed to save draft: $e');
    }
  }

  Future<void> submit() async {
    if (_formKey.currentState!.validate()) {
      _formKey.currentState!.save();
      await _saveDraft(bio);
      setState(() => _state = OnboardingState.generating);

      try {
        final baseUrl = const String.fromEnvironment('API_BASE_URL', defaultValue: 'http://localhost:8080');
        final response = await _client.post(
          Uri.parse('$baseUrl/api/onboarding/start'),
          headers: {'Content-Type': 'application/json'},
          body: jsonEncode({
            'bio': bio,
            'company_name': businessName.isEmpty ? 'AI Generated Store' : businessName,
            'business_type': 'Auto',
            'selling_categories': ['food', 'physical'],
            'payment_pref': 'online',
            'admin_email': 'admin@test.com',
            'admin_name': 'Admin User',
            'admin_password': 'password123',
            'website_template': selectedTemplate,
            'first_product_name': 'Custom Cake Deposit',
            'first_product_price': '25.00',
            'domain_choice': domainChoice,
            'price_type': 'fixed',
          }),
        );

        if (response.statusCode == 200) {
          if (mounted) setState(() => _state = OnboardingState.dashboard);
        } else {
          // If error occurs, go back to input.
          if (mounted) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(content: Text('Network error. Please try again.')),
            );
            setState(() => _state = OnboardingState.input);
          }
        }
      } catch (e) {
        print('Error: \$e');
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text('Network error. Please try again.')),
          );
          setState(() => _state = OnboardingState.input);
        }
      }
    }
  }

  Future<void> launchStore() async {
    try {
      final baseUrl = const String.fromEnvironment('API_BASE_URL', defaultValue: 'http://localhost:8080');
      final response = await _client.post(
        Uri.parse('$baseUrl/api/onboarding/launch'),
      );
      if (response.statusCode == 200) {
        setState(() => _state = OnboardingState.live);
      }
    } catch (e) {
      print('Error launching: \$e');
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
      case OnboardingState.input:
        return _buildInputState();
      case OnboardingState.generating:
        return _buildGeneratingState();
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
        child: LayoutBuilder(
        builder: (context, constraints) {
          return SingleChildScrollView(
            child: ConstrainedBox(
              constraints: BoxConstraints(minHeight: constraints.maxHeight),
              child: IntrinsicHeight(
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    Text(
                      'Welcome to OHC Smart Builder',
                      style: TextStyle(fontFamily: 'Outfit', fontSize: 32, fontWeight: FontWeight.bold, color: Color(0xFF1D1D1F), letterSpacing: -0.5),
                      textAlign: TextAlign.center,
                    ),
                    SizedBox(height: 16),
                    Text(
                      'Tell us about your business, and AI will build it.',
                      style: TextStyle(fontFamily: 'Inter', fontSize: 16, color: Colors.grey[600]),
                      textAlign: TextAlign.center,
                    ),
                    SizedBox(height: 32),
                    ClipRRect(
                      borderRadius: BorderRadius.circular(16),
                      child: BackdropFilter(
                        filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
                        child: TextFormField(
                          key: Key('bio-input'),
                          controller: _bioController,
                          autofocus: true,
                          textInputAction: TextInputAction.done,
                          textCapitalization: TextCapitalization.sentences,
                          keyboardType: TextInputType.multiline,
                          maxLines: 4,
                          decoration: InputDecoration(
                            labelText: 'Business Bio',
                            hintText: 'e.g., I bake custom vegan cakes in Seattle. Maya\'s Cakes.',
                            filled: true,
                            fillColor: Colors.white.withOpacity(0.5),
                            border: OutlineInputBorder(borderRadius: BorderRadius.circular(16), borderSide: BorderSide.none),
                            contentPadding: EdgeInsets.all(20),
                          ),
                          style: TextStyle(fontFamily: 'Inter', fontSize: 16),
                          onChanged: (value) {
                            bio = value;
                            if (_debounce?.isActive ?? false) _debounce!.cancel();
                            _debounce = Timer(const Duration(milliseconds: 500), () {
                              _saveDraft(value);
                            });
                          },
                          validator: (value) => value == null || value.isEmpty ? 'Required' : null,
                          onSaved: (value) => bio = value!,
                        ),
                      ),
                    ),
                    SizedBox(height: 32),
                    ElevatedButton(
                      onPressed: () {
                        if (_formKey.currentState!.validate()) {
                          _formKey.currentState!.save();
                          submit();
                        }
                      },
                      style: ElevatedButton.styleFrom(
                        backgroundColor: Color(0xFF0066FF),
                        foregroundColor: Colors.white,
                        padding: EdgeInsets.symmetric(vertical: 18),
                        minimumSize: Size(double.infinity, 50),
                        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
                        elevation: 0,
                      ),
                      child: Text('Build My Storefront', style: TextStyle(fontFamily: 'Inter', fontSize: 16, fontWeight: FontWeight.w600)),
                    ),
                  ],
                ),
              ),
            ),
          );
        },
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
                'Welcome, your store is ready',
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
            padding: EdgeInsets.all(24),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
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
                      Icon(
                        Icons.check_circle,
                        size: 48,
                        color: Color(0xFF34C759),
                      ),
                      SizedBox(height: 16),
                      Text(
                        'Storefront Generated!',
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      SizedBox(height: 8),
                      Text(
                        'Your AI agent has created a draft based on your business profile.',
                        textAlign: TextAlign.center,
                        style: TextStyle(color: Colors.grey[600], fontSize: 14),
                      ),
                      SizedBox(height: 24),
                      ElevatedButton(
                        onPressed: () {
                          setState(() => _state = OnboardingState.draft);
                        },
                        style: ElevatedButton.styleFrom(
                          backgroundColor: Color(0xFF0066FF),
                          foregroundColor: Colors.white,
                          padding: EdgeInsets.symmetric(vertical: 16),
                          minimumSize: Size(double.infinity, 50),
                          shape: RoundedRectangleBorder(
                            borderRadius: BorderRadius.circular(12),
                          ),
                          elevation: 0,
                        ),
                        child: Text(
                          'Preview Site',
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
              ],
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
                style: TextStyle(
                  color: Colors.white,
                  fontSize: 12,
                  fontWeight: FontWeight.bold,
                ),
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
                SizedBox(height: 24),
                ConstrainedBox(
                  constraints: BoxConstraints(minWidth: 44, minHeight: 44),
                  child: IconButton(
                    icon: Icon(Icons.edit, color: Colors.blue),
                    onPressed: () {},
                    tooltip: 'Edit Preview',
                  ),
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
