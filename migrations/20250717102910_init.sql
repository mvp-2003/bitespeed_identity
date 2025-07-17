-- Initial schema for BiteSpeed Identity Reconciliation
CREATE TABLE IF NOT EXISTS contacts (
    id BIGSERIAL PRIMARY KEY,
    email TEXT,
    phone_number TEXT,
    linked_id BIGINT REFERENCES contacts(id) ON DELETE SET NULL,
    link_precedence TEXT NOT NULL CHECK (link_precedence IN ('primary','secondary')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_contacts_email ON contacts(email);
CREATE INDEX IF NOT EXISTS idx_contacts_phone ON contacts(phone_number);
CREATE INDEX IF NOT EXISTS idx_contacts_linked ON contacts(linked_id);
