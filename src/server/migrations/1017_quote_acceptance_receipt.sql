-- A local acceptance receipt binds one invoice and one checkout attempt to the
-- existing quote. Historical quotes/invoices are not guessed or backfilled.
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS acceptance_receipt JSONB;

-- Preserve exact signed line adjustments alongside legacy major-unit fields.
-- Historical values remain unknown until separately reconciled.
ALTER TABLE invoice_line_items ADD COLUMN IF NOT EXISTS unit_price_cents BIGINT;
ALTER TABLE invoice_line_items ADD COLUMN IF NOT EXISTS amount_cents BIGINT;

-- Existing workers also write quotes directly. Receipt-backed economic terms
-- stay immutable across those writers; payment-status transitions remain valid.
CREATE OR REPLACE FUNCTION ohc_quote_acceptance_guard() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    IF OLD.acceptance_receipt IS NOT NULL THEN
        IF NEW.acceptance_receipt IS NULL OR
           ROW(NEW.id,NEW.tenant_id,NEW.customer_id,NEW.status,NEW.total_amount_cents,
               NEW.required_deposit_cents,NEW.stripe_payment_link,NEW.valid_until,
               NEW.proposed_slot_id,NEW.service_id,NEW.updated_at)
           IS DISTINCT FROM
           ROW(OLD.id,OLD.tenant_id,OLD.customer_id,OLD.status,OLD.total_amount_cents,
               OLD.required_deposit_cents,OLD.stripe_payment_link,OLD.valid_until,
               OLD.proposed_slot_id,OLD.service_id,OLD.updated_at) THEN
            RAISE EXCEPTION 'accepted_quote_is_immutable';
        END IF;
    ELSIF ROW(NEW.tenant_id,NEW.customer_id,NEW.status,NEW.total_amount_cents,
              NEW.required_deposit_cents,NEW.stripe_payment_link,NEW.valid_until,
              NEW.proposed_slot_id,NEW.service_id,NEW.updated_at)
          IS DISTINCT FROM
          ROW(OLD.tenant_id,OLD.customer_id,OLD.status,OLD.total_amount_cents,
              OLD.required_deposit_cents,OLD.stripe_payment_link,OLD.valid_until,
              OLD.proposed_slot_id,OLD.service_id,OLD.updated_at) THEN
        NEW.updated_at := GREATEST(clock_timestamp(), OLD.updated_at + INTERVAL '1 microsecond');
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS ohc_quote_acceptance_guard ON quotes;
CREATE TRIGGER ohc_quote_acceptance_guard BEFORE UPDATE ON quotes
FOR EACH ROW EXECUTE FUNCTION ohc_quote_acceptance_guard();

CREATE OR REPLACE FUNCTION ohc_quote_line_review_guard() RETURNS trigger
LANGUAGE plpgsql AS $$
DECLARE
    parent RECORD;
    parent_ids TEXT[];
BEGIN
    parent_ids := CASE WHEN TG_OP='INSERT' THEN ARRAY[NEW.quote_id::text]
                       WHEN TG_OP='DELETE' THEN ARRAY[OLD.quote_id::text]
                       ELSE ARRAY[OLD.quote_id::text,NEW.quote_id::text] END;
    FOR parent IN SELECT id,acceptance_receipt FROM quotes
                  WHERE id::text=ANY(parent_ids) ORDER BY id FOR UPDATE LOOP
        IF parent.acceptance_receipt IS NOT NULL THEN
            RAISE EXCEPTION 'accepted_quote_lines_are_immutable';
        END IF;
        -- Include non-API/worker line mutations in the same observed token.
        UPDATE quotes SET updated_at=GREATEST(clock_timestamp(),updated_at+INTERVAL '1 microsecond')
        WHERE id=parent.id;
    END LOOP;
    IF TG_OP='DELETE' THEN RETURN OLD; END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS ohc_quote_line_review_guard ON quote_line_items;
CREATE TRIGGER ohc_quote_line_review_guard BEFORE INSERT OR UPDATE OR DELETE ON quote_line_items
FOR EACH ROW EXECUTE FUNCTION ohc_quote_line_review_guard();

CREATE OR REPLACE FUNCTION ohc_accepted_invoice_guard() RETURNS trigger
LANGUAGE plpgsql AS $$
DECLARE
    parent RECORD;
    parent_ids TEXT[];
BEGIN
    parent_ids := CASE WHEN TG_OP='INSERT' THEN ARRAY[NEW.quote_id::text]
                       ELSE ARRAY[OLD.quote_id::text,NEW.quote_id::text] END;
    FOR parent IN SELECT id,acceptance_receipt FROM quotes
                  WHERE id::text=ANY(parent_ids) ORDER BY id FOR UPDATE LOOP
        IF parent.acceptance_receipt IS NOT NULL THEN
            IF TG_OP='INSERT' THEN RAISE EXCEPTION 'accepted_quote_already_has_invoice'; END IF;
            IF ROW(NEW.id,NEW.tenant_id,NEW.quote_id,NEW.customer_id,NEW.client_id,
                   NEW.total_amount,NEW.total_amount_cents,NEW.currency)
               IS DISTINCT FROM
               ROW(OLD.id,OLD.tenant_id,OLD.quote_id,OLD.customer_id,OLD.client_id,
                   OLD.total_amount,OLD.total_amount_cents,OLD.currency) THEN
                RAISE EXCEPTION 'accepted_invoice_terms_are_immutable';
            END IF;
        END IF;
    END LOOP;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS ohc_accepted_invoice_guard ON invoices;
CREATE TRIGGER ohc_accepted_invoice_guard BEFORE INSERT OR UPDATE ON invoices
FOR EACH ROW EXECUTE FUNCTION ohc_accepted_invoice_guard();

CREATE OR REPLACE FUNCTION ohc_accepted_invoice_line_guard() RETURNS trigger
LANGUAGE plpgsql AS $$
DECLARE
    parent RECORD;
    invoice_ids TEXT[];
BEGIN
    invoice_ids := CASE WHEN TG_OP='INSERT' THEN ARRAY[NEW.invoice_id::text]
                        WHEN TG_OP='DELETE' THEN ARRAY[OLD.invoice_id::text]
                        ELSE ARRAY[OLD.invoice_id::text,NEW.invoice_id::text] END;
    FOR parent IN SELECT q.id,q.acceptance_receipt FROM quotes q
                  JOIN invoices i ON i.quote_id::text=q.id::text AND i.tenant_id=q.tenant_id
                  WHERE i.id::text=ANY(invoice_ids) ORDER BY q.id FOR UPDATE OF q LOOP
        IF parent.acceptance_receipt IS NOT NULL THEN
            RAISE EXCEPTION 'accepted_invoice_lines_are_immutable';
        END IF;
    END LOOP;
    IF TG_OP='DELETE' THEN RETURN OLD; END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS ohc_accepted_invoice_line_guard ON invoice_line_items;
CREATE TRIGGER ohc_accepted_invoice_line_guard BEFORE INSERT OR UPDATE OR DELETE ON invoice_line_items
FOR EACH ROW EXECUTE FUNCTION ohc_accepted_invoice_line_guard();
