class HelpArticle {
  final String title;
  final String description;
  final String category;
  final String content;

  HelpArticle({
    required this.title,
    required this.description,
    required this.category,
    required this.content,
  });
}

class VideoTutorial {
  final String title;
  final String description;
  final String duration;
  final String thumbnailUrl;

  VideoTutorial({
    required this.title,
    required this.description,
    required this.duration,
    required this.thumbnailUrl,
  });
}

class TooltipContent {
  final String text;

  TooltipContent({required this.text});
}

class HelpRegistry {
  static final HelpRegistry _instance = HelpRegistry._internal();

  factory HelpRegistry() {
    return _instance;
  }

  HelpRegistry._internal();

  final Map<String, TooltipContent> _tooltips = {
    'unified_inbox': TooltipContent(text: 'See all your messages from email and social media in one place.'),
    'agent_updates': TooltipContent(text: 'Review and approve actions your AI agents want to take.'),
    'user_management': TooltipContent(text: 'Invite team members to help run your business.'),
    'referral_link': TooltipContent(text: 'Share this link to invite a new team member.'),
    'new_message': TooltipContent(text: 'Draft a new message to a customer.'),
    'reply_box': TooltipContent(text: 'Type your reply to the customer here.'),
  };

  final List<HelpArticle> _articles = [
    HelpArticle(
      category: 'Getting Started',
      title: 'Set up your store in 5 minutes',
      description: 'Follow our simple guide to add your first product and go live.',
      content: '1. Go to My Store. 2. Tap Add Product. 3. Enter details and tap Save.',
    ),
    HelpArticle(
      category: 'My Store',
      title: 'How to add products',
      description: 'Learn how to list new items, add photos, and set prices.',
      content: 'Take clear photos of your item. Add a title, price, and short description.',
    ),
    HelpArticle(
      category: 'Payments & Billing',
      title: 'How to accept Apple Pay',
      description: 'Enable Apple Pay with one click in your payment settings.',
      content: 'Go to Settings > Payments. Toggle Apple Pay to On. You are all set!',
    ),
    HelpArticle(
      category: 'AI Agents',
      title: 'What can the Customer Success Helper do?',
      description: 'Your helper can reply to customer emails and Instagram DMs automatically.',
      content: 'The Customer Success Helper reads incoming messages and suggests polite, accurate replies based on your store policies.',
    ),
    HelpArticle(
      category: 'Marketing',
      title: 'How to run a promotion',
      description: 'Learn how to create discount codes and share them on social media.',
      content: 'Go to Marketing. Tap New Promotion. Choose a percentage off and share the code.',
    ),
    HelpArticle(
      category: 'Account & Billing',
      title: 'How to change your subscription',
      description: 'Find out how to upgrade or downgrade your plan and view past invoices.',
      content: 'Go to Account > Billing. Choose your new plan and confirm.',
    ),
  ];

  final List<VideoTutorial> _videos = [
    VideoTutorial(
      title: 'Welcome to OHC',
      description: 'A quick tour of your new business dashboard.',
      duration: '1:20',
      thumbnailUrl: 'https://via.placeholder.com/150/000000/FFFFFF/?text=Welcome',
    ),
    VideoTutorial(
      title: 'Adding Products',
      description: 'How to take great photos and write descriptions.',
      duration: '2:15',
      thumbnailUrl: 'https://via.placeholder.com/150/000000/FFFFFF/?text=Products',
    ),
    VideoTutorial(
      title: 'Setting up Payments',
      description: 'Get paid faster with Apple Pay and credit cards.',
      duration: '1:45',
      thumbnailUrl: 'https://via.placeholder.com/150/000000/FFFFFF/?text=Payments',
    ),
    VideoTutorial(
      title: 'Using AI Helpers',
      description: 'Let AI answer customer questions for you.',
      duration: '3:00',
      thumbnailUrl: 'https://via.placeholder.com/150/000000/FFFFFF/?text=AI+Helpers',
    ),
  ];

  String? getTooltip(String id) {
    return _tooltips[id]?.text;
  }

  List<HelpArticle> get articles => _articles;
  List<VideoTutorial> get videos => _videos;
}
