import React from 'react';
import Link from 'next/link';

export default function AIAgentsHelp() {
  return (
    <article className="prose prose-blue max-w-none">
      <div className="mb-8">
        <Link href="/help" className="text-sm font-medium text-blue-600 hover:text-blue-800 flex items-center mb-6">
          <svg className="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
          Back to Help Menu
        </Link>
        <h1 className="text-3xl sm:text-4xl font-extrabold text-gray-900 font-outfit mb-4">Your AI Helpers</h1>
        <p className="text-lg text-gray-600 leading-relaxed">Learn how to hire AI helpers and give them tasks to do.</p>
      </div>

      <div className="space-y-8 text-gray-800">
        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">What are AI Helpers?</h2>
          <p className="mb-4">
            Think of AI Helpers (or Agents) as your digital employees. They are smart computer programs that work for you inside the app. They can do tasks like writing emails, updating your store, or answering customer questions.
          </p>
          <p>
            They never sleep, and they are always ready to help you save time!
          </p>
        </section>

        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">How to Hire an AI Helper</h2>
          <p className="mb-4">
            You can add different helpers to your team based on what you need.
          </p>
          <ul className="list-disc pl-6 mb-4 space-y-2">
            <li>Go to the <strong>Agents</strong> page from the main menu.</li>
            <li>Click on <strong>Hire Agent</strong>.</li>
            <li>You will see a list of different helpers. For example, a "Marketing Agent" to help with emails, or a "Support Agent" to answer customer questions.</li>
            <li>Click <strong>Hire</strong> next to the one you want.</li>
          </ul>
        </section>

        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">Giving Tasks to your Helpers</h2>
          <p className="mb-4">
            Once you have a helper on your team, you can tell them what to do. You just talk to them in plain text, just like texting a friend!
          </p>
          <ul className="list-disc pl-6 mb-4 space-y-2">
            <li>On the <strong>Agents</strong> page, click on the helper you want to talk to.</li>
            <li>In the chat box, type what you need. For example: "Please write a short email telling my customers about a 10% off summer sale."</li>
            <li>Press enter to send the task.</li>
          </ul>
          <p className="mb-4">
            The helper will think about it, and then show you what they created. You can ask them to change it ("Make it sound more fun!") or say it looks good.
          </p>
        </section>

        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">How Helpers Talk to Each Other</h2>
          <p className="mb-4">
            Our system is very smart. Sometimes, a big task requires more than one helper. They will actually meet in a "Virtual Meeting Room" behind the scenes to plan the best way to help you.
          </p>
          <p>
            You don't need to do anything for this! Just give them a big task, and they will figure out how to work together to get it done for you.
          </p>
          <p className="bg-blue-50 p-4 rounded-xl border border-blue-100 mt-4 text-sm">
            <strong>Tip:</strong> The more details you give your helpers, the better job they will do. Don't be afraid to be specific!
          </p>
        </section>
      </div>
    </article>
  );
}
