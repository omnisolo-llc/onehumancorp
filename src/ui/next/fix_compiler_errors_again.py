import sys

def fix_file(filepath, replacements):
    with open(filepath, 'r') as f:
        content = f.read()

    for old, new in replacements:
        content = content.replace(old, new)

    with open(filepath, 'w') as f:
        f.write(content)

fix_file('src/server/lib.rs', [
    ('stripe_client.create_checkout_session(product_id, "default_customer", amount_usd, Some(lock_id.clone())).await', 'stripe_client.create_checkout_session(product_id, "default_customer", amount_usd, Some(lock_id.clone())).await')
])
