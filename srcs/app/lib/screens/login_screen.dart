import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'dart:ui';
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

  Future<void> _editBackendUrl(
    BuildContext context,
    String current,
  ) async {
    final controller = TextEditingController(text: current);
    final result = await showDialog<String>(
      context: context,
      builder:
          (context) => AlertDialog(
            title: const Text('Edit Backend URL'),
            content: TextField(
              controller: controller,
              decoration: const InputDecoration(
                labelText: 'URL (e.g. http://localhost:8080)',
              ),
            ),
            actions: [
              TextButton(
                onPressed: () => Navigator.pop(context),
                child: const Text('Cancel'),
              ),
              TextButton(
                onPressed: () => Navigator.pop(context, controller.text),
                child: const Text('Save'),
              ),
            ],
          ),
    );
    if (result != null && result.isNotEmpty) {
      ref.read(clientSettingsProvider.notifier).updateBackendUrl(result);
    }
  }

  @override
  Widget build(BuildContext context) {
    final currentBackendUrl = ref.watch(clientSettingsProvider).valueOrNull?.backendUrl ?? 'http://localhost:18789';

    return Scaffold(
      body: Stack(
        children: [
          Center(
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 400),
              child: Stack(
                children: [
                  Card(
                    elevation: 4,
                    child: Padding(
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
                                      ? SizedBox(
                                          height: 20,
                                          width: 20,
                                          child: CircularProgressIndicator(
                                            strokeWidth: 2,
                                            color: Theme.of(context).colorScheme.onPrimary,
                                          ),
                                        )
                                      : const Text('Sign In'),
                            ),
                            const SizedBox(height: 16),
                            OutlinedButton.icon(
                              onPressed: _loading ? null : () {
                                // OAuth flow for Thin Client Mode
                                ScaffoldMessenger.of(context).showSnackBar(
                                  const SnackBar(content: Text('OAuth login initiated...')),
                                );
                              },
                              icon: const Icon(Icons.login),
                              label: const Text('Sign In with OAuth'),
                            ),
                          ],
                        ),
                      ),
                    ),
                  ),
                  if (_loading)
                    Positioned.fill(
                      child: ClipRRect(
                        borderRadius: BorderRadius.circular(12),
                        child: BackdropFilter(
                          filter: ImageFilter.compose(
                            outer: ColorFilter.matrix(<double>[
                              1.168, -0.153, -0.015, 0, 0,
                              -0.046, 1.061, -0.015, 0, 0,
                              -0.046, -0.152, 1.198, 0, 0,
                              0, 0, 0, 1, 0,
                            ]),
                            inner: ImageFilter.blur(sigmaX: 5.0, sigmaY: 5.0),
                          ),
                          child: Container(
                            color: Theme.of(context).colorScheme.surface.withOpacity(0.3),
                            child: const Center(
                              child: CircularProgressIndicator(),
                            ),
                          ),
                        ),
                      ),
                    ),
                ],
              ),
            ),
          ),
          Positioned(
            bottom: 24,
            right: 24,
            child: Semantics(
              label: 'Configure remote connection',
              button: true,
              child: Tooltip(
                message: 'Configure Remote Connection',
                child: ClipRRect(
                  borderRadius: BorderRadius.circular(12),
                  child: BackdropFilter(
                    filter: ImageFilter.compose(
                      outer: ColorFilter.matrix(<double>[
                        1.168, -0.153, -0.015, 0, 0,
                        -0.046, 1.061, -0.015, 0, 0,
                        -0.046, -0.152, 1.198, 0, 0,
                        0, 0, 0, 1, 0,
                      ]),
                      inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                    ),
                    child: Material(
                      color: Theme.of(context).colorScheme.surface.withOpacity(0.1),
                      child: InkWell(
                        onTap: () => _editBackendUrl(context, currentBackendUrl),
                        borderRadius: BorderRadius.circular(12),
                        child: Padding(
                          padding: const EdgeInsets.all(12),
                          child: Row(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              Icon(
                                Icons.settings_ethernet,
                                color: Theme.of(context).colorScheme.primary,
                              ),
                              const SizedBox(width: 8),
                              Text(
                                'Remote Connection',
                                style: TextStyle(
                                  color: Theme.of(context).colorScheme.onSurface,
                                  fontWeight: FontWeight.w600,
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
        ],
      ),
    );
  }
}
