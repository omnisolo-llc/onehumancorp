import React from 'react';
import Link from 'next/link';

export default function MarketingHelp() {
  return (
    <article className="prose prose-blue max-w-none">
      <div className="mb-8">
        <Link href="/help" className="text-sm font-medium text-blue-600 hover:text-blue-800 flex items-center mb-6">
          <svg className="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
          Back to Help Menu
        </Link>
        <h1 className="text-3xl sm:text-4xl font-extrabold text-gray-900 font-outfit mb-4">Finding Customers</h1>
        <p className="text-lg text-gray-600 leading-relaxed">Send emails to customers and grow your business easily.</p>
      </div>

      <div className="space-y-8 text-gray-800">
        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">Growing Your Business</h2>
          <p className="mb-4">
            Having a great store is only the first step. Next, you need people to visit it! We have easy tools to help you reach out to your customers and keep them coming back.
          </p>
        </section>

        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">Sending Emails (Campaigns)</h2>
          <p className="mb-4">
            Sending an email is a great way to tell people about a sale or a new product.
          </p>
          <ul className="list-disc pl-6 mb-4 space-y-2">
            <li>Go to the <strong>Campaigns</strong> menu.</li>
            <li>Click <strong>Create New Email</strong>.</li>
            <li>You can type the email yourself, or use your <strong>AI Marketing Helper</strong> to write it for you!</li>
            <li>Choose who should get the email (like all past customers).</li>
            <li>Click <strong>Send</strong>.</li>
          </ul>
        </section>

        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">Using AI to Write for You</h2>
          <p className="mb-4">
            Not sure what to say in an email or social media post? Your AI Helper is an expert writer.
          </p>
          <p className="mb-4">
            Go to your <strong>Agents</strong> page and ask your helper something like: <em>"Can you write a short Facebook post saying we have a weekend sale on all coffee mugs?"</em>
          </p>
          <p>
            The helper will give you a great post that you can copy and paste right into Facebook or Instagram!
          </p>
        </section>

        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">Getting Reviews</h2>
          <p className="mb-4">
            When new customers see good reviews, they are more likely to buy.
          </p>
          <p>
            You can set up our system to automatically email people a few days after they buy something, politely asking if they would leave a review. Go to the <strong>Review Campaigns</strong> page to turn this on. It is an easy way to build trust without doing any extra work.
          </p>
        </section>
      </div>
    </article>
  );
}
