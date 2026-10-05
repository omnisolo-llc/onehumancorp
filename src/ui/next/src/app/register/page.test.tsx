import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import RegisterPage from "./page";

const push = vi.fn();
vi.mock("next/navigation", () => ({ useRouter: () => ({ push }) }));
const { navigateToPublicAuth } = vi.hoisted(() => ({ navigateToPublicAuth: vi.fn() }));
vi.mock("@/lib/auth/publicNavigation", () => ({ navigateToPublicAuth }));

describe("registration entry", () => {
  beforeEach(() => {
    push.mockReset();
    navigateToPublicAuth.mockReset();
    sessionStorage.clear();
    vi.mocked(fetch).mockReset();
  });

  it("shows the persisted closed policy without collecting credentials", async () => {
    vi.mocked(fetch).mockResolvedValueOnce(Response.json({
      registration_mode: "closed",
      registration_available: false,
      email_verification_required: true,
    }));
    render(<RegisterPage />);

    expect(await screen.findByText(/registration is currently closed/i)).toBeDefined();
    expect(screen.queryByLabelText(/email address/i)).toBeNull();
    expect(screen.queryByLabelText(/username/i)).toBeNull();
    expect(screen.queryByLabelText(/^password$/i)).toBeNull();
    expect(screen.getByRole("link", { name: "Sign in" })).toHaveAttribute("href", "/login");
    expect(navigateToPublicAuth).not.toHaveBeenCalled();
  });

  it("collects only email before verification and stores a bounded challenge", async () => {
    const storedAtNavigation: (string | null)[] = [];
    navigateToPublicAuth.mockImplementationOnce(() => {
      storedAtNavigation.push(sessionStorage.getItem("omnisolo-registration-challenge"));
    });
    vi.mocked(fetch)
      .mockResolvedValueOnce(Response.json({
        registration_mode: "open",
        registration_available: true,
        email_verification_required: true,
      }))
      .mockResolvedValueOnce(Response.json({ challenge_id: "challenge-7", expires_in_seconds: 900 }, { status: 202 }));
    render(<RegisterPage />);
    const user = userEvent.setup();
    await user.type(await screen.findByLabelText(/email address/i), "Alice@example.test");
    expect(screen.queryByLabelText(/username/i)).toBeNull();
    expect(screen.queryByLabelText(/^password$/i)).toBeNull();
    await user.click(screen.getByRole("button", { name: /verify email/i }));

    await waitFor(() => expect(navigateToPublicAuth).toHaveBeenCalledExactlyOnceWith("/verify-email"));
    expect(push).not.toHaveBeenCalled();
    expect(fetch).toHaveBeenLastCalledWith("/api/v1/auth/registration/email/start", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ email: "Alice@example.test" }),
    });
    expect(JSON.parse(sessionStorage.getItem("omnisolo-registration-challenge") ?? "null")).toEqual({
      challengeId: "challenge-7",
      email: "alice@example.test",
    });
    expect(storedAtNavigation).toEqual([JSON.stringify({ challengeId: "challenge-7", email: "alice@example.test" })]);
  });

  it.each([
    { status: 403, body: { challenge_id: "rejected" } },
    { status: 202, body: {} },
    { status: 202, body: { challenge_id: 7 } },
    { status: 202, body: { challenge_id: "x".repeat(65) } },
  ])("does not store or navigate after a rejected or malformed challenge %#", async ({ status, body }) => {
    vi.mocked(fetch)
      .mockResolvedValueOnce(Response.json({ registration_mode: "open", registration_available: true, email_verification_required: true }))
      .mockResolvedValueOnce(Response.json(body, { status }));
    render(<RegisterPage />);
    const user = userEvent.setup();
    await user.type(await screen.findByLabelText(/email address/i), "Alice@example.test");
    await user.click(screen.getByRole("button", { name: /verify email/i }));

    expect(await screen.findByRole("alert")).toHaveTextContent("We could not send a verification code");
    expect(sessionStorage.getItem("omnisolo-registration-challenge")).toBeNull();
    expect(navigateToPublicAuth).not.toHaveBeenCalled();
    expect(push).not.toHaveBeenCalled();
    expect(screen.getByLabelText(/email address/i)).toHaveValue("Alice@example.test");
  });
});
