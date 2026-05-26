"use client";

import React, { useEffect, useState } from "react";
import { useParams } from "next/navigation";

export default function HelpTopicPage() {
  const params = useParams();
  const topic = params.topic as string;
  const [content, setContent] = useState<any>(null);

  useEffect(() => {
    // In a real app, fetch the markdown or full content from API.
    // For this prototype, we'll hardcode some sample content based on the route.
    const sampleData: Record<string, any> = {
      "getting-started": {
        title: "Getting Started",
        body: "Welcome to One Human Corp! This guide will help you set up your store quickly. First, enter a simple description of what you sell. Our AI will automatically generate a professional storefront for you. Next, connect your bank to accept payments."
      },
      "my-store": {
        title: "My Store",
        body: "Managing your store is easy. You can add new products, edit existing ones, and change your store's colors and logos to match your brand."
      },
      "payments": {
        title: "Getting Paid",
        body: "We use Stripe to make sure you get paid safely and quickly. Once you link your bank account, your earnings will be automatically deposited."
      },
      "ai-agents": {
        title: "Your AI Helpers",
        body: "AI helpers are like your own digital team. You can hire an AI helper to handle customer questions, design a marketing email, or even reorganize your product list."
      },
      "marketing": {
        title: "Finding Customers",
        body: "Growing your business has never been simpler. Use our easy tools to send emails or set up seasonal promotions to bring customers back to your store."
      },
      "account-billing": {
        title: "Account & Billing",
        body: "View your current subscription plan, download your monthly invoices, and invite your team members to help manage the store."
      }
    };

    setContent(sampleData[topic] || { title: "Topic Not Found", body: "We couldn't find the help article you're looking for." });
  }, [topic]);

  if (!content) return <div className="p-8 text-center text-gray-500 font-inter">Loading...</div>;

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto bg-white p-8 rounded-2xl shadow-sm border border-gray-100">
        <h1 className="text-3xl font-bold text-gray-900 mb-6 font-outfit">{content.title}</h1>
        <p className="text-gray-700 leading-relaxed text-lg">{content.body}</p>
        <div className="mt-8 pt-8 border-t border-gray-100">
          <a href="/" className="text-blue-600 font-bold hover:underline">← Back to Dashboard</a>
        </div>
      </div>
    </div>
  );
}
