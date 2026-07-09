"use client";

import { usePathname } from "next/navigation";
import { AppShell } from "./AppShell";

const shellRoutes: Record<string, { title: string; subtitle?: string }> = {
  "/agents": {
    title: "Agents",
    subtitle: "Manage expert teams, workflows, and assistant capabilities.",
  },
  "/calendar": {
    title: "Calendar",
    subtitle: "Manage schedule, bookings, and upcoming work.",
  },
  "/langgraph": {
    title: "LangGraph",
    subtitle: "Explicit state graph workflows.",
  },
  "/visual-workflow": {
    title: "Visual Workflow",
    subtitle: "Block-based visual workflow construction.",
  },
  "/agent-protocol": {
    title: "Agent Protocol",
    subtitle: "Standardized Agent Protocol interactions.",
  }
};

const routesWithOwnShell = new Set([
  "/agent-protocol",
  "/action-center",
  "/agents",
  "/ai-usage-paywall",
  "/assistant",
  "/business-analytics",
  "/cost-dashboard",
  "/dashboard",
  "/diagnostics",
  "/embed-builder",
  "/exit-intent-builder",
  "/feed",
  "/finance",
  "/inbox",
  "/integrations",
  "/inventory",
  "/kairos",
  "/lead-magnet-generator",
  "/orders",
  "/pipeline",
  "/products",
  "/scaling",
  "/services",
  "/settings",
  "/triage",
  "/visual-workflow",
]);

const standaloneRoutes = new Set([
  "/",
  "/booking-widget",
  "/checkout",
  "/hybrid-landing",
  "/leave-review",
  "/login",
  "/pricing",
  "/referrals",
  "/share-and-save-widget",
  "/share-card",
  "/share-cards",
  "/storefront-widget",
  "/testimonial-widget",
  "/unlock",
  "/waitlist",
  "/work-intake-widget",
  "/onboarding",
]);

const titleOverrides: Record<string, string> = {
  "/abandoned-cart": "Abandoned Cart",
  "/actor-model": "Actor Model",
  "/affiliate-badge-builder": "Affiliate Badge Builder",
  "/agent-marketplace": "Agent Marketplace",
  "/agent-protocol": "Agent Protocol",
  "/analytics": "Analytics",
  "/api-docs": "API Docs",
  "/booking": "Booking",
  "/brand-studio": "Brand Studio",
  "/builder": "Builder",
  "/onboarding": "Setup",
  "/cart-recovery": "Cart Recovery",
  "/changelog": "Changelog",
  "/chaos-report": "Chaos Report",
  "/compliance-feed": "Compliance Feed",
  "/customer-referral-program": "Customer Referrals",
  "/email-signature-generator": "Email Signature Generator",
  "/expert-team": "Expert Team",
  "/flash-sale-generator": "Flash Sale Generator",
  "/fulfillment-hub": "Fulfillment Hub",
  "/gift-cards": "Gift Cards",
  "/giveaway": "Giveaway",
  "/group-buy-widget": "Group Buy Widget",
  "/help": "Help",
  "/incidents": "Incidents",
  "/invoice-generator": "Invoice Generator",
  "/link-in-bio-generator": "Link In Bio Generator",
  "/loyalty-program": "Loyalty Program",
  "/merch": "Merch",
  "/milestones": "Milestones",
  "/nova-mission-track": "Nova Mission Track",
  "/promoter": "Promoter",
  "/proposal-generator": "Proposal Generator",
  "/quoting": "Quoting",
  "/anthropic-guardrails": "Anthropic Guardrails",
  "/ralph-loop": "Ralph Loop",
  "/referrals": "Referrals",
  "/review-campaigns": "Review Campaigns",
  "/scribe-mission-track": "Scribe Mission Track",
  "/seasonal-promo": "Seasonal Promo",
  "/smart-pricing": "Smart Pricing",
  "/social-proof-nudge": "Social Proof Nudge",
  "/sona": "Sona",
  "/store-wrap": "Store Wrap",
  "/storefront-builder": "Storefront Builder",
  "/subscriptions": "Subscriptions",
  "/team": "Team",
  "/trial-extension": "Trial Extension",
  "/upgrade-roi": "Upgrade ROI",
  "/verification-loops": "Verification Loops",
  "/visual-workflow": "Visual Workflow",
  "/pydantic-validation": "Pydantic Tool Schema Validation",
  "/website-builder": "Website Builder",
  "/whatsapp-link-generator": "WhatsApp Link Generator",
  "/win-back": "Win Back",
  "/wrapped": "Wrapped",
};

function matchesRoute(pathname: string, route: string) {
  return pathname === route || pathname.startsWith(`${route}/`);
}

function titleFromPath(pathname: string) {
  const route = `/${pathname.split("/").filter(Boolean)[0] || ""}`;
  if (titleOverrides[route]) return titleOverrides[route];

  return route
    .slice(1)
    .split("-")
    .filter(Boolean)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(" ") || "Dashboard";
}

function routeConfig(pathname: string | null) {
  if (!pathname) return null;
  if ([...routesWithOwnShell].some((route) => matchesRoute(pathname, route))) return null;
  if ([...standaloneRoutes].some((route) => matchesRoute(pathname, route))) return null;

  const route = Object.keys(shellRoutes)
    .sort((a, b) => b.length - a.length)
    .find((prefix) => pathname === prefix || pathname.startsWith(`${prefix}/`));
  return route
    ? shellRoutes[route]
    : {
      title: titleFromPath(pathname),
      subtitle: "Use this workspace from the dashboard navigation.",
    };
}

export function ProductShellGuard({ children }: { children: React.ReactNode }) {
  const pathname = usePathname();
  const config = routeConfig(pathname);

  if (!config) return <>{children}</>;

  return (
    <AppShell title={config.title} subtitle={config.subtitle}>
      {children}
    </AppShell>
  );
}
