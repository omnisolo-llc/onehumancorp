import React from 'react';
import Link from 'next/link';

export default function GettingStartedHelp() {
  return (
    <article className="prose prose-blue max-w-none">
      <div className="mb-8">
        <Link href="/help" className="text-sm font-medium text-blue-600 hover:text-blue-800 flex items-center mb-6">
          <svg className="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
          Back to Help Menu
        </Link>
        <h1 className="text-3xl sm:text-4xl font-extrabold text-gray-900 font-outfit mb-4">Getting Started</h1>
        <p className="text-lg text-gray-600 leading-relaxed">Learn how to easily set up your store and accept your first payment.</p>
      </div>

      <div className="space-y-8 text-gray-800">
        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">Welcome to One Human Corp!</h2>
          <p className="mb-4">
            We are so happy you are here! Starting a business is a big step, and we are here to help you every step of the way. Think of us as your digital team. We handle the hard parts so you can focus on what you do best: running your business.
          </p>
        </section>

        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">1. Tell us about your business</h2>
          <p className="mb-4">
            The first step is simple. Just tell our AI (Artificial Intelligence) what you sell. You do not need to be a writer or a computer expert.
          </p>
          <ul className="list-disc pl-6 mb-4 space-y-2">
            <li>Go to the <strong>Home</strong> page.</li>
            <li>Look for the box that says "Tell us what you sell."</li>
            <li>Type a short sentence, like "I sell homemade dog treats" or "I am a local plumber."</li>
            <li>Click the <strong>Generate Store</strong> button.</li>
          </ul>
          <p>
            Our AI will take your words and build a complete online store for you in seconds! It will write the text, pick a nice layout, and set up your products.
          </p>
        </section>

        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">2. Review your new store</h2>
          <p className="mb-4">
            Once the AI is done, take a look at your new store. Do you like it? If not, you can always change things later.
          </p>
          <p>
            You can find your store by clicking on <strong>My Store</strong> in the main menu.
          </p>
        </section>

        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">3. Set up payments to get paid</h2>
          <p className="mb-4">
            Now it is time for the best part: getting paid! We use a safe system called Stripe to connect to your bank account.
          </p>
          <ul className="list-disc pl-6 mb-4 space-y-2">
            <li>Click on <strong>Setup</strong> in the main menu.</li>
            <li>Click on <strong>Connect Stripe</strong>.</li>
            <li>Follow the simple steps to securely link your bank. You will need your bank account number and routing number.</li>
          </ul>
          <p className="bg-blue-50 p-4 rounded-xl border border-blue-100 mt-4 text-sm">
            <strong>Tip:</strong> Don't worry, your bank info is totally secure. We don't store it, and neither do your customers.
          </p>
        </section>

        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">4. Launch your store!</h2>
          <p className="mb-4">
            Once you are happy with how your store looks and your bank is connected, you are ready to go live!
          </p>
          <p>
            Click the big green <strong>Launch Store</strong> button. Now anyone on the internet can find your business and buy from you. Congratulations!
          </p>
        </section>
      </div>
    </article>
  );
}
