export type ShellRoute = {
  owner: "guard" | "page";
  title: string;
  subtitle?: string;
};

const standardSubtitle = "Use this workspace from the dashboard navigation.";

const pageOwnedExactRoutes = new Set([
  "/agent-activity",
  "/ai-usage-paywall",
  "/analytics",
  "/assistant",
  "/business-analytics",
  "/cost-dashboard",
  "/dashboard",
  "/dashboard/campaigns",
  "/diagnostics",
  "/edge-storefront-setup",
  "/embed-builder",
  "/feed",
  "/finance",
  "/inbox",
  "/integrations",
  "/inventory",
  "/kairos",
  "/kitchen",
  "/lead-magnet-generator",
  "/operations",
  "/orders",
  "/pipeline",
  "/products",
  "/scaling",
  "/services",
  "/settings",
  "/staff",
  "/triage",
  "/viral-product-widget",
]);

const guardOwnedExactRoutes = new Set([
  "/dashboard/bookings",
  "/dashboard/daily-work",
  "/dashboard/ledger",
  "/dashboard/receipt",
  "/products/new",
  "/proposals/customer-view",
  "/proposals/new",
  "/services/new",
]);

const pageOwnedDynamicPrefixes = ["/proposals", "/quotes"] as const;

const routeMetadata: Record<string, { title: string; subtitle?: string }> = {
  "/actor-model": { title: "Actor Model" },
  "/affiliate-badge-builder": { title: "Affiliate Badge Builder" },
  "/agent-marketplace": { title: "Agent Marketplace" },
  "/agent-protocol": {
    title: "Agent Protocol",
    subtitle: "Standardized Agent Protocol interactions.",
  },
  "/agents": {
    title: "Agents",
    subtitle: "Manage expert teams, workflows, and assistant capabilities.",
  },
  "/analytics": { title: "Analytics" },
  "/anthropic-guardrails": { title: "Anthropic Guardrails" },
  "/api-docs": { title: "API Docs" },
  "/booking": { title: "Booking" },
  "/brand-studio": { title: "Brand Studio" },
  "/builder": { title: "Builder" },
  "/calendar": {
    title: "Calendar",
    subtitle: "Manage schedule, bookings, and upcoming work.",
  },
  "/cart-recovery": { title: "Cart Recovery" },
  "/changelog": { title: "Changelog" },
  "/chaos-report": { title: "Chaos Report" },
  "/compliance-feed": { title: "Compliance Feed" },
  "/customer-referral-program": { title: "Customer Referrals" },
  "/email-signature-generator": { title: "Email Signature Generator" },
  "/expert-team": { title: "Expert Team" },
  "/flash-sale-generator": { title: "Flash Sale Generator" },
  "/fulfillment-hub": { title: "Fulfillment Hub" },
  "/gift-cards": { title: "Gift Cards" },
  "/giveaway": { title: "Giveaway" },
  "/group-buy-widget": { title: "Group Buy Widget" },
  "/help": { title: "Help" },
  "/incidents": { title: "Incidents" },
  "/invoice-generator": { title: "Invoice Generator" },
  "/langgraph": {
    title: "LangGraph",
    subtitle: "Explicit state graph workflows.",
  },
  "/link-in-bio-generator": { title: "Link In Bio Generator" },
  "/login": {
    title: "Login",
    subtitle: "Access your business workspace.",
  },
  "/loyalty-program": { title: "Loyalty Program" },
  "/merch": { title: "Merch" },
  "/milestones": { title: "Milestones" },
  "/nova-mission-track": { title: "Nova Mission Track" },
  "/onboarding": {
    title: "Setup",
    subtitle: "Configure your business workspace.",
  },
  "/promoter": { title: "Promoter" },
  "/proposal-generator": { title: "Proposal Generator" },
  "/pydantic-validation": { title: "Pydantic Tool Schema Validation" },
  "/quoting": { title: "Quoting" },
  "/ralph-loop": { title: "Ralph Loop" },
  "/referrals": { title: "Referrals" },
  "/review-campaigns": { title: "Review Campaigns" },
  "/scribe-mission-track": { title: "Scribe Mission Track" },
  "/seasonal-promo": { title: "Seasonal Promo" },
  "/smart-pricing": { title: "Smart Pricing" },
  "/social-proof-nudge": { title: "Social Proof Nudge" },
  "/sona": { title: "Sona" },
  "/store-wrap": { title: "Store Wrap" },
  "/storefront-builder": { title: "Storefront Builder" },
  "/subscriptions": { title: "Subscriptions" },
  "/team": { title: "Team" },
  "/trial-extension": { title: "Trial Extension" },
  "/upgrade-roi": { title: "Upgrade ROI" },
  "/verification-loops": { title: "Verification Loops" },
  "/visual-workflow": {
    title: "Visual Workflow",
    subtitle: "Block-based visual workflow construction.",
  },
  "/website-builder": { title: "Website Builder" },
  "/whatsapp-link-generator": { title: "WhatsApp Link Generator" },
  "/win-back": { title: "Win Back" },
  "/wrapped": { title: "Wrapped" },
};

function matchesPrefix(pathname: string, prefix: string) {
  return pathname === prefix || pathname.startsWith(`${prefix}/`);
}

function matchesSingleSegmentRoute(pathname: string, prefix: string) {
  if (!pathname.startsWith(`${prefix}/`)) return false;

  const remainder = pathname.slice(prefix.length + 1);
  return remainder.length > 0 && !remainder.includes("/");
}

function isPageOwned(pathname: string) {
  if (guardOwnedExactRoutes.has(pathname)) return false;
  if (pageOwnedExactRoutes.has(pathname)) return true;

  return pageOwnedDynamicPrefixes.some((prefix) => matchesSingleSegmentRoute(pathname, prefix));
}

function longestMatchingPrefix(pathname: string, prefixes: readonly string[]) {
  return prefixes
    .filter((prefix) => matchesPrefix(pathname, prefix))
    .sort((left, right) => right.length - left.length)[0];
}

function titleFromPath(pathname: string) {
  const segment = pathname.split("/").filter(Boolean)[0];
  if (!segment) return "Dashboard";

  return segment
    .split("-")
    .filter(Boolean)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(" ");
}

export function resolveShellRoute(pathname: string | null): ShellRoute {
  const safePathname = pathname || "/";
  const metadataPrefix = longestMatchingPrefix(safePathname, Object.keys(routeMetadata));
  const metadata = metadataPrefix ? routeMetadata[metadataPrefix] : undefined;

  return {
    owner: isPageOwned(safePathname) ? "page" : "guard",
    title: metadata?.title ?? titleFromPath(safePathname),
    subtitle: metadata?.subtitle ?? standardSubtitle,
  };
}
