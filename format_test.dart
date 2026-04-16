void main() {
  String formatRole(String role) {
    if (role.isEmpty) return role;
    return role
        .replaceAll('_', ' ')
        .toLowerCase()
        .split(' ')
        .map((word) {
          if (word == 'ai') return 'AI';
          if (word == 'ceo') return 'CEO';
          if (word == 'qa') return 'QA';
          if (word == 'cfo') return 'CFO';
          if (word == 'seo') return 'SEO';
          if (word == 'llm') return 'LLM';
          if (word.isEmpty) return word;
          return word[0].toUpperCase() + word.substring(1);
        })
        .join(' ');
  }
  print(formatRole("AI_NEWS_COLLECTOR"));
}
