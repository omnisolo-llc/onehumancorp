class HelpArticle {
  final String id;
  final String title;
  final String category;
  final String content;

  const HelpArticle({
    required this.id,
    required this.title,
    required this.category,
    required this.content,
  });

  factory HelpArticle.fromJson(Map<String, dynamic> json) {
    return HelpArticle(
      id: json['id'] as String,
      title: json['title'] as String,
      category: json['category'] as String,
      content: json['content'] as String,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'title': title,
      'category': category,
      'content': content,
    };
  }
}
