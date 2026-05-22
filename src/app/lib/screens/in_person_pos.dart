import 'dart:convert';
import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:shared_preferences/shared_preferences.dart';

class StripeTerminalService {
  final http.Client client;
  StripeTerminalService({required this.client});

  Future<bool> triggerTapToPay(double amount) async {
    // Mocking Stripe Terminal SDK "Tap to Pay" flow delay
    await Future.delayed(Duration(seconds: 2));
    return true; // Simulate success
  }

  Future<void> emitOfflinePaymentCompleted(double amount) async {
    try {
      final prefs = await SharedPreferences.getInstance();
      final token = prefs.getString('auth_token') ?? 'mock_token';
      final baseUrl = const String.fromEnvironment('API_BASE_URL', defaultValue: 'http://localhost:18789');

      await client.post(
        Uri.parse('$baseUrl/api/events'),
        headers: {
          'Content-Type': 'application/json',
          'Authorization': 'Bearer $token'
        },
        body: json.encode({
          'event_type': 'OfflinePaymentCompleted',
          'amount': amount,
          'currency': 'USD',
          'source': 'stripe_terminal_mobile',
        }),
      );
    } catch (e) {
      print('Failed to emit OfflinePaymentCompleted: $e');
    }
  }
}

class InPersonPosScreen extends StatefulWidget {
  final http.Client? httpClient;
  InPersonPosScreen({this.httpClient});

  @override
  _InPersonPosScreenState createState() => _InPersonPosScreenState();
}

class _InPersonPosScreenState extends State<InPersonPosScreen> {
  late StripeTerminalService _terminalService;
  TextEditingController _amountController = TextEditingController();
  List<Map<String, dynamic>> _cart = [];
  bool _isProcessing = false;
  bool _isSuccess = false;

  @override
  void initState() {
    super.initState();
    _terminalService = StripeTerminalService(client: widget.httpClient ?? http.Client());
  }

  void _addCustomAmount() {
    final amountText = _amountController.text;
    if (amountText.isNotEmpty) {
      final amount = double.tryParse(amountText);
      if (amount != null && amount > 0) {
        setState(() {
          _cart.add({'name': 'Custom Amount', 'price': amount});
          _amountController.clear();
        });
      }
    }
  }

  double get _totalAmount {
    return _cart.fold(0.0, (sum, item) => sum + item['price']);
  }

  Future<void> _processCheckout() async {
    if (_totalAmount <= 0) return;

    setState(() {
      _isProcessing = true;
    });

    _showTapToPayModal();

    final success = await _terminalService.triggerTapToPay(_totalAmount);

    if (success) {
      await _terminalService.emitOfflinePaymentCompleted(_totalAmount);
      Navigator.of(context).pop(); // Close modal
      setState(() {
        _isProcessing = false;
        _isSuccess = true;
      });
    }
  }

  void _showTapToPayModal() {
    showDialog(
      context: context,
      barrierDismissible: false,
      builder: (BuildContext context) {
        return Dialog(
          backgroundColor: Colors.transparent,
          elevation: 0,
          child: Container(
            decoration: BoxDecoration(
              color: Colors.white.withOpacity(0.65),
              borderRadius: BorderRadius.circular(16),
              border: Border.all(color: Colors.white.withOpacity(0.4), width: 1),
              boxShadow: [
                BoxShadow(
                  color: Colors.black.withOpacity(0.05),
                  blurRadius: 10,
                  offset: Offset(0, 5),
                ),
              ],
            ),
            child: ClipRRect(
              borderRadius: BorderRadius.circular(16),
              child: BackdropFilter(
                filter: ImageFilter.blur(sigmaX: 30, sigmaY: 30),
                child: Padding(
                  padding: EdgeInsets.all(32),
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Icon(Icons.contactless, size: 64, color: Color(0xFF0066FF)),
                      SizedBox(height: 24),
                      Text(
                        'Present card to device',
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                          color: Color(0xFF1D1D1F),
                        ),
                        textAlign: TextAlign.center,
                      ),
                      SizedBox(height: 12),
                      Text(
                        '\$${_totalAmount.toStringAsFixed(2)}',
                        style: TextStyle(
                          fontFamily: 'Inter',
                          fontSize: 24,
                          color: Color(0xFF1D1D1F),
                        ),
                      ),
                      SizedBox(height: 24),
                      CircularProgressIndicator(valueColor: AlwaysStoppedAnimation<Color>(Color(0xFF0066FF))),
                    ],
                  ),
                ),
              ),
            ),
          ),
        );
      },
    );
  }

  Widget _buildCartItem(Map<String, dynamic> item, int index) {
    return ListTile(
      title: Text(item['name'], style: TextStyle(fontFamily: 'Inter')),
      trailing: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text('\$${item['price'].toStringAsFixed(2)}', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
          IconButton(
            icon: Icon(Icons.remove_circle_outline, color: Colors.red[300]),
            onPressed: () {
              setState(() {
                _cart.removeAt(index);
              });
            },
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    if (_isSuccess) {
      return Scaffold(
        backgroundColor: Color(0xFFF5F5F7),
        body: Center(
          child: Padding(
            padding: EdgeInsets.all(24),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Container(
                  padding: EdgeInsets.all(20),
                  decoration: BoxDecoration(
                    color: Color(0xFF34C759).withOpacity(0.1),
                    shape: BoxShape.circle,
                  ),
                  child: Icon(Icons.check_circle, size: 80, color: Color(0xFF34C759)),
                ),
                SizedBox(height: 32),
                Text(
                  'Payment Successful',
                  style: TextStyle(fontFamily: 'Outfit', fontSize: 28, fontWeight: FontWeight.bold),
                ),
                SizedBox(height: 16),
                Text(
                  '\$${_totalAmount.toStringAsFixed(2)} has been recorded and inventory updated.',
                  textAlign: TextAlign.center,
                  style: TextStyle(fontFamily: 'Inter', fontSize: 16, color: Colors.grey[600]),
                ),
                SizedBox(height: 48),
                ElevatedButton(
                  onPressed: () {
                    Navigator.of(context).pop();
                  },
                  style: ElevatedButton.styleFrom(
                    backgroundColor: Color(0xFF0066FF),
                    foregroundColor: Colors.white,
                    padding: EdgeInsets.symmetric(vertical: 16),
                    minimumSize: Size(double.infinity, 50),
                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                  ),
                  child: Text('Done'),
                ),
                SizedBox(height: 16),
                TextButton(
                  onPressed: () {
                    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Receipt sent!')));
                  },
                  child: Text('Email/SMS Receipt', style: TextStyle(color: Color(0xFF0066FF))),
                ),
              ],
            ),
          ),
        ),
      );
    }

    return Scaffold(
      backgroundColor: Color(0xFFF5F5F7),
      appBar: AppBar(
        title: Text('In-Person Sale', style: TextStyle(fontFamily: 'Outfit', color: Colors.black87, fontWeight: FontWeight.bold)),
        backgroundColor: Colors.white.withOpacity(0.8),
        elevation: 0,
        iconTheme: IconThemeData(color: Colors.black87),
        flexibleSpace: ClipRect(
          child: BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 10.0, sigmaY: 10.0),
            child: Container(color: Colors.transparent),
          ),
        ),
      ),
      body: Padding(
        padding: EdgeInsets.all(16),
        child: Column(
          children: [
            Container(
              padding: EdgeInsets.symmetric(horizontal: 16, vertical: 8),
              decoration: BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.circular(12),
                border: Border.all(color: Colors.grey[200]!),
              ),
              child: Row(
                children: [
                  Text('\$', style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold)),
                  SizedBox(width: 8),
                  Expanded(
                    child: TextField(
                      controller: _amountController,
                      keyboardType: TextInputType.numberWithOptions(decimal: true),
                      decoration: InputDecoration(
                        border: InputBorder.none,
                        hintText: '0.00',
                        hintStyle: TextStyle(fontSize: 24, color: Colors.grey[400]),
                      ),
                      style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, fontFamily: 'Inter'),
                    ),
                  ),
                  ElevatedButton(
                    onPressed: _addCustomAmount,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.grey[200],
                      foregroundColor: Colors.black87,
                      elevation: 0,
                      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                    ),
                    child: Text('Add'),
                  ),
                ],
              ),
            ),
            SizedBox(height: 24),
            Expanded(
              child: _cart.isEmpty
                  ? Center(child: Text('Cart is empty', style: TextStyle(color: Colors.grey)))
                  : ListView.builder(
                      itemCount: _cart.length,
                      itemBuilder: (context, index) {
                        return _buildCartItem(_cart[index], index);
                      },
                    ),
            ),
            Container(
              padding: EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.circular(16),
                boxShadow: [
                  BoxShadow(
                    color: Colors.black.withOpacity(0.05),
                    blurRadius: 10,
                    offset: Offset(0, -5),
                  ),
                ],
              ),
              child: Column(
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      Text('Total', style: TextStyle(fontFamily: 'Outfit', fontSize: 20, fontWeight: FontWeight.bold)),
                      Text('\$${_totalAmount.toStringAsFixed(2)}', style: TextStyle(fontFamily: 'Inter', fontSize: 24, fontWeight: FontWeight.bold)),
                    ],
                  ),
                  SizedBox(height: 16),
                  ElevatedButton(
                    onPressed: _totalAmount > 0 && !_isProcessing ? _processCheckout : null,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Color(0xFF0066FF),
                      foregroundColor: Colors.white,
                      padding: EdgeInsets.symmetric(vertical: 16),
                      minimumSize: Size(double.infinity, 50),
                      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                    ),
                    child: Text('Checkout (Tap to Pay)', style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
