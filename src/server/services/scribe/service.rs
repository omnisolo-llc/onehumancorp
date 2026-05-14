use tonic::{Request, Response, Status};
use crate::proto::orchestration::*;
use std::sync::Arc;
use crate::hub::Hub;

pub struct MyScribeService {
    _hub: Arc<Hub>,
}

impl MyScribeService {
    pub fn new(hub: Arc<Hub>) -> Self {
        MyScribeService { _hub: hub }
    }

    fn get_all_articles(&self) -> Vec<HelpArticle> {
        vec![
            HelpArticle {
                id: "getting-started".to_string(),
                title: "Getting Started with OneHumanCorp".to_string(),
                content: "Welcome to OneHumanCorp! This guide will help you set up your business in minutes. First, complete the Setup Wizard to define your business type and name. Once you finish the wizard, your store will be live immediately. You can access the wizard anytime from the main menu if you need to update your business profile.".to_string(),
                category: "Getting Started".to_string(),
                tags: vec!["onboarding".to_string(), "setup".to_string()],
            },
            HelpArticle {
                id: "payments-stripe".to_string(),
                title: "How to Accept Payments with Stripe".to_string(),
                content: "Stripe is our preferred payment partner for global businesses. To connect Stripe, go to the 'My Plan' page and click on 'Connect Stripe'. You will be redirected to Stripe's secure site to complete the setup. Once connected, your store will automatically show credit card payment options to your customers.".to_string(),
                category: "Payments".to_string(),
                tags: vec!["billing".to_string(), "money".to_string(), "stripe".to_string()],
            },
            HelpArticle {
                id: "payments-mercadopago".to_string(),
                title: "Accepting Payments in Latin America with Mercado Pago".to_string(),
                content: "For businesses operating in Latin America, we support Mercado Pago. You can activate this integration in the Billing settings. This allows you to accept local payment methods including credit cards and bank transfers common in the region.".to_string(),
                category: "Payments".to_string(),
                tags: vec!["billing".to_string(), "money".to_string(), "latam".to_string()],
            },
            HelpArticle {
                id: "ai-agents-intro".to_string(),
                title: "Working with AI Agents".to_string(),
                content: "AI Agents are your digital teammates. They can handle marketing, customer support, and operations. You can hire new agents from the Agents screen. Each agent has a specific role and set of skills. For example, the Marketing Pro agent can manage your social media posts, while the Customer Success agent handles inquiries in your inbox.".to_string(),
                category: "AI Agents".to_string(),
                tags: vec!["agents".to_string(), "automation".to_string()],
            },
            HelpArticle {
                id: "marketing-agent".to_string(),
                title: "How the Marketing Agent Works".to_string(),
                content: "The Marketing Pro agent is designed to help you grow your business. Once hired, it will analyze your products and create social media content for you. It can post to Instagram and Facebook automatically, and even track how many people are clicking on your links. You can review and approve all posts before they go live.".to_string(),
                category: "AI Agents".to_string(),
                tags: vec!["marketing".to_string(), "social media".to_string()],
            },
            HelpArticle {
                id: "customer-support-agent".to_string(),
                title: "Setting up Customer Support AI".to_string(),
                content: "The Customer Success agent monitors your inbox 24/7. When a customer sends a message, the AI will try to answer based on your store's information. If it doesn't know the answer, it will flag the message for your review. This ensures your customers get instant replies even while you are asleep.".to_string(),
                category: "AI Agents".to_string(),
                tags: vec!["support".to_string(), "inbox".to_string()],
            },
            HelpArticle {
                id: "storefront-customization".to_string(),
                title: "Customizing Your Storefront".to_string(),
                content: "Your storefront is where customers see your products. You can change the colors, fonts, and layout in the 'Store' section. We offer several professional templates that are optimized for both desktop and mobile devices. You don't need any design skills to make a beautiful site.".to_string(),
                category: "My Store".to_string(),
                tags: vec!["design".to_string(), "website".to_string()],
            },
            HelpArticle {
                id: "product-management".to_string(),
                title: "Adding and Managing Products".to_string(),
                content: "To add a product, click the 'Add Item' button on your dashboard. You can upload images, set a price, and write a description. You can also organize products into categories to make it easier for customers to find what they are looking for.".to_string(),
                category: "My Store".to_string(),
                tags: vec!["products".to_string(), "inventory".to_string()],
            },
            HelpArticle {
                id: "analytics-basics".to_string(),
                title: "Understanding Your Analytics".to_string(),
                content: "The Analytics section shows you how many people are visiting your site and how much you are selling. You can see which products are your bestsellers and where your traffic is coming from. Use this information to decide which products to promote or where to focus your marketing efforts.".to_string(),
                category: "Marketing".to_string(),
                tags: vec!["data".to_string(), "sales".to_string()],
            },
            HelpArticle {
                id: "mobile-experience".to_string(),
                title: "Using OHC on Your Mobile Phone".to_string(),
                content: "OneHumanCorp is built to be used on the go. You can manage your entire business from your smartphone. The interface is optimized for touch, so you can easily chat with customers, check sales, and hire agents from anywhere.".to_string(),
                category: "Getting Started".to_string(),
                tags: vec!["mobile".to_string(), "app".to_string()],
            },
            HelpArticle {
                id: "subscription-plans".to_string(),
                title: "Choosing the Right Plan".to_string(),
                content: "We offer several plans to fit businesses of all sizes. The Free plan is great for getting started, while the Pro and Business plans offer more AI agents, more storage, and priority support. You can upgrade or downgrade your plan at any time from the billing settings.".to_string(),
                category: "Account & Billing".to_string(),
                tags: vec!["pricing".to_string(), "subscription".to_string()],
            },
            HelpArticle {
                id: "data-privacy".to_string(),
                title: "How We Protect Your Data".to_string(),
                content: "Your privacy is our priority. We use industry-standard encryption to protect your business information and your customers' data. We never sell your data to third parties. For more details, you can read our full Privacy Policy in the legal section.".to_string(),
                category: "Account & Billing".to_string(),
                tags: vec!["security".to_string(), "privacy".to_string()],
            },
            HelpArticle {
                id: "domain-setup".to_string(),
                title: "Connecting a Custom Domain".to_string(),
                content: "By default, your store is available at a '.ohc.app' address. If you want a more professional look, you can connect your own domain (like 'www.yourbusiness.com'). Go to the Domain settings in the Store section to start the connection process.".to_string(),
                category: "My Store".to_string(),
                tags: vec!["domain".to_string(), "branding".to_string()],
            },
            HelpArticle {
                id: "email-notifications".to_string(),
                title: "Setting Up Email Notifications".to_string(),
                content: "Stay updated on your business with email alerts. You can choose to receive an email for every new order, new customer message, or when an AI agent needs your approval. You can customize these settings in your profile under 'Notifications'.".to_string(),
                category: "Account & Billing".to_string(),
                tags: vec!["email".to_string(), "alerts".to_string()],
            },
            HelpArticle {
                id: "inventory-tracking".to_string(),
                title: "Tracking Your Inventory".to_string(),
                content: "Never oversell again. You can enable inventory tracking for any of your products. When a customer makes a purchase, the stock level will automatically decrease. You will receive an alert when a product is running low so you can restock in time.".to_string(),
                category: "My Store".to_string(),
                tags: vec!["inventory".to_string(), "stock".to_string()],
            },
            HelpArticle {
                id: "coupons-discounts".to_string(),
                title: "Creating Coupons and Discounts".to_string(),
                content: "Boost your sales by offering discounts. You can create coupon codes that customers can enter at checkout. You can set discounts as a percentage (e.g., 10% off) or a fixed amount (e.g., $5 off). You can also set expiration dates for your coupons.".to_string(),
                category: "Marketing".to_string(),
                tags: vec!["sales".to_string(), "coupons".to_string()],
            },
            HelpArticle {
                id: "customer-reviews".to_string(),
                title: "Managing Customer Reviews".to_string(),
                content: "Social proof is key to building trust. You can enable reviews on your product pages. Customers will receive an email after their purchase asking them to leave a rating and a comment. You can review all comments before they are published on your site.".to_string(),
                category: "Marketing".to_string(),
                tags: vec!["reviews".to_string(), "trust".to_string()],
            },
            HelpArticle {
                id: "shipping-options".to_string(),
                title: "Setting Up Shipping Rates".to_string(),
                content: "If you sell physical products, you need to set up shipping. You can offer free shipping, fixed rates, or calculated rates based on the customer's location. We integrate with major carriers to help you print shipping labels easily from your dashboard.".to_string(),
                category: "My Store".to_string(),
                tags: vec!["shipping".to_string(), "delivery".to_string()],
            },
            HelpArticle {
                id: "tax-settings".to_string(),
                title: "Configuring Taxes for Your Sales".to_string(),
                content: "Handling taxes can be complex. OHC helps by automatically calculating sales tax based on your business location and the customer's shipping address. You should consult with a tax professional to ensure you are compliant with local laws.".to_string(),
                category: "Account & Billing".to_string(),
                tags: vec!["taxes".to_string(), "legal".to_string()],
            },
            HelpArticle {
                id: "team-access".to_string(),
                title: "Inviting Your Team Members".to_string(),
                content: "You don't have to run your business alone. You can invite your employees or partners to join your OHC account. You can assign different roles to control what each person can see and do. This is separate from hiring AI agents.".to_string(),
                category: "Account & Billing".to_string(),
                tags: vec!["team".to_string(), "collaboration".to_string()],
            },
            HelpArticle {
                id: "multi-currency".to_string(),
                title: "Selling in Multiple Currencies".to_string(),
                content: "Reach customers worldwide by accepting payments in different currencies. You can set a base currency for your store, and OHC will handle the conversion for your customers using real-time exchange rates. This ensures you always get paid the correct amount while making it easy for international buyers.".to_string(),
                category: "Payments".to_string(),
                tags: vec!["international".to_string(), "currency".to_string()],
            },
            HelpArticle {
                id: "social-media-integration".to_string(),
                title: "Connecting Your Social Media Accounts".to_string(),
                content: "Link your Instagram, Facebook, and Twitter accounts to OHC. This allows your Marketing AI agent to post updates, share products, and engage with your followers directly from your dashboard. Go to the 'Integrations' section to get started.".to_string(),
                category: "Marketing".to_string(),
                tags: vec!["social".to_string(), "instagram".to_string(), "facebook".to_string()],
            },
            HelpArticle {
                id: "advanced-seo".to_string(),
                title: "Optimizing Your Store for Search Engines (SEO)".to_string(),
                content: "Help more people find your business on Google. You can add meta titles, descriptions, and keywords to every product and page. Our AI also automatically generates SEO-friendly URLs and image alt tags to improve your ranking without any extra work on your part.".to_string(),
                category: "Marketing".to_string(),
                tags: vec!["google".to_string(), "seo".to_string(), "traffic".to_string()],
            },
            HelpArticle {
                id: "custom-receipts".to_string(),
                title: "Customizing Your Customer Receipts".to_string(),
                content: "Make your brand stand out even after the sale. You can customize the emails your customers receive after making a purchase. Add your logo, a personal thank you note, and even a discount code for their next order. These settings are available in the 'Store' -> 'Emails' section.".to_string(),
                category: "My Store".to_string(),
                tags: vec!["branding".to_string(), "emails".to_string(), "customer service".to_string()],
            },
            HelpArticle {
                id: "data-exports".to_string(),
                title: "Exporting Your Business Data".to_string(),
                content: "Your data belongs to you. You can export your list of customers, orders, and products as a CSV file at any time. This is useful for doing your own accounting or moving your data to another tool. Look for the 'Export' button in each respective section of the dashboard.".to_string(),
                category: "Account & Billing".to_string(),
                tags: vec!["data".to_string(), "csv".to_string(), "accounting".to_string()],
            },
            HelpArticle {
                id: "mobile-notifications".to_string(),
                title: "Push Notifications on Your Phone".to_string(),
                content: "Never miss a sale. Enable push notifications in the OHC mobile app to get instant alerts on your phone. You'll know the moment a customer places an order or asks a question. You can toggle these alerts in the 'Settings' tab of the mobile app.".to_string(),
                category: "Getting Started".to_string(),
                tags: vec!["mobile".to_string(), "notifications".to_string(), "alerts".to_string()],
            },
            HelpArticle {
                id: "loyalty-programs".to_string(),
                title: "Setting Up a Loyalty Program".to_string(),
                content: "Encourage repeat business by rewarding your best customers. You can set up a simple points-based system where customers earn credit for every dollar they spend. They can then use these points for discounts on future purchases. Enable this in the 'Marketing' -> 'Loyalty' section.".to_string(),
                category: "Marketing".to_string(),
                tags: vec!["loyalty".to_string(), "rewards".to_string(), "retention".to_string()],
            },
            HelpArticle {
                id: "store-themes".to_string(),
                title: "Changing Your Store Theme".to_string(),
                content: "Refresh your store's look with a new theme. We provide a variety of professional templates designed for different industries. Whether you're selling handmade crafts or professional services, there's a theme for you. Switching themes is safe and won't delete your products or content.".to_string(),
                category: "My Store".to_string(),
                tags: vec!["design".to_string(), "themes".to_string(), "look".to_string()],
            },
            HelpArticle {
                id: "api-keys".to_string(),
                title: "Managing Your API Keys".to_string(),
                content: "For advanced users, API keys allow you to connect OHC to custom-built software. You can generate and manage these keys in the 'Software' section. Keep your keys secret! If a key is compromised, you should revoke it immediately and generate a new one.".to_string(),
                category: "Advanced".to_string(),
                tags: vec!["api".to_string(), "developer".to_string(), "security".to_string()],
            },
            HelpArticle {
                id: "bulk-actions".to_string(),
                title: "Using Bulk Actions for Products".to_string(),
                content: "Save time by managing multiple products at once. You can select several items from your product list and update their prices, categories, or inventory status in one click. Look for the checkboxes next to your products to reveal the bulk action menu.".to_string(),
                category: "My Store".to_string(),
                tags: vec!["products".to_string(), "management".to_string(), "productivity".to_string()],
            },
            HelpArticle {
                id: "refund-process".to_string(),
                title: "How to Issue a Refund".to_string(),
                content: "Sometimes things don't work out. To issue a refund, find the order in the 'Orders' list and click 'Issue Refund'. You can choose to refund the full amount or a partial amount. The funds will be returned to the customer's original payment method automatically.".to_string(),
                category: "Payments".to_string(),
                tags: vec!["refunds".to_string(), "returns".to_string(), "customer service".to_string()],
            },
            HelpArticle {
                id: "holiday-mode".to_string(),
                title: "Enabling Holiday Mode".to_string(),
                content: "Taking a break? Enable 'Holiday Mode' to temporarily pause your store. Your products will still be visible, but customers won't be able to place new orders. You can add a custom message to your banner to let them know when you'll be back.".to_string(),
                category: "My Store".to_string(),
                tags: vec!["vacation".to_string(), "pause".to_string(), "break".to_string()],
            },
            HelpArticle {
                id: "abandoned-carts".to_string(),
                title: "Recovering Abandoned Carts".to_string(),
                content: "Bring back customers who left without buying. OHC can automatically send a friendly email reminder to people who added items to their cart but didn't finish checkout. You can even include a small discount code to sweeten the deal and win the sale.".to_string(),
                category: "Marketing".to_string(),
                tags: vec!["sales".to_string(), "recovery".to_string(), "email".to_string()],
            },
            HelpArticle {
                id: "subscription-management".to_string(),
                title: "Managing Recurring Subscriptions".to_string(),
                content: "If you sell subscription-based services, you can manage them all in the 'Subscriptions' tab. See which customers have active plans, view upcoming renewals, and handle cancellations or plan changes easily. Your AI agent can also help identify customers who might be likely to cancel.".to_string(),
                category: "Payments".to_string(),
                tags: vec!["subscriptions".to_string(), "recurring".to_string(), "revenue".to_string()],
            },
            HelpArticle {
                id: "custom-forms".to_string(),
                title: "Creating Custom Contact Forms".to_string(),
                content: "Gather exactly the information you need from your customers. You can build custom forms for your 'Contact Us' page or for specific services. Add text fields, checkboxes, and dropdowns. Submissions will go directly to your OHC inbox for your review.".to_string(),
                category: "My Store".to_string(),
                tags: vec!["forms".to_string(), "contact".to_string(), "data collection".to_string()],
            },
            HelpArticle {
                id: "gift-cards".to_string(),
                title: "Selling Digital Gift Cards".to_string(),
                content: "Allow your customers to share your store with their friends. You can create digital gift cards of any value. When purchased, the customer receives a unique code via email which can be redeemed at checkout. You can track all issued gift cards and their remaining balances in the 'Payments' section.".to_string(),
                category: "Payments".to_string(),
                tags: vec!["gift cards".to_string(), "credits".to_string(), "sales".to_string()],
            },
            HelpArticle {
                id: "wholesale-pricing".to_string(),
                title: "Setting Up Wholesale Pricing".to_string(),
                content: "Do you sell in bulk? You can create special pricing for wholesale customers. You can define a minimum order quantity and a discounted price per unit. You can also mark specific customers as 'Wholesale' so they automatically see these lower prices when logged in.".to_string(),
                category: "Payments".to_string(),
                tags: vec!["wholesale".to_string(), "b2b".to_string(), "discounts".to_string()],
            },
            HelpArticle {
                id: "shipping-labels".to_string(),
                title: "Printing Shipping Labels".to_string(),
                content: "Save a trip to the post office. OHC allows you to purchase and print shipping labels directly from your dashboard. We support major carriers like USPS, UPS, and DHL. Your order information is automatically pulled into the label, and tracking numbers are sent to your customers as soon as the label is printed.".to_string(),
                category: "My Store".to_string(),
                tags: vec!["shipping".to_string(), "logistics".to_string(), "fulfillment".to_string()],
            },
            HelpArticle {
                id: "returns-exchanges".to_string(),
                title: "Managing Returns and Exchanges".to_string(),
                content: "Make returns easy for you and your customers. You can create a self-service return portal where customers can start a return request. You can then approve the request, generate a return label, and track the item as it comes back to you. Once received, you can issue a refund or send out an exchange item.".to_string(),
                category: "My Store".to_string(),
                tags: vec!["returns".to_string(), "exchanges".to_string(), "customer service".to_string()],
            },
            HelpArticle {
                id: "multi-language".to_string(),
                title: "Translating Your Store to Multiple Languages".to_string(),
                content: "Speak your customers' language. OHC supports multi-language storefronts. You can add translations for your product names, descriptions, and site navigation. Our AI can even provide a first-pass translation for you. Customers can then switch between languages using a toggle on your site.".to_string(),
                category: "My Store".to_string(),
                tags: vec!["international".to_string(), "language".to_string(), "localization".to_string()],
            },
            HelpArticle {
                id: "seo-checklists".to_string(),
                title: "Your SEO Launch Checklist".to_string(),
                content: "Before you go live, follow this checklist to ensure your store is optimized for search engines: 1. Add a meta title and description for your homepage. 2. Ensure every product has a unique description. 3. Upload 'Alt Text' for all images. 4. Connect your Google Search Console. Our Marketing agent can help automate many of these steps.".to_string(),
                category: "Marketing".to_string(),
                tags: vec!["seo".to_string(), "checklist".to_string(), "launch".to_string()],
            },
            HelpArticle {
                id: "social-selling".to_string(),
                title: "Selling Directly on Instagram and Facebook".to_string(),
                content: "Don't just post, sell! With OHC, you can sync your product catalog with Instagram and Facebook Shops. This allows customers to browse and buy your products without ever leaving their favorite social media apps. All orders are still managed in your OHC dashboard.".to_string(),
                category: "Marketing".to_string(),
                tags: vec!["social selling".to_string(), "instagram shops".to_string(), "facebook shops".to_string()],
            },
            HelpArticle {
                id: "email-campaigns".to_string(),
                title: "Running Email Marketing Campaigns".to_string(),
                content: "Keep your customers coming back with beautiful email campaigns. You can use our drag-and-drop builder to create newsletters, product announcements, and special offers. You can segment your customer list based on their purchase history to send more targeted messages.".to_string(),
                category: "Marketing".to_string(),
                tags: vec!["email marketing".to_string(), "newsletters".to_string(), "campaigns".to_string()],
            },
            HelpArticle {
                id: "webhook-integrations".to_string(),
                title: "Advanced: Using Webhooks".to_string(),
                content: "Automate your workflows with webhooks. You can set up OHC to send real-time notifications to other apps (like Slack or Zapier) whenever a specific event occurs, such as a new order or a new customer message. This is an advanced feature found in the 'Software' -> 'Webhooks' section.".to_string(),
                category: "Advanced".to_string(),
                tags: vec!["webhooks".to_string(), "automation".to_string(), "developer".to_string()],
            },
            HelpArticle {
                id: "custom-css".to_string(),
                title: "Adding Custom CSS to Your Store".to_string(),
                content: "For full control over your store's design, you can add custom CSS. This allows you to fine-tune your layout, change any font or color, and add unique animations. Go to 'Store' -> 'Advanced Design' to inject your custom code. We recommend only using this if you are comfortable with CSS or have a designer to help.".to_string(),
                category: "Advanced".to_string(),
                tags: vec!["css".to_string(), "design".to_string(), "developer".to_string()],
            },
            HelpArticle {
                id: "pos-integration".to_string(),
                title: "Using OHC as a Point of Sale (POS)".to_string(),
                content: "Sell in person as easily as you sell online. OHC can be used as a Point of Sale system on your tablet or smartphone. Sync your inventory across all channels, so you never sell the same item twice. Connect compatible card readers to accept physical payments at your storefront or pop-up shop.".to_string(),
                category: "Payments".to_string(),
                tags: vec!["pos".to_string(), "in-person".to_string(), "retail".to_string()],
            },
            HelpArticle {
                id: "subscription-billing-cycles".to_string(),
                title: "Understanding Subscription Billing Cycles".to_string(),
                content: "When you sell a subscription, OHC automatically handles the recurring billing. You can choose cycles like weekly, monthly, or yearly. Customers are charged on the same day each period. If a payment fails, OHC will automatically retry three times before notifying you. You can see the status of all upcoming charges in the Subscriptions dashboard.".to_string(),
                category: "Payments".to_string(),
                tags: vec!["billing".to_string(), "recurring".to_string(), "revenue".to_string()],
            },
            HelpArticle {
                id: "custom-domain-email".to_string(),
                title: "Setting Up Email with Your Custom Domain".to_string(),
                content: "Look more professional with an email address that matches your domain (e.g., info@yourbusiness.com). OHC provides integrated email hosting options. Once your domain is connected, you can create up to 5 custom mailboxes. We guide you through setting up the MX records required to send and receive mail reliably.".to_string(),
                category: "My Store".to_string(),
                tags: vec!["email".to_string(), "domain".to_string(), "branding".to_string()],
            },
            HelpArticle {
                id: "marketing-analytics-funnel".to_string(),
                title: "Using the Sales Funnel Analytics".to_string(),
                content: "The sales funnel shows you exactly where your customers are dropping off. You can see how many people visited your site, how many added items to their cart, and how many completed a purchase. If you see a big drop at the cart stage, you might want to review your shipping rates or checkout process. This data helps you make informed decisions to increase your conversion rate.".to_string(),
                category: "Marketing".to_string(),
                tags: vec!["data".to_string(), "conversion".to_string(), "analytics".to_string()],
            },
            HelpArticle {
                id: "automated-discount-rules".to_string(),
                title: "Setting Up Automated Discount Rules".to_string(),
                content: "Run sales without the manual work. You can set up rules that apply discounts automatically when certain conditions are met, such as 'Buy 2 Get 1 Free' or '15% off orders over $100'. These discounts will be clearly visible to customers on the product pages and applied at checkout. You can schedule these rules to start and end at specific times.".to_string(),
                category: "Marketing".to_string(),
                tags: vec!["sales".to_string(), "discounts".to_string(), "automation".to_string()],
            },
            HelpArticle {
                id: "local-seo-optimization".to_string(),
                title: "Optimizing for Local Customers".to_string(),
                content: "If you have a physical location, local SEO is crucial. Make sure your business address and phone number are correct in your profile. OHC will automatically generate local schema markup for your site, making it easier for Google to show your business to people searching nearby. You can also add a Google Maps widget to your 'Contact' page with one click.".to_string(),
                category: "Marketing".to_string(),
                tags: vec!["local".to_string(), "seo".to_string(), "maps".to_string()],
            },
            HelpArticle {
                id: "bulk-product-import".to_string(),
                title: "Importing Products in Bulk via CSV".to_string(),
                content: "Moving from another platform? You can import hundreds of products at once using our CSV import tool. Download our template, fill in your product details (names, descriptions, prices, image URLs), and upload it. OHC will validate your data and create the products for you in minutes. If there are any errors, we'll provide a detailed report to help you fix them.".to_string(),
                category: "My Store".to_string(),
                tags: vec!["import".to_string(), "csv".to_string(), "migration".to_string()],
            },
            HelpArticle {
                id: "customer-tags-segmentation".to_string(),
                title: "Using Customer Tags for Segmentation".to_string(),
                content: "Organize your customers to provide better service. You can add tags to customer profiles, such as 'VIP', 'Frequent Buyer', or 'Wholesale'. You can then filter your customer list by these tags to send targeted email campaigns or offer special rewards. AI agents also use these tags to tailor their interactions with your customers.".to_string(),
                category: "Marketing".to_string(),
                tags: vec!["customers".to_string(), "crm".to_string(), "segmentation".to_string()],
            },
            HelpArticle {
                id: "transactional-email-customization".to_string(),
                title: "Customizing Transactional Emails".to_string(),
                content: "Transactional emails include order confirmations, shipping updates, and password resets. You can edit the text of these emails to match your brand's voice. We provide a simple editor with placeholders for customer names and order numbers. You can also send a test email to yourself to make sure everything looks perfect before your customers see it.".to_string(),
                category: "My Store".to_string(),
                tags: vec!["emails".to_string(), "branding".to_string(), "notifications".to_string()],
            },
            HelpArticle {
                id: "store-access-control".to_string(),
                title: "Setting Up Staff Permissions".to_string(),
                content: "Security is vital for your business. When you invite team members, you can choose exactly what they can access. For example, you can allow a warehouse worker to manage orders and inventory but prevent them from seeing your financial reports. You can update these permissions at any time from the 'Team' settings.".to_string(),
                category: "Account & Billing".to_string(),
                tags: vec!["security".to_string(), "team".to_string(), "permissions".to_string()],
            },
            HelpArticle {
                id: "two-factor-authentication".to_string(),
                title: "Securing Your Account with 2FA".to_string(),
                content: "Protect your business from unauthorized access by enabling Two-Factor Authentication (2FA). When enabled, you'll need to enter a code from your phone whenever you log in from a new device. We support standard authenticator apps like Google Authenticator or Authy. You can enable this in your profile security settings.".to_string(),
                category: "Account & Billing".to_string(),
                tags: vec!["security".to_string(), "login".to_string(), "2fa".to_string()],
            },
        ]
    }
}

#[tonic::async_trait]
impl crate::proto::orchestration::scribe_service_server::ScribeService for MyScribeService {
    async fn get_help_articles(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<HelpArticlesResponse>, Status> {
        Ok(Response::new(HelpArticlesResponse {
            articles: self.get_all_articles(),
        }))
    }

    async fn search_help(
        &self,
        request: Request<SearchHelpRequest>,
    ) -> Result<Response<HelpArticlesResponse>, Status> {
        let query = request.into_inner().query.to_lowercase();
        let articles = self.get_all_articles()
            .into_iter()
            .filter(|a| a.title.to_lowercase().contains(&query) || a.content.to_lowercase().contains(&query))
            .collect();

        Ok(Response::new(HelpArticlesResponse { articles }))
    }

    async fn get_tooltips(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<TooltipsResponse>, Status> {
        let tooltips = vec![
            Tooltip {
                element_id: "hire-agent-btn".to_string(),
                text: "Add a new AI teammate to your business.".to_string(),
                page_route: "/agents".to_string(),
            },
            Tooltip {
                element_id: "billing-nav".to_string(),
                text: "Manage your earnings and payment methods.".to_string(),
                page_route: "".to_string(),
            },
            Tooltip {
                element_id: "setup-nav".to_string(),
                text: "Re-run the setup guide if you need to change your business details.".to_string(),
                page_route: "".to_string(),
            },
            Tooltip {
                element_id: "agents-nav".to_string(),
                text: "See all your active AI teammates.".to_string(),
                page_route: "".to_string(),
            },
            Tooltip {
                element_id: "api-nav".to_string(),
                text: "Advanced: Connect OHC to your other software tools.".to_string(),
                page_route: "".to_string(),
            },
            Tooltip {
                element_id: "dashboard-nav".to_string(),
                text: "View your business overview and recent activity.".to_string(),
                page_route: "".to_string(),
            },
            Tooltip {
                element_id: "pricing-nav".to_string(),
                text: "Choose a plan that fits your business growth.".to_string(),
                page_route: "".to_string(),
            },
            Tooltip {
                element_id: "my-plan-nav".to_string(),
                text: "View your current usage and billing details.".to_string(),
                page_route: "".to_string(),
            },
            Tooltip {
                element_id: "business-status-indicator".to_string(),
                text: "Your current business operational status.".to_string(),
                page_route: "/dashboard".to_string(),
            },
            Tooltip {
                element_id: "check-messages-btn".to_string(),
                text: "Open your unified customer inbox.".to_string(),
                page_route: "/dashboard".to_string(),
            },
        ];

        Ok(Response::new(TooltipsResponse { tooltips }))
    }

    async fn get_walkthrough(
        &self,
        request: Request<WalkthroughRequest>,
    ) -> Result<Response<WalkthroughResponse>, Status> {
        let id = request.into_inner().walkthrough_id;

        if id == "first-payment" {
            return Ok(Response::new(WalkthroughResponse {
                id: "first-payment".to_string(),
                name: "Accept your first payment".to_string(),
                steps: vec![
                    WalkthroughStep {
                        element_id: "billing-btn".to_string(),
                        title: "Go to Billing".to_string(),
                        content: "First, let's head over to the Billing section.".to_string(),
                        order: 1,
                    },
                    WalkthroughStep {
                        element_id: "pricing-screen".to_string(),
                        title: "Pick a Plan".to_string(),
                        content: "Choose the plan that works best for you.".to_string(),
                        order: 2,
                    },
                ],
            }));
        }

        Err(Status::not_found("Walkthrough not found"))
    }

    async fn get_video_tutorials(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<VideoTutorialsResponse>, Status> {
        let tutorials = vec![
            VideoTutorial {
                id: "setup-video".to_string(),
                title: "Setting up your store in 60 seconds".to_string(),
                url: "https://assets.ohc.app/videos/setup_guide.mp4".to_string(),
                duration_seconds: 60,
                thumbnail_url: "https://assets.ohc.app/videos/setup_thumb.jpg".to_string(),
            },
            VideoTutorial {
                id: "payments-video".to_string(),
                title: "Accepting your first payment".to_string(),
                url: "https://assets.ohc.app/videos/payments_guide.mp4".to_string(),
                duration_seconds: 85,
                thumbnail_url: "https://assets.ohc.app/videos/payments_thumb.jpg".to_string(),
            },
            VideoTutorial {
                id: "agents-video".to_string(),
                title: "How to hire and manage AI agents".to_string(),
                url: "https://assets.ohc.app/videos/agents_guide.mp4".to_string(),
                duration_seconds: 120,
                thumbnail_url: "https://assets.ohc.app/videos/agents_thumb.jpg".to_string(),
            },
            VideoTutorial {
                id: "marketing-video".to_string(),
                title: "Automating your social media marketing".to_string(),
                url: "https://assets.ohc.app/videos/marketing_guide.mp4".to_string(),
                duration_seconds: 90,
                thumbnail_url: "https://assets.ohc.app/videos/marketing_thumb.jpg".to_string(),
            },
            VideoTutorial {
                id: "domain-video".to_string(),
                title: "Connecting a custom domain to your store".to_string(),
                url: "https://assets.ohc.app/videos/domain_guide.mp4".to_string(),
                duration_seconds: 75,
                thumbnail_url: "https://assets.ohc.app/videos/domain_thumb.jpg".to_string(),
            },
            VideoTutorial {
                id: "products-video".to_string(),
                title: "Optimizing your product listings".to_string(),
                url: "https://assets.ohc.app/videos/products_guide.mp4".to_string(),
                duration_seconds: 110,
                thumbnail_url: "https://assets.ohc.app/videos/products_thumb.jpg".to_string(),
            },
            VideoTutorial {
                id: "analytics-video".to_string(),
                title: "Reading your business performance data".to_string(),
                url: "https://assets.ohc.app/videos/analytics_guide.mp4".to_string(),
                duration_seconds: 65,
                thumbnail_url: "https://assets.ohc.app/videos/analytics_thumb.jpg".to_string(),
            },
            VideoTutorial {
                id: "mobile-video".to_string(),
                title: "Managing your business on mobile".to_string(),
                url: "https://assets.ohc.app/videos/mobile_guide.mp4".to_string(),
                duration_seconds: 50,
                thumbnail_url: "https://assets.ohc.app/videos/mobile_thumb.jpg".to_string(),
            },
            VideoTutorial {
                id: "support-video".to_string(),
                title: "Setting up AI customer support".to_string(),
                url: "https://assets.ohc.app/videos/support_guide.mp4".to_string(),
                duration_seconds: 95,
                thumbnail_url: "https://assets.ohc.app/videos/support_thumb.jpg".to_string(),
            },
            VideoTutorial {
                id: "advanced-video".to_string(),
                title: "Advanced features and API access".to_string(),
                url: "https://assets.ohc.app/videos/advanced_guide.mp4".to_string(),
                duration_seconds: 130,
                thumbnail_url: "https://assets.ohc.app/videos/advanced_thumb.jpg".to_string(),
            },
        ];

        Ok(Response::new(VideoTutorialsResponse { tutorials }))
    }

    async fn get_release_notes(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<ReleaseNotesResponse>, Status> {
        let notes = vec![
            ReleaseNote {
                id: "v0.4.42".to_string(),
                version: "v0.4.42".to_string(),
                date: "2024-05-20".to_string(),
                title: "Better AI Support and Faster Payments".to_string(),
                content: "We've improved our AI agents to be even more helpful. They now understand your business context better and can provide more accurate marketing advice. Also, payments processing is now 2x faster with our new optimized gateway routing for Latin America and Europe.".to_string(),
                screenshot_url: "https://assets.ohc.app/releases/v0.4.42.jpg".to_string(),
            },
            ReleaseNote {
                id: "v0.4.41".to_string(),
                version: "v0.4.41".to_string(),
                date: "2024-05-12".to_string(),
                title: "Enhanced Mobile Dashboard".to_string(),
                content: "The dashboard is now even easier to use on small screens. We've simplified the navigation and added quick-action buttons for your most common daily tasks, like adding a new product or checking your latest messages.".to_string(),
                screenshot_url: "https://assets.ohc.app/releases/v0.4.41.jpg".to_string(),
            },
            ReleaseNote {
                id: "v0.4.40".to_string(),
                version: "v0.4.40".to_string(),
                date: "2024-05-01".to_string(),
                title: "Introducing the Marketing Pro Agent".to_string(),
                content: "Meet your new marketing teammate! The Marketing Pro agent can now automatically create social media posts based on your inventory. It helps you keep your Instagram and Facebook feeds active without you having to lift a finger.".to_string(),
                screenshot_url: "https://assets.ohc.app/releases/v0.4.40.jpg".to_string(),
            },
            ReleaseNote {
                id: "v0.4.39".to_string(),
                version: "v0.4.39".to_string(),
                date: "2024-04-20".to_string(),
                title: "Bulk Inventory Management".to_string(),
                content: "Manage your products faster. You can now select multiple products to update their stock levels, change their categories, or delete them in one go. Saving you hours of manual work every week.".to_string(),
                screenshot_url: "https://assets.ohc.app/releases/v0.4.39.jpg".to_string(),
            },
            ReleaseNote {
                id: "v0.4.38".to_string(),
                version: "v0.4.38".to_string(),
                date: "2024-04-10".to_string(),
                title: "New Store Templates".to_string(),
                content: "We've added 5 new professional store templates optimized for service businesses and creative artisans. Each template is fully responsive and looks great on mobile, tablet, and desktop.".to_string(),
                screenshot_url: "https://assets.ohc.app/releases/v0.4.38.jpg".to_string(),
            },
        ];

        Ok(Response::new(ReleaseNotesResponse { notes }))
    }

    async fn ask_help_ai(
        &self,
        request: Request<HelpChatRequest>,
    ) -> Result<Response<HelpChatResponse>, Status> {
        let msg = request.into_inner().message.to_lowercase();
        let mut response = "I'm your OHC assistant. How can I help you today?".to_string();
        let mut related_articles = vec![];

        if msg.contains("payment") || msg.contains("stripe") {
            response = "To set up payments, you should connect your Stripe or Mercado Pago account in the billing section.".to_string();
            related_articles.push(self.get_all_articles().into_iter().find(|a| a.id == "payments-stripe").unwrap());
        } else if msg.contains("agent") {
            response = "AI agents are your digital teammates. You can hire them to handle marketing or customer support.".to_string();
            related_articles.push(self.get_all_articles().into_iter().find(|a| a.id == "ai-agents-intro").unwrap());
        } else if msg.contains("setup") || msg.contains("start") {
            response = "The best way to start is by using our Setup Wizard, which guides you through creating your store.".to_string();
            related_articles.push(self.get_all_articles().into_iter().find(|a| a.id == "getting-started").unwrap());
        }

        Ok(Response::new(HelpChatResponse {
            response,
            related_articles,
        }))
    }
}
