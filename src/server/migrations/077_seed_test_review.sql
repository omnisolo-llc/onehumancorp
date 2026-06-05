-- Seed test reviews for reputation E2E
-- Issue #24084

INSERT INTO reviews (id, tenant_id, customer_id, order_id, rating, comment)
VALUES ('test-review-1', 'default', 'test-cust', 'test-order', 5, 'Great service! Highly recommended.')
ON CONFLICT (id) DO NOTHING;

INSERT INTO review_replies (id, tenant_id, review_id, content, status)
VALUES ('test-reply-1', 'default', 'test-review-1', 'Thank you so much! The Publicist drafted this.', 'Drafted')
ON CONFLICT (id) DO NOTHING;

INSERT INTO reputation_profiles (id, tenant_id, average_rating, total_reviews)
VALUES ('test-rep-1', 'default', 5.0, 1)
ON CONFLICT (tenant_id) DO UPDATE SET total_reviews = reputation_profiles.total_reviews;
