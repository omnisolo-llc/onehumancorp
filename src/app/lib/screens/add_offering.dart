import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';

enum OfferingType { physical, digital, service }

class AddOfferingScreen extends StatefulWidget {
  @override
  _AddOfferingScreenState createState() => _AddOfferingScreenState();
}

class _AddOfferingScreenState extends State<AddOfferingScreen> {
  final _formKey = GlobalKey<FormState>();
  OfferingType _type = OfferingType.physical;
  String _name = '';
  double _price = 0.0;
  double _duration = 60.0;
  String _description = '';
  bool _isGenerating = false;

  Future<void> generateAIDescription() async {
    if (_name.isEmpty) return;
    setState(() => _isGenerating = true);

    try {
      final String? token = window.localStorage['token'];
      final response = await http.post(
        Uri.parse('http://localhost:8081/ohc.organization.CatalogService/SuggestDescription'),
        headers: {
          'Content-Type': 'application/json',
          'Authorization': 'Bearer $token',
        },
        body: jsonEncode({
          'name': _name,
          'type': _type.index,
        }),
      );

      if (response.statusCode == 200) {
        final data = jsonDecode(response.body);
        setState(() {
          _descriptionController.text = data['description'] ?? '';
          _isGenerating = false;
        });
      } else {
         setState(() => _isGenerating = false);
      }
    } catch (e) {
      setState(() => _isGenerating = false);
    }
  }

  Future<void> saveOffering() async {
    if (_formKey.currentState!.validate()) {
      _formKey.currentState!.save();

      final payload = {
        'item': {
          'name': _name,
          'description': _description,
          'price_cents': (_price * 100).toInt(),
          'currency': 'USD',
          'type': _type.index, // Matches Protobuf enum
          'duration_minutes': _type == OfferingType.service ? _duration.toInt() : 0,
          'organization_id': 'test-org',
          'metadata_json': '{}',
        }
      };

      try {
        final response = await http.post(
          Uri.parse('http://localhost:8081/ohc.organization.CatalogService/CreateCatalogItem'),
          headers: {
            'Content-Type': 'application/json',
            'x-spiffe-id': 'spiffe://onehumancorp.io/test-org/user'
          },
          body: jsonEncode(payload),
        );

        if (response.statusCode == 200) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text('Offering saved successfully!')),
          );
          Navigator.pop(context);
        }
      } catch (e) {
        print('Error saving: $e');
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Color(0xFFF5F5F7),
      appBar: AppBar(
        title: Text('Add New Offering', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        backgroundColor: Colors.transparent,
        elevation: 0,
        foregroundColor: Color(0xFF1D1D1F),
      ),
      body: Center(
        child: Container(
          width: 375,
          child: SingleChildScrollView(
            padding: EdgeInsets.all(24),
            child: Form(
              key: _formKey,
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  Text('What are you offering?', style: TextStyle(fontSize: 18, fontWeight: FontWeight.w600)),
                  SizedBox(height: 16),
                  _buildTypeSelector(),
                  SizedBox(height: 32),
                  _buildTextField('Name', (v) => _name = v!, hint: 'e.g., Guitar Lesson'),
                  SizedBox(height: 16),
                  _buildPriceField(),
                  if (_type == OfferingType.service) ...[
                    SizedBox(height: 32),
                    Text('Duration: ${_duration.toInt()} minutes', style: TextStyle(fontWeight: FontWeight.w500)),
                    Slider(
                      value: _duration,
                      min: 15,
                      max: 240,
                      divisions: 15,
                      activeColor: Color(0xFF0066FF),
                      onChanged: (v) => setState(() => _duration = v),
                    ),
                  ],
                  SizedBox(height: 32),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      Text('Description', style: TextStyle(fontWeight: FontWeight.w600)),
                      TextButton(
                        onPressed: _isGenerating ? null : generateAIDescription,
                        child: Text(_isGenerating ? 'Magic...' : '✨ AI Suggest'),
                      ),
                    ],
                  ),
                  _buildDescriptionField(),
                  SizedBox(height: 48),
                  ElevatedButton(
                    onPressed: saveOffering,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Color(0xFF0066FF),
                      foregroundColor: Colors.white,
                      padding: EdgeInsets.symmetric(vertical: 18),
                      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
                      elevation: 0,
                    ),
                    child: Text('Add Offering', style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildTypeSelector() {
    return Row(
      children: [
        _typeButton('Physical', OfferingType.physical, Icons.inventory_2),
        SizedBox(width: 8),
        _typeButton('Digital', OfferingType.digital, Icons.download),
        SizedBox(width: 8),
        _typeButton('Service', OfferingType.service, Icons.calendar_today),
      ],
    );
  }

  Widget _typeButton(String label, OfferingType type, IconData icon) {
    bool selected = _type == type;
    return Expanded(
      child: GestureDetector(
        onTap: () => setState(() => _type = type),
        child: Container(
          padding: EdgeInsets.symmetric(vertical: 12),
          decoration: BoxDecoration(
            color: selected ? Color(0xFF0066FF) : Colors.white,
            borderRadius: BorderRadius.circular(12),
            border: Border.all(color: selected ? Color(0xFF0066FF) : Colors.grey[300]!),
          ),
          child: Column(
            children: [
              Icon(icon, color: selected ? Colors.white : Colors.grey[600], size: 20),
              SizedBox(height: 4),
              Text(label, style: TextStyle(color: selected ? Colors.white : Colors.grey[600], fontSize: 12, fontWeight: FontWeight.w600)),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildTextField(String label, FormFieldSetter<String> onSaved, {String? hint}) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(label, style: TextStyle(fontWeight: FontWeight.w500)),
        SizedBox(height: 8),
        TextFormField(
          decoration: InputDecoration(
            hintText: hint,
            filled: true,
            fillColor: Colors.white,
            border: OutlineInputBorder(borderRadius: BorderRadius.circular(12), borderSide: BorderSide.none),
            contentPadding: EdgeInsets.all(16),
          ),
          onChanged: (v) => _name = v,
          onSaved: onSaved,
          validator: (v) => v == null || v.isEmpty ? 'Required' : null,
        ),
      ],
    );
  }

  Widget _buildPriceField() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text('Price (USD)', style: TextStyle(fontWeight: FontWeight.w500)),
        SizedBox(height: 8),
        TextFormField(
          keyboardType: TextInputType.number,
          decoration: InputDecoration(
            prefixText: '\$ ',
            filled: true,
            fillColor: Colors.white,
            border: OutlineInputBorder(borderRadius: BorderRadius.circular(12), borderSide: BorderSide.none),
          ),
          onSaved: (v) => _price = double.tryParse(v ?? '0') ?? 0.0,
        ),
      ],
    );
  }

  Widget _buildDescriptionField() {
    return TextFormField(
      controller: TextEditingController(text: _description),
      maxLines: 4,
      decoration: InputDecoration(
        hintText: 'Tell customers about your offering...',
        filled: true,
        fillColor: Colors.white,
        border: OutlineInputBorder(borderRadius: BorderRadius.circular(12), borderSide: BorderSide.none),
      ),
      onSaved: (v) => _description = v ?? '',
    );
  }
}
