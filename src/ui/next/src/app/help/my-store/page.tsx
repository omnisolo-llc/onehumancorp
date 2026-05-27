import React from 'react';
import Link from 'next/link';

export default function MyStoreHelp() {
  return (
    <article className="prose prose-blue max-w-none">
      <div className="mb-8">
        <Link href="/help" className="text-sm font-medium text-blue-600 hover:text-blue-800 flex items-center mb-6">
          <svg className="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
          Back to Help Menu
        </Link>
        <h1 className="text-3xl sm:text-4xl font-extrabold text-gray-900 font-outfit mb-4">My Store</h1>
        <p className="text-lg text-gray-600 leading-relaxed">Add products, track what's in stock, and change how your store looks.</p>
      </div>

      <div className="space-y-8 text-gray-800">
        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">Your Digital Storefront</h2>
          <p className="mb-4">
            Your online store is like a real shop, but on the internet. It is where customers come to see what you sell and buy from you. You can easily manage everything from your <strong>My Store</strong> page.
          </p>
        </section>

        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">Adding New Products</h2>
          <p className="mb-4">
            Got a new item to sell? Here is how to add it so customers can buy it:
          </p>
          <ul className="list-disc pl-6 mb-4 space-y-2">
            <li>Go to the <strong>My Store</strong> page from the main menu.</li>
            <li>Click the <strong>Add Product</strong> button.</li>
            <li>Type in the name of the item.</li>
            <li>Add a short description so people know what it is.</li>
            <li>Set the price.</li>
            <li>Upload a clear photo of the item from your phone or computer.</li>
            <li>Click <strong>Save</strong>.</li>
          </ul>
          <p>
            That's it! Your new item is now visible to customers.
          </p>
        </section>

        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">Editing Products and Tracking Stock</h2>
          <p className="mb-4">
            If you need to change a price, fix a spelling mistake, or mark an item as sold out, you can do that easily.
          </p>
          <ul className="list-disc pl-6 mb-4 space-y-2">
            <li>On the <strong>My Store</strong> page, find the item you want to change.</li>
            <li>Click on the item to edit it.</li>
            <li>Make your changes, like updating the price or changing the text.</li>
            <li>If you run out of an item, change the "Stock" number to 0. It will automatically show as "Sold Out" on your store so people don't buy something you don't have.</li>
            <li>Click <strong>Save Changes</strong>.</li>
          </ul>
        </section>

        <section>
          <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">Changing How Your Store Looks</h2>
          <p className="mb-4">
            You can make your store match your business style by changing the colors and adding your logo.
          </p>
          <ul className="list-disc pl-6 mb-4 space-y-2">
            <li>Go to the <strong>Setup</strong> page.</li>
            <li>Look for the <strong>Store Design</strong> section.</li>
            <li>Click to upload your logo picture.</li>
            <li>Pick the main colors you want for your buttons and text.</li>
            <li>The changes will show up on your store right away!</li>
          </ul>
          <p className="bg-blue-50 p-4 rounded-xl border border-blue-100 mt-4 text-sm">
            <strong>Tip:</strong> Keep it simple! A clear logo and one or two brand colors look best and make your store look very professional.
          </p>
        </section>
      </div>
    </article>
  );
}
