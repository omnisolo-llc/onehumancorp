import { NextResponse } from 'next/server';

export async function GET() {
  const tooltips = {
    // Navigation
    "nav-home": "Go back to your main dashboard to see your sales at a glance.",
    "nav-store": "Change how your store looks and what you sell.",
    "nav-payments": "See how much money you've made and connect your bank account.",
    "nav-agents": "Meet your AI team and give them new tasks to do.",
    "nav-marketing": "Tell more people about your store and find new customers.",
    "nav-billing": "See your bills and change your monthly plan.",

    // Onboarding & Builder
    "bio-input-tooltip": "Tell us about your business in plain words. The AI will use this to build your store.",
    "generate-btn-tooltip": "Click here to let our AI agents build your store for you automatically.",
    "launch-btn-tooltip": "Ready to go? Click this to show your store to the whole world.",
    "vibe-selector-tooltip": "Pick a style that matches your brand's personality.",

    // Dashboard & Team
    "team-activity-tooltip": "Watch your AI workers finish tasks in real-time.",
    "swarm-online-tooltip": "This means your AI team is awake and ready to work.",
    "department-card-tooltip": "Click to see what this specific AI team is working on right now.",
    "approval-btn-tooltip": "Check and approve the work your AI agents have done for you.",

    // Marketing & Growth
    "referral-tooltip": "Share this link with other business owners. You both get rewards!",
    "embed-code-tooltip": "Copy this code to put your store on any other website you own.",
    "geo-score-tooltip": "This shows how easy it is for AI search engines like ChatGPT to find your business.",

    // Payments & Billing
    "stripe-connect-tooltip": "Connect your bank account so you can get paid for your sales.",
    "plan-upgrade-tooltip": "Get more AI agents and extra features by moving to a better plan.",
  };

  return NextResponse.json(tooltips);
}
