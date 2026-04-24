class HelpArticle {
  final String id;
  final String title;
  final String topic;
  final String content;
  final List<String> tags;

  const HelpArticle({
    required this.id,
    required this.title,
    required this.topic,
    required this.content,
    this.tags = const [],
  });

  factory HelpArticle.fromJson(Map<String, dynamic> json) {
    return HelpArticle(
      id: json['id'] as String,
      title: json['title'] as String,
      topic: json['topic'] as String,
      content: json['content'] as String,
      tags: (json['tags'] as List<dynamic>?)?.cast<String>() ?? [],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'title': title,
      'topic': topic,
      'content': content,
      'tags': tags,
    };
  }
}
