import { render, screen, fireEvent, waitFor, act } from "@testing-library/react";
import React from "react";
import ClientPortalPage from "./page";
import { expect, test, vi, describe } from "vitest";

describe("ClientPortalPage", () => {
  test("renders overview hub correctly and lists pending tasks", () => {
    render(<ClientPortalPage />);

    expect(screen.getByText("Welcome Back, Acme Corporation!")).toBeDefined();
    expect(screen.getByText("Today's Priorities")).toBeDefined();
    expect(screen.getByText("My Subscriptions & Perks")).toBeDefined();
    expect(screen.getByText("1 Outstanding Proposal")).toBeDefined();
    expect(screen.getByText("1 Pending Invoice Payment")).toBeDefined();
  });

  test("allows switching tabs to Proposals & Quotes and signing proposal", async () => {
    render(<ClientPortalPage />);

    // Click Proposals Tab
    const proposalsTab = screen.getByRole("button", { name: /📜 Proposals & Quotes/i });
    fireEvent.click(proposalsTab);

    expect(screen.getByText("Active Proposals & Cost Estimations")).toBeDefined();
    expect(screen.getAllByText("Automated Supply Chain & MRP Integration")[0]).toBeDefined();

    // Type signature and submit without consent first
    const signInput = screen.getByPlaceholderText("Full Legal Name");
    fireEvent.change(signInput, { target: { value: "John Doe" } });

    const submitBtn = screen.getByRole("button", { name: /Sign & Approve Proposal/i });
    fireEvent.click(submitBtn);

    expect(screen.getByText("Please provide signature text and check the consent box.")).toBeDefined();

    // Check consent and submit
    const consentCheckbox = screen.getByRole("checkbox");
    fireEvent.click(consentCheckbox);
    fireEvent.click(submitBtn);

    expect(screen.getByText(/Successfully approved proposal QT-9021/i)).toBeDefined();
  });

  test("allows switching tabs to Invoices & Billing and making a payment", async () => {
    vi.useFakeTimers();
    render(<ClientPortalPage />);

    // Click Billing Tab
    const billingTab = screen.getByRole("button", { name: /💳 Invoices & Billing/i });
    fireEvent.click(billingTab);

    expect(screen.getByText("Invoices & Milestone Billing")).toBeDefined();
    expect(screen.getAllByText("Initial Deposit - Supply Chain Setup")[0]).toBeDefined();

    // Fill in card details
    const cardInput = screen.getByPlaceholderText("4000 1234 5678 9010");
    const expiryInput = screen.getByPlaceholderText("MM/YY");
    const cvcInput = screen.getByPlaceholderText("123");

    fireEvent.change(cardInput, { target: { value: "4000123456789010" } });
    fireEvent.change(expiryInput, { target: { value: "12/29" } });
    fireEvent.change(cvcInput, { target: { value: "999" } });

    const payBtn = screen.getByRole("button", { name: /Authorize Payment/i });
    fireEvent.click(payBtn);

    // Assert loader / text change
    expect(screen.getByText(/Processing Payment/i)).toBeDefined();

    // Fast-forward timers
    act(() => {
      vi.advanceTimersByTime(1500);
    });

    expect(screen.getByText(/Payment of \$1500\.00 received successfully/i)).toBeDefined();
    expect(screen.getByText("Invoice Fully Paid")).toBeDefined();

    vi.useRealTimers();
  });

  test("allows switching to Digital Products and ticking course lessons", () => {
    render(<ClientPortalPage />);

    // Click Digital Products Tab
    const digitalTab = screen.getByRole("button", { name: /🎓 Digital Products/i });
    fireEvent.click(digitalTab);

    expect(screen.getByText("Digital Products, Online Courses & Podcasts")).toBeDefined();
    expect(screen.getByText("Swarm Intelligence & Business Automations Masterclass")).toBeDefined();
    expect(screen.getByText("60% Completed")).toBeDefined();

    // Check one of the pending lessons
    const lessonCheckbox = screen.getAllByRole("checkbox")[3]; // fourth lesson: Optimizing Lead Gen
    fireEvent.click(lessonCheckbox);

    // Progress should update to 80%
    expect(screen.getByText("80% Completed")).toBeDefined();
  });

  test("allows streaming a podcast episode", () => {
    render(<ClientPortalPage />);

    const digitalTab = screen.getByRole("button", { name: /🎓 Digital Products/i });
    fireEvent.click(digitalTab);

    expect(screen.getByText("No Episode Selected")).toBeDefined();

    // Select the first podcast episode
    const playEpisodeBtn = screen.getAllByRole("button", { name: /▶️/i })[0];
    fireEvent.click(playEpisodeBtn);

    expect(screen.getByText("NOW PLAYING")).toBeDefined();
    expect(screen.getAllByText("Episode 14: Scaling Multi-Agent Systems Offline-First")[0]).toBeDefined();
  });

  test("allows viewing project tracker tasks and statuses", () => {
    render(<ClientPortalPage />);

    const projectsTab = screen.getByRole("button", { name: /⚙️ Project Tracker/i });
    fireEvent.click(projectsTab);

    expect(screen.getByText("Project Tracker & Active Workflows")).toBeDefined();
    expect(screen.getByText("Configure PostgreSQL Schema & Schema Migrations")).toBeDefined();
    expect(screen.getByText("Deploy Sandbox Environment & Secure Integrations")).toBeDefined();
  });

  test("allows submitting support ticket and chatting in support", async () => {
    vi.useFakeTimers();
    render(<ClientPortalPage />);

    const supportTab = screen.getByRole("button", { name: /💬 Help & Live Chat/i });
    fireEvent.click(supportTab);

    expect(screen.getByText("Create Helpdesk Ticket")).toBeDefined();

    // Submit ticket
    const ticketDescInput = screen.getByPlaceholderText("Detail your request...");
    fireEvent.change(ticketDescInput, { target: { value: "We need custom webhooks set up." } });

    const submitTicketBtn = screen.getByRole("button", { name: /Submit Support Ticket/i });
    fireEvent.click(submitTicketBtn);

    expect(screen.getByText(/Support ticket created successfully! Priority: MEDIUM/i)).toBeDefined();

    // Send Live Chat Message
    const chatInput = screen.getByPlaceholderText(/Type a message or ask about your bills/i);
    fireEvent.change(chatInput, { target: { value: "Tell me about my invoice" } });

    const sendMsgBtn = screen.getByRole("button", { name: "➤" });
    fireEvent.click(sendMsgBtn);

    expect(screen.getByText("Tell me about my invoice")).toBeDefined();

    // Wait for automated agent response
    act(() => {
      vi.advanceTimersByTime(1000);
    });

    expect(screen.getByText(/I see you're asking about billing. You can view your current invoices/i)).toBeDefined();

    vi.useRealTimers();
  });
});
