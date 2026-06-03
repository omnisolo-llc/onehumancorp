"use client";

import React, { useState } from "react";

export default function AutoQuoteBookPage() {
  const [customerDescription, setCustomerDescription] = useState("");
  const [generatedQuote, setGeneratedQuote] = useState<string | null>(null);
  const [isGenerating, setIsGenerating] = useState(false);

  const handleGenerateQuote = async () => {
    setIsGenerating(true);
    try {
      const response = await fetch("/api/v1/sales/generate", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          tenant_id: "test_tenant",
          customer_description: customerDescription,
        }),
      });

      if (!response.ok) {
        throw new Error("Failed to generate quote");
      }

      const data = await response.json();
      setGeneratedQuote(data.quote);
    } catch (error) {
      console.error(error);
      setGeneratedQuote("Error generating quote. Please try again.");
    } finally {
      setIsGenerating(false);
    }
  };

  return (
    <div className="flex-1 space-y-4 p-4 md:p-8 pt-6 max-w-7xl mx-auto">
      <div className="flex items-center justify-between space-y-2">
        <h2 className="text-3xl font-bold tracking-tight">Auto-Quote & Book Dashboard</h2>
      </div>
      <p className="text-muted-foreground">
        Configure your AI Salesperson to automatically draft quotes and booking links.
      </p>

      <div className="mt-8 grid gap-8 md:grid-cols-2">
        <div className="border rounded-lg p-6 bg-card text-card-foreground shadow-sm">
          <div className="flex flex-col space-y-1.5 mb-4">
            <h3 className="font-semibold leading-none tracking-tight">Customer Inquiry Simulator</h3>
            <p className="text-sm text-muted-foreground">
              Test how your AI Salesperson responds to customer requests.
            </p>
          </div>
          <div className="space-y-4">
            <div className="space-y-2">
              <label className="text-sm font-medium">Customer Message</label>
              <textarea
                className="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                placeholder="e.g. I need help fixing a broken pipe in my bathroom..."
                value={customerDescription}
                onChange={(e) => setCustomerDescription(e.target.value)}
                rows={4}
              />
            </div>
            <button
              className="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background bg-primary text-primary-foreground hover:bg-primary/90 h-10 py-2 px-4"
              onClick={handleGenerateQuote}
              disabled={!customerDescription || isGenerating}
            >
              {isGenerating ? "Generating..." : "Generate Quote"}
            </button>
          </div>
        </div>

        {generatedQuote && (
          <div className="border rounded-lg p-6 bg-card text-card-foreground shadow-sm">
            <div className="flex flex-col space-y-1.5 mb-4">
              <h3 className="font-semibold leading-none tracking-tight">AI Salesperson Response</h3>
              <p className="text-sm text-muted-foreground">Generated Quote & Booking Link</p>
            </div>
            <div>
              <div className="rounded-lg bg-muted p-4 whitespace-pre-wrap text-sm">
                {generatedQuote}
              </div>
              <div className="mt-4">
                <button className="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background border border-input hover:bg-accent hover:text-accent-foreground h-10 py-2 px-4 w-full">
                  Approve & Send to Customer
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
