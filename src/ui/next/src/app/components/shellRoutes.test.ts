import { describe, expect, test } from "vitest";
import { resolveShellRoute } from "./shellRoutes";

describe("resolveShellRoute", () => {
  test.each([
    "/login",
    "/onboarding",
    "/booking-widget",
    "/storefront-widget",
    "/website-builder",
  ])("assigns formerly standalone route %s to the guard", (pathname) => {
    expect(resolveShellRoute(pathname).owner).toBe("guard");
  });

  test.each([
    "/agent-activity",
    "/ai-usage-paywall",
    "/analytics",
    "/assistant",
    "/business-analytics",
    "/cost-dashboard",
    "/dashboard",
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
  ])("assigns exact shell-owning route %s to the page", (pathname) => {
    expect(resolveShellRoute(pathname).owner).toBe("page");
  });

  test.each(["/dashboard/campaigns", "/proposals/example", "/quotes/example"])(
    "uses page ownership for shell-owning descendant %s",
    (pathname) => {
      expect(resolveShellRoute(pathname).owner).toBe("page");
    },
  );

  test.each([
    ["/dashboard/bookings", "Dashboard"],
    ["/dashboard/daily-work", "Dashboard"],
    ["/dashboard/ledger", "Dashboard"],
    ["/dashboard/receipt", "Dashboard"],
    ["/orders/example-id", "Orders"],
    ["/products/new", "Products"],
    ["/proposals/new", "Proposals"],
    ["/proposals/customer-view", "Proposals"],
    ["/services/new", "Services"],
  ])("assigns shell-less descendant %s to the guard", (pathname, title) => {
    expect(resolveShellRoute(pathname)).toEqual({
      owner: "guard",
      title,
      subtitle: "Use this workspace from the dashboard navigation.",
    });
  });

  test.each(["/agents", "/visual-workflow"])(
    "assigns shell-less workspace route %s to the guard",
    (pathname) => {
      expect(resolveShellRoute(pathname).owner).toBe("guard");
    },
  );

  test("derives metadata for unknown routes", () => {
    expect(resolveShellRoute("/new-feature")).toEqual({
      owner: "guard",
      title: "New Feature",
      subtitle: "Use this workspace from the dashboard navigation.",
    });
  });

  test.each([
    ["/login", "Login", "Access your business workspace."],
    ["/onboarding", "Setup", "Configure your business workspace."],
  ])("uses explicit metadata for %s", (pathname, title, subtitle) => {
    expect(resolveShellRoute(pathname)).toEqual({
      owner: "guard",
      title,
      subtitle,
    });
  });

  test("does not assign page ownership to a near-prefix route", () => {
    expect(resolveShellRoute("/dashboarding")).toEqual({
      owner: "guard",
      title: "Dashboarding",
      subtitle: "Use this workspace from the dashboard navigation.",
    });
  });

  test("safely resolves a null pathname to the dashboard shell", () => {
    expect(resolveShellRoute(null)).toEqual({
      owner: "guard",
      title: "Dashboard",
      subtitle: "Use this workspace from the dashboard navigation.",
    });
  });
});
