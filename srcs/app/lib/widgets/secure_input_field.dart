import 'package:flutter/material.dart';

class SecureInputField extends StatefulWidget {
  final TextEditingController controller;
  final String labelText;
  final String? hintText;
  final String? errorText;
  final ValueChanged<String>? onChanged;
  final IconData defaultIcon;
  final IconData obscuredIcon;

  const SecureInputField({
    super.key,
    required this.controller,
    required this.labelText,
    this.hintText,
    this.errorText,
    this.onChanged,
    this.defaultIcon = Icons.visibility,
    this.obscuredIcon = Icons.visibility_off,
  });

  @override
  State<SecureInputField> createState() => _SecureInputFieldState();
}

class _SecureInputFieldState extends State<SecureInputField> {
  bool _isObscure = true;

  @override
  Widget build(BuildContext context) {
    return TextField(
      onChanged: widget.onChanged,
      controller: widget.controller,
      obscureText: _isObscure,
      style: const TextStyle(
        fontFamily: 'Inter',
        color: Colors.white,
      ),
      decoration: InputDecoration(
        labelText: widget.labelText,
        hintText: widget.hintText,
        errorText: widget.errorText,
        errorStyle: const TextStyle(fontFamily: 'Inter', color: Colors.redAccent),
        errorBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: const BorderSide(color: Colors.redAccent),
        ),
        focusedErrorBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: const BorderSide(color: Colors.redAccent, width: 2),
        ),
        labelStyle: TextStyle(
          fontFamily: 'Inter',
          color: Colors.white.withValues(alpha: 0.7),
        ),
        hintStyle: TextStyle(
          fontFamily: 'Inter',
          color: Colors.white.withValues(alpha: 0.3),
        ),
        enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(
            color: Colors.white.withValues(alpha: 0.2),
          ),
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: const BorderSide(
            color: Colors.blueAccent,
          ),
        ),
        suffixIcon: IconButton(
          icon: Icon(
            _isObscure ? widget.obscuredIcon : widget.defaultIcon,
            color: Colors.white.withValues(alpha: 0.7),
          ),
          onPressed: () {
            setState(() {
              _isObscure = !_isObscure;
            });
          },
        ),
      ),
    );
  }
}
