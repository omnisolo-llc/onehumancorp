
BEGIN;
CREATE TEMP TABLE products (
  id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, title TEXT NOT NULL,
  description TEXT, type TEXT, price_cents BIGINT, seo_title TEXT,
  seo_description TEXT, seo_schema_json JSONB
) ON COMMIT DROP;
INSERT INTO products (id, tenant_id, title, description, type, price_cents)
VALUES ('cake-a', 'tenant-a', 'Cake', 'Fresh cake', 'Product', 5000),
       ('cake-b', 'tenant-b', 'Other cake', 'Private cake', 'Product', 6000);
CREATE FUNCTION pg_temp.apply_seo(TEXT,TEXT,JSONB,TEXT,TEXT,TEXT,TEXT,TEXT,BIGINT)
RETURNS BIGINT LANGUAGE SQL AS $production$
  WITH changed AS (PRODUCTION_SQL RETURNING 1) SELECT COUNT(*) FROM changed
$production$;
DO $checks$
DECLARE affected BIGINT;
BEGIN
  -- Newer edit commits and generates before an older provider result finishes.
  UPDATE products SET price_cents = 4500 WHERE id = 'cake-a';
  SELECT pg_temp.apply_seo('Current $45','Fresh','{"offers":{"price":45}}',
    'tenant-a','cake-a','Cake','Fresh cake','Product',4500) INTO affected;
  IF affected <> 1 THEN RAISE EXCEPTION 'Current snapshot was not applied'; END IF;
  SELECT pg_temp.apply_seo('Stale $50','Fresh','{"offers":{"price":50}}',
    'tenant-a','cake-a','Cake','Fresh cake','Product',5000) INTO affected;
  IF affected <> 0 THEN RAISE EXCEPTION 'Older snapshot overwrote the newer edit'; END IF;
  IF (SELECT seo_schema_json #>> '{offers,price}' FROM products WHERE id = 'cake-a') <> '45'
    THEN RAISE EXCEPTION 'Public SEO contains the obsolete price'; END IF;

  -- Every input matters even when the monetary value stays the same.
  UPDATE products SET title = 'Renamed cake' WHERE id = 'cake-a';
  SELECT pg_temp.apply_seo('Stale name','','{}','tenant-a','cake-a','Cake','Fresh cake','Product',4500) INTO affected;
  IF affected <> 0 THEN RAISE EXCEPTION 'Title is missing from snapshot guard'; END IF;
  UPDATE products SET title = 'Cake', description = 'Changed description' WHERE id = 'cake-a';
  SELECT pg_temp.apply_seo('Stale description','','{}','tenant-a','cake-a','Cake','Fresh cake','Product',4500) INTO affected;
  IF affected <> 0 THEN RAISE EXCEPTION 'Description is missing from snapshot guard'; END IF;
  UPDATE products SET description = 'Fresh cake', type = 'Service' WHERE id = 'cake-a';
  SELECT pg_temp.apply_seo('Stale type','','{}','tenant-a','cake-a','Cake','Fresh cake','Product',4500) INTO affected;
  IF affected <> 0 THEN RAISE EXCEPTION 'Type is missing from snapshot guard'; END IF;
  IF (SELECT seo_title FROM products WHERE id = 'cake-a') <> 'Current $45'
    THEN RAISE EXCEPTION 'Rejected snapshot changed metadata'; END IF;

  -- Identical supplied inputs cannot authorize a different tenant's product.
  SELECT pg_temp.apply_seo('Cross tenant','','{}','tenant-a','cake-b','Other cake','Private cake','Product',6000) INTO affected;
  IF affected <> 0 OR (SELECT seo_title IS NOT NULL FROM products WHERE id = 'cake-b')
    THEN RAISE EXCEPTION 'SEO write crossed tenant boundary'; END IF;
  SELECT pg_temp.apply_seo('Missing','','{}','tenant-a','missing','Cake','Fresh cake','Product',4500) INTO affected;
  IF affected <> 0 THEN RAISE EXCEPTION 'Missing product was reported updated'; END IF;

  -- Null fields use the same defaults as the event parser; cents remain exact.
  UPDATE products SET description = NULL, type = NULL, price_cents = 4501 WHERE id = 'cake-a';
  SELECT pg_temp.apply_seo('Null-safe current','','{"offers":{"price":45.01}}','tenant-a','cake-a','Cake','','Product',4501) INTO affected;
  IF affected <> 1 THEN RAISE EXCEPTION 'Normalized current snapshot was rejected'; END IF;
  IF (SELECT seo_schema_json #>> '{offers,price}' FROM products WHERE id = 'cake-a') <> '45.01'
    THEN RAISE EXCEPTION 'Persisted cents were not retained'; END IF;
  UPDATE products SET price_cents = NULL WHERE id = 'cake-a';
  SELECT pg_temp.apply_seo('Null price','','{"offers":{"price":0}}','tenant-a','cake-a','Cake','','Product',0) INTO affected;
  IF affected <> 1 THEN RAISE EXCEPTION 'Null cents did not use the catalog default'; END IF;
END
$checks$;
ROLLBACK;
SELECT 'Product SEO PostgreSQL regression: 7 scenarios passed';
