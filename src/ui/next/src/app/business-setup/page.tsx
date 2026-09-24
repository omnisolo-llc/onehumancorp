"use client";

import Link from "next/link";
import { AppShell } from "../components/AppShell";

export default function BusinessSetupPage() {
  return (
    <AppShell
      title="Business Setup"
      subtitle="Launch your business workspace in minutes."
    >
      <div className="flex flex-col items-center justify-center min-h-[60vh] text-center p-6">
        <h1 className="text-3xl sm:text-4xl font-extrabold font-outfit text-gray-900 mb-4">
          Your business, live in minutes.
        </h1>
        <p className="text-gray-600 max-w-lg mb-8 text-base">
          Let your dedicated AI agents handle store setup, payments, and operations. Start with zero technical configuration.
        </p>
        <Link
          href="/onboarding"
          className="app-button primary text-lg px-8 py-3.5 rounded-xl shadow-lg hover:shadow-xl transition-all font-semibold"
        >
          Start Business Setup
        </Link>
      </div>
    </AppShell>
  );
}
