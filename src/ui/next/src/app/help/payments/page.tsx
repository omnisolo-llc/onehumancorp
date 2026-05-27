import React from 'react';
import Link from 'next/link';

export default function PaymentsHelp() {
  return (
    <article className="prose prose-blue max-w-none">
      <div className="mb-8">
        <Link href="/help" className="text-sm font-medium text-blue-600 hover:text-blue-800 flex items-center mb-6">
          <svg className="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
          Back to Help Menu
        </Link>
        <h1 className="text-3xl sm:text-4xl font-extrabold text-gray-900 font-outfit mb-4">Getting Paid</h1>
        <p className="text-lg text-gray-600 leading-relaxed">Set up how you get paid, view deposits, and handle simple taxes.</p>
      </div>

      <div className="space-y-8 text-gray-800">
        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">How Getting Paid Works</h2>
          <p className="mb-4">
            When a customer buys something from your online store, they pay with their credit card or debit card. We securely process that payment and then move the money into your bank account.
          </p>
        </section>

        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">Setting up your Bank Account</h2>
          <p className="mb-4">
            To get your money, you must connect a bank account. We partner with Stripe, a very safe and trusted company, to handle this securely.
          </p>
          <ul className="list-disc pl-6 mb-4 space-y-2">
            <li>Go to the <strong>Setup</strong> menu.</li>
            <li>Click on <strong>Connect Bank (Stripe)</strong>.</li>
            <li>You will leave our site for a moment to go to Stripe's safe setup page.</li>
            <li>Follow the steps on the screen. You will need your bank account number and routing number (you can find these on a check or in your bank app).</li>
            <li>Once you finish, you will come back to our site. That's it!</li>
          </ul>
        </section>

        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">When do I get my money?</h2>
          <p className="mb-4">
            After a customer pays, it takes a little time for the money to clear the banks.
          </p>
          <p className="mb-4">
            Usually, the money will show up in your bank account in <strong>2 to 3 business days</strong>. Weekends and holidays can make it take a bit longer.
          </p>
          <p>
            You can check the <strong>Dashboard</strong> page to see a list of recent orders and when the money is scheduled to reach your bank.
          </p>
        </section>

        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">Understanding Processing Fees</h2>
          <p className="mb-4">
            Like all credit card machines, there is a small fee every time a customer pays with a card. This fee pays the credit card companies (like Visa or Mastercard) and keeps the system secure.
          </p>
          <p className="mb-4">
            The fee is automatically taken out of the total before the money goes into your bank. You do not need to pay a separate bill for these fees.
          </p>
          <p className="bg-blue-50 p-4 rounded-xl border border-blue-100 mt-4 text-sm">
            <strong>Example:</strong> If a customer buys a $10 item, the fee is taken from the $10, and you will receive the rest (around $9.40 depending on your plan) in your bank account.
          </p>
        </section>
      </div>
    </article>
  );
}
