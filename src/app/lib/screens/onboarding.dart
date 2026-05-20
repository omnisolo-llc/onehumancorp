import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';

class OnboardingScreen extends StatefulWidget {
  @override
  _OnboardingScreenState createState() => _OnboardingScreenState();
}

class _OnboardingScreenState extends State<OnboardingScreen> {
  final PageController _pageController = PageController();
  final _formKey = GlobalKey<FormState>();
  String businessName = '';
  String businessCategory = 'Bakery';

  Future<void> submit() async {
    if (_formKey.currentState!.validate()) {
      _formKey.currentState!.save();

      // Move to Magic screen
      _pageController.nextPage(
        duration: Duration(milliseconds: 300),
        curve: Curves.easeInOut,
      );

      try {
        final response = await http.post(
          Uri.parse('http://localhost:8080/api/onboarding/start'),
          headers: {'Content-Type': 'application/json'},
          body: jsonEncode({
            'company_name': businessName,
            'business_type': businessCategory,
            // Only name and category are sent, the rest is deferred as per design doc
          }),
        );

        if (response.statusCode == 200) {
          // Move to Reveal screen
          _pageController.nextPage(
            duration: Duration(milliseconds: 300),
            curve: Curves.easeInOut,
          );
        } else {
          // In a real app, handle non-200, e.g. show snackbar
          // For now just jump back to input if it fails so they aren't stuck
          _pageController.previousPage(
            duration: Duration(milliseconds: 300),
            curve: Curves.easeInOut,
          );
        }
      } catch (e) {
        print('Error: \$e');
        _pageController.previousPage(
          duration: Duration(milliseconds: 300),
          curve: Curves.easeInOut,
        );
      }
    }
  }

  Widget _buildGlassmorphismContainer({required Widget child}) {
    return Container(
      decoration: BoxDecoration(
        color: Colors.white.withValues(alpha: 0.1),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: Colors.white.withValues(alpha: 0.2)),
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(16),
        child: BackdropFilter(
          filter: ImageFilter.blur(sigmaX: 10, sigmaY: 10),
          child: Padding(
            padding: const EdgeInsets.all(24.0),
            child: child,
          ),
        ),
      ),
    );
  }

  Widget _buildAcquisitionScreen() {
    return Center(
      child: Container(
        width: 375,
        padding: EdgeInsets.all(24),
        child: Form(
          key: _formKey,
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Text(
                'What do you do?',
                style: TextStyle(fontSize: 32, fontWeight: FontWeight.bold),
                textAlign: TextAlign.center,
              ),
              SizedBox(height: 32),
              TextFormField(
                autofocus: true,
                decoration: InputDecoration(
                  labelText: 'Business Name',
                  hintText: 'e.g. Maya\'s Cakes',
                  border: OutlineInputBorder(),
                ),
                validator: (value) => value == null || value.isEmpty ? 'Required' : null,
                onSaved: (value) => businessName = value!,
              ),
              SizedBox(height: 16),
              DropdownButtonFormField<String>(
                decoration: InputDecoration(
                  labelText: 'Category',
                  border: OutlineInputBorder(),
                ),
                initialValue: businessCategory,
                items: ['Bakery', 'Handyman', 'Boutique', 'Tutor', 'Food Cart']
                    .map((c) => DropdownMenuItem(value: c, child: Text(c)))
                    .toList(),
                onChanged: (value) => setState(() => businessCategory = value!),
                onSaved: (value) => businessCategory = value!,
              ),
              SizedBox(height: 32),
              ElevatedButton(
                onPressed: submit,
                style: ElevatedButton.styleFrom(
                  minimumSize: Size(double.infinity, 50),
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                ),
                child: Text('Next', style: TextStyle(fontSize: 18)),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildMagicScreen() {
    return Center(
      child: Container(
        width: 375,
        padding: EdgeInsets.all(24),
        child: _buildGlassmorphismContainer(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              CircularProgressIndicator(),
              SizedBox(height: 24),
              Text(
                'The Promoter is building your business...',
                style: TextStyle(fontSize: 20, fontWeight: FontWeight.w600),
                textAlign: TextAlign.center,
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildRevealScreen() {
    return Center(
      child: Container(
        width: 375,
        padding: EdgeInsets.all(24),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            _buildGlassmorphismContainer(
              child: Column(
                children: [
                  Icon(Icons.store, size: 64, color: Colors.blue),
                  SizedBox(height: 16),
                  Text(
                    businessName.isNotEmpty ? businessName : 'Your Store',
                    style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
                  ),
                  Text(
                    businessCategory,
                    style: TextStyle(fontSize: 16, color: Colors.grey[600]),
                  ),
                ],
              ),
            ),
            SizedBox(height: 32),
            Text(
              'Looks great. Let\'s get paid.',
              style: TextStyle(fontSize: 20, fontWeight: FontWeight.w500),
              textAlign: TextAlign.center,
            ),
            SizedBox(height: 16),
            ElevatedButton(
              onPressed: () {
                // In a real app this would trigger the actual stripe integration
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(content: Text('Redirecting to Stripe...')),
                );
              },
              style: ElevatedButton.styleFrom(
                minimumSize: Size(double.infinity, 50),
                backgroundColor: Colors.black,
                foregroundColor: Colors.white,
                shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
              ),
              child: Text('Connect Bank/Stripe', style: TextStyle(fontSize: 16)),
            ),
          ],
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      // Gradient background to make glassmorphism pop
      body: Container(
        decoration: BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Colors.blue[50]!, Colors.purple[50]!],
          ),
        ),
        child: SafeArea(
          child: PageView(
            controller: _pageController,
            physics: NeverScrollableScrollPhysics(), // User shouldn't swipe back/forth manually during this flow
            children: [
              _buildAcquisitionScreen(),
              _buildMagicScreen(),
              _buildRevealScreen(),
            ],
          ),
        ),
      ),
    );
  }
}
