"use client";

import React from 'react';
import { useParams, useRouter } from 'next/navigation';

export default function HelpArticlePage() {
  const { article } = useParams();
  const router = useRouter();

  const articles: Record<string, { title: string, content: React.ReactNode }> = {
    "getting-started": {
      title: "Getting Started with Your Store",
      content: (
        <>
          <p className="text-gray-700 mb-4 leading-relaxed text-lg">
            Welcome to OneHumanCorp! Setting up your store is quick and easy. Our app helps you get everything ready to sell online.
          </p>
          <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Step 1: Tell us about your business</h2>
          <p className="text-gray-700 mb-4">
            Start by telling us what you sell and who your customers are. Keep it simple! Just describe what makes your shop special.
          </p>
          <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Step 2: Let AI build your store</h2>
          <p className="text-gray-700 mb-4">
            Once you tell us about your business, click the "Generate" button. Our AI will build your store for you. It will pick a design and write some text to get you started.
          </p>
          <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Step 3: Launch to the world</h2>
          <p className="text-gray-700 mb-4">
            When you are happy with how your store looks, click the "Launch" button. This makes your store live on the internet so customers can visit and buy from you!
          </p>
          <div className="mt-8 bg-blue-50 p-4 rounded-lg border border-blue-100">
            <p className="text-blue-800 font-medium">Need more help? Click the chat button to ask our AI assistant any questions you have.</p>
          </div>
        </>
      )
    },
    "my-store": {
      title: "Managing My Store",
      content: (
        <>
          <p className="text-gray-700 mb-4 leading-relaxed text-lg">
            Your store is where you show off what you sell. You can easily add new items, keep track of what you have in stock, and change how your store looks.
          </p>
          <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Adding Products</h2>
          <p className="text-gray-700 mb-4">
            To add a new item, go to the products page and click "Add Product". You can upload a picture, type in a name and description, and set the price. Our AI can even help you write a catchy description!
          </p>
          <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Tracking Your Stock</h2>
          <p className="text-gray-700 mb-4">
            When you add a product, you can tell the app how many you have to sell. When someone buys it, the number goes down on its own. This helps you know when you need to make or buy more.
          </p>
          <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Changing How Your Store Looks</h2>
          <p className="text-gray-700 mb-4">
            You can pick different colors, fonts, and layouts to make your store match your brand. Just go to the Storefront Builder to try out different styles.
          </p>
        </>
      )
    },
    "marketing": {
      title: "Finding Customers",
      content: (
        <>
          <p className="text-gray-700 mb-4 leading-relaxed text-lg">
            To grow your business, you need people to know about it. We have tools to help you find and talk to customers.
          </p>
          <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Sending Emails</h2>
          <p className="text-gray-700 mb-4">
            You can send emails to people who have bought from you before or signed up on your store. You can use this to tell them about new products or special sales. Our AI can even help you write the emails!
          </p>
          <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Running Promos and Sales</h2>
          <p className="text-gray-700 mb-4">
            Everyone loves a good deal. You can easily set up a weekend sale or a holiday promotion. You can choose to give a percentage off or a set amount of money off.
          </p>
          <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Sharing Your Store</h2>
          <p className="text-gray-700 mb-4">
            Don't forget to share your store link on social media or with your friends and family. You can find your store's link on your Dashboard.
          </p>
        </>
      )
    },
    "account-billing": {
      title: "Account & Billing",
      content: (
        <>
          <p className="text-gray-700 mb-4 leading-relaxed text-lg">
            Manage your monthly plan, view your past bills, and invite people to help run your business.
          </p>
          <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Managing Your Plan</h2>
          <p className="text-gray-700 mb-4">
            You can check what plan you are on by going to the Billing page. If your business is growing and you need more features, you can upgrade your plan at any time.
          </p>
          <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Viewing Your Bills</h2>
          <p className="text-gray-700 mb-4">
            You can see a history of all the payments you have made to OneHumanCorp. This makes it easy to keep track of your expenses for your own records.
          </p>
          <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Inviting Team Members</h2>
          <p className="text-gray-700 mb-4">
            If you have business partners or staff who need to access your store settings, you can invite them to your team. Just enter their email address and they will get an invite to join.
          </p>
        </>
      )
    },
    "payments": {
      title: "Getting Paid",
      content: (
        <>
          <p className="text-gray-700 mb-4 leading-relaxed text-lg">
            Getting paid is the most exciting part! We make it secure and easy for your customers to pay you.
          </p>
          <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Connecting Your Bank Account</h2>
          <p className="text-gray-700 mb-4">
            To start taking money, you need to connect a bank account. We use Stripe, a safe and trusted system. Just click the "Connect Stripe" button in your setup to securely link your bank.
          </p>
          <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Viewing Your Deposits</h2>
          <p className="text-gray-700 mb-4">
            When a customer buys something, the money goes into your connected bank account. You can check the Dashboard to see your recent sales and see when the money will arrive in your bank.
          </p>
          <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Taxes and Fees</h2>
          <p className="text-gray-700 mb-4">
            We help handle simple taxes for you at checkout. A small fee is taken out of each sale to cover the cost of securely moving the money from the customer's card to your bank.
          </p>
        </>
      )
    },
    "ai-agents": {
      title: "Your AI Helpers",
      content: (
        <>
          <p className="text-gray-700 mb-4 leading-relaxed text-lg">
            Running a business takes a lot of work. That's why we give you AI helpers—smart computer programs that can do tasks for you, like a real team!
          </p>
          <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Hiring AI Helpers</h2>
          <p className="text-gray-700 mb-4">
            Go to the AI Departments page to see all the helpers you can hire. Some helpers are good at marketing, some are good at writing, and others are good at keeping track of numbers.
          </p>
          <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Giving Them Tasks</h2>
          <p className="text-gray-700 mb-4">
            Once you hire a helper, you can tell them what to do. You just type what you need in plain English. For example, "Write an email to my customers about a summer sale." The helper will do the work and show it to you.
          </p>
          <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Approving Their Work</h2>
          <p className="text-gray-700 mb-4">
            Helpers are smart, but you are the boss. Before they send an email or change your store, they will ask for your permission. You can check your Inbox to review and approve their tasks.
          </p>
        </>
      )
    }
  };

  const articleData = article && typeof article === 'string' && articles[article] ? articles[article] : { title: "Article Not Found", content: <p>We couldn't find the article you're looking for.</p> };

  return (
    <div className="min-h-screen bg-[#F5F5F7] py-8 sm:py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto w-full">
        <button
          onClick={() => router.push('/help')}
          className="mb-6 text-blue-600 hover:text-blue-800 font-bold flex items-center gap-2 px-3 py-2 -ml-3 rounded-xl hover:bg-blue-50/50 min-h-[44px] transition-colors"
          aria-label="Back to Help Center"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          Back to Help Center
        </button>
        <div className="bg-white/70 backdrop-blur-[20px] saturate-200 p-6 sm:p-10 rounded-3xl shadow-[0_8px_32px_rgba(0,0,0,0.05)] border border-white/60 transition-all">
          <h1 className="text-3xl sm:text-4xl font-extrabold font-outfit text-[#1D1D1F] mb-8 tracking-tight">{articleData.title}</h1>
          <div className="prose prose-blue prose-lg max-w-none text-gray-700 leading-relaxed marker:text-blue-500 prose-headings:font-outfit prose-headings:text-[#1D1D1F]">
            {articleData.content}
          </div>
        </div>
      </div>
    </div>
  );
}
