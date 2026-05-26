import React from 'react';
import Link from 'next/link';
import { notFound } from 'next/navigation';

const articles: Record<string, { title: string; content: React.ReactNode }> = {
  'getting-started': {
    title: 'Getting Started',
    content: (
      <>
        <p>Welcome! We are excited to have you here. Setting up your store is fast and easy.</p>
        <h2 className="text-xl font-bold mt-6 mb-2">Step 1: Add your details</h2>
        <p>First, tell us your store name and what you sell. You can do this on the setup page. Make sure it sounds like you!</p>
        <h2 className="text-xl font-bold mt-6 mb-2">Step 2: Connect your bank</h2>
        <p>To get paid, you need to connect your bank account. Go to the payments page and follow the simple steps. It is safe and secure.</p>
        <h2 className="text-xl font-bold mt-6 mb-2">Step 3: Launch!</h2>
        <p>Once your details are in and your bank is linked, hit the Launch button. Your store will be live on the internet for customers to visit.</p>
      </>
    )
  },
  'my-store': {
    title: 'My Store',
    content: (
      <>
        <p>Your store is where the magic happens. Here is how to manage it.</p>
        <h2 className="text-xl font-bold mt-6 mb-2">Adding items to sell</h2>
        <p>Click "Add Product" to put a new item in your store. Take a clear photo, write a short title, and set a fair price. If you have different sizes or colors, you can add those too.</p>
        <h2 className="text-xl font-bold mt-6 mb-2">Tracking stock</h2>
        <p>You can tell the system how many items you have left. When a customer buys one, the number goes down automatically. This way, you never sell something you do not have.</p>
        <h2 className="text-xl font-bold mt-6 mb-2">Changing how it looks</h2>
        <p>Want a different color or a new logo? Go to the "Look & Feel" section. You can make your store match your style with just a few clicks.</p>
      </>
    )
  },
  'payments': {
    title: 'Getting Paid',
    content: (
      <>
        <p>Getting paid should be simple. Here is how it works.</p>
        <h2 className="text-xl font-bold mt-6 mb-2">Setting up payments</h2>
        <p>We use a safe system called Stripe. You just need to link your bank account. After a customer buys something, the money goes straight to your bank.</p>
        <h2 className="text-xl font-bold mt-6 mb-2">When do I get paid?</h2>
        <p>Usually, the money arrives in your bank account in 2 to 3 days. You can check the exact date on your dashboard.</p>
        <h2 className="text-xl font-bold mt-6 mb-2">Taxes</h2>
        <p>We help you figure out simple taxes. You can set the tax rate for your area, and it will be added to the customer's total at checkout.</p>
      </>
    )
  },
  'ai-agents': {
    title: 'Your AI Helpers',
    content: (
      <>
        <p>Think of AI helpers as your extra hands. They can do tasks for you so you can focus on running your business.</p>
        <h2 className="text-xl font-bold mt-6 mb-2">Hiring a helper</h2>
        <p>Go to the "AI Team" page. You can hire a helper to write descriptions for your items, send emails to customers, or even answer questions on your site.</p>
        <h2 className="text-xl font-bold mt-6 mb-2">Giving them tasks</h2>
        <p>Once hired, you can give your helper a task. Tell them exactly what you need in plain words. For example, "Write a fun description for my new red shoes."</p>
        <h2 className="text-xl font-bold mt-6 mb-2">Checking their work</h2>
        <p>Before any work is sent to customers, you get to check it. If you like it, click approve. If not, you can ask them to try again.</p>
      </>
    )
  },
  'marketing': {
    title: 'Finding Customers',
    content: (
      <>
        <p>A great store needs customers. Here is how to bring them in.</p>
        <h2 className="text-xl font-bold mt-6 mb-2">Sending emails</h2>
        <p>You can send emails to people who have visited your store. Tell them about new items or special sales. Our tools make it easy to design nice emails.</p>
        <h2 className="text-xl font-bold mt-6 mb-2">Abandoned carts</h2>
        <p>Sometimes people add items to their cart but do not buy. You can turn on a setting to email them and remind them to finish checking out. This is a great way to save a sale.</p>
        <h2 className="text-xl font-bold mt-6 mb-2">Sharing links</h2>
        <p>Share your store link on social media or with friends. The more places you put your link, the more people will visit.</p>
      </>
    )
  },
  'account-billing': {
    title: 'Account & Billing',
    content: (
      <>
        <p>Keep your account details up to date and manage your plan here.</p>
        <h2 className="text-xl font-bold mt-6 mb-2">Your plan</h2>
        <p>You can see what plan you are on and how much it costs. If you need more features, you can upgrade your plan at any time.</p>
        <h2 className="text-xl font-bold mt-6 mb-2">Viewing your bills</h2>
        <p>All your past bills are stored here. You can look at them or print them out for your records.</p>
        <h2 className="text-xl font-bold mt-6 mb-2">Adding team members</h2>
        <p>If you have people helping you run your business, you can invite them to your account. You can choose what they are allowed to see and do.</p>
      </>
    )
  }
};

export default function HelpArticlePage({ params }: { params: { slug: string } }) {
  const article = articles[params.slug];

  if (!article) {
    notFound();
  }

  return (
    <div className="min-h-screen bg-gray-50 flex justify-center font-inter">
      <div className="w-full max-w-[375px] sm:max-w-xl bg-white min-h-screen shadow-xl relative flex flex-col">
        {/* Header */}
        <header className="px-5 pt-10 pb-4 bg-white sticky top-0 z-20 border-b border-gray-200 flex items-center gap-4">
          <Link href="/dashboard" className="text-gray-500 hover:text-gray-900 transition-colors flex items-center justify-center w-10 h-10 rounded-full bg-gray-100 hover:bg-gray-200">
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
            </svg>
          </Link>
          <h1 className="text-2xl font-extrabold font-outfit text-gray-900 tracking-tight">Help Center</h1>
        </header>

        {/* Content */}
        <main className="flex-1 p-6 overflow-y-auto">
          <article className="prose prose-blue max-w-none text-gray-700">
            <h1 className="text-3xl font-extrabold text-gray-900 mb-6 font-outfit">{article.title}</h1>
            <div className="space-y-4 leading-relaxed text-sm md:text-base">
              {article.content}
            </div>
          </article>
        </main>
      </div>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
