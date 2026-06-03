INSERT INTO vendors (id, tenant_id, name, contact_info)
VALUES
  ('e2e-vendor-1', 'e2e-tenant', 'Acme Supplies', 'contact@acmesupplies.com')
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name,
    contact_info = EXCLUDED.contact_info,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO raw_materials (id, tenant_id, name, current_quantity, reorder_threshold)
VALUES
  ('e2e-rm-1', 'e2e-tenant', 'Premium Cocoa', 50, 20)
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name,
    current_quantity = EXCLUDED.current_quantity,
    reorder_threshold = EXCLUDED.reorder_threshold,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO bom_items (id, tenant_id, finished_good_id, raw_material_id, quantity_required)
VALUES
  ('e2e-bom-1', 'e2e-tenant', 'e2e-product-cake', 'e2e-rm-1', 2)
ON CONFLICT (id) DO UPDATE
SET finished_good_id = EXCLUDED.finished_good_id,
    raw_material_id = EXCLUDED.raw_material_id,
    quantity_required = EXCLUDED.quantity_required;
