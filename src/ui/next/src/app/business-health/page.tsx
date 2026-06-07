"use client";

import React, { useState } from 'react';
import Link from 'next/link';
import { useRouter } from 'next/navigation';

export default function BusinessHealthPage() {
  const router = useRouter();
  const [score, setScore] = useState(65);

  const actionItems = [
    {
      id: 1,
      title: "Add 3 more products",
      points: 15,
      completed: false,
      icon: "🛍️",
      link: "/products"
    },
    {
      id: 2,
      title: "Connect a custom domain",
      points: 10,
      completed: false,
      icon: "🌐",
      link: "/settings"
    },
    {
      id: 3,
      title: "Setup automated review requests",
      points: 10,
      completed: false,
      icon: "⭐",
      link: "/review-campaigns"
    },
    {
      id: 4,
      title: "Complete store setup wizard",
      points: 20,
      completed: true,
      icon: "✅",
      link: "/business-setup"
    },
    {
      id: 5,
      title: "Add your first product",
      points: 20,
      completed: true,
      icon: "📦",
      link: "/products"
    },
    {
      id: 6,
      title: "Create a share card",
      points: 25,
      completed: true,
      icon: "🎴",
      link: "/share-cards"
    }
  ];

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-[#0A0A0A] p-6 lg:p-12 font-inter">
      <div className="max-w-4xl mx-auto space-y-8">

        {/* Header */}
        <div className="flex items-center justify-between">
            <div className="flex items-center space-x-4">
                <button
                    onClick={() => router.push('/dashboard')}
                    className="p-2 hover:bg-gray-200 dark:hover:bg-gray-800 rounded-full transition-colors text-gray-600 dark:text-gray-400"
                    aria-label="Back to dashboard"
                >
                    <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
                </button>
                <h1 className="text-3xl font-bold font-outfit text-gray-900 dark:text-white tracking-tight">Business Health</h1>
            </div>
            <div className="px-4 py-1.5 bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-400 rounded-full text-sm font-semibold">
                Pro Feature
            </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            {/* Score Card */}
            <div className="md:col-span-1 glassmorphism bg-white dark:bg-[#1D1D1F] border border-gray-200 dark:border-white/10 rounded-3xl p-8 flex flex-col items-center justify-center text-center shadow-lg relative overflow-hidden">
                 {/* Decorative background element */}
                 <div className="absolute inset-0 bg-gradient-to-br from-blue-500/10 to-purple-500/10 pointer-events-none"></div>

                 <h2 className="text-xl font-semibold font-outfit text-gray-900 dark:text-white mb-6 relative z-10">Overall Score</h2>

                 <div className="relative w-48 h-48 mb-6 flex items-center justify-center z-10">
                    <svg className="w-full h-full transform -rotate-90" viewBox="0 0 100 100">
                        <circle
                            cx="50" cy="50" r="45"
                            fill="transparent"
                            stroke="currentColor"
                            strokeWidth="8"
                            className="text-gray-200 dark:text-gray-800"
                        />
                        <circle
                            cx="50" cy="50" r="45"
                            fill="transparent"
                            stroke="currentColor"
                            strokeWidth="8"
                            strokeDasharray="282.7"
                            strokeDashoffset={282.7 - (282.7 * score) / 100}
                            className={`${score >= 80 ? 'text-green-500' : score >= 50 ? 'text-blue-500' : 'text-orange-500'} transition-all duration-1000 ease-out`}
                            strokeLinecap="round"
                        />
                    </svg>
                    <div className="absolute flex flex-col items-center justify-center">
                        <span className="text-5xl font-bold font-outfit text-gray-900 dark:text-white tracking-tighter">{score}</span>
                        <span className="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-widest mt-1">/ 100</span>
                    </div>
                 </div>

                 <p className="text-gray-600 dark:text-gray-300 font-medium relative z-10">
                     {score >= 80 ? 'Excellent! Your store is highly optimized.' : score >= 50 ? 'Good start! Complete action items to grow.' : 'Needs attention! Let\'s boost your score.'}
                 </p>
            </div>

            {/* Action Items */}
            <div className="md:col-span-2 space-y-6">
                <div className="bg-white dark:bg-[#1D1D1F] border border-gray-200 dark:border-white/10 rounded-3xl p-6 shadow-sm">
                    <h3 className="text-lg font-bold font-outfit text-gray-900 dark:text-white mb-4 flex items-center gap-2">
                        <span>🎯</span> Action Items to Improve
                    </h3>
                    <div className="space-y-3">
                        {actionItems.filter(item => !item.completed).map((item) => (
                            <Link href={item.link} key={item.id} className="flex items-center justify-between p-4 rounded-2xl border border-gray-100 dark:border-white/5 bg-gray-50 dark:bg-black/20 hover:bg-white dark:hover:bg-white/5 hover:border-blue-200 dark:hover:border-blue-500/30 transition-all group">
                                <div className="flex items-center space-x-4">
                                    <div className="w-10 h-10 rounded-full bg-white dark:bg-[#1D1D1F] shadow-sm flex items-center justify-center text-xl">
                                        {item.icon}
                                    </div>
                                    <div>
                                        <h4 className="font-semibold text-gray-900 dark:text-white group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors">{item.title}</h4>
                                        <p className="text-sm text-gray-500 dark:text-gray-400">Boosts score by +{item.points}</p>
                                    </div>
                                </div>
                                <div className="w-8 h-8 rounded-full border-2 border-gray-300 dark:border-gray-600 flex items-center justify-center text-transparent group-hover:border-blue-500 transition-colors">
                                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                                </div>
                            </Link>
                        ))}
                    </div>
                </div>

                <div className="bg-white dark:bg-[#1D1D1F] border border-gray-200 dark:border-white/10 rounded-3xl p-6 shadow-sm opacity-70">
                    <h3 className="text-lg font-bold font-outfit text-gray-900 dark:text-white mb-4 flex items-center gap-2">
                        <span className="grayscale">🏆</span> Completed Milestones
                    </h3>
                    <div className="space-y-3">
                        {actionItems.filter(item => item.completed).map((item) => (
                            <div key={item.id} className="flex items-center justify-between p-4 rounded-2xl border border-gray-100 dark:border-white/5 bg-gray-50 dark:bg-black/20">
                                <div className="flex items-center space-x-4">
                                    <div className="w-10 h-10 rounded-full bg-green-50 dark:bg-green-900/20 text-green-600 dark:text-green-400 flex items-center justify-center text-xl">
                                        {item.icon}
                                    </div>
                                    <div>
                                        <h4 className="font-semibold text-gray-500 dark:text-gray-400 line-through">{item.title}</h4>
                                    </div>
                                </div>
                                <div className="w-8 h-8 rounded-full bg-green-500 flex items-center justify-center text-white">
                                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>
                                </div>
                            </div>
                        ))}
                    </div>
                </div>
            </div>
        </div>

      </div>
    </div>
  );
}
