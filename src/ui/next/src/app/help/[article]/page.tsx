"use client";

import React from 'react';
import { useParams } from 'next/navigation';

export default function HelpArticlePage() {
  const { article } = useParams();

  const articles: Record<string, { title: string, content: React.ReactNode }> = {
    "getting-started": {
      title: "Getting Started",
      content: (
        <>
          <p>Welcome to OneHumanCorp! Let's get your store set up.</p>
          <h2 className="text-xl font-bold mt-4">1. Describe Your Business</h2>
          <p>Go to the Builder page and type a short description of what you sell. Our AI will do the rest!</p>
          <h2 className="text-xl font-bold mt-4">2. Launch Your Store</h2>
          <p>Click the "Launch Store" button to make it live for the world to see.</p>
        </>
      )
    },
    "my-store": {
      title: "My Store",
      content: (
        <>
          <p>Your store is where you show off what you sell. You can easily add new items, keep track of what you have in stock, and change how your store looks.</p>
          <h2 className="text-xl font-bold mt-4">Adding Products</h2>
          <p>To add a new item, go to the products page and click "Add Product". You can upload a picture, type in a name and description, and set the price. Our AI can even help you write a catchy description!</p>
          <h2 className="text-xl font-bold mt-4">Tracking Your Stock</h2>
          <p>When you add a product, you can tell the system how many you have to sell. When someone buys it, the number goes down automatically. This helps you know when you need to make or buy more.</p>
          <h2 className="text-xl font-bold mt-4">Changing How Your Store Looks</h2>
          <p>You can pick different colors, fonts, and layouts to make your store match your brand. Just go to the Storefront Builder to try out different styles.</p>
        </>
      )
    },
    "payments": {
      title: "Getting Paid",
      content: (
        <>
          <p>Getting paid is the most exciting part! We make it secure and easy for your customers to pay you.</p>
          <h2 className="text-xl font-bold mt-4">Connecting Your Bank Account</h2>
          <p>To start taking money, you need to connect a bank account. We use Stripe, a safe and trusted system. Just click the "Connect Stripe" button in your setup to securely link your bank.</p>
          <h2 className="text-xl font-bold mt-4">Viewing Your Deposits</h2>
          <p>When a customer buys something, the money goes into your connected bank account. You can check the Dashboard to see your recent sales and see when the money will arrive in your bank.</p>
          <h2 className="text-xl font-bold mt-4">Taxes and Fees</h2>
          <p>We help handle simple taxes for you at checkout. A small fee is taken out of each sale to cover the cost of securely moving the money from the customer's card to your bank.</p>
        </>
      )
    },
    "ai-agents": {
      title: "Your AI Helpers",
      content: (
        <>
          <p>Running a business takes a lot of work. That's why we give you AI helpers—smart computer programs that can do tasks for you, like a real team!</p>
          <h2 className="text-xl font-bold mt-4">Hiring AI Helpers</h2>
          <p>Go to the AI Departments page to see all the helpers you can hire. Some helpers are good at marketing, some are good at writing, and others are good at keeping track of numbers.</p>
          <h2 className="text-xl font-bold mt-4">Giving Them Tasks</h2>
          <p>Once you hire a helper, you can tell them what to do. You just type what you need in plain English. For example, "Write an email to my customers about a summer sale." The helper will do the work and show it to you.</p>
          <h2 className="text-xl font-bold mt-4">Approving Their Work</h2>
          <p>Helpers are smart, but you are the boss. Before they send an email or change your store, they will ask for your permission. You can check your Inbox to review and approve their tasks.</p>
        </>
      )
    },
    "marketing": {
      title: "Finding Customers",
      content: (
        <>
          <p>To grow your business, you need people to know about it. We have tools to help you find and talk to customers.</p>
          <h2 className="text-xl font-bold mt-4">Sending Emails</h2>
          <p>You can send emails to people who have bought from you before or signed up on your store. You can use this to tell them about new products or special sales. Our AI can even help you write the emails!</p>
          <h2 className="text-xl font-bold mt-4">Running Promos and Sales</h2>
          <p>Everyone loves a good deal. You can easily set up a weekend sale or a holiday promotion. You can choose to give a percentage off or a set amount of money off.</p>
          <h2 className="text-xl font-bold mt-4">Sharing Your Store</h2>
          <p>Don't forget to share your store link on social media or with your friends and family. You can find your store's link on your Dashboard.</p>
        </>
      )
    },
    "account-billing": {
      title: "Account & Billing",
      content: (
        <>
          <p>Manage your subscription plan, view your past bills, and invite people to help run your business.</p>
          <h2 className="text-xl font-bold mt-4">Managing Your Plan</h2>
          <p>You can check what plan you are on by going to the Billing page. If your business is growing and you need more features, you can upgrade your plan at any time.</p>
          <h2 className="text-xl font-bold mt-4">Viewing Your Bills</h2>
          <p>You can see a history of all the payments you have made to OneHumanCorp. This makes it easy to keep track of your expenses for your own records.</p>
          <h2 className="text-xl font-bold mt-4">Inviting Team Members</h2>
          <p>If you have business partners or staff who need to access your store settings, you can invite them to your team. Just enter their email address and they will get an invite to join.</p>
        </>
      )
    }
  };

  const articleData = article && typeof article === 'string' && articles[article] ? articles[article] : { title: "Article Not Found", content: <p>We couldn't find the article you're looking for.</p> };

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto bg-white p-8 rounded-xl shadow-sm border border-gray-100">
        <h1 className="text-3xl font-bold text-gray-900 mb-6">{articleData.title}</h1>
        <div className="prose prose-blue max-w-none text-gray-700">
          {articleData.content}
        </div>
      </div>
    </div>
  );
}
