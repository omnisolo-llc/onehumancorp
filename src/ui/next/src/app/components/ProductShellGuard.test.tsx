import { render, screen } from "@testing-library/react";
import { beforeEach, expect, test, vi } from "vitest";
import { ProductShellGuard } from "./ProductShellGuard";

const navigationMock = vi.hoisted(() => ({
<<<<<<< HEAD
  pathname: "/onboarding",
=======
  pathname: "/business-setup",
>>>>>>> dac923c2 (Fix Vitest environment and onboarding adminName logic (#30375))
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
  navigationMock.pathname = "/onboarding";
=======
  navigationMock.pathname = "/business-setup";
>>>>>>> dac923c2 (Fix Vitest environment and onboarding adminName logic (#30375))
});

test("wraps dashboard workspace routes that do not own an app shell", () => {
  render(
    <ProductShellGuard>
      <div>Workspace content</div>
    </ProductShellGuard>,
  );

  expect(screen.getByTestId("app-shell")).toBeDefined();
<<<<<<< HEAD
  expect(screen.getByRole("heading", { name: "Setup" })).toBeDefined();
=======
  expect(screen.getByRole("heading", { name: "Business Setup" })).toBeDefined();
>>>>>>> dac923c2 (Fix Vitest environment and onboarding adminName logic (#30375))
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
