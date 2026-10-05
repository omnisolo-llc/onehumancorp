import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { navigateToPublicAuth } from "./publicNavigation";

describe("public authentication document navigation", () => {
  const assign = vi.fn();
  const replace = vi.fn();

  beforeEach(() => {
    assign.mockReset();
    replace.mockReset();
    vi.stubGlobal("window", { location: { assign, replace } });
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it.each(["/login", "/register", "/verify-email"] as const)("loads %s as a new document", (path) => {
    navigateToPublicAuth(path);
    expect(assign).toHaveBeenCalledExactlyOnceWith(path);
    expect(replace).not.toHaveBeenCalled();
  });

  it.each(["/login", "/register", "/verify-email"] as const)("replaces history with the %s document when requested", (path) => {
    navigateToPublicAuth(path, true);
    expect(replace).toHaveBeenCalledExactlyOnceWith(path);
    expect(assign).not.toHaveBeenCalled();
  });
});
