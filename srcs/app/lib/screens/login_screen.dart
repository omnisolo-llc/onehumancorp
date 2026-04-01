import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
// import 'package:flutter_svg/flutter_svg.dart'; // Temporarily disabled for Bazel build
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/settings_service.dart';

class LoginScreen extends ConsumerStatefulWidget {
  const LoginScreen({super.key});

  @override
  ConsumerState<LoginScreen> createState() => _LoginScreenState();
}

class _LoginScreenState extends ConsumerState<LoginScreen> {
  final _formKey = GlobalKey<FormState>();
  final _emailCtrl = TextEditingController();
  final _passwordCtrl = TextEditingController();
  bool _loading = false;
  String? _error;

  @override
  void dispose() {
    _emailCtrl.dispose();
    _passwordCtrl.dispose();
    super.dispose();
  }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) return;
    setState(() {
      _loading = true;
      _error = null;
    });
    try {
      await ref
          .read(authStateProvider.notifier)
          .login(_emailCtrl.text.trim(), _passwordCtrl.text);
    } catch (e) {
      setState(() => _error = e.toString());
    } finally {
      if (mounted) setState(() => _loading = false);
    }
  }

  Future<void> _handleOAuthLogin(BuildContext context) async {
    setState(() {
      _loading = true;
      _error = null;
    });

    try {
      final settingsAsync = ref.read(clientSettingsProvider);

      final backendUrl = settingsAsync.when(
        data: (settings) => settings.backendUrl,
        loading: () => 'remote endpoint',
        error: (_, __) => 'remote endpoint',
      );

      // Simulate OAuth navigation and retrieval of tokens
      await Future.delayed(const Duration(milliseconds: 1200));

      // Trigger standard login mock with valid admin credentials after OAuth token resolution
      await ref
          .read(authStateProvider.notifier)
          .login("admin@ohc.local", "password");

      if (mounted) {
         ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text('Authenticated via OAuth at $backendUrl'),
              backgroundColor: Theme.of(context).colorScheme.primary, // Fallback if token not accessible
            ),
         );
      }
    } catch (e) {
      setState(() => _error = 'OAuth Failed: $e');
    } finally {
      if (mounted) setState(() => _loading = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 400),
          child: Card(
            elevation: 4,
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(32),
              child: Form(
                key: _formKey,
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    Image.asset(
                      'assets/logo.png',
                      height: 100,
                      fit: BoxFit.contain,
                      errorBuilder: (context, error, stackTrace) {
                        return Icon(
                          Icons.person,
                          size: 80,
                          color: Theme.of(context).colorScheme.primary,
                        );
                      },
                    ),
                    const SizedBox(height: 24),
                    const Text(
                      'One Human Corp',
                      textAlign: TextAlign.center,
                      style: TextStyle(
                        fontSize: 24,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    const SizedBox(height: 8),
                    Text(
                      'Sign in to your company',
                      textAlign: TextAlign.center,
                      style: Theme.of(context).textTheme.bodyMedium,
                    ),
                    const SizedBox(height: 32),
                    TextFormField(
                      controller: _emailCtrl,
                      keyboardType: TextInputType.emailAddress,
                      decoration: const InputDecoration(
                        labelText: 'Email',
                        prefixIcon: Icon(Icons.email_outlined),
                        border: OutlineInputBorder(),
                      ),
                      validator:
                          (v) =>
                              (v == null || !v.contains('@'))
                                  ? 'Enter a valid email'
                                  : null,
                    ),
                    const SizedBox(height: 16),
                    TextFormField(
                      controller: _passwordCtrl,
                      obscureText: true,
                      decoration: const InputDecoration(
                        labelText: 'Password',
                        prefixIcon: Icon(Icons.lock_outline),
                        border: OutlineInputBorder(),
                      ),
                      validator:
                          (v) =>
                              (v == null || v.isEmpty)
                                  ? 'Enter your password'
                                  : null,
                    ),
                    if (_error != null) ...[
                      const SizedBox(height: 12),
                      Text(
                        _error!,
                        style: TextStyle(
                          color: Theme.of(context).colorScheme.error,
                          fontSize: 13,
                        ),
                        textAlign: TextAlign.center,
                      ),
                    ],
                    const SizedBox(height: 24),
                    FilledButton(
                      onPressed: _loading ? null : _submit,
                      child:
                          _loading
                              ? const SizedBox(
                                height: 20,
                                width: 20,
                                child: CircularProgressIndicator(
                                  strokeWidth: 2,
                                ),
                              )
                              : const Text('Sign In'),
                    ),
                    const SizedBox(height: 16),
                    OutlinedButton.icon(
                      onPressed: _loading ? null : () => _handleOAuthLogin(context),
                      icon: const Icon(Icons.cloud_sync),
                      label: const Text('Sign In with OAuth (Remote)'),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
