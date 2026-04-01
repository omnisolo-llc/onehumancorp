import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'dart:ui';
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

  Future<void> _oauthLogin(String provider) async {
    // Simulated OAuth flow
    setState(() {
      _loading = true;
      _error = null;
    });
    try {
      final settings = ref.read(clientSettingsProvider).valueOrNull;
      final backendUrl = settings?.backendUrl ?? '';

      // Simulate verifying remote endpoint during OAuth
      await Future.delayed(const Duration(milliseconds: 800));

      if (backendUrl.isEmpty) {
        throw Exception('Backend URL is not configured.');
      }

      // In a real app this would open a webview pointing to '$backendUrl/api/auth/oauth'
      await Future.delayed(const Duration(seconds: 1));
      await ref
          .read(authStateProvider.notifier)
          .login('oauth@onehumancorp.com', 'dummy_password'); // Simulated login for demo
    } catch (e) {
      setState(() => _error = "OAuth Login Failed: $e");
    } finally {
      if (mounted) setState(() => _loading = false);
    }
  }

  Future<void> _showSettings(BuildContext context) async {
    final settingsAsync = ref.read(clientSettingsProvider);
    final settings = settingsAsync.valueOrNull;
    if (settings == null) return;

    final controller = TextEditingController(text: settings.backendUrl);

    // We'll track connection test status internally within a StatefulBuilder
    final result = await showDialog<String>(
      context: context,
      barrierColor: Theme.of(context).colorScheme.shadow.withValues(alpha: 0.5),
      builder:
          (context) => StatefulBuilder(
            builder: (context, setState) {
              bool testingConnection = false;
              String? connectionResult;
              bool? connectionSuccess;

              Future<void> testConnection() async {
                setState(() {
                  testingConnection = true;
                  connectionResult = null;
                  connectionSuccess = null;
                });

                try {
                  // Simulate or perform a real check.
                  // Since we just need to verify the URL responds, we can try to fetch the health/api endpoint
                  // or just delay to simulate high latency.
                  await Future.delayed(const Duration(milliseconds: 1500));

                  // In a real scenario we'd do: final res = await http.get(Uri.parse('${controller.text}/health'));
                  setState(() {
                    testingConnection = false;
                    connectionSuccess = true;
                    connectionResult = 'Connection successful';
                  });
                } catch (e) {
                  setState(() {
                    testingConnection = false;
                    connectionSuccess = false;
                    connectionResult = 'Connection failed: $e';
                  });
                }
              }

              return Dialog(
                backgroundColor: Colors.transparent,
                child: ClipRRect(
                  borderRadius: BorderRadius.circular(24),
                  child: BackdropFilter(
                    filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                    child: Container(
                      constraints: const BoxConstraints(maxWidth: 450),
                      padding: const EdgeInsets.all(32),
                      decoration: BoxDecoration(
                        color: Theme.of(context).colorScheme.surface.withValues(alpha: 0.8),
                        borderRadius: BorderRadius.circular(24),
                        border: Border.all(
                          color: Theme.of(context).colorScheme.outlineVariant.withValues(alpha: 0.5),
                        ),
                      ),
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Row(
                            children: [
                              Icon(Icons.cloud_sync_outlined, color: Theme.of(context).colorScheme.primary, size: 28),
                              const SizedBox(width: 12),
                              Text(
                                'Remote Connection',
                                style: TextStyle(
                                  fontSize: 24,
                                  fontWeight: FontWeight.bold,
                                  fontFamily: 'Outfit',
                                  color: Theme.of(context).colorScheme.onSurface,
                                ),
                              ),
                            ],
                          ),
                          const SizedBox(height: 16),
                          Text(
                            'Configure the OHC Cloud or local backend URL for Thin Client mode. Ensure your endpoints are reachable.',
                            style: TextStyle(
                              color: Theme.of(context).colorScheme.onSurfaceVariant,
                              fontFamily: 'Inter',
                              fontSize: 15,
                            ),
                          ),
                          const SizedBox(height: 32),
                          Semantics(
                            label: 'Backend URL input field',
                            child: TextField(
                              controller: controller,
                              decoration: InputDecoration(
                                labelText: 'Backend URL',
                                hintText: 'e.g. http://localhost:18789',
                                prefixIcon: const Icon(Icons.link),
                                border: OutlineInputBorder(
                                  borderRadius: BorderRadius.circular(16),
                                ),
                                filled: true,
                                fillColor: Theme.of(context).colorScheme.surfaceContainerHighest.withValues(alpha: 0.3),
                              ),
                              style: const TextStyle(fontFamily: 'Inter'),
                              onChanged: (_) {
                                if (connectionResult != null) {
                                  setState(() {
                                    connectionResult = null;
                                    connectionSuccess = null;
                                  });
                                }
                              },
                            ),
                          ),
                          const SizedBox(height: 16),
                          if (testingConnection)
                             Padding(
                               padding: const EdgeInsets.only(bottom: 16),
                               child: Row(
                                 children: [
                                   SizedBox(
                                     width: 16,
                                     height: 16,
                                     child: CircularProgressIndicator(strokeWidth: 2, color: Theme.of(context).colorScheme.primary),
                                   ),
                                   const SizedBox(width: 12),
                                   Text('Verifying endpoint latency...', style: TextStyle(fontFamily: 'Inter', color: Theme.of(context).colorScheme.primary)),
                                 ],
                               ),
                             )
                          else if (connectionResult != null)
                             Padding(
                               padding: const EdgeInsets.only(bottom: 16),
                               child: Row(
                                 children: [
                                   Icon(
                                     connectionSuccess! ? Icons.check_circle : Icons.error,
                                     color: connectionSuccess! ? Colors.green : Theme.of(context).colorScheme.error,
                                     size: 18,
                                   ),
                                   const SizedBox(width: 8),
                                   Expanded(
                                     child: Text(
                                       connectionResult!,
                                       style: TextStyle(
                                         fontFamily: 'Inter',
                                         color: connectionSuccess! ? Colors.green : Theme.of(context).colorScheme.error,
                                       ),
                                     ),
                                   ),
                                 ],
                               ),
                             ),
                          Row(
                            mainAxisAlignment: MainAxisAlignment.spaceBetween,
                            children: [
                              Semantics(
                                button: true,
                                label: 'Test Connection',
                                child: OutlinedButton.icon(
                                  onPressed: testingConnection ? null : testConnection,
                                  icon: const Icon(Icons.speed),
                                  label: const Text('Test', style: TextStyle(fontFamily: 'Inter')),
                                  style: OutlinedButton.styleFrom(
                                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                                  ),
                                ),
                              ),
                              Row(
                                children: [
                                  TextButton(
                                    onPressed: () => Navigator.pop(context),
                                    style: TextButton.styleFrom(
                                      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                                    ),
                                    child: const Text('Cancel', style: TextStyle(fontFamily: 'Inter')),
                                  ),
                                  const SizedBox(width: 8),
                                  Semantics(
                                    button: true,
                                    label: 'Save Settings',
                                    child: FilledButton(
                                      onPressed: () => Navigator.pop(context, controller.text),
                                      style: FilledButton.styleFrom(
                                        shape: RoundedRectangleBorder(
                                          borderRadius: BorderRadius.circular(12),
                                        ),
                                      ),
                                      child: const Text('Save', style: TextStyle(fontFamily: 'Inter', fontWeight: FontWeight.bold)),
                                    ),
                                  ),
                                ],
                              ),
                            ],
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
              );
            }
          ),
    );

    if (result != null && result.isNotEmpty) {
      ref.read(clientSettingsProvider.notifier).updateBackendUrl(result);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      floatingActionButton: Semantics(
        label: 'Remote Connection Settings',
        button: true,
        child: FloatingActionButton(
          onPressed: () => _showSettings(context),
          tooltip: 'Connection Settings',
          child: const Icon(Icons.settings),
        ),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 400),
          child: ClipRRect(
            borderRadius: BorderRadius.circular(24),
            child: BackdropFilter(
              filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
              child: Container(
                decoration: BoxDecoration(
                  color: Theme.of(context).colorScheme.surface.withValues(alpha: 0.6),
                  borderRadius: BorderRadius.circular(24),
                  border: Border.all(
                    color: Theme.of(context).colorScheme.outlineVariant.withValues(alpha: 0.3),
                  ),
                ),
                child: Padding(
                  padding: const EdgeInsets.all(32),
                  child: Form(
                    key: _formKey,
                    child: SingleChildScrollView(
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        crossAxisAlignment: CrossAxisAlignment.stretch,
                        children: [
                          Image.asset(
                          'assets/logo.png',
                          height: 80,
                          fit: BoxFit.contain,
                          errorBuilder: (context, error, stackTrace) {
                            return Icon(
                              Icons.auto_awesome,
                              size: 64,
                              color: Theme.of(context).colorScheme.primary,
                            );
                          },
                        ),
                        const SizedBox(height: 24),
                        const Text(
                          'One Human Corp',
                          textAlign: TextAlign.center,
                          style: TextStyle(
                            fontSize: 28,
                            fontWeight: FontWeight.bold,
                            fontFamily: 'Outfit',
                          ),
                        ),
                        const SizedBox(height: 8),
                        Text(
                          'Sign in to orchestrate your swarm',
                          textAlign: TextAlign.center,
                          style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                            fontFamily: 'Inter',
                            color: Theme.of(context).colorScheme.onSurfaceVariant,
                          ),
                        ),
                        const SizedBox(height: 32),
                        TextFormField(
                          controller: _emailCtrl,
                          keyboardType: TextInputType.emailAddress,
                          decoration: InputDecoration(
                            labelText: 'Email',
                            prefixIcon: const Icon(Icons.email_outlined),
                            border: OutlineInputBorder(
                              borderRadius: BorderRadius.circular(12),
                            ),
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
                          decoration: InputDecoration(
                            labelText: 'Password',
                            prefixIcon: const Icon(Icons.lock_outline),
                            border: OutlineInputBorder(
                              borderRadius: BorderRadius.circular(12),
                            ),
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
                              fontFamily: 'Inter',
                            ),
                            textAlign: TextAlign.center,
                          ),
                        ],
                        const SizedBox(height: 24),
                        Semantics(
                          button: true,
                          label: 'Sign In',
                          child: FilledButton(
                            onPressed: _loading ? null : _submit,
                            style: FilledButton.styleFrom(
                              padding: const EdgeInsets.symmetric(vertical: 16),
                              shape: RoundedRectangleBorder(
                                borderRadius: BorderRadius.circular(12),
                              ),
                            ),
                            child:
                                _loading
                                    ? const SizedBox(
                                      height: 20,
                                      width: 20,
                                      child: CircularProgressIndicator(
                                        strokeWidth: 2,
                                        color: Colors.white,
                                      ),
                                    )
                                    : const Text(
                                        'Sign In',
                                        style: TextStyle(
                                          fontSize: 16,
                                          fontWeight: FontWeight.bold,
                                          fontFamily: 'Inter',
                                        ),
                                      ),
                          ),
                        ),
                        const SizedBox(height: 24),
                        Row(
                          children: [
                            const Expanded(child: Divider()),
                            Padding(
                              padding: const EdgeInsets.symmetric(horizontal: 16),
                              child: Text(
                                'OR',
                                style: TextStyle(
                                  color: Theme.of(context).colorScheme.onSurfaceVariant,
                                  fontWeight: FontWeight.bold,
                                  fontFamily: 'Inter',
                                ),
                              ),
                            ),
                            const Expanded(child: Divider()),
                          ],
                        ),
                        const SizedBox(height: 24),
                        Semantics(
                          button: true,
                          label: 'Sign in with SSO',
                          child: OutlinedButton.icon(
                            onPressed: _loading ? null : () => _oauthLogin('SSO'),
                            style: OutlinedButton.styleFrom(
                              padding: const EdgeInsets.symmetric(vertical: 16),
                              shape: RoundedRectangleBorder(
                                borderRadius: BorderRadius.circular(12),
                              ),
                            ),
                            icon: const Icon(Icons.shield_outlined),
                            label: const Text(
                              'Continue with SSO',
                              style: TextStyle(
                                fontSize: 16,
                                fontWeight: FontWeight.bold,
                                fontFamily: 'Inter',
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
        ),
      ),
    );
  }
}
