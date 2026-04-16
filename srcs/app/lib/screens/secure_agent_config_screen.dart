import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/secure_input_field.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class SecureAgentConfigScreen extends StatefulWidget {
  const SecureAgentConfigScreen({super.key});

  @override
  State<SecureAgentConfigScreen> createState() =>
      _SecureAgentConfigScreenState();
}

class _SecureAgentConfigScreenState extends State<SecureAgentConfigScreen> {
  final TextEditingController _spiffeIdController = TextEditingController();
  bool _isSaving = false;

  @override
  void dispose() {
    _spiffeIdController.dispose();
    super.dispose();
  }

  String? _errorMessage;

  Future<void> _saveConfig() async {
    setState(() {
      _errorMessage = null;
    });

    final token = _spiffeIdController.text.trim();
    if (token.isEmpty) {
      setState(() {
        _errorMessage = 'SPIFFE Enrollment Token cannot be empty.';
      });
      return;
    }

    setState(() {
      _isSaving = true;
    });

    try {
      final prefs = await SharedPreferences.getInstance();
      await prefs.setString('spiffe_enrollment_token', token);

      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text(
              'Configuration saved successfully.',
              style: TextStyle(fontFamily: 'Inter'),
            ),
            backgroundColor: Colors.green,
          ),
        );
        Navigator.of(context).pop();
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(
              'Failed to save configuration: $e',
              style: const TextStyle(fontFamily: 'Inter'),
            ),
            backgroundColor: Theme.of(context).colorScheme.error,
          ),
        );
      }
    } finally {
      if (mounted) {
        setState(() {
          _isSaving = false;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text(
          'Secure Agent Config',
          style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold),
        ),
      ),
      body: Center(
        child: SizedBox(
          width: 400,
          child: GlassCard(
            padding: const EdgeInsets.all(32),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text(
                  'Agent Configuration',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 24,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                ),
                const SizedBox(height: 16),
                Text(
                  'Enter your secure SPIFFE Enrollment Token to authenticate the agent.',
                  style: TextStyle(
                    fontFamily: 'Inter',
                    fontSize: 14,
                    color: Colors.white.withValues(alpha: 0.7),
                  ),
                ),
                const SizedBox(height: 24),
                SecureInputField(
                  controller: _spiffeIdController,
                  labelText: 'SPIFFE Enrollment Token',
                  hintText: 'e.g. spiffe://ohc.os/agent/1234',
                ),
                if (_errorMessage != null) ...[
                  const SizedBox(height: 8),
                  Text(
                    _errorMessage!,
                    style: TextStyle(
                      fontFamily: 'Inter',
                      fontSize: 12,
                      color: Theme.of(context).colorScheme.error,
                    ),
                  ),
                ],
                const SizedBox(height: 32),
                SizedBox(
                  width: double.infinity,
                  child: ElevatedButton(
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.blueAccent,
                      foregroundColor: Colors.white,
                      padding: const EdgeInsets.symmetric(vertical: 16),
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(12),
                      ),
                    ),
                    onPressed: _isSaving ? null : _saveConfig,
                    child:
                        _isSaving
                            ? const SizedBox(
                              width: 24,
                              height: 24,
                              child: CircularProgressIndicator(
                                color: Colors.white,
                                strokeWidth: 2,
                              ),
                            )
                            : const Text(
                              'Save Configuration',
                              style: TextStyle(
                                fontFamily: 'Outfit',
                                fontWeight: FontWeight.bold,
                                fontSize: 16,
                              ),
                            ),
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
