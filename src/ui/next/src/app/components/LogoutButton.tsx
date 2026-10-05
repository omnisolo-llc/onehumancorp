"use client";

import { navigateToPublicAuth } from "@/lib/auth/publicNavigation";
import { useState } from "react";
import { notifyQueueIdentityChange } from "../../lib/sync/queueIdentity";

export function LogoutButton() {
  const [pending, setPending] = useState(false);
  const [error, setError] = useState(false);

  async function logout() {
    if (pending) return;
    setPending(true);
    setError(false);
    notifyQueueIdentityChange();
    try {
      const response = await fetch("/api/v1/auth/logout", { method: "POST" });
      if (!response.ok) throw new Error("logout failed");
      navigateToPublicAuth("/login", true);
    } catch {
      setError(true);
      setPending(false);
    } finally { notifyQueueIdentityChange(); }
  }

  return (
    <div className="relative">
      <button
        className="app-button min-h-[44px]"
        disabled={pending}
        onClick={logout}
        type="button"
      >
        {pending ? "Logging out…" : "Log out"}
      </button>
      {error && (
        <span
          className="absolute right-0 top-full z-20 mt-2 w-56 rounded-lg border border-red-300 bg-red-50 p-2 text-xs text-red-800 shadow-lg dark:border-red-900 dark:bg-red-950 dark:text-red-200"
          role="alert"
        >
          Logout failed. Please try again.
        </span>
      )}
    </div>
  );
}
