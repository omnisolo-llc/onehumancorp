import { render, screen } from "@testing-library/react";
import { beforeEach, expect, test, vi } from "vitest";
import { PublicAwareApplicationFrame } from "./PublicAwareApplicationFrame";

const navigationMock = vi.hoisted(() => ({ pathname: "/login" }));

vi.mock("next/navigation", () => ({
  usePathname: () => navigationMock.pathname,
}));

vi.mock("./ProductShellGuard", () => ({
  ProductShellGuard: ({ children }: { children: React.ReactNode }) => (
    <section data-testid="product-shell">{children}</section>
  ),
}));

beforeEach(() => {
  navigationMock.pathname = "/login";
});

test.each(["/login", "/register", "/verify-email"])(
  "keeps authenticated-only widgets unmounted on %s",
  (pathname) => {
    navigationMock.pathname = pathname;
    render(
      <PublicAwareApplicationFrame applicationWidgets={<div>Private widget</div>}>
        <div>Public content</div>
      </PublicAwareApplicationFrame>,
    );

    expect(screen.getByText("Public content")).toBeDefined();
    expect(screen.queryByText("Private widget")).toBeNull();
    expect(screen.queryByTestId("product-shell")).toBeNull();
  },
);

test("renders the product shell and authenticated widgets for protected pages", () => {
  navigationMock.pathname = "/dashboard";
  render(
    <PublicAwareApplicationFrame applicationWidgets={<div>Private widget</div>}>
      <div>Protected content</div>
    </PublicAwareApplicationFrame>,
  );

  expect(screen.getByTestId("product-shell")).toBeDefined();
  expect(screen.getByText("Protected content")).toBeDefined();
  expect(screen.getByText("Private widget")).toBeDefined();
});
