export default function MyStore() {
  return (
    <div className="max-w-3xl mx-auto p-6 mt-10 bg-white rounded-xl shadow-sm border border-gray-100 font-inter">
      <h1 className="text-3xl font-extrabold font-outfit text-gray-900 mb-6">Managing My Store</h1>

      <p className="text-gray-700 mb-4 leading-relaxed text-lg">
        Your store is where you show off what you sell. You can easily add new items, keep track of what you have in stock, and change how your store looks.
      </p>

      <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Adding Products</h2>
      <p className="text-gray-700 mb-4">
        To add a new item, go to the products page and click "Add Product". You can upload a picture, type in a name and description, and set the price. Our AI can even help you write a catchy description!
      </p>

      <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Tracking Your Stock</h2>
      <p className="text-gray-700 mb-4">
        When you add a product, you can tell the system how many you have to sell. When someone buys it, the number goes down automatically. This helps you know when you need to make or buy more.
      </p>

      <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Changing How Your Store Looks</h2>
      <p className="text-gray-700 mb-4">
        You can pick different colors, fonts, and layouts to make your store match your brand. Just go to the Storefront Builder to try out different styles.
      </p>
    </div>
  );
}