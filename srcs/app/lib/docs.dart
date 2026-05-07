import 'dart:ui';
import 'package:flutter/material.dart';

// --- Tooltip Registry ---
class TooltipRegistry {
  static final Map<String, String> _tooltips = {
    'revenue': 'Total money you have made from all sales. Updates instantly.',
    'approvals': 'AI helpers waiting for your permission to send messages or do tasks.',
    'orders': 'Recent customer purchases that need to be packed and shipped.',
    'help_center': 'Find answers, guides, and learn how to use the app.',
    'ai_chat': 'Ask your AI helper any question about using the app.',
  };

  static String getTooltip(String key) {
    return _tooltips[key] ?? '';
  }
}

class ContextualTooltip extends StatelessWidget {
  final String tooltipKey;
  final Widget child;

  const ContextualTooltip({super.key, required this.tooltipKey, required this.child});

  @override
  Widget build(BuildContext context) {
    final text = TooltipRegistry.getTooltip(tooltipKey);
    if (text.isEmpty) return child;
    return Tooltip(
      message: text,
      preferBelow: false,
      textStyle: const TextStyle(color: Colors.white, fontSize: 14),
      decoration: BoxDecoration(
        color: const Color(0xFF1E293B),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: Colors.white24),
      ),
      padding: const EdgeInsets.all(12),
      margin: const EdgeInsets.symmetric(horizontal: 20),
      triggerMode: TooltipTriggerMode.longPress,
      child: child,
    );
  }
}

class GlassCard extends StatelessWidget {
  final Widget child;
  const GlassCard({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 15.0, sigmaY: 15.0),
        child: Container(
          decoration: BoxDecoration(
            color: Colors.white.withAlpha(12),
            borderRadius: BorderRadius.circular(12),
            border: Border.all(color: Colors.white.withAlpha(25)),
          ),
          padding: const EdgeInsets.all(16),
          child: child,
        ),
      ),
    );
  }
}

// --- Help Center ---
class HelpCenterScreen extends StatefulWidget {
  const HelpCenterScreen({super.key});

  @override
  State<HelpCenterScreen> createState() => _HelpCenterScreenState();
}

class _HelpCenterScreenState extends State<HelpCenterScreen> {
  final _searchController = TextEditingController();

  final List<Map<String, String>> _articles = [
    {'category': 'Getting Started', 'title': 'Set up your store', 'desc': 'Add your first product and go live.'},
    {'category': 'My Store', 'title': 'Adding products', 'desc': 'Learn how to list new items with photos.'},
    {'category': 'Payments', 'title': 'Accept Apple Pay', 'desc': 'Enable one-click payments.'},
    {'category': 'AI Agents', 'title': 'What can helpers do?', 'desc': 'Your AI can reply to customer emails.'},
    {'category': 'Marketing', 'title': 'Run a promotion', 'desc': 'Create discount codes for social media.'},
    {'category': 'Account & Billing', 'title': 'Change your plan', 'desc': 'Upgrade or downgrade your subscription.'},
  ];

  @override
  Widget build(BuildContext context) {
    final query = _searchController.text.toLowerCase();
    final filtered = _articles.where((a) => a['title']!.toLowerCase().contains(query) || a['desc']!.toLowerCase().contains(query)).toList();

    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      appBar: AppBar(
        title: const Text('Help Center', style: TextStyle(color: Colors.white, fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
        iconTheme: const IconThemeData(color: Colors.white),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 600),
          child: Padding(
            padding: const EdgeInsets.all(20),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                TextField(
                  controller: _searchController,
                  style: const TextStyle(color: Colors.white),
                  onChanged: (_) => setState(() {}),
                  decoration: InputDecoration(
                    hintText: 'Search help articles...',
                    hintStyle: const TextStyle(color: Colors.white54),
                    filled: true,
                    fillColor: Colors.white10,
                    prefixIcon: const Icon(Icons.search, color: Colors.white54),
                    border: OutlineInputBorder(borderRadius: BorderRadius.circular(12), borderSide: BorderSide.none),
                  ),
                ),
                const SizedBox(height: 20),
                Expanded(
                  child: ListView.builder(
                    itemCount: filtered.length,
                    itemBuilder: (context, index) {
                      final a = filtered[index];
                      return Padding(
                        padding: const EdgeInsets.only(bottom: 16),
                        child: GlassCard(
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(a['category']!, style: const TextStyle(color: Color(0xFF94A3B8), fontSize: 12)),
                              const SizedBox(height: 4),
                              Text(a['title']!, style: const TextStyle(color: Colors.white, fontSize: 18, fontWeight: FontWeight.bold)),
                              const SizedBox(height: 4),
                              Text(a['desc']!, style: const TextStyle(color: Colors.white70)),
                            ],
                          ),
                        ),
                      );
                    },
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

// --- AI Chat ---
class AiHelpChatScreen extends StatefulWidget {
  const AiHelpChatScreen({super.key});

  @override
  State<AiHelpChatScreen> createState() => _AiHelpChatScreenState();
}

class _AiHelpChatScreenState extends State<AiHelpChatScreen> {
  final _controller = TextEditingController();
  final List<Map<String, String>> _messages = [
    {'sender': 'AI', 'text': 'Hi! I am your Help Agent. What do you need help with?', 'link': ''}
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      appBar: AppBar(
        title: const Text('AI Help', style: TextStyle(color: Colors.white, fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
        iconTheme: const IconThemeData(color: Colors.white),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 600),
          child: Column(
            children: [
              Expanded(
                child: ListView.builder(
                  padding: const EdgeInsets.all(20),
                  itemCount: _messages.length,
                  itemBuilder: (context, index) {
                    final msg = _messages[index];
                    final isUser = msg['sender'] == 'User';
                    return Align(
                      alignment: isUser ? Alignment.centerRight : Alignment.centerLeft,
                      child: Container(
                        margin: const EdgeInsets.only(bottom: 16),
                        padding: const EdgeInsets.all(16),
                        decoration: BoxDecoration(
                          color: isUser ? const Color(0xFF1E293B) : const Color(0xFF334155),
                          borderRadius: BorderRadius.circular(12),
                        ),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text(isUser ? 'You' : 'Help Agent', style: const TextStyle(color: Color(0xFF94A3B8), fontSize: 12)),
                            const SizedBox(height: 4),
                            Text(msg['text']!, style: const TextStyle(color: Colors.white)),
                            if (msg['link']!.isNotEmpty) ...[
                              const SizedBox(height: 8),
                              Text('Read the full article →', style: const TextStyle(color: Color(0xFF0EA5E9))),
                            ]
                          ],
                        ),
                      ),
                    );
                  },
                ),
              ),
              Padding(
                padding: const EdgeInsets.all(20),
                child: Row(
                  children: [
                    Expanded(
                      child: TextField(
                        controller: _controller,
                        style: const TextStyle(color: Colors.white),
                        decoration: InputDecoration(
                          hintText: 'Type your question...',
                          hintStyle: const TextStyle(color: Colors.white54),
                          filled: true,
                          fillColor: Colors.white10,
                          border: OutlineInputBorder(borderRadius: BorderRadius.circular(12), borderSide: BorderSide.none),
                        ),
                      ),
                    ),
                    const SizedBox(width: 12),
                    ElevatedButton(
                      onPressed: () {
                        if (_controller.text.isNotEmpty) {
                          setState(() {
                            _messages.add({'sender': 'User', 'text': _controller.text, 'link': ''});
                            _controller.clear();
                          });
                          Future.delayed(const Duration(seconds: 1), () {
                            if (mounted) {
                              setState(() {
                                _messages.add({'sender': 'AI', 'text': 'I can help with that. Please check this guide.', 'link': 'yes'});
                              });
                            }
                          });
                        }
                      },
                      style: ElevatedButton.styleFrom(
                        backgroundColor: const Color(0xFF6B4EFF),
                        padding: const EdgeInsets.all(16),
                        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                      ),
                      child: const Icon(Icons.send, color: Colors.white),
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

// --- Video Tutorials ---
class VideoTutorialsScreen extends StatelessWidget {
  const VideoTutorialsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final videos = [
      {'title': 'How to add a product', 'desc': 'Learn the basics of listing an item.', 'duration': '1:25'},
      {'title': 'Setting up payments', 'desc': 'Accept Apple Pay and credit cards.', 'duration': '1:00'},
      {'title': 'Activate AI Support', 'desc': 'Let AI handle customer chats.', 'duration': '1:28'},
    ];

    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      appBar: AppBar(
        title: const Text('Video Tutorials', style: TextStyle(color: Colors.white, fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
        iconTheme: const IconThemeData(color: Colors.white),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 600),
          child: ListView.builder(
            padding: const EdgeInsets.all(20),
            itemCount: videos.length,
            itemBuilder: (context, index) {
              final v = videos[index];
              return Padding(
                padding: const EdgeInsets.only(bottom: 20),
                child: GlassCard(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Container(
                        height: 200,
                        width: double.infinity,
                        decoration: BoxDecoration(
                          color: Colors.black45,
                          borderRadius: BorderRadius.circular(8),
                        ),
                        child: const Center(
                          child: Icon(Icons.play_circle_fill, size: 64, color: Colors.white),
                        ),
                      ),
                      const SizedBox(height: 16),
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Expanded(child: Text(v['title']!, style: const TextStyle(color: Colors.white, fontSize: 18, fontWeight: FontWeight.bold))),
                          Text(v['duration']!, style: const TextStyle(color: Colors.white54)),
                        ],
                      ),
                      const SizedBox(height: 8),
                      Text(v['desc']!, style: const TextStyle(color: Colors.white70)),
                    ],
                  ),
                ),
              );
            },
          ),
        ),
      ),
    );
  }
}

// --- API Documentation ---
class ApiDocsScreen extends StatefulWidget {
  const ApiDocsScreen({super.key});

  @override
  State<ApiDocsScreen> createState() => _ApiDocsScreenState();
}

class _ApiDocsScreenState extends State<ApiDocsScreen> {
  bool _advanced = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      appBar: AppBar(
        title: const Text('Connect Custom Software', style: TextStyle(color: Colors.white, fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
        iconTheme: const IconThemeData(color: Colors.white),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 600),
          child: Padding(
            padding: const EdgeInsets.all(20),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text('Advanced Feature', style: TextStyle(color: Colors.redAccent, fontWeight: FontWeight.bold)),
                const SizedBox(height: 8),
                const Text(
                  'Connect other tools to your store. If you are not a software developer, you do not need this page.',
                  style: TextStyle(color: Colors.white70),
                ),
                const SizedBox(height: 20),
                Row(
                  children: [
                    Switch(
                      value: _advanced,
                      onChanged: (v) => setState(() => _advanced = v),
                      activeTrackColor: const Color(0xFF6B4EFF),
                    ),
                    const SizedBox(width: 8),
                    const Text('Show Developer Documentation', style: TextStyle(color: Colors.white)),
                  ],
                ),
                if (_advanced) ...[
                  const SizedBox(height: 20),
                  Expanded(
                    child: ListView(
                      children: [
                        GlassCard(
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Row(
                                children: [
                                  const Text('GET', style: TextStyle(color: Colors.greenAccent, fontWeight: FontWeight.bold)),
                                  const SizedBox(width: 12),
                                  const Text('/v1/products', style: TextStyle(color: Colors.white, fontWeight: FontWeight.bold)),
                                ],
                              ),
                              const SizedBox(height: 8),
                              const Text('List all products in your store.', style: TextStyle(color: Colors.white70)),
                              const SizedBox(height: 16),
                              ElevatedButton(
                                onPressed: () {},
                                style: ElevatedButton.styleFrom(backgroundColor: Colors.white10),
                                child: const Text('Run Test', style: TextStyle(color: Colors.white)),
                              ),
                            ],
                          ),
                        ),
                      ],
                    ),
                  ),
                ]
              ],
            ),
          ),
        ),
      ),
    );
  }
}

// --- Release Notes ---
class ReleaseNotesScreen extends StatelessWidget {
  const ReleaseNotesScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      appBar: AppBar(
        title: const Text("What's New", style: TextStyle(color: Colors.white, fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
        iconTheme: const IconThemeData(color: Colors.white),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 600),
          child: ListView(
            padding: const EdgeInsets.all(20),
            children: [
              const Text('Version 1.2.0', style: TextStyle(color: Colors.white70, fontWeight: FontWeight.bold)),
              const SizedBox(height: 8),
              GlassCard(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text('⚡ Supercharged Analytics', style: TextStyle(color: Colors.white, fontSize: 18, fontWeight: FontWeight.bold)),
                    const SizedBox(height: 8),
                    const Text('We upgraded the stats dashboard so you can see exactly where your customers are coming from.', style: TextStyle(color: Colors.white70)),
                    const SizedBox(height: 16),
                    Container(
                      height: 150,
                      width: double.infinity,
                      decoration: BoxDecoration(color: Colors.black26, borderRadius: BorderRadius.circular(8)),
                      child: const Center(child: Text('📸 Screenshot', style: TextStyle(color: Colors.white54))),
                    ),
                  ],
                ),
              ),
              const SizedBox(height: 20),
              const Text('Read full changelog on website →', style: TextStyle(color: Color(0xFF0EA5E9))),
            ],
          ),
        ),
      ),
    );
  }
}

// --- Interactive Walkthrough Overlay ---
class WalkthroughOverlay extends StatelessWidget {
  final VoidCallback onDismiss;

  const WalkthroughOverlay({super.key, required this.onDismiss});

  @override
  Widget build(BuildContext context) {
    return Positioned(
      bottom: 120,
      right: 20,
      child: Material(
        color: Colors.transparent,
        child: Container(
          width: 250,
          padding: const EdgeInsets.all(16),
          decoration: BoxDecoration(
            color: const Color(0xFF6B4EFF),
            borderRadius: const BorderRadius.only(
              topLeft: Radius.circular(16),
              topRight: Radius.circular(16),
              bottomLeft: Radius.circular(16),
              bottomRight: Radius.circular(4),
            ),
            boxShadow: [
              BoxShadow(color: Colors.black26, blurRadius: 10, offset: Offset(0, 4)),
            ],
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              const Text(
                'Welcome to your store!',
                style: TextStyle(color: Colors.white, fontSize: 16, fontWeight: FontWeight.bold, fontFamily: 'Outfit'),
              ),
              const SizedBox(height: 8),
              const Text(
                'I am here to guide you. Tap the "?" or chat button below anytime you need help!',
                style: TextStyle(color: Colors.white, fontSize: 14),
              ),
              const SizedBox(height: 12),
              Align(
                alignment: Alignment.centerRight,
                child: TextButton(
                  onPressed: onDismiss,
                  child: const Text('Got it', style: TextStyle(color: Colors.white, fontWeight: FontWeight.bold)),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

// --- Menu Screen ---
class MenuScreen extends StatelessWidget {
  const MenuScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      appBar: AppBar(
        title: const Text('Menu', style: TextStyle(color: Colors.white, fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
        iconTheme: const IconThemeData(color: Colors.white),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 600),
          child: ListView(
            padding: const EdgeInsets.all(20),
            children: [
              ListTile(
                leading: const Icon(Icons.ondemand_video, color: Colors.white),
                title: const Text('Video Tutorials', style: TextStyle(color: Colors.white)),
                onTap: () => Navigator.push(context, MaterialPageRoute(builder: (_) => const VideoTutorialsScreen())),
              ),
              ListTile(
                leading: const Icon(Icons.api, color: Colors.white),
                title: const Text('API Documentation', style: TextStyle(color: Colors.white)),
                onTap: () => Navigator.push(context, MaterialPageRoute(builder: (_) => const ApiDocsScreen())),
              ),
              ListTile(
                leading: const Icon(Icons.new_releases, color: Colors.white),
                title: const Text('Release Notes', style: TextStyle(color: Colors.white)),
                onTap: () => Navigator.push(context, MaterialPageRoute(builder: (_) => const ReleaseNotesScreen())),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
