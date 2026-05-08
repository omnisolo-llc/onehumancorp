import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:share_plus/share_plus.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../main.dart'; // For GlassContainer
import '../providers/referral_provider.dart';

class ReferralProgramScreen extends ConsumerWidget {
  const ReferralProgramScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final referralState = ref.watch(referralProvider);

    return Scaffold(
      backgroundColor: const Color(0xFF0F172A),
      appBar: AppBar(
        title: const Text('Refer a Friend', style: TextStyle(fontFamily: 'Outfit', color: Colors.white)),
        backgroundColor: Colors.transparent,
        elevation: 0,
        iconTheme: const IconThemeData(color: Colors.white),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(20),
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 600),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                const Icon(Icons.card_giftcard, size: 64, color: Color(0xFF6B4EFF)),
                const SizedBox(height: 20),
                const Text(
                  'Share OHC, get 1 month free Pro!',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 28,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                  textAlign: TextAlign.center,
                ),
                const SizedBox(height: 10),
                const Text(
                  'When your friend launches their business with OHC, you both get 1 month of Pro tier for free.',
                  style: TextStyle(fontSize: 16, color: Colors.white70),
                  textAlign: TextAlign.center,
                ),
                const SizedBox(height: 30),
                GlassContainer(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text(
                        'Your Invite Link',
                        style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold, color: Colors.white),
                      ),
                      const SizedBox(height: 15),
                      Container(
                        padding: const EdgeInsets.symmetric(horizontal: 15, vertical: 12),
                        decoration: BoxDecoration(
                          color: Colors.black.withAlpha(50),
                          borderRadius: BorderRadius.circular(8),
                          border: Border.all(color: Colors.white.withAlpha(20)),
                        ),
                        child: Row(
                          children: [
                            Expanded(
                              child: Text(
                                referralState.inviteLink,
                                style: const TextStyle(color: Colors.white, fontFamily: 'monospace'),
                              ),
                            ),
                            IconButton(
                              icon: const Icon(Icons.copy, color: Colors.white),
                              onPressed: () {
                                Clipboard.setData(ClipboardData(text: referralState.inviteLink));
                                ScaffoldMessenger.of(context).showSnackBar(
                                  const SnackBar(content: Text('Link copied to clipboard')),
                                );
                              },
                              tooltip: 'Copy Link',
                            ),
                          ],
                        ),
                      ),
                      const SizedBox(height: 15),
                      SizedBox(
                        width: double.infinity,
                        child: ElevatedButton.icon(
                          onPressed: () {
                            Share.share(
                              'Launch your business with OHC! Use my invite link to get 1 month of Pro tier for free: ${referralState.inviteLink}',
                              subject: 'You\'re invited to OHC!',
                            );
                          },
                          icon: const Icon(Icons.share, color: Colors.white),
                          label: const Text('Share Link', style: TextStyle(color: Colors.white, fontSize: 16)),
                          style: ElevatedButton.styleFrom(
                            backgroundColor: const Color(0xFF6B4EFF),
                            padding: const EdgeInsets.symmetric(vertical: 15),
                            shape: RoundedRectangleBorder(
                              borderRadius: BorderRadius.circular(10),
                            ),
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
                const SizedBox(height: 30),
                const Text('Your Invites', style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white)),
                const SizedBox(height: 10),
                ...referralState.invites.map((invite) => _buildInviteCard(
                  invite.email,
                  invite.status,
                  invite.status == 'ACCEPTED' ? Colors.green : Colors.orange,
                )),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildInviteCard(String email, String status, Color statusColor) {
    return Card(
      color: Colors.white.withAlpha(15),
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: ListTile(
        leading: const Icon(Icons.person, color: Colors.white70),
        title: Text(email, style: const TextStyle(fontWeight: FontWeight.bold, color: Colors.white)),
        trailing: Container(
          padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 5),
          decoration: BoxDecoration(
            color: statusColor.withAlpha(30),
            borderRadius: BorderRadius.circular(20),
            border: Border.all(color: statusColor.withAlpha(100)),
          ),
          child: Text(
            status,
            style: TextStyle(color: statusColor, fontSize: 12, fontWeight: FontWeight.bold),
          ),
        ),
      ),
    );
  }
}
