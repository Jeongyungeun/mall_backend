-- Add migration script here
--! Up
CREATE TABLE IF NOT EXISTS item (
    id VARCHAR(64) PRIMARY KEY,           -- ItemId
    name VARCHAR(255) NOT NULL,           -- ItemName
    price INTEGER,                        -- ItemPrice (u32 → INTEGER, NULL 허용)
    item_type VARCHAR(32) NOT NULL,       -- ItemType (enum → string 저장)
    item_images TEXT[],                   -- ItemImage (Vec<String> → TEXT 배열)
    description TEXT,                     -- ItemDescription (option)
    updated_at TIMESTAMPTZ NOT NULL,      -- DateTime<Utc>
    created_at TIMESTAMPTZ NOT NULL       -- DateTime<Utc>
);


--! Down
DROP TABLE IF EXISTS item;