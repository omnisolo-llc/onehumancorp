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
    "stripe-setup-tooltip": "Connect your bank account securely with Stripe to start getting paid.",
    "checkout-pay-now-tooltip": "Click here to securely finish your purchase and process your payment.",
    "checkout-tap-to-pay-tooltip": "Tap your card or phone on the reader to pay in person.",
    "checkout-cancel-tooltip": "Go back to the previous screen without buying anything.",
    "kairos-nav-link-tooltip": "Click here to see what your AI helpers are working on and how they plan.",
    "total-sales-tooltip": "Total revenue generated from your sales today.",
    "visitors-tooltip": "Number of unique visitors who viewed your store today.",
    "agents-tab-tooltip": "Hire and manage your AI assistants here.",
    "walkthrough-btn-tooltip": "Start an interactive guide to learn how to use OHC.",
    "dept-operations-tooltip": "Handles the day-to-day execution of orders, bookings, inventory, and deliveries.",
    "dept-marketing-tooltip": "Gets the business noticed. Handles everything from website design to social media to getting found on Google.",
    "dept-sales-tooltip": "Turns interest into revenue. Helps the business owner find and win customers.",
    "dept-customer-success-tooltip": "Keeps customers happy and coming back. Handles all post-sale relationship management.",
    "dept-finance-tooltip": "Makes sure money flows correctly. Handles pricing, payments, and financial visibility.",
    "dept-legal-tooltip": "Keeps the business safe and compliant. Handles contracts, policies, and regulatory requirements.",
    "dept-advisory-tooltip": "Acts as a personal business consultant. Analyzes performance and gives actionable advice."
  });
}
