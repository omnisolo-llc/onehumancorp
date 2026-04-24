

class VideoTutorial {
  final String id;
  final String title;
  final String duration;
  final String description;
  final String url;
  final String thumbnail;

  VideoTutorial({
    required this.id,
    required this.title,
    required this.duration,
    required this.description,
    required this.url,
    required this.thumbnail,
  });

  factory VideoTutorial.fromJson(Map<String, dynamic> json) {
    return VideoTutorial(
      id: json['id'] as String,
      title: json['title'] as String,
      duration: json['duration'] as String,
      description: json['description'] as String,
      url: json['url'] as String,
      thumbnail: json['thumbnail'] as String,
    );
  }
}
