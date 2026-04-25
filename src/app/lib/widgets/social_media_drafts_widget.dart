import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class SocialMediaDraftsWidget extends StatefulWidget {
  const SocialMediaDraftsWidget({super.key});

  @override
  State<SocialMediaDraftsWidget> createState() => _SocialMediaDraftsWidgetState();
}

class _SocialMediaDraftsWidgetState extends State<SocialMediaDraftsWidget> {
  final List<_DraftPost> _drafts = [
    _DraftPost(
      platform: 'Instagram',
      content: 'Excited to announce our new product line! 🚀 Get ready for something amazing. #newarrival #smallbusiness',
      icon: Icons.camera_alt,
      status: 'Draft-for-Review',
    ),
    _DraftPost(
      platform: 'X',
      content: 'Flash sale starting tomorrow! ⚡️ Don\'t miss out on 20% off all items. Link in bio! #sale',
      icon: Icons.share,
      status: 'Draft-for-Review',
    ),
  ];

  @override
  Widget build(BuildContext context) {
    if (_drafts.isEmpty) {
      return const SizedBox.shrink();
    }

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text(
          'Social Media Drafts',
          style: TextStyle(
            fontSize: 20,
            fontWeight: FontWeight.bold,
            fontFamily: 'Outfit',
          ),
        ),
        const SizedBox(height: 16),
        SizedBox(
          height: 260,
          child: ListView.separated(
            scrollDirection: Axis.horizontal,
            itemCount: _drafts.length,
            separatorBuilder: (context, index) => const SizedBox(width: 16),
            itemBuilder: (context, index) {
              final draft = _drafts[index];
              return SizedBox(
                width: 300,
                child: GlassCard(
                  child: Padding(
                    padding: const EdgeInsets.all(16.0),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          children: [
                            Icon(draft.icon, size: 20, color: Theme.of(context).colorScheme.primary),
                            const SizedBox(width: 8),
                            Text(
                              draft.platform,
                              style: const TextStyle(
                                fontWeight: FontWeight.bold,
                                fontFamily: 'Inter',
                              ),
                            ),
                            const Spacer(),
                            Flexible(
                              child: Container(
                                padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                                decoration: BoxDecoration(
                                  color: Theme.of(context).colorScheme.tertiaryContainer,
                                  borderRadius: BorderRadius.circular(12),
                                ),
                                child: Text(
                                  draft.status,
                                  style: TextStyle(
                                    fontSize: 10,
                                    color: Theme.of(context).colorScheme.onTertiaryContainer,
                                    fontFamily: 'Inter',
                                    fontWeight: FontWeight.bold,
                                  ),
                                  overflow: TextOverflow.ellipsis,
                                ),
                              ),
                            ),
                          ],
                        ),
                        const SizedBox(height: 12),
                        Expanded(
                          child: Text(
                            draft.content,
                            style: const TextStyle(
                              fontFamily: 'Inter',
                              fontSize: 14,
                            ),
                            maxLines: 4,
                            overflow: TextOverflow.ellipsis,
                          ),
                        ),
                        const SizedBox(height: 12),
                        Wrap(
                          spacing: 8,
                          alignment: WrapAlignment.end,
                          children: [
                            TextButton.icon(
                              onPressed: () {
                                setState(() {
                                  _drafts.removeAt(index);
                                });
                              },
                              icon: const Icon(Icons.close, size: 18),
                              label: const Text('Reject'),
                              style: TextButton.styleFrom(
                                foregroundColor: Theme.of(context).colorScheme.error,
                              ),
                            ),
                            FilledButton.icon(
                              onPressed: () {
                                setState(() {
                                  _drafts.removeAt(index);
                                });
                                ScaffoldMessenger.of(context).showSnackBar(
                                  const SnackBar(content: Text('Post scheduled!')),
                                );
                              },
                              icon: const Icon(Icons.check, size: 18),
                              label: const Text('Approve'),
                            ),
                          ],
                        ),
                      ],
                    ),
                  ),
                ),
              );
            },
          ),
        ),
      ],
    );
  }
}

class _DraftPost {
  final String platform;
  final String content;
  final IconData icon;
  final String status;

  _DraftPost({
    required this.platform,
    required this.content,
    required this.icon,
    required this.status,
  });
}
