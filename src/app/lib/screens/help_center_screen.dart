import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import '../models/help_article.dart';
import '../services/help_service.dart';

class HelpCenterScreen extends ConsumerStatefulWidget {
  const HelpCenterScreen({super.key});

  @override
  ConsumerState<HelpCenterScreen> createState() => _HelpCenterScreenState();
}

class _HelpCenterScreenState extends ConsumerState<HelpCenterScreen> {
  final TextEditingController _searchController = TextEditingController();
  String _searchQuery = '';

  @override
  void dispose() {
    _searchController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final helpService = ref.watch(helpServiceProvider);
    final isSearching = _searchQuery.isNotEmpty;

    return Scaffold(
      appBar: AppBar(
        title: const Text(
          'Help Center',
          style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold),
        ),
      ),
      body: CustomScrollView(
        slivers: [
          SliverToBoxAdapter(
            child: Padding(
              padding: const EdgeInsets.all(16.0),
              child: TextField(
                controller: _searchController,
                onChanged: (val) {
                  setState(() {
                    _searchQuery = val;
                  });
                },
                decoration: InputDecoration(
                  hintText: 'Search for help...',
                  prefixIcon: const Icon(Icons.search),
                  suffixIcon: _searchQuery.isNotEmpty
                      ? IconButton(
                          icon: const Icon(Icons.clear),
                          onPressed: () {
                            _searchController.clear();
                            setState(() {
                              _searchQuery = '';
                            });
                          },
                        )
                      : null,
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(12),
                  ),
                ),
              ),
            ),
          ),
          if (isSearching)
            _buildSearchResults(helpService)
          else
            _buildTopicsList(helpService),
        ],
      ),
    );
  }

  Widget _buildSearchResults(HelpService helpService) {
    final results = helpService.searchArticles(_searchQuery);

    if (results.isEmpty) {
      return const SliverToBoxAdapter(
        child: Padding(
          padding: EdgeInsets.all(32.0),
          child: Center(
            child: Text(
              'No results found.',
              style: TextStyle(fontFamily: 'Inter', fontSize: 16),
            ),
          ),
        ),
      );
    }

    return SliverList(
      delegate: SliverChildBuilderDelegate(
        (context, index) {
          final article = results[index];
          return ListTile(
            title: Text(article.title, style: const TextStyle(fontWeight: FontWeight.bold, fontFamily: 'Inter')),
            subtitle: Text(article.topic, style: const TextStyle(fontFamily: 'Inter')),
            trailing: const Icon(Icons.chevron_right),
            onTap: () => context.go('/help/${article.id}'),
          );
        },
        childCount: results.length,
      ),
    );
  }

  Widget _buildTopicsList(HelpService helpService) {
    final topics = helpService.getTopics();

    return SliverList(
      delegate: SliverChildBuilderDelegate(
        (context, index) {
          final topic = topics[index];
          final articles = helpService.getArticlesByTopic(topic);

          return Padding(
            padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 16.0),
            child: Card(
              elevation: 0,
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(16),
                side: BorderSide(
                  color: Theme.of(context).colorScheme.outlineVariant,
                ),
              ),
              child: ExpansionTile(
                title: Text(
                  topic,
                  style: const TextStyle(
                    fontFamily: 'Outfit',
                    fontWeight: FontWeight.bold,
                    fontSize: 18,
                  ),
                ),
                children: articles.map((article) {
                  return ListTile(
                    title: Text(
                      article.title,
                      style: const TextStyle(fontFamily: 'Inter'),
                    ),
                    trailing: const Icon(Icons.chevron_right, size: 20),
                    onTap: () => context.go('/help/${article.id}'),
                  );
                }).toList(),
              ),
            ),
          );
        },
        childCount: topics.length,
      ),
    );
  }
}

class HelpArticleScreen extends ConsumerWidget {
  final String articleId;
  const HelpArticleScreen({super.key, required this.articleId});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final helpService = ref.watch(helpServiceProvider);
    final article = helpService.getArticleById(articleId);

    if (article == null) {
      return Scaffold(
        appBar: AppBar(title: const Text('Article not found')),
        body: const Center(child: Text('The requested help article could not be found.')),
      );
    }

    return Scaffold(
      appBar: AppBar(
        title: const Text('Help Center', style: TextStyle(fontFamily: 'Outfit')),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              article.topic,
              style: TextStyle(
                fontFamily: 'Inter',
                color: Theme.of(context).colorScheme.primary,
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 8),
            Text(
              article.title,
              style: const TextStyle(
                fontFamily: 'Outfit',
                fontSize: 28,
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 24),
            Text(
              article.content,
              style: const TextStyle(
                fontFamily: 'Inter',
                fontSize: 16,
                height: 1.5,
              ),
            ),
          ],
        ),
      ),
    );
  }
}
