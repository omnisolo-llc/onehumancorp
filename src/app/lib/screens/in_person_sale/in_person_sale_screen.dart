import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:stripe_terminal/stripe_terminal.dart';

class InPersonSaleScreen extends StatefulWidget {
  @override
  _InPersonSaleScreenState createState() => _InPersonSaleScreenState();
}

class _InPersonSaleScreenState extends State<InPersonSaleScreen> {
  final TextEditingController _amountController = TextEditingController();
  bool _isProcessing = false;

  Future<void> _processPayment() async {
    final amountText = _amountController.text;
    if (amountText.isEmpty) return;

    final amountDouble = double.tryParse(amountText);
    if (amountDouble == null || amountDouble <= 0) return;

    setState(() {
      _isProcessing = true;
    });

    try {
      // Hardware-Free POS: Simulate Stripe Terminal Tap to Pay
      // In a real implementation we would initialize StripeTerminal and collectPaymentMethod.
      await Future.delayed(const Duration(seconds: 2));

      // After successful payment, sync via KAIROS Mesh
      final amountCents = (amountDouble * 100).toInt();

      final baseUrl = const String.fromEnvironment('API_BASE_URL', defaultValue: 'http://localhost:8080');
      final broadcastRequest = {
        'topic': 'OfflinePaymentCompleted',
        'message': {
          'id': 'payment-${DateTime.now().millisecondsSinceEpoch}',
          'from_agent': 'FlutterApp',
          'to_agent': 'Mesh',
          'type': 'OfflinePaymentCompleted',
          'content': jsonEncode({'amount_cents': amountCents, 'organization_id': 'e2e-tenant'}),
          'meeting_id': '',
          'occurred_at_unix': DateTime.now().millisecondsSinceEpoch,
        }
      };

      await http.post(
        Uri.parse('$baseUrl/api/mesh/v2/broadcast'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode(broadcastRequest),
      );

      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Payment Successful! synced to Operations & Finance agents.')),
      );
      Navigator.pop(context);
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Payment failed: \$e')),
      );
    } finally {
      if (mounted) {
        setState(() {
          _isProcessing = false;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('In-Person Sale', style: TextStyle(fontFamily: 'Outfit')),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            TextField(
              controller: _amountController,
              keyboardType: const TextInputType.numberWithOptions(decimal: true),
              decoration: const InputDecoration(
                labelText: 'Amount (\$)',
                border: OutlineInputBorder(),
                prefixIcon: Icon(Icons.attach_money),
              ),
            ),
            const SizedBox(height: 24),
            SizedBox(
              width: double.infinity,
              height: 56,
              child: ElevatedButton.icon(
                onPressed: _isProcessing ? null : _processPayment,
                icon: _isProcessing
                    ? const SizedBox(width: 24, height: 24, child: CircularProgressIndicator(color: Colors.white, strokeWidth: 2))
                    : const Icon(Icons.contactless),
                label: Text(
                  _isProcessing ? 'Processing...' : 'Tap to Pay (Hardware-Free)',
                  style: const TextStyle(fontSize: 18),
                ),
                style: ElevatedButton.styleFrom(
                  backgroundColor: const Color(0xFF0EA5E9),
                  foregroundColor: Colors.white,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  @override
  void dispose() {
    _amountController.dispose();
    super.dispose();
  }
}
