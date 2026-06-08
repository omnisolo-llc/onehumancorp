"use client";

import React from "react";
import Link from "next/link";

function HelpCard({ title, description, href }: { title: string, description: string, href: string }) {
  return (
    <Link href={href}>
      <div className="backdrop-blur-[30px] saturate-[210%] bg-white/65 border border-white/40 shadow-[0_8px_32px_rgba(0,0,0,0.1)] rounded-2xl dark:bg-[#16161a]/70 dark:border-white/10 p-4 md:p-6 transition-transform hover:scale-[1.02]">
        <h3 className="font-outfit text-lg font-bold text-[#1d1d1f] dark:text-white mb-2">{title}</h3>
        <p className="font-inter text-sm text-[#86868b] dark:text-gray-300">{description}</p>
      </div>
    </Link>
  );
}

function VideoCard({ title, duration }: { title: string, duration: string }) {
  return (
    <div className="backdrop-blur-[30px] saturate-[210%] bg-white/65 border border-white/40 shadow-[0_8px_32px_rgba(0,0,0,0.1)] rounded-2xl dark:bg-[#16161a]/70 dark:border-white/10 overflow-hidden flex flex-col">
      <div className="w-full aspect-video bg-black/5 dark:bg-white/5 relative flex items-center justify-center">
        {/* Placeholder for actual video thumbnail/player */}
        <div className="w-12 h-12 rounded-full bg-blue-500/80 flex items-center justify-center text-white cursor-pointer backdrop-blur-md">
          <svg className="w-6 h-6 ml-1" fill="currentColor" viewBox="0 0 20 20"><path d="M4 4l12 6-12 6z"/></svg>
        </div>
      </div>
      <div className="p-4 md:p-6">
        <h3 className="font-outfit text-sm font-semibold text-[#1d1d1f] dark:text-white mb-1">{title}</h3>
        <span className="font-inter text-xs text-[#86868b] dark:text-gray-400">{duration}</span>
      </div>
    </div>
  );
}

export default function HelpCenterPage() {
  return (
    <div className="min-h-screen bg-[#F5F5F7]/80 dark:bg-[#000000]/80 p-4 md:p-8 backdrop-blur-[20px] saturate-200">
      <div className="max-w-4xl mx-auto space-y-8 md:space-y-12">
        <div className="text-center space-y-4">
          <h1 className="font-outfit text-3xl md:text-5xl font-extrabold text-[#1d1d1f] dark:text-white tracking-tight">Help Center</h1>
          <div className="relative max-w-2xl mx-auto">
             <input
               type="text"
               placeholder="Search for help articles and videos..."
               className="w-full px-6 py-4 rounded-full border border-gray-200 dark:border-white/10 bg-white/80 dark:bg-black/50 backdrop-blur-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 font-inter text-base"
             />
             <svg className="absolute right-6 top-4 w-6 h-6 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/></svg>
          </div>
        </div>

        <section>
          <h2 className="font-outfit text-2xl font-bold text-[#1d1d1f] dark:text-white mb-6 border-b border-gray-200 dark:border-white/10 pb-2">Getting Started</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 md:gap-6">
            <HelpCard title="Getting Started" description="Learn how to easily set up your store and accept your first payment." href="/help/getting-started" />
            <HelpCard title="My Store" description="Add products, track what's in stock, and change how your store looks." href="/help/my-store" />
          </div>
        </section>

        <section>
          <h2 className="font-outfit text-2xl font-bold text-[#1d1d1f] dark:text-white mb-6 border-b border-gray-200 dark:border-white/10 pb-2">Payments & Billing</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 md:gap-6">
            <HelpCard title="Getting Paid" description="Set up how you get paid, view deposits, and handle simple taxes." href="/help/payments" />
            <HelpCard title="Account & Billing" description="View your bills, manage your plan, and invite team members." href="/help/billing" />
          </div>
        </section>

        <section>
          <h2 className="font-outfit text-2xl font-bold text-[#1d1d1f] dark:text-white mb-6 border-b border-gray-200 dark:border-white/10 pb-2">AI Assistants & Marketing</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 md:gap-6">
            <HelpCard title="Your AI Helpers" description="Learn how to hire AI helpers and give them tasks to do." href="/help/ai-agents" />
            <HelpCard title="Finding Customers" description="Send emails to customers and grow your business easily." href="/help/marketing" />
          </div>
        </section>

        <section>
          <h2 className="font-outfit text-2xl font-bold text-[#1d1d1f] dark:text-white mb-6 border-b border-gray-200 dark:border-white/10 pb-2">Video Tutorials</h2>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 md:gap-6">
             <VideoCard title="How to set up your first store easily" duration="1:20" />
             <VideoCard title="Accept your first payment" duration="1:15" />
             <VideoCard title="Activate your AI Support Agent" duration="0:50" />
             <VideoCard title="Adding staff to your account" duration="1:05" />
             <VideoCard title="Review an order" duration="1:10" />
             <VideoCard title="Manage inventory" duration="1:00" />
          </div>
        </section>
      </div>
    </div>
  );
}
