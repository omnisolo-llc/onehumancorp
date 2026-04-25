import 'package:flutter/material.dart';

class HelpCategory {
  final String id;
  final String title;
  final IconData icon;
  final List<HelpArticle> articles;

  const HelpCategory({
    required this.id,
    required this.title,
    required this.icon,
    this.articles = const [],
  });
}

class HelpArticle {
  final String id;
  final String title;
  final String content;
  final String categoryId;
  final List<String> keywords;

  const HelpArticle({
    required this.id,
    required this.title,
    required this.content,
    required this.categoryId,
    required this.keywords,
  });
}
