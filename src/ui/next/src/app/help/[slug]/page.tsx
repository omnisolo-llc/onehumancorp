import React from 'react';
import Link from 'next/link';
import { notFound } from 'next/navigation';

type Article = {
  title: string;
  content: React.ReactNode;
};

const articles: Record<string, Article> = {
  'getting-started': {
    title: 'Getting Started',
    content: (
      <>
        <p>Welcome to OneHumanCorp! We are so excited to help you start your business.</p>

        <h3 className="text-xl font-bold text-gray-900 mt-6 mb-3 font-outfit">Step 1: Tell us about your business</h3>
        <p>When you first log in, you will see a text box. Simply type what your business does. For example, "I sell handmade soap". Our AI will build your entire store from that one sentence!</p>

        <h3 className="text-xl font-bold text-gray-900 mt-6 mb-3 font-outfit">Step 2: Check your store details</h3>
        <p>Go to the <strong>Setup</strong> tab to add your store name, logo, and a short description. This helps your customers know who they are buying from.</p>

        <h3 className="text-xl font-bold text-gray-900 mt-6 mb-3 font-outfit">Step 3: Add your first product</h3>
        <p>Go to the <strong>My Store</strong> tab and click "Add Product". Upload a nice photo, give it a name, set a price, and write a small description. Then click Save.</p>

        <h3 className="text-xl font-bold text-gray-900 mt-6 mb-3 font-outfit">Step 4: Launch!</h3>
        <p>Once you are happy with how things look, click the "Launch Store" button. Now anyone on the internet can visit your store and buy your products!</p>
      </>
    )
  },
  'my-store': {
    title: 'My Store',
    content: (
      <>
        <p>The <strong>My Store</strong> section is where you manage what you sell and how your shop looks to customers.</p>

        <h3 className="text-xl font-bold text-gray-900 mt-6 mb-3 font-outfit">Adding and Editing Products</h3>
        <p>Click "Add Product" to put a new item in your shop. You can change the price, photo, or description anytime by clicking on a product you already added.</p>

        <h3 className="text-xl font-bold text-gray-900 mt-6 mb-3 font-outfit">Tracking Stock</h3>
        <p>If you only have 10 items to sell, you can put "10" in the quantity box. We will automatically stop selling the item when you run out, so you never accidentally sell something you don't have.</p>

        <h3 className="text-xl font-bold text-gray-900 mt-6 mb-3 font-outfit">Changing Your Store Design</h3>
        <p>Want a different color? Click on "Design Settings" to pick colors that match your brand. You can also upload a banner image for the top of your store.</p>
      </>
    )
  },
  'payments': {
    title: 'Getting Paid',
    content: (
      <>
        <p>Getting paid is the most important part! We make it simple to collect money from your customers.</p>

        <h3 className="text-xl font-bold text-gray-900 mt-6 mb-3 font-outfit">Connecting Your Bank</h3>
        <p>Go to the <strong>Setup</strong> tab and click on "Payments". Follow the instructions to connect your bank account. This is where we will send your money when you make a sale.</p>

        <h3 className="text-xl font-bold text-gray-900 mt-6 mb-3 font-outfit">When do I get my money?</h3>
        <p>After a customer buys something, the money usually takes 2 to 3 business days to show up in your bank account.</p>

        <h3 className="text-xl font-bold text-gray-900 mt-6 mb-3 font-outfit">What about taxes?</h3>
        <p>You can turn on "Auto-Tax" in the payment settings. We will figure out how much tax to charge based on where your customer lives, so you don't have to worry about it.</p>
      </>
    )
  },
  'ai-agents': {
    title: 'Your AI Helpers',
    content: (
      <>
        <p>Think of AI Agents as your digital employees. They work for you 24/7 to help run your business.</p>

        <h3 className="text-xl font-bold text-gray-900 mt-6 mb-3 font-outfit">What can they do?</h3>
        <p>AI Helpers can write descriptions for your products, answer questions from your customers, and even help you design your website.</p>

        <h3 className="text-xl font-bold text-gray-900 mt-6 mb-3 font-outfit">How to give them a task</h3>
        <p>Go to the <strong>Agents</strong> tab. You can type a message like, "Write a fun description for my new blue coffee mug." The AI will reply with a ready-to-use description in seconds!</p>

        <h3 className="text-xl font-bold text-gray-900 mt-6 mb-3 font-outfit">The Support Agent</h3>
        <p>If you ever get stuck, just click the floating help button in the bottom corner of your screen. Our Support Agent is always there to answer your questions about how to use OneHumanCorp.</p>
      </>
    )
  },
  'marketing': {
    title: 'Finding Customers',
    content: (
      <>
        <p>A beautiful store is great, but you also need people to visit it! Here is how to find customers.</p>

        <h3 className="text-xl font-bold text-gray-900 mt-6 mb-3 font-outfit">Sharing Your Link</h3>
        <p>The easiest way to get customers is to share your store link on Facebook, Instagram, or in text messages to friends. Just copy the link at the top of your dashboard.</p>

        <h3 className="text-xl font-bold text-gray-900 mt-6 mb-3 font-outfit">Sending Emails</h3>
        <p>When people buy from you, we save their email address. You can use our Marketing tab to send them an email when you have a sale or a new product.</p>

        <h3 className="text-xl font-bold text-gray-900 mt-6 mb-3 font-outfit">Offering Discounts</h3>
        <p>Everyone loves a sale! You can create a discount code like "WELCOME10" that gives people 10% off their order. Share this code on your social media to encourage people to buy.</p>
      </>
    )
  },
  'account-billing': {
    title: 'Account & Billing',
    content: (
      <>
        <p>Manage the details of your OneHumanCorp account here.</p>

        <h3 className="text-xl font-bold text-gray-900 mt-6 mb-3 font-outfit">Your Subscription Plan</h3>
        <p>Go to the <strong>Account</strong> tab to see which plan you are on. You can upgrade to get more features, or update your credit card on file.</p>

        <h3 className="text-xl font-bold text-gray-900 mt-6 mb-3 font-outfit">Inviting Team Members</h3>
        <p>If you have employees or a business partner, you can invite them to your account. Click "Invite Team Member" and enter their email. They will get their own login.</p>

        <h3 className="text-xl font-bold text-gray-900 mt-6 mb-3 font-outfit">Getting Free Credits</h3>
        <p>We love when you share! If you invite another business owner to use OneHumanCorp and they sign up, we will give you free credits to use on premium AI features.</p>
      </>
    )
  }
};

export default function ArticlePage({ params }: { params: { slug: string } }) {
  const article = articles[params.slug];

  if (!article) {
    notFound();
  }

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto">

        <Link href="/help" className="inline-flex items-center text-blue-600 hover:text-blue-800 font-medium mb-8 group">
          <svg className="w-4 h-4 mr-2 transform group-hover:-translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" />
          </svg>
          Back to Help Center
        </Link>

        <div className="bg-white p-8 rounded-2xl shadow-sm border border-gray-100">
          <h1 className="text-3xl font-bold text-gray-900 mb-8 font-outfit border-b border-gray-100 pb-6">{article.title}</h1>
          <div className="prose prose-blue max-w-none text-gray-700 leading-relaxed space-y-4">
            {article.content}
          </div>
        </div>

        <div className="mt-8 text-center text-gray-500 text-sm">
          <p>Did this answer your question? If not, ask our AI Support Agent!</p>
        </div>
      </div>
    </div>
  );
}