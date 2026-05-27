import React from 'react';
import Link from 'next/link';

export default function AccountBillingHelp() {
  return (
    <article className="prose prose-blue max-w-none">
      <div className="mb-8">
        <Link href="/help" className="text-sm font-medium text-blue-600 hover:text-blue-800 flex items-center mb-6">
          <svg className="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
          Back to Help Menu
        </Link>
        <h1 className="text-3xl sm:text-4xl font-extrabold text-gray-900 font-outfit mb-4">Account & Billing</h1>
        <p className="text-lg text-gray-600 leading-relaxed">View your bills, manage your plan, and invite team members.</p>
      </div>

      <div className="space-y-8 text-gray-800">
        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">Managing Your Plan</h2>
          <p className="mb-4">
            You can easily check what plan you are on, upgrade for more features, or update your credit card on file.
          </p>
          <ul className="list-disc pl-6 mb-4 space-y-2">
            <li>Go to the <strong>Account</strong> menu in the top corner.</li>
            <li>Click on <strong>Billing & Plans</strong>.</li>
            <li>Here, you will see your current plan and when it renews.</li>
            <li>To change your credit card, click <strong>Update Payment Method</strong>.</li>
          </ul>
        </section>

        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">Viewing Past Bills</h2>
          <p className="mb-4">
            Need a receipt for your taxes? You can print out all your past bills.
          </p>
          <ul className="list-disc pl-6 mb-4 space-y-2">
            <li>On the <strong>Billing & Plans</strong> page, scroll down to the bottom.</li>
            <li>You will see a list of <strong>Past Invoices</strong>.</li>
            <li>Click on the date to view or download a copy of the bill.</li>
          </ul>
        </section>

        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">Adding Team Members</h2>
          <p className="mb-4">
            If you have employees or a partner who needs to log in, you can give them their own access without sharing your password.
          </p>
          <ul className="list-disc pl-6 mb-4 space-y-2">
            <li>Go to the <strong>Team</strong> page from the main menu.</li>
            <li>Click <strong>Invite Member</strong>.</li>
            <li>Type in their email address.</li>
            <li>Choose what they are allowed to do (like "Manage Store" or "View Reports").</li>
            <li>Click <strong>Send Invite</strong>. They will get an email with a link to create their own secure password.</li>
          </ul>
        </section>

        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">Earning Free Credits</h2>
          <p className="mb-4">
            Did you know you can get free credits to use our premium AI tools?
          </p>
          <p>
            Go to the <strong>Referrals</strong> page to get your special invite link. If you send that link to a friend and they sign up for a store, both you and your friend will get free credits added to your account!
          </p>
        </section>
      </div>
    </article>
  );
}
