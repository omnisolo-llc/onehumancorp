import 'package:flutter/services.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class WelcomeChecklistWidget extends StatefulWidget {
  const WelcomeChecklistWidget({super.key});

  @override
  State<WelcomeChecklistWidget> createState() => _WelcomeChecklistWidgetState();
}

class _WelcomeChecklistWidgetState extends State<WelcomeChecklistWidget> {

  final Set<String> _checkedItems = {'business_live'};

  @override
  void initState() {
    super.initState();
    _loadState();
  }

  Future<void> _loadState() async {
    final prefs = await SharedPreferences.getInstance();
    final items = prefs.getStringList('welcome_checklist') ?? ['business_live'];
    setState(() {
      _checkedItems.clear();
      _checkedItems.addAll(items);
    });
  }

  Future<void> _saveState() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setStringList('welcome_checklist', _checkedItems.toList());
  }

  void _toggleCheck(String key) {
    setState(() {
      if (_checkedItems.contains(key)) {
        _checkedItems.remove(key);
      } else {
        _checkedItems.add(key);
      }
      _saveState();
    });
  }


  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return Padding(
      padding: const EdgeInsets.only(bottom: 24.0),
      child: GlassCard(
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                'Welcome Checklist',
                style: TextStyle(
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                  fontFamily: 'Outfit',
                  color: colors.onSurface,
                ),
              ),
              const SizedBox(height: 8),
              Text(
                'Complete these steps to get your business fully operational.',
                style: TextStyle(
                  fontSize: 14,
                  fontFamily: 'Inter',
                  color: colors.onSurfaceVariant,
                ),
              ),
              const SizedBox(height: 16),
              _ChecklistItem(
                title: 'Business live',
                isChecked: true, // Always checked since they passed onboarding
                onTap: null,
              ),
              _ChecklistItem(
                title: 'Add 3 more products',
                isChecked: _checkedItems.contains('products'),
                onTap: () {
                  _toggleCheck('products');
                  // Navigate to dashboard as specified in the plan.
                  // Since we are already on the dashboard, this might seem redundant,
                  // but we'll fulfill the specification or perhaps a specific tab.
                  // Let's just navigate.
                  context.go('/dashboard');
                },
              ),
              _ChecklistItem(
                title: 'Connect Instagram',
                isChecked: _checkedItems.contains('instagram'),
                onTap: () {
                  _toggleCheck('instagram');
                  context.go('/integrations');
                },
              ),
              _ChecklistItem(
                title: 'Share your link with a friend',
                isChecked: _checkedItems.contains('share'),
                onTap: () {
                  _toggleCheck('share');
                  Clipboard.setData(const ClipboardData(text: 'https://ohc.io/business/my-link'));
                  ScaffoldMessenger.of(context).showSnackBar(
                    SnackBar(
                      content: Text(
                        'Link copied to clipboard!',
                        style: TextStyle(fontFamily: 'Inter', color: colors.onPrimaryContainer),
                      ),
                      backgroundColor: colors.primaryContainer,
                      behavior: SnackBarBehavior.floating,
                    ),
                  );
                },
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _ChecklistItem extends StatelessWidget {
  final String title;
  final bool isChecked;
  final VoidCallback? onTap;

  const _ChecklistItem({
    required this.title,
    required this.isChecked,
    this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(8),
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 4.0),
        child: Row(
          children: [
            Icon(
              isChecked ? Icons.check_circle : Icons.circle_outlined,
              color: isChecked ? Colors.green : colors.onSurfaceVariant,
              size: 24,
            ),
            const SizedBox(width: 12),
            Expanded(
              child: Text(
                title,
                style: TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 16,
                  color: isChecked ? colors.onSurfaceVariant : colors.onSurface,
                  decoration: isChecked ? TextDecoration.lineThrough : null,
                ),
              ),
            ),
            if (onTap != null)
              Icon(
                Icons.arrow_forward_ios,
                size: 16,
                color: colors.onSurfaceVariant,
              ),
          ],
        ),
      ),
    );
  }
}
