import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:go_router/go_router.dart';

final helpArticlesProvider = FutureProvider.family<List<dynamic>, String>((ref, query) async {
  final api = ref.watch(apiServiceProvider);
  if (api == null) return [];
  return api.listHelpArticles(query: query);
});

class HelpCenterScreen extends ConsumerStatefulWidget {
  final String? articleId;
  const HelpCenterScreen({super.key, this.articleId});

  @override
  ConsumerState<HelpCenterScreen> createState() => _HelpCenterScreenState();
}

class _HelpCenterScreenState extends ConsumerState<HelpCenterScreen> with SingleTickerProviderStateMixin {
  final TextEditingController _searchController = TextEditingController();
  String _query = '';
  late TabController _tabController;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 2, vsync: this);
    if (widget.articleId != null) {
      // In a real app, we might need to search or filter for this specific ID
    }
  }

  @override
  void dispose() {
    _tabController.dispose();
    _searchController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final api = ref.watch(apiServiceProvider);
    return Scaffold(
      appBar: AppBar(
        title: const Text('Help Center', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        bottom: TabBar(
          controller: _tabController,
          tabs: const [
            Tab(text: 'Articles', icon: Icon(Icons.article_outlined)),
            Tab(text: 'Video Tutorials', icon: Icon(Icons.play_circle_outline)),
          ],
        ),
      ),
      body: TabBarView(
        controller: _tabController,
        children: [
          _buildArticlesTab(api),
          _buildVideosTab(api),
        ],
      ),
    );
  }

  Widget _buildArticlesTab(ApiService? api) {
    final articlesAsync = ref.watch(helpArticlesProvider(_query));

    return Column(
      children: [
        Padding(
          padding: const EdgeInsets.all(16.0),
          child: TextField(
            controller: _searchController,
            decoration: InputDecoration(
              hintText: 'Search for help...',
              prefixIcon: const Icon(Icons.search),
              border: OutlineInputBorder(borderRadius: BorderRadius.circular(12)),
              suffixIcon:
                  _query.isNotEmpty
                      ? IconButton(
                        icon: const Icon(Icons.clear),
                        onPressed: () {
                          _searchController.clear();
                          setState(() => _query = '');
                        },
                      )
                      : null,
            ),
            onChanged: (value) => setState(() => _query = value),
          ),
        ),
        Expanded(
          child: articlesAsync.when(
            loading: () => const Center(child: CircularProgressIndicator()),
            error: (e, _) => Center(child: Text('Error: $e')),
            data: (articles) {
              if (articles.isEmpty) {
                return const Center(
                  child: Text('No articles found.', style: TextStyle(fontFamily: 'Inter')),
                );
              }
              return ListView.builder(
                padding: const EdgeInsets.all(16),
                itemCount: articles.length + 1, // +1 for footer
                itemBuilder: (context, index) {
                  if (index == articles.length) {
                    return _buildFooter(context);
                  }
                  final article = articles[index];
                  final isTarget = widget.articleId != null && article['id'] == widget.articleId;

                  return AnimatedOpacity(
                    duration: Duration(milliseconds: 400 + (index * 100)),
                    opacity: 1.0,
                    child: Padding(
                      padding: const EdgeInsets.only(bottom: 16.0),
                      child: GlassCard(
                        borderColor: isTarget ? Colors.cyanAccent : null,
                        child: ExpansionTile(
                          initiallyExpanded: isTarget,
                          title: Text(
                            article['title'],
                            style: const TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold),
                          ),
                          subtitle: Text(
                            article['topic'],
                            style: const TextStyle(fontFamily: 'Inter', fontSize: 12),
                          ),
                          children: [
                            Padding(
                              padding: const EdgeInsets.all(16.0),
                              child: Text(article['content'], style: const TextStyle(fontFamily: 'Inter')),
                            ),
                          ],
                        ),
                      ),
                    ),
                  );
                },
              );
            },
          ),
        ),
      ],
    );
  }

  Widget _buildVideosTab(ApiService? api) {
    return FutureBuilder<List<dynamic>>(
      future: api?.listHelpVideos(),
      builder: (context, snapshot) {
        if (snapshot.connectionState == ConnectionState.waiting) {
          return const Center(child: CircularProgressIndicator());
        }
        if (snapshot.hasError) {
          return Center(child: Text('Error: ${snapshot.error}'));
        }
        final videos = snapshot.data ?? [];
        if (videos.isEmpty) {
          return const Center(child: Text('No videos available.', style: TextStyle(fontFamily: 'Inter')));
        }
        return ListView.builder(
          padding: const EdgeInsets.all(16),
          itemCount: videos.length + 1, // +1 for footer
          itemBuilder: (context, index) {
            if (index == videos.length) {
              return _buildFooter(context);
            }
            final video = videos[index];
            return Padding(
              padding: const EdgeInsets.only(bottom: 16.0),
              child: GlassCard(
                child: ListTile(
                  leading: Stack(
                    alignment: Alignment.center,
                    children: [
                      const Icon(Icons.video_library, size: 40),
                      const Icon(Icons.play_arrow, color: Colors.white, size: 24),
                    ],
                  ),
                  title: Text(video['title'], style: const TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
                  subtitle: Text('Duration: ${video['duration']}', style: const TextStyle(fontFamily: 'Inter', fontSize: 12)),
                  onTap: () {
                    showDialog(
                      context: context,
                      builder: (context) => Center(
                        child: Padding(
                          padding: const EdgeInsets.all(32.0),
                          child: GlassCard(
                            child: Column(
                              mainAxisSize: MainAxisSize.min,
                              children: [
                                AspectRatio(
                                  aspectRatio: 9 / 16,
                                  child: Container(
                                    color: Colors.black,
                                    child: const Center(
                                      child: Icon(Icons.play_arrow, size: 64, color: Colors.white54),
                                    ),
                                  ),
                                ),
                                Padding(
                                  padding: const EdgeInsets.all(16.0),
                                  child: Row(
                                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                                    children: [
                                      Text(video['title'], style: const TextStyle(fontWeight: FontWeight.bold)),
                                      TextButton(
                                        onPressed: () => Navigator.pop(context),
                                        child: const Text('Close'),
                                      ),
                                    ],
                                  ),
                                ),
                              ],
                            ),
                          ),
                        ),
                      ),
                    );
                  },
                ),
              ),
            );
          },
        );
      },
    );
  }

  Widget _buildFooter(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 32.0),
      child: Center(
        child: Column(
          children: [
            const Divider(),
            const SizedBox(height: 16),
            const Text('Looking for advanced integration?', style: TextStyle(fontSize: 14, color: Colors.white70)),
            TextButton.icon(
              onPressed: () => context.push('/api-docs'),
              icon: const Icon(Icons.code, size: 18),
              label: const Text('View API Documentation'),
            ),
          ],
        ),
      ),
    );
  }
}
