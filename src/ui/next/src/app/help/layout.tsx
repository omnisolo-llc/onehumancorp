import type { Metadata } from "next";
import React from "react";
<<<<<<< HEAD
import { TooltipProvider } from "../../components/TooltipRegistry";
=======
>>>>>>> 7b2282170 (🗺️ Guide: [blocked no-work finding: F14: economics/owner outcomes] (#38156))

export const metadata: Metadata = {
  title: "Help Center | OmniSolo OneHumanCorp",
  description: "In-App Help Center for OmniSolo work assistant.",
};

export default function HelpLayout({
  children,
}: {
  children: React.ReactNode;
}) {
<<<<<<< HEAD
  return (
    <TooltipProvider>
      <div className="help-layout">{children}</div>
    </TooltipProvider>
  );
=======
  return <div className="help-layout">{children}</div>;
>>>>>>> 7b2282170 (🗺️ Guide: [blocked no-work finding: F14: economics/owner outcomes] (#38156))
}
