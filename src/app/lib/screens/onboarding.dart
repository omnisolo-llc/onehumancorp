import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'dart:async';
import 'package:flutter/foundation.dart';

class OnboardingScreen extends StatefulWidget {
  @override
  _OnboardingScreenState createState() => _OnboardingScreenState();
}

class _OnboardingScreenState extends State<OnboardingScreen> {
  final _formKey = GlobalKey<FormState>();
  final _nameController = TextEditingController();
  String businessName = '';
  String businessCategory = 'Bakery';
  bool isGenerating = false;

  // Read API URL from environment variable, falling back to localhost
  String get _apiUrl {
    if (kIsWeb) {
      return 'http://localhost:8080';
    }
    // Android emulator alias for localhost
    return 'http://10.0.2.2:8080';
  }

  @override
  void initState() {
    super.initState();
    _loadState();
  }

  @override
  void dispose() {
    _nameController.dispose();
    super.dispose();
  }

  Future<void> _loadState() async {
    try {
      final response = await http.get(
        Uri.parse('\$_apiUrl/api/onboarding/state'),
        headers: {'X-Tenant-ID': 'default_tenant', 'X-User-ID': 'default_user'},
      );
      if (response.statusCode == 200) {
        final data = jsonDecode(response.body);
        if (data != null && mounted) {
          setState(() {
            businessName = data['company_name'] ?? '';
            final category = data['business_type'];
            if (category != null && ['Bakery', 'Handyman', 'Boutique', 'Tutor', 'Food Cart'].contains(category)) {
              businessCategory = category;
            }
            _nameController.text = businessName;
          });
        }
      }
    } catch (e) {
      print('Failed to load state: \$e');
    }
  }

  Future<void> _saveState() async {
    try {
      await http.post(
        Uri.parse('\$_apiUrl/api/onboarding/state'),
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': 'default_tenant',
          'X-User-ID': 'default_user'
        },
        body: jsonEncode({
          'company_name': businessName,
          'business_type': businessCategory,
          'step': 1,
        }),
      );
    } catch (e) {
      print('Failed to save state: \$e');
    }
  }

  Future<void> submit() async {
    if (_formKey.currentState!.validate()) {
      _formKey.currentState!.save();
      setState(() => isGenerating = true);

      // Auto-save before submitting
      await _saveState();

      try {
        final response = await http.post(
          Uri.parse('\$_apiUrl/api/onboarding/start'),
          headers: {'Content-Type': 'application/json'},
          body: jsonEncode({
            'company_name': businessName,
            'business_type': businessCategory,
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
          Navigator.pushReplacement(context, MaterialPageRoute(builder: (context) => StoreLiveScreen()));
        }
      } catch (e) {
        print('Error: \$e');
      } finally {
        if (mounted) setState(() => isGenerating = false);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      extendBodyBehindAppBar: true,
      appBar: AppBar(
        title: Text('Start Your Business', style: TextStyle(color: Colors.white)),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      body: Container(
        decoration: BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Colors.blue.shade900, Colors.purple.shade900],
          ),
        ),
        child: Center(
          child: ConstrainedBox(
            constraints: BoxConstraints(maxWidth: 375),
            child: Padding(
              padding: EdgeInsets.all(24),
              child: ClipRRect(
                borderRadius: BorderRadius.circular(20),
                child: BackdropFilter(
                  filter: ImageFilter.blur(sigmaX: 10, sigmaY: 10),
                  child: Container(
                    padding: EdgeInsets.all(24),
                    decoration: BoxDecoration(
                      color: Colors.white.withValues(alpha: 0.1),
                      borderRadius: BorderRadius.circular(20),
                      border: Border.all(color: Colors.white.withValues(alpha: 0.2)),
                    ),
                    child: Form(
                      key: _formKey,
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          Text(
                            'Build your business in minutes',
                            style: TextStyle(
                              fontSize: 24,
                              fontWeight: FontWeight.bold,
                              color: Colors.white,
                            ),
                            textAlign: TextAlign.center,
                          ),
                          SizedBox(height: 32),
                          TextFormField(
                            controller: _nameController,
                            style: TextStyle(color: Colors.white),
                            textInputAction: TextInputAction.next,
                            decoration: InputDecoration(
                              labelText: 'Business Name',
                              labelStyle: TextStyle(color: Colors.white70),
                              filled: true,
                              fillColor: Colors.white.withValues(alpha: 0.1),
                              border: OutlineInputBorder(
                                borderRadius: BorderRadius.circular(12),
                                borderSide: BorderSide.none,
                              ),
                            ),
                            validator: (value) => value == null || value.isEmpty ? 'Required' : null,
                            onSaved: (value) => businessName = value ?? '',
                          ),
                          SizedBox(height: 16),
                          DropdownButtonFormField<String>(
                            dropdownColor: Colors.purple.shade900,
                            style: TextStyle(color: Colors.white),
                            decoration: InputDecoration(
                              labelText: 'Category',
                              labelStyle: TextStyle(color: Colors.white70),
                              filled: true,
                              fillColor: Colors.white.withValues(alpha: 0.1),
                              border: OutlineInputBorder(
                                borderRadius: BorderRadius.circular(12),
                                borderSide: BorderSide.none,
                              ),
                            ),
                            value: businessCategory,
                            items: ['Bakery', 'Handyman', 'Boutique', 'Tutor', 'Food Cart']
                                .map((c) => DropdownMenuItem(value: c, child: Text(c)))
                                .toList(),
                            onChanged: (value) {
                              if (value != null) {
                                setState(() => businessCategory = value);
                              }
                            },
                            onSaved: (value) => businessCategory = value ?? 'Bakery',
                            validator: (value) => value == null ? 'Required' : null,
                          ),
                          SizedBox(height: 32),
                          SizedBox(
                            width: double.infinity,
                            height: 50,
                            child: ElevatedButton(
                              style: ElevatedButton.styleFrom(
                                backgroundColor: Colors.white.withValues(alpha: 0.2),
                                foregroundColor: Colors.white,
                                shape: RoundedRectangleBorder(
                                  borderRadius: BorderRadius.circular(12),
                                ),
                              ),
                              onPressed: isGenerating ? null : submit,
                              child: isGenerating
                                  ? CircularProgressIndicator(color: Colors.white)
                                  : Text('Start Setup', style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
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
        ),
      ),
    );
  }
}

class StoreLiveScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        decoration: BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Colors.blue.shade900, Colors.purple.shade900],
          ),
        ),
        child: Center(
          child: Container(
            width: 375,
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Icon(Icons.check_circle, size: 80, color: Colors.greenAccent),
                SizedBox(height: 24),
                Text(
                  'Store Live',
                  style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white),
                  textAlign: TextAlign.center,
                ),
                Text('Success! Your business is live!', style: TextStyle(fontSize: 16, color: Colors.white70)),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
