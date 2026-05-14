# Connecting a Custom Domain Name

When you first create your store, you get a free web address that looks like this: `your-store-name.onehumancorp.com`.

If you want your business to look more professional, you should buy your own domain name (like `www.your-store-name.com`) and connect it to your store.

## Step 1: Buy a Domain Name

OneHumanCorp does not sell domain names. You need to buy one from a "Domain Registrar." Popular options include:
- Google Domains
- GoDaddy
- Namecheap

Go to one of those websites, search for the name you want, and buy it. It usually costs about \$10 to \$20 per year.

## Step 2: Tell OHC About Your Domain

Once you own the name, you need to tell our system to expect it.

1. Go to **My Store > Settings > Domains**.
2. Click **Connect Existing Domain**.
3. Type in the exact name you bought (e.g., `mybakery.com`).
4. Click **Next**.

## Step 3: Change Your DNS Settings

This is the only technical part, but you can do it! You need to go back to the website where you bought the domain and point it towards OneHumanCorp's servers.

1. Log in to your Domain Registrar (e.g., GoDaddy).
2. Find the "DNS Settings" or "DNS Management" page for your domain.
3. You need to create an **A Record**.
   - Set the Name/Host to `@`.
   - Set the Value/Target to `192.168.1.100` *(Note: use the exact IP address shown on your OHC Domain settings page)*.
4. You also need to create a **CNAME Record**.
   - Set the Name/Host to `www`.
   - Set the Value/Target to `shops.onehumancorp.com`.
5. Save your changes.

## Step 4: Wait Patiently

DNS changes do not happen instantly. It is like telling the post office you moved—it takes time for the message to spread around the world.

It usually takes between 1 hour and 24 hours for your new domain to start working.

You can check the status on the **My Store > Settings > Domains** page. When the status changes to "Connected," you are good to go! All your customers will now see your professional custom web address.
