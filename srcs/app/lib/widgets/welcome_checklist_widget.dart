import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:shared_preferences/shared_preferences.dart';

class WelcomeChecklistWidget extends StatefulWidget {
  const WelcomeChecklistWidget({super.key});

  @override
  State<WelcomeChecklistWidget> createState() => _WelcomeChecklistWidgetState();
}

class _WelcomeChecklistWidgetState extends State<WelcomeChecklistWidget> {
  bool _isVisible = true;
  bool _isLoading = true;

  @override
  void initState() {
    super.initState();
    _loadVisibility();
  }

  Future<void> _loadVisibility() async {
    final prefs = await SharedPreferences.getInstance();
    final isDismissed = prefs.getBool('welcome_checklist_dismissed') ?? false;
    if (mounted) {
      setState(() {
        _isVisible = !isDismissed;
        _isLoading = false;
      });
    }
  }

  Future<void> _dismiss() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setBool('welcome_checklist_dismissed', true);
    if (mounted) {
      setState(() {
        _isVisible = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    if (_isLoading || !_isVisible) {
      return const SizedBox.shrink();
    }

    return Container(
      margin: const EdgeInsets.only(bottom: 24),
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surfaceContainerHighest.withValues(alpha: 0.1),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: Theme.of(context).colorScheme.primary.withValues(alpha: 0.3)),
      ),
      child: GlassCard(
        child: Padding(
          padding: const EdgeInsets.all(20),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Expanded(
                    child: Text(
                      "You're set up! Here's what to do next",
                      style: TextStyle(
                        fontWeight: FontWeight.bold,
                        fontFamily: 'Outfit',
                        fontSize: 18,
                        color: Theme.of(context).colorScheme.onSurface,
                      ),
                    ),
                  ),
                  IconButton(
                    icon: const Icon(Icons.close),
                    onPressed: _dismiss,
                    tooltip: 'Dismiss',
                  ),
                ],
              ),
              const SizedBox(height: 16),
              _buildChecklistItem(
                context,
                icon: Icons.check_circle,
                iconColor: Colors.green,
                text: "Business live",
                onTap: () {},
                isCompleted: true,
              ),
              _buildChecklistItem(
                context,
                icon: Icons.radio_button_unchecked,
                iconColor: Theme.of(context).colorScheme.primary,
                text: "Add 3 more products",
                onTap: () {
                  context.go('/service');
                },
                isCompleted: false,
              ),
              _buildChecklistItem(
                context,
                icon: Icons.radio_button_unchecked,
                iconColor: Theme.of(context).colorScheme.primary,
                text: "Connect Instagram",
                onTap: () {
                  context.go('/channels');
                },
                isCompleted: false,
              ),
              _buildChecklistItem(
                context,
                icon: Icons.radio_button_unchecked,
                iconColor: Theme.of(context).colorScheme.primary,
                text: "Share your link with a friend",
                onTap: () {
                  Clipboard.setData(const ClipboardData(text: 'https://mybusiness.ohc.app'));
                  ScaffoldMessenger.of(context).showSnackBar(
                    const SnackBar(content: Text('Link copied to clipboard!')),
                  );
                },
                isCompleted: false,
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildChecklistItem(BuildContext context, {
    required IconData icon,
    required Color iconColor,
    required String text,
    required VoidCallback onTap,
    required bool isCompleted,
  }) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(8),
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: 8.0),
        child: Row(
          children: [
            Icon(icon, color: iconColor),
            const SizedBox(width: 12),
            Expanded(
              child: Text(
                text,
                style: TextStyle(
                  fontFamily: 'Inter',
                  decoration: isCompleted ? TextDecoration.lineThrough : null,
                  color: isCompleted
                      ? Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.5)
                      : Theme.of(context).colorScheme.onSurface,
                ),
              ),
            ),
            if (!isCompleted)
              Icon(Icons.chevron_right, color: Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.5)),
          ],
        ),
      ),
    );
  }
}
