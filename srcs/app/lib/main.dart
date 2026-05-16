import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'screens/help/video_tutorials_screen.dart';
import 'screens/unified_inbox_screen.dart';
import 'widgets/walkthrough_overlay.dart';
import 'screens/business_setup_wizard_screen.dart';
import 'screens/help/help_center_screen.dart';
import 'screens/help/ai_help_chat_screen.dart';


import 'screens/referral_program_screen.dart';
import 'widgets/milestone_notification.dart';
import 'providers/action_center_provider.dart';

void main() {
  runApp(const ProviderScope(child: OHCApp()));
}

class OHCApp extends StatelessWidget {
  const OHCApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'One Human Corp',
      theme: ThemeData(
        useMaterial3: true,
        fontFamily: 'Inter',
        colorScheme: ColorScheme.fromSeed(
          seedColor: const Color(0xFF6B4EFF),
          brightness: Brightness.dark,
        ),
      ),
      home: const BusinessSetupWizardScreen(),
    );
  }
}

class GlassContainer extends StatelessWidget {
  final Widget child;
  const GlassContainer({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(20),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          decoration: BoxDecoration(
            color: Colors.white.withAlpha(25),
            borderRadius: BorderRadius.circular(20),
            border: Border.all(color: Colors.white.withAlpha(51)),
          ),
          padding: const EdgeInsets.all(20),
          child: child,
        ),
      ),
    );
  }
}

class DashboardScreen extends ConsumerStatefulWidget {
  const DashboardScreen({super.key});

  @override
  ConsumerState<DashboardScreen> createState() => _DashboardScreenState();
}

class _DashboardScreenState extends ConsumerState<DashboardScreen> {
  int _currentStep = -1;

  void _startTour() {
    setState(() {
      _currentStep = 0;
    });
  }

  void _nextStep() {
    setState(() {
      _currentStep++;
    });
  }

  void _skipTour() {
    setState(() {
      _currentStep = -1;
    });
  }

  @override
  Widget build(BuildContext context) {
    final pendingCount = ref.watch(actionCenterProvider).actions.length;

    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      drawer: Drawer(
        backgroundColor: const Color(0xFF1E293B),
        child: ListView(
          padding: EdgeInsets.zero,
          children: [
            const DrawerHeader(
              decoration: BoxDecoration(color: Color(0xFF6B4EFF)),
              child: Text('Menu', style: TextStyle(color: Colors.white, fontSize: 24)),
            ),
            ListTile(
              leading: const Icon(Icons.tour, color: Colors.white),
              title: const Text('App Tour', style: TextStyle(color: Colors.white)),
              onTap: () {
                Navigator.pop(context); // Close drawer
                _startTour();
              },
            ),
            ListTile(
              leading: const Icon(Icons.video_library, color: Colors.white),
              title: const Text('Video Tutorials', style: TextStyle(color: Colors.white)),
              onTap: () {
                Navigator.pop(context); // Close drawer
                Navigator.push(context, MaterialPageRoute(builder: (context) => const VideoTutorialsScreen()));
              },
            ),
          ],
        ),
      ),
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
        actions: [
IconButton(
            icon: const Icon(Icons.help_outline, color: Colors.white),
            onPressed: () {
              Navigator.push(context, MaterialPageRoute(builder: (context) => const HelpCenterScreen()));
            },
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () {
          Navigator.push(context, MaterialPageRoute(builder: (context) => const AiHelpChatScreen()));
        },
        backgroundColor: const Color(0xFF6B4EFF),
        icon: const Icon(Icons.support_agent, color: Colors.white),
        label: const Text('Ask Anything', style: TextStyle(color: Colors.white)),
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 600),
          child: Padding(
            padding: const EdgeInsets.all(20),
            child: ListView(

              children: [
                const SizedBox(height: 20),
                const Text(
                  "Dashboard",
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 28,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                ),
                const SizedBox(height: 20),
                const MilestoneNotification(
                  title: '🎉 You just got your 10th order!',
                  message: 'Keep up the great work!',
                ),
                const SizedBox(height: 20),
                WalkthroughHighlight(
                  showHighlight: _currentStep == 0,
                  speechBubbleText: "This is your total revenue. It updates automatically when you make a sale!",
                  onDismiss: _nextStep,
                  child: GlassContainer(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: const [
                        Text(
                          'Revenue',
                          style: TextStyle(fontSize: 14, color: Colors.white70),
                        ),
                        SizedBox(height: 5),
                        Text(
                          '\$0.00',
                          style: TextStyle(fontSize: 32, fontWeight: FontWeight.bold, color: Colors.white),
                        ),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: 20),
                WalkthroughHighlight(
                  showHighlight: _currentStep == 1,
                  speechBubbleText: "Check this list to see what you should do next to grow your business.",
                  onDismiss: _nextStep,
                  child: GlassContainer(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text(
                        "Welcome Checklist",
                        style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold, color: Colors.white),
                      ),
                      const SizedBox(height: 10),
                      const Row(
                        children: [
                          Icon(Icons.check_circle, color: Color(0xFF22C55E), size: 18),
                          SizedBox(width: 8),
                          Text("Business live", style: TextStyle(color: Colors.white)),
                        ],
                      ),
                      const SizedBox(height: 5),
                      InkWell(
                        onTap: () { /* Add product flow */ },
                        child: const Row(
                          children: [
                            Icon(Icons.radio_button_unchecked, color: Colors.white70, size: 18),
                            SizedBox(width: 8),
                            Text("Add 3 more products", style: TextStyle(color: Colors.white, decoration: TextDecoration.underline)),
                          ],
                        ),
                      ),
                      const SizedBox(height: 5),
                      InkWell(
                        onTap: () { /* Connect Instagram flow */ },
                        child: const Row(
                          children: [
                            Icon(Icons.radio_button_unchecked, color: Colors.white70, size: 18),
                            SizedBox(width: 8),
                            Text("Connect Instagram", style: TextStyle(color: Colors.white, decoration: TextDecoration.underline)),
                          ],
                        ),
                      ),
                      const SizedBox(height: 5),
                      InkWell(
                        onTap: () {
                          Navigator.push(
                            context,
                            MaterialPageRoute(builder: (context) => const ReferralProgramScreen()),
                          );
                        },
                        child: const Row(
                          children: [
                            Icon(Icons.radio_button_unchecked, color: Colors.white70, size: 18),
                            SizedBox(width: 8),
                            Text("Share your link with a friend", style: TextStyle(color: Colors.white, decoration: TextDecoration.underline)),
                          ],
                        ),
                      ),
                    ],
                  ),
                  ),
                ),
                const SizedBox(height: 20),
                GlassContainer(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text(
                        'Pending Agent Approvals',
                        style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold, color: Colors.white),
                      ),
                      const SizedBox(height: 10),
                      Text(
                        pendingCount > 0
                            ? 'You have $pendingCount pending approval(s).'
                            : 'No pending approvals.',
                        style: const TextStyle(color: Colors.white70),
                      ),
                    ],
                  ),
                ),
                const SizedBox(height: 20),
                GlassContainer(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: const [
                      Text(
                        'Recent Orders',
                        style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold, color: Colors.white),
                      ),
                      SizedBox(height: 10),
                      Text(
                        'No orders yet.',
                        style: TextStyle(color: Colors.white70),
                      ),
                    ],
                  ),
                ),
                const SizedBox(height: 20),
                WalkthroughHighlight(
                  showHighlight: _currentStep == 2,
                  speechBubbleText: "All your customer messages end up here. Reply to them quickly!",
                  onDismiss: _skipTour,
                  child: ElevatedButton(
                    key: const Key('inboxBtn'),
                    onPressed: () {
                      Navigator.push(
                        context,
                        MaterialPageRoute(builder: (context) => const UnifiedInboxScreen()),
                      );
                    },
                    style: ElevatedButton.styleFrom(
                      backgroundColor: const Color(0xFF6B4EFF),
                      padding: const EdgeInsets.symmetric(vertical: 15),
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(10),
                      ),
                    ),
                    child: const Text('Inbox', style: TextStyle(color: Colors.white, fontSize: 16)),
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
