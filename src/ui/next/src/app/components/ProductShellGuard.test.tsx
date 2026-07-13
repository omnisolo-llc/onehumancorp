import { render, screen } from "@testing-library/react";
import { beforeEach, expect, test, vi } from "vitest";
import { ProductShellGuard } from "./ProductShellGuard";

const navigationMock = vi.hoisted(() => ({
  pathname: "/calendar",
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
  navigationMock.pathname = "/calendar";
});

test("wraps dashboard workspace routes that do not own an app shell", () => {
  render(
    <ProductShellGuard>
      <div>Workspace content</div>
    </ProductShellGuard>,
  );

  expect(screen.getByTestId("app-shell")).toBeDefined();
  expect(screen.getByRole("heading", { name: "Calendar" })).toBeDefined();
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

test("wraps formerly standalone widget routes in the dashboard shell", () => {
  navigationMock.pathname = "/work-intake-widget";

  render(
    <ProductShellGuard>
      <div>Widget content</div>
    </ProductShellGuard>,
  );

  expect(screen.getByTestId("app-shell")).toBeDefined();
  expect(screen.getByRole("heading", { name: "Work Intake Widget" })).toBeDefined();
  expect(screen.getByText("Widget content")).toBeDefined();
});
