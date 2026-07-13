"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { WithTooltip } from "../../components/TooltipRegistry";
import { VoiceAssistant } from "../../components/VoiceAssistant";
import { Omnibox } from "./Omnibox";

type StatusItem = {
  label: string;
  value: string;
  tone?: "good" | "warn" | "bad" | "neutral";
};

type ShellAction = {
  label: string;
  href: string;
  primary?: boolean;
  icon?: IconName;
};

type IconName =
  | "activity"
  | "analytics"
  | "assistant"
  | "calendar"
  | "campaigns"
  | "cost"
  | "dashboard"
  | "diagnostics"
  | "inbox"
  | "integrations"
  | "inventory"
  | "orders"
  | "plus"
  | "settings"
  | "setup"
  | "team";

type NavItem = {
  label: string;
  href: string;
  icon: IconName;
};

const primaryNav: NavItem[] = [
  { label: "Dashboard", href: "/dashboard", icon: "dashboard" },
  { label: "Assistant", href: "/assistant", icon: "assistant" },
  { label: "Setup", href: "/onboarding", icon: "setup" },
  { label: "Triage", href: "/triage", icon: "inbox" },
  { label: "Orders", href: "/orders", icon: "orders" },
  { label: "Inbox", href: "/inbox", icon: "inbox" },
  { label: "Inventory", href: "/inventory", icon: "inventory" },
  { label: "Kairos", href: "/kairos", icon: "activity" },
  { label: "AI Departments", href: "/agents", icon: "team" },
  { label: "Analytics", href: "/business-analytics", icon: "analytics" },
  { label: "Campaigns", href: "/dashboard/campaigns", icon: "campaigns" },
  { label: "Lead Magnets", href: "/lead-magnet-generator", icon: "campaigns" },
  { label: "Settings", href: "/settings", icon: "settings" },
  { label: "AI Usage", href: "/ai-usage-paywall", icon: "activity" },
  { label: "What's New", href: "/changelog", icon: "activity" },
];

const secondaryNav: NavItem[] = [
  { label: "Calendar", href: "/calendar", icon: "calendar" },
  { label: "LangGraph", href: "/langgraph", icon: "activity" },
  { label: "Visual Workflow", href: "/visual-workflow", icon: "activity" },
  { label: "Agent Protocol", href: "/agent-protocol", icon: "activity" },
  { label: "Integrations", href: "/integrations", icon: "integrations" },
  { label: "Cost", href: "/cost-dashboard", icon: "cost" },
  { label: "Diagnostics", href: "/diagnostics", icon: "diagnostics" },
  { label: "Help", href: "/help", icon: "activity" },
];

function ShellIcon({ name }: { name: IconName }) {
  const paths: Record<IconName, string[]> = {
    activity: ["M4 12h4l2-7 4 14 2-7h4"],
    analytics: ["M5 19V9", "M12 19V5", "M19 19v-7"],
    assistant: ["M12 2l2.4 7.6 8 1-6.2 5.6 1.8 7.8-7-4.2-7 4.2 1.8-7.8-6.2-5.6 8-1z"],
    calendar: ["M7 3v4", "M17 3v4", "M4 9h16", "M5 5h14v16H5z"],
    campaigns: ["M4 6h10", "M4 12h7", "M4 18h10", "M16 9l4-4", "M20 5v10", "M16 15l4 4"],
    cost: ["M12 3v18", "M17 7.5c-.8-1.1-2.2-1.8-4-1.8-2.3 0-4 1.1-4 2.8 0 4.2 8 1.8 8 6 0 1.7-1.8 2.8-4 2.8-1.9 0-3.5-.7-4.4-1.9"],
    dashboard: ["M4 5h7v7H4z", "M13 5h7v4h-7z", "M13 11h7v8h-7z", "M4 14h7v5H4z"],
    diagnostics: ["M12 9v4", "M12 17h.01", "M10.3 4.7 3.9 16.2A2 2 0 0 0 5.6 19h12.8a2 2 0 0 0 1.7-2.8L13.7 4.7a2 2 0 0 0-3.4 0z"],
    inbox: ["M4 5h16v14H4z", "M4 13h5l2 3h2l2-3h5"],
    integrations: ["M8 7h8", "M8 17h8", "M7 7a3 3 0 1 1-3-3 3 3 0 0 1 3 3z", "M20 17a3 3 0 1 1-3-3 3 3 0 0 1 3 3z"],
    inventory: ["M4 7 12 3l8 4-8 4-8-4z", "M4 12l8 4 8-4", "M4 17l8 4 8-4"],
    orders: ["M7 4h10l2 4v16H5V8l2-4z", "M9 12h6", "M9 16h6"],
    plus: ["M12 5v14", "M5 12h14"],
    settings: ["M12 15.5a3.5 3.5 0 1 0 0-7 3.5 3.5 0 0 0 0 7z", "M19.4 15a1.8 1.8 0 0 0 .36 2l.04.04a2 2 0 1 1-2.83 2.83l-.04-.04a1.8 1.8 0 0 0-2-.36 1.8 1.8 0 0 0-1.1 1.65V21a2 2 0 1 1-4 0v-.06a1.8 1.8 0 0 0-1.1-1.65 1.8 1.8 0 0 0-2 .36l-.04.04a2 2 0 1 1-2.83-2.83l.04-.04a1.8 1.8 0 0 0 .36-2 1.8 1.8 0 0 0-1.65-1.1H3a2 2 0 1 1 0-4h.06a1.8 1.8 0 0 0 1.65-1.1 1.8 1.8 0 0 0-.36-2l-.04-.04A2 2 0 1 1 7.14 3.7l.04.04a1.8 1.8 0 0 0 2 .36 1.8 1.8 0 0 0 1.1-1.65V3a2 2 0 1 1 4 0v.06a1.8 1.8 0 0 0 1.1 1.65 1.8 1.8 0 0 0 2-.36l.04-.04a2 2 0 1 1 2.83 2.83l-.04.04a1.8 1.8 0 0 0-.36 2c.29.67.93 1.1 1.65 1.1H21a2 2 0 1 1 0 4h-.06a1.8 1.8 0 0 0-1.54 1z"],
    setup: ["M4 7h16", "M4 12h10", "M4 17h7", "M17 14v6", "M14 17h6"],
    team: ["M9 11a3 3 0 1 0 0-6 3 3 0 0 0 0 6z", "M17 12a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5z", "M3.5 20a5.5 5.5 0 0 1 11 0", "M14.5 19a4 4 0 0 1 6 0"],
  };

  return (
    <svg className="app-icon" aria-hidden="true" fill="none" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.9" viewBox="0 0 24 24">
      {paths[name].map((d) => <path key={d} d={d} />)}
    </svg>
  );
}

function actionIcon(action: ShellAction): IconName {
  if (action.icon) return action.icon;
  if (action.href.includes("orders")) return "orders";
  if (action.href.includes("inventory")) return "inventory";
  if (action.href.includes("dashboard")) return "dashboard";
  return action.primary ? "plus" : "activity";
}

function NavLink({ item }: { item: NavItem }) {
  const pathname = usePathname();
  const active = pathname === item.href || (pathname || "").startsWith(`${item.href}/`);

  const link = (
    <Link className={`app-nav-link ${active ? "is-active" : ""}`} href={item.href}>
      <span className="app-nav-marker"><ShellIcon name={item.icon} /></span>
      <span>{item.label}</span>
    </Link>
  );

  if (item.href === "/kairos") {
    return (
      <WithTooltip id="kairos-nav-link-tooltip" defaultText="Click here to see what your AI helpers are working on and how they plan.">
        {link}
      </WithTooltip>
    );
  }

  if (item.href === "/dashboard") {
    return <WithTooltip id="dashboard-tooltip" defaultText="View your daily sales and overall business health.">{link}</WithTooltip>;
  }

  if (item.href === "/changelog") {
    return <WithTooltip id="changelog-nav-tooltip" defaultText="See what's new in the latest updates.">{link}</WithTooltip>;
  }

  if (item.href === "/inventory") {
    return <WithTooltip id="inventory-tooltip" defaultText="Manage your inventory, prices, and stock levels.">{link}</WithTooltip>;
  }

  if (item.href === "/orders") {
    return <WithTooltip id="orders-tooltip" defaultText="See what customers bought and track order fulfillment.">{link}</WithTooltip>;
  }

  if (item.href === "/help") {
    return <WithTooltip id="help-nav-tooltip" defaultText="Help Center">{link}</WithTooltip>;
  }

  return link;
}

export function AppShell({
  title,
  subtitle,
  children,
  actions = [],
  statusItems = [],
}: {
  title: string;
  subtitle?: string;
  children: React.ReactNode;
  actions?: ShellAction[];
  statusItems?: StatusItem[];
}) {
  return (
    <div className="app-shell">
      <aside className="app-sidebar">
        <div className="app-brand">
          <div className="app-brand-mark">O</div>
          <div>
            <div className="app-brand-title">OHC Network</div>
            <div className="app-brand-subtitle">Application</div>
          </div>
        </div>

        <nav className="app-nav" aria-label="Primary">
          {primaryNav.map((item) => <NavLink key={item.href} item={item} />)}
        </nav>

        <div className="app-nav-section">System</div>
        <nav className="app-nav" aria-label="System">
          {secondaryNav.map((item) => <NavLink key={item.href} item={item} />)}
        </nav>
      </aside>

      <div className="app-main">
        <header className="app-topbar">
          <div className="min-w-0">
            <div className="app-breadcrumb">Site: default</div>
            <h1 id="dashboard-title" className="app-title">{title}</h1>
            {subtitle && <p className="app-subtitle">{subtitle}</p>}
          </div>
          <div className="app-topbar-right">
            <div className="app-status-strip">
              {statusItems.map((item) => (
                <div key={item.label} className="app-status-item">
                  <span className={`app-dot ${item.tone || "neutral"}`} />
                  <span className="app-status-label">{item.label}</span>
                  <span className="app-status-value">{item.value}</span>
                </div>
              ))}
            </div>
            {actions.map((action) => (
              <Link
                key={action.href}
                href={action.href}
                className={action.primary ? "app-button primary min-h-[44px]" : "app-button min-h-[44px]"}
              >
                <ShellIcon name={actionIcon(action)} />
                {action.label}
              </Link>
            ))}
            <WithTooltip id="help-btn-tooltip-appshell" defaultText="Need help? Click here to access our Help Center and tutorials.">
              <Link href="/help" className="app-button min-h-[44px] flex items-center justify-center aspect-square rounded-full px-3 hover:bg-black/10 dark:hover:bg-white/20 backdrop-blur-[30px] saturate-[210%] bg-white/60 dark:bg-black/40 border border-white/40 dark:border-white/10 shadow-sm transition-all" aria-label="Help Center">
                <span style={{ fontWeight: 'bold', fontSize: '1.2rem' }}>?</span>
              </Link>
            </WithTooltip>
            <VoiceAssistant />
          </div>
        </header>
        <main className="app-page">{children}
        </main>
        <Omnibox />
      </div>
    </div>
  );
}
