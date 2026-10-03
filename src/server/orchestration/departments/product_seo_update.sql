UPDATE products
SET seo_title = $1, seo_description = $2, seo_schema_json = $3
WHERE tenant_id = $4 AND id = $5
  AND title = $6
  AND COALESCE(description, '') = $7
  AND COALESCE(type, 'Product') = $8
  AND COALESCE(price_cents, 0) = $9
