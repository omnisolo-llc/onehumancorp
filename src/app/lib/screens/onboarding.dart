import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'dart:async';
import 'package:shared_preferences/shared_preferences.dart';
import 'agent_dashboard.dart';

enum OnboardingState { welcome, step1, step2, step3, step4, generating, live }

class OnboardingScreen extends StatefulWidget {
  final http.Client? httpClient;

  const OnboardingScreen({Key? key, this.httpClient}) : super(key: key);

  @override
  _OnboardingScreenState createState() => _OnboardingScreenState();
}

class _OnboardingScreenState extends State<OnboardingScreen> {
  final _formKey = GlobalKey<FormState>();
  String businessName = '';
  String businessCategory = '';
  String visualStyle = '';
  String itemName = '';
  String itemPrice = '';
  bool paymentConnected = false;

  OnboardingState _state = OnboardingState.welcome;
  late final http.Client _client;
  late final TextEditingController _businessNameController;
  late final TextEditingController _businessCategoryController;
  late final TextEditingController _itemNameController;
  late final TextEditingController _itemPriceController;
  Timer? _debounce;

  @override
  void initState() {
    super.initState();
    _client = widget.httpClient ?? http.Client();
    _businessNameController = TextEditingController();
    _businessCategoryController = TextEditingController();
    _itemNameController = TextEditingController();
    _itemPriceController = TextEditingController();
    _loadProgress();
  }

  @override
  void dispose() {
    _debounce?.cancel();
    _businessNameController.dispose();
    _businessCategoryController.dispose();
    _itemNameController.dispose();
    _itemPriceController.dispose();
    super.dispose();
  }

  Future<void> _loadProgress() async {
    final prefs = await SharedPreferences.getInstance();
    setState(() {
      businessName = prefs.getString('businessName') ?? '';
      businessCategory = prefs.getString('businessCategory') ?? '';
      visualStyle = prefs.getString('visualStyle') ?? '';
      itemName = prefs.getString('itemName') ?? '';
      itemPrice = prefs.getString('itemPrice') ?? '';
      paymentConnected = prefs.getBool('paymentConnected') ?? false;

      _businessNameController.text = businessName;
      _businessCategoryController.text = businessCategory;
      _itemNameController.text = itemName;
      _itemPriceController.text = itemPrice;

      final stateIndex = prefs.getInt('onboardingState');
      if (stateIndex != null && stateIndex >= 0 && stateIndex < OnboardingState.values.length) {
        _state = OnboardingState.values[stateIndex];
      }
    });
  }

  Future<void> _saveProgress() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString('businessName', businessName);
    await prefs.setString('businessCategory', businessCategory);
    await prefs.setString('visualStyle', visualStyle);
    await prefs.setString('itemName', itemName);
    await prefs.setString('itemPrice', itemPrice);
    await prefs.setBool('paymentConnected', paymentConnected);
    await prefs.setInt('onboardingState', _state.index);
  }

  Future<void> submit() async {
    setState(() => _state = OnboardingState.generating);

    try {
      final baseUrl = const String.fromEnvironment('API_BASE_URL', defaultValue: 'http://localhost:8080');
      final response = await _client.post(
        Uri.parse('$baseUrl/api/onboarding/start'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({
          'company_name': businessName,
          'business_type': businessCategory,
          'website_template': visualStyle,
          'first_product_name': itemName,
          'first_product_price': itemPrice,
          'payment_connected': paymentConnected,
          'admin_email': 'admin@test.com',
          'admin_name': 'Admin User',
          'admin_password': 'password123',
          'domain_choice': 'subdomain',
          'price_type': 'fixed',
        }),
      );

      if (response.statusCode == 200) {
        if (mounted) {
          setState(() => _state = OnboardingState.live);
          _saveProgress();
        }
      } else {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text('Network error. Please try again.')),
          );
          setState(() => _state = OnboardingState.step4);
        }
      }
    } catch (e) {
      print('Error: \$e');
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Network error. Please try again.')),
        );
        setState(() => _state = OnboardingState.step4);
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
      return StoreLiveScreen(businessName: businessName);
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
      case OnboardingState.step3:
        return _buildStep3State();
      case OnboardingState.step4:
        return _buildStep4State();
      case OnboardingState.generating:
        return _buildGeneratingState();
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
              setState(() {
                _state = OnboardingState.step1;
                _saveProgress();
              });
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
              'Step 1 of 4',
              style: TextStyle(
                fontFamily: 'Inter',
                fontSize: 14,
                fontWeight: FontWeight.w600,
                color: Color(0xFF0066FF),
              ),
            ),
            SizedBox(height: 8),
            Text(
              'Let\'s get to know your business',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 32,
                fontWeight: FontWeight.bold,
                color: Color(0xFF1D1D1F),
                letterSpacing: -0.5,
              ),
            ),
            SizedBox(height: 32),
            _buildTextField(
              controller: _businessNameController,
              label: 'Business Name',
              hint: 'e.g., Maya\'s Cakes',
              keyStr: 'business-name-input',
              onChanged: (val) {
                businessName = val;
                _saveProgress();
              },
            ),
            SizedBox(height: 16),
            _buildTextField(
              controller: _businessCategoryController,
              label: 'Business Category',
              hint: 'e.g., Bakery, Handyman',
              keyStr: 'business-category-input',
              onChanged: (val) {
                businessCategory = val;
                _saveProgress();
              },
            ),
            SizedBox(height: 48),
            ElevatedButton(
              onPressed: () {
                if (_formKey.currentState!.validate()) {
                  setState(() {
                    _state = OnboardingState.step2;
                    _saveProgress();
                  });
                }
              },
              style: ElevatedButton.styleFrom(
                backgroundColor: Color(0xFF0066FF),
                foregroundColor: Colors.white,
                padding: EdgeInsets.symmetric(vertical: 18),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(16),
                ),
                elevation: 0,
              ),
              child: Text(
                'Next',
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

  Widget _buildTextField({
    required TextEditingController controller,
    required String label,
    required String hint,
    required String keyStr,
    required Function(String) onChanged,
    TextInputType keyboardType = TextInputType.text,
  }) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
        child: TextFormField(
          key: Key(keyStr),
          controller: controller,
          textInputAction: TextInputAction.next,
          textCapitalization: TextCapitalization.words,
          keyboardType: keyboardType,
          decoration: InputDecoration(
            labelText: label,
            hintText: hint,
            filled: true,
            fillColor: Colors.white.withOpacity(0.5),
            border: OutlineInputBorder(
              borderRadius: BorderRadius.circular(16),
              borderSide: BorderSide.none,
            ),
            contentPadding: EdgeInsets.all(20),
          ),
          style: TextStyle(fontFamily: 'Inter', fontSize: 16),
          onChanged: onChanged,
          validator: (value) => value == null || value.isEmpty ? 'Required' : null,
        ),
      ),
    );
  }

  Widget _buildStep2State() {
    return SingleChildScrollView(
      child: Padding(
        padding: EdgeInsets.all(24),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text(
              'Step 2 of 4',
              style: TextStyle(
                fontFamily: 'Inter',
                fontSize: 14,
                fontWeight: FontWeight.w600,
                color: Color(0xFF0066FF),
              ),
            ),
            SizedBox(height: 8),
            Text(
              'Pick a style',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 32,
                fontWeight: FontWeight.bold,
                color: Color(0xFF1D1D1F),
                letterSpacing: -0.5,
              ),
            ),
            SizedBox(height: 32),
            Column(
              children: [
                _buildStyleCard('Elegant', Icons.auto_awesome),
                SizedBox(height: 16),
                _buildStyleCard('Playful', Icons.color_lens),
                SizedBox(height: 16),
                _buildStyleCard('Professional', Icons.business_center),
              ],
            ),
            SizedBox(height: 16),
            ElevatedButton(
              onPressed: () {
                if (visualStyle.isNotEmpty) {
                  setState(() {
                    _state = OnboardingState.step3;
                    _saveProgress();
                  });
                } else {
                  ScaffoldMessenger.of(context).showSnackBar(
                    SnackBar(content: Text('Please select a visual style')),
                  );
                }
              },
              style: ElevatedButton.styleFrom(
                backgroundColor: Color(0xFF0066FF),
                foregroundColor: Colors.white,
                padding: EdgeInsets.symmetric(vertical: 18),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(16),
                ),
                elevation: 0,
              ),
              child: Text(
                'Next',
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

  Widget _buildStyleCard(String styleName, IconData icon) {
    bool isSelected = visualStyle == styleName;
    return GestureDetector(
      onTap: () {
        setState(() {
          visualStyle = styleName;
          _saveProgress();
        });
      },
      child: Container(
        padding: EdgeInsets.all(20),
        decoration: BoxDecoration(
          color: isSelected ? Color(0xFF0066FF).withOpacity(0.1) : Colors.white,
          borderRadius: BorderRadius.circular(16),
          border: Border.all(
            color: isSelected ? Color(0xFF0066FF) : Colors.grey.withOpacity(0.2),
            width: 2,
          ),
        ),
        child: Row(
          children: [
            Icon(icon, size: 32, color: isSelected ? Color(0xFF0066FF) : Colors.grey[600]),
            SizedBox(width: 16),
            Text(
              styleName,
              style: TextStyle(
                fontFamily: 'Inter',
                fontSize: 18,
                fontWeight: FontWeight.w600,
                color: isSelected ? Color(0xFF0066FF) : Color(0xFF1D1D1F),
              ),
            ),
            Spacer(),
            if (isSelected) Icon(Icons.check_circle, color: Color(0xFF0066FF)),
          ],
        ),
      ),
    );
  }

  Widget _buildStep3State() {
    return SingleChildScrollView(
      child: Padding(
        padding: EdgeInsets.all(24),
        child: Form(
          key: _formKey,
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Text(
                'Step 3 of 4',
                style: TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 14,
                  fontWeight: FontWeight.w600,
                  color: Color(0xFF0066FF),
                ),
              ),
              SizedBox(height: 8),
              Text(
                'Add your first item',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 32,
                  fontWeight: FontWeight.bold,
                  color: Color(0xFF1D1D1F),
                  letterSpacing: -0.5,
                ),
              ),
              SizedBox(height: 32),
              Center(
                child: Container(
                  width: 120,
                  height: 120,
                  decoration: BoxDecoration(
                    color: Colors.grey[200],
                    borderRadius: BorderRadius.circular(16),
                    border: Border.all(color: Colors.grey[300]!, width: 2),
                  ),
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(Icons.add_a_photo, color: Colors.grey[500], size: 32),
                      SizedBox(height: 8),
                      Text(
                        'Add Photo',
                        style: TextStyle(
                          fontFamily: 'Inter',
                          fontSize: 14,
                          color: Colors.grey[600],
                        ),
                      ),
                    ],
                  ),
                ),
              ),
              SizedBox(height: 32),
              _buildTextField(
                controller: _itemNameController,
                label: 'Item Name',
                hint: 'e.g., Custom Birthday Cake',
                keyStr: 'item-name-input',
                onChanged: (val) {
                  itemName = val;
                  _saveProgress();
                },
              ),
              SizedBox(height: 16),
              _buildTextField(
                controller: _itemPriceController,
                label: 'Price',
                hint: 'e.g., 25.00',
                keyStr: 'item-price-input',
                keyboardType: TextInputType.numberWithOptions(decimal: true),
                onChanged: (val) {
                  itemPrice = val;
                  _saveProgress();
                },
              ),
              SizedBox(height: 48),
              ElevatedButton(
                onPressed: () {
                  if (_formKey.currentState!.validate()) {
                    setState(() {
                      _state = OnboardingState.step4;
                      _saveProgress();
                    });
                  }
                },
                style: ElevatedButton.styleFrom(
                  backgroundColor: Color(0xFF0066FF),
                  foregroundColor: Colors.white,
                  padding: EdgeInsets.symmetric(vertical: 18),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(16),
                  ),
                  elevation: 0,
                ),
                child: Text(
                  'Next',
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

  Widget _buildStep4State() {
    return SingleChildScrollView(
      child: Padding(
        padding: EdgeInsets.all(24),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text(
              'Step 4 of 4',
              style: TextStyle(
                fontFamily: 'Inter',
                fontSize: 14,
                fontWeight: FontWeight.w600,
                color: Color(0xFF0066FF),
              ),
            ),
            SizedBox(height: 8),
            Text(
              'Get Paid',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 32,
                fontWeight: FontWeight.bold,
                color: Color(0xFF1D1D1F),
                letterSpacing: -0.5,
              ),
            ),
            SizedBox(height: 16),
            Text(
              'Where should we send your money?',
              style: TextStyle(
                fontFamily: 'Inter',
                fontSize: 16,
                color: Colors.grey[600],
              ),
            ),
            SizedBox(height: 32),
            GestureDetector(
              onTap: () {
                setState(() {
                  paymentConnected = !paymentConnected;
                  _saveProgress();
                });
              },
              child: Container(
                padding: EdgeInsets.all(20),
                decoration: BoxDecoration(
                  color: paymentConnected ? Color(0xFF34C759).withOpacity(0.1) : Colors.white,
                  borderRadius: BorderRadius.circular(16),
                  border: Border.all(
                    color: paymentConnected ? Color(0xFF34C759) : Colors.grey.withOpacity(0.2),
                    width: 2,
                  ),
                ),
                child: Row(
                  children: [
                    Icon(
                      paymentConnected ? Icons.account_balance : Icons.account_balance_outlined,
                      size: 32,
                      color: paymentConnected ? Color(0xFF34C759) : Colors.grey[600],
                    ),
                    SizedBox(width: 16),
                    Expanded(
                      child: Text(
                        paymentConnected ? 'Bank Connected' : 'Connect Bank Account',
                        style: TextStyle(
                          fontFamily: 'Inter',
                          fontSize: 18,
                          fontWeight: FontWeight.w600,
                          color: paymentConnected ? Color(0xFF34C759) : Color(0xFF1D1D1F),
                        ),
                      ),
                    ),
                    if (paymentConnected) Icon(Icons.check_circle, color: Color(0xFF34C759)),
                  ],
                ),
              ),
            ),
            SizedBox(height: 48),
            ElevatedButton(
              onPressed: paymentConnected ? submit : null,
              style: ElevatedButton.styleFrom(
                backgroundColor: paymentConnected ? Color(0xFF0066FF) : Colors.grey[300],
                foregroundColor: Colors.white,
                padding: EdgeInsets.symmetric(vertical: 18),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(16),
                ),
                elevation: 0,
              ),
              child: Text(
                'Launch My Business',
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
}

class StoreLiveScreen extends StatefulWidget {
  final String businessName;

  const StoreLiveScreen({Key? key, required this.businessName}) : super(key: key);

  @override
  _StoreLiveScreenState createState() => _StoreLiveScreenState();
}

class _StoreLiveScreenState extends State<StoreLiveScreen> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _scaleAnimation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: Duration(milliseconds: 600),
    );
    _scaleAnimation = CurvedAnimation(
      parent: _controller,
      curve: Curves.elasticOut,
    );
    _controller.forward();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    // Generate dummy link based on business name
    String subdomain = widget.businessName.replaceAll(RegExp(r'[^a-zA-Z0-9]'), '').toLowerCase();
    if (subdomain.isEmpty) subdomain = 'store';
    String dummyLink = 'https://$subdomain.ohc.app';

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
                      ScaleTransition(
                        scale: _scaleAnimation,
                        child: Container(
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
                      ),
                      SizedBox(height: 32),
                      Text(
                        'Store Live!',
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
                      SizedBox(height: 32),
                      Container(
                        padding: EdgeInsets.symmetric(vertical: 12, horizontal: 16),
                        margin: EdgeInsets.symmetric(horizontal: 24),
                        decoration: BoxDecoration(
                          color: Colors.white,
                          borderRadius: BorderRadius.circular(8),
                          border: Border.all(color: Colors.grey[300]!),
                        ),
                        child: Row(
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: [
                            Expanded(
                              child: Text(
                                dummyLink,
                                style: TextStyle(
                                  fontFamily: 'Inter',
                                  fontSize: 14,
                                  color: Color(0xFF0066FF),
                                ),
                                overflow: TextOverflow.ellipsis,
                              ),
                            ),
                            Icon(Icons.copy, size: 20, color: Colors.grey[600]),
                          ],
                        ),
                      ),
                      SizedBox(height: 48),
                      Padding(
                        padding: const EdgeInsets.symmetric(horizontal: 24.0),
                        child: ElevatedButton(
                          onPressed: () async {
                            final prefs = await SharedPreferences.getInstance();
                            await prefs.remove('onboardingState');
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
