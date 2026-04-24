import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class VideoTutorialList extends StatelessWidget {
  const VideoTutorialList({super.key});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 200,
      child: ListView.builder(
        scrollDirection: Axis.horizontal,
        itemCount: 5,
        itemBuilder: (context, index) {
          final titles = [
            'How to set up your store',
            'Adding your first product',
            'Connecting payments',
            'Hiring AI Agents',
            'Understanding analytics'
          ];
          return Padding(
            padding: const EdgeInsets.only(right: 16),
            child: GlassCard(
              child: SizedBox(
                width: 280,
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    Expanded(
                      child: Container(
                        decoration: BoxDecoration(
                          color: Colors.black26,
                          borderRadius: const BorderRadius.vertical(top: Radius.circular(12)),
                        ),
                        child: const Center(
                          child: Icon(Icons.play_circle_fill, size: 48, color: Colors.white70),
                        ),
                      ),
                    ),
                    Padding(
                      padding: const EdgeInsets.all(12),
                      child: Text(
                        titles[index],
                        style: const TextStyle(fontWeight: FontWeight.bold),
                        maxLines: 2,
                        overflow: TextOverflow.ellipsis,
                      ),
                    ),
                  ],
                ),
              ),
            ),
          );
        },
      ),
    );
  }
}
