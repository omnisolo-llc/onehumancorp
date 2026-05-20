import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';

class OnboardingScreen extends StatefulWidget {
  @override
  _OnboardingScreenState createState() => _OnboardingScreenState();
}

class _OnboardingScreenState extends State<OnboardingScreen> {
  final _formKey = GlobalKey<FormState>();
  String businessName = '';
  String businessCategory = 'Bakery';
  bool isGenerating = false;

  Future<void> submit() async {
    if (_formKey.currentState!.validate()) {
      _formKey.currentState!.save();
      setState(() => isGenerating = true);

      final startTime = DateTime.now();

      try {
        final response = await http.post(
          Uri.parse('http://localhost:8080/api/onboarding/start'),
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
          // Navigate to Success / Store Live
          Navigator.pushReplacement(context, MaterialPageRoute(builder: (context) => StoreLiveScreen()));
        }
      } catch (e) {
        print('Error: \$e');
      } finally {
        setState(() => isGenerating = false);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Start Your Business')),
      body: Center(
        child: Container(
          width: 375, // Mobile viewport constraint
          padding: EdgeInsets.all(24),
          child: Form(
            key: _formKey,
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Text(
                  'Build your bakery in 3 minutes',
                  style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
                  textAlign: TextAlign.center,
                ),
                SizedBox(height: 32),
                TextFormField(
                  decoration: InputDecoration(
                    labelText: 'Business Name',
                    border: OutlineInputBorder(),
                  ),
                  validator: (value) => value!.isEmpty ? 'Required' : null,
                  onSaved: (value) => businessName = value!,
                ),
                SizedBox(height: 16),
                DropdownButtonFormField<String>(
                  decoration: InputDecoration(
                    labelText: 'Category',
                    border: OutlineInputBorder(),
                  ),
                  value: businessCategory,
                  items: ['Bakery', 'Handyman', 'Boutique', 'Tutor', 'Food Cart']
                      .map((c) => DropdownMenuItem(value: c, child: Text(c)))
                      .toList(),
                  onChanged: (value) => setState(() => businessCategory = value!),
                  onSaved: (value) => businessCategory = value!,
                  validator: (value) => value == null ? 'Required' : null,
                ),
                SizedBox(height: 32),
                SizedBox(
                  width: double.infinity,
                  height: 50,
                  child: ElevatedButton(
                    onPressed: isGenerating ? null : submit,
                    child: isGenerating
                        ? CircularProgressIndicator(color: Colors.white)
                        : Text('Start Setup'),
                  ),
                ),
              ],
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
      body: Center(
        child: Container(
          width: 375,
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(Icons.check_circle, size: 80, color: Colors.green),
              SizedBox(height: 24),
              Text(
                'Store Live',
                style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
                textAlign: TextAlign.center,
              ),
              Text('Success! Your business is live!', style: TextStyle(fontSize: 16)),
            ],
          ),
        ),
      ),
    );
  }
}
