"use client";

import React, { useState } from "react";
import { Play } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Textarea } from "@/components/ui/textarea";

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
        <Card>
          <CardHeader>
            <CardTitle>Customer Inquiry Simulator</CardTitle>
            <CardDescription>
              Test how your AI Salesperson responds to customer requests.
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <label className="text-sm font-medium">Customer Message</label>
              <Textarea
                placeholder="e.g. I need help fixing a broken pipe in my bathroom..."
                value={customerDescription}
                onChange={(e) => setCustomerDescription(e.target.value)}
                rows={4}
              />
            </div>
            <Button
              onClick={handleGenerateQuote}
              disabled={!customerDescription || isGenerating}
            >
              {isGenerating ? "Generating..." : "Generate Quote"}
            </Button>
          </CardContent>
        </Card>

        {generatedQuote && (
          <Card>
            <CardHeader>
              <CardTitle>AI Salesperson Response</CardTitle>
              <CardDescription>Generated Quote & Booking Link</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="rounded-lg bg-muted p-4 whitespace-pre-wrap">
                {generatedQuote}
              </div>
              <div className="mt-4">
                <Button variant="outline" className="w-full">
                  Approve & Send to Customer
                </Button>
              </div>
            </CardContent>
          </Card>
        )}
      </div>
    </div>
  );
}
