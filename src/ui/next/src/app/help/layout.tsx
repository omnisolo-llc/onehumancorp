import type { Metadata } from "next";
import React from "react";
import { TooltipProvider } from "../../components/TooltipRegistry";

export const metadata: Metadata = {
  title: "Help Center | OmniSolo OneHumanCorp",
  description: "In-App Help Center for OmniSolo work assistant.",
};

export default function HelpLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <TooltipProvider>
      <div className="help-layout">{children}</div>
    </TooltipProvider>
  );
}
