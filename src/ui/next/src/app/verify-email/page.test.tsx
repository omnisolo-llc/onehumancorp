import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import VerifyEmailPage from "./page";

const replace = vi.fn();
vi.mock("next/navigation", () => ({ useRouter: () => ({ replace }) }));

describe("email verification", () => {
  beforeEach(() => {
    replace.mockReset();
    sessionStorage.clear();
    sessionStorage.setItem(
      "ohc-registration-challenge",
      JSON.stringify({ challengeId: "challenge-7", email: "alice@example.test" }),
    );
    vi.mocked(fetch).mockReset();
  });

  it("does not expose account credentials until the email code succeeds", async () => {
    vi.mocked(fetch).mockResolvedValueOnce(Response.json({
      registration_ticket: "ticket-7",
      expires_in_seconds: 1200,
    }));
    render(<VerifyEmailPage />);
    const user = userEvent.setup();

    expect(await screen.findByLabelText(/verification code/i)).toBeDefined();
    expect(screen.queryByLabelText(/username/i)).toBeNull();
    expect(screen.queryByLabelText(/^password$/i)).toBeNull();
    await user.type(screen.getByLabelText(/verification code/i), "123456");
    await user.click(screen.getByRole("button", { name: /verify email/i }));

    expect(await screen.findByText(/email verified/i)).toBeDefined();
    expect(screen.getByLabelText(/username/i)).toBeDefined();
    expect(screen.getByLabelText(/workspace id/i)).toBeDefined();
    expect(screen.getByLabelText(/^password$/i)).toBeDefined();
  });

  it("creates a sealed session only after submitting the verified ticket", async () => {
    sessionStorage.setItem("ohc-registration-ticket", "ticket-7");
    vi.mocked(fetch)
      .mockResolvedValueOnce(Response.json({ registration_ticket: "ticket-7", expires_in_seconds: 1200 }))
      .mockResolvedValueOnce(Response.json({ user: { id: "user-7" }, next: "/onboarding" }, { status: 201 }));
    render(<VerifyEmailPage />);
    const user = userEvent.setup();
    await user.type(await screen.findByLabelText(/verification code/i), "123456");
    await user.click(screen.getByRole("button", { name: /verify email/i }));
    await user.type(await screen.findByLabelText(/username/i), "alice.ops");
    await user.type(screen.getByLabelText(/workspace id/i), "alice-shop");
    await user.type(screen.getByLabelText(/^password$/i), "violet river cabin orbit");
    await user.click(screen.getByRole("button", { name: /create account/i }));

    await waitFor(() => expect(replace).toHaveBeenCalledWith("/onboarding"));
    expect(fetch).toHaveBeenLastCalledWith("/api/v1/auth/register?next=%2Fonboarding", expect.objectContaining({
      method: "POST",
      body: JSON.stringify({
        registration_ticket: "ticket-7",
        organization_id: "alice-shop",
        username: "alice.ops",
        password: "violet river cabin orbit",
      }),
    }));
    expect(sessionStorage.getItem("ohc-registration-ticket")).toBeNull();
  });
});
