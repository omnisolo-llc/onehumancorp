import { render, screen } from "@testing-library/react";
import { beforeEach, expect, test, vi } from "vitest";
import { ProductShellGuard } from "./ProductShellGuard";

const navigationMock = vi.hoisted(() => ({
<<<<<<< HEAD
  pathname: "/business-analytics",
=======
  pathname: "/onboarding",
>>>>>>> 5aad3344 (Update prices to /9/9 per requirements)
}));

vi.mock("next/navigation", () => ({
  usePathname: () => navigationMock.pathname,
}));

vi.mock("./AppShell", () => ({
  AppShell: ({ title, subtitle, children }: { title: string; subtitle?: string; children: React.ReactNode }) => (
    <section data-testid="app-shell">
      <h1>{title}</h1>
      {subtitle && <p>{subtitle}</p>}
      {children}
    </section>
  ),
}));

beforeEach(() => {
<<<<<<< HEAD
  navigationMock.pathname = "/business-analytics";
=======
  navigationMock.pathname = "/onboarding";
>>>>>>> 5aad3344 (Update prices to /9/9 per requirements)
});

test("wraps dashboard workspace routes that do not own an app shell", () => {
  render(
    <ProductShellGuard>
      <div>Workspace content</div>
    </ProductShellGuard>,
  );

  expect(screen.getByTestId("app-shell")).toBeDefined();
<<<<<<< HEAD
  expect(screen.getByRole("heading", { name: "Analytics" })).toBeDefined();
=======
  expect(screen.getByRole("heading", { name: "Setup" })).toBeDefined();
>>>>>>> 5aad3344 (Update prices to /9/9 per requirements)
  expect(screen.getByText("Workspace content")).toBeDefined();
});

test("does not double wrap routes that already render AppShell", () => {
  navigationMock.pathname = "/assistant";

  render(
    <ProductShellGuard>
      <div>Assistant content</div>
    </ProductShellGuard>,
  );

  expect(screen.queryByTestId("app-shell")).toBeNull();
  expect(screen.getByText("Assistant content")).toBeDefined();
});

test("keeps standalone public and widget routes outside the dashboard shell", () => {
  navigationMock.pathname = "/work-intake-widget";

  render(
    <ProductShellGuard>
      <div>Widget content</div>
    </ProductShellGuard>,
  );

  expect(screen.queryByTestId("app-shell")).toBeNull();
  expect(screen.getByText("Widget content")).toBeDefined();
});
