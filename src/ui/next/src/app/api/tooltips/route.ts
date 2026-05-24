import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json({
    "bio-input-tooltip": "Tell us what you sell and who your customers are. Keep it simple!",
    "generate-btn-tooltip": "Click here to have our AI build your ready-to-launch store.",
    "launch-btn-tooltip": "Make your store live on the internet so customers can visit.",
    "team-activity-tooltip": "See exactly what your AI helpers are doing right now.",
    "referral-tooltip": "Share this link with friends. You earn credits if they sign up!",
    "swarm-online-tooltip": "Your AI helpers are working hard on your tasks right now.",
    "department-card-tooltip": "Click here to see tasks that need your approval.",
    "nav-dashboard-tooltip": "Check your sales, recent orders, and how your store is doing.",
    "nav-agents-tooltip": "See your AI team, give them tasks, or hire new helpers.",
    "nav-setup-tooltip": "Set up your business info, logo, and how you get paid.",
    "credit-tooltip": "Get free credits for premium tools by inviting a friend.",
    "help-btn-tooltip": "Need help? Click here for guides, videos, and to ask our AI.",
    "changelog-nav-tooltip": "See the latest updates and new features we just added.",
    "nav-inbox-tooltip": "View and reply to messages from your customers.",
    "nav-reviews-tooltip": "Manage your customer reviews and run review-gathering campaigns.",
    "nav-social-tooltip": "Create and share beautiful social media cards for your products.",
    "nav-promos-tooltip": "Set up special discounts and seasonal sales for your store.",
    "nav-agents-tooltip": "Hire and manage your AI workforce here.",
    "metric-sales-tooltip": "Total money earned from sales today.",
    "metric-customers-tooltip": "Number of people who have bought from your store.",
    "metric-orders-tooltip": "Orders that need to be packed and shipped."
  });
}
