import { describe, expect, it } from "vitest";
import { deleteStateCookie, serializeStateCookie } from "./oidcFlow";

describe("OIDC state cookie policy", () => {
  it("sets an exact browser-compliant __Host- state cookie", () => {
    expect(serializeStateCookie("__Host-ohc_oidc_state", "sealed.state", true)).toBe(
      "__Host-ohc_oidc_state=sealed.state; Path=/; Max-Age=600; HttpOnly; Secure; SameSite=Lax",
    );
  });

  it("clears the __Host- state cookie with the same root path and no Domain", () => {
    expect(deleteStateCookie("__Host-ohc_oidc_state", true)).toBe(
      "__Host-ohc_oidc_state=; Path=/; Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; Secure; SameSite=Lax",
    );
  });
});
