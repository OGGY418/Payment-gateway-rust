-- Drop old table if exists
DROP TABLE IF EXISTS payments;

-- Create new payment_requests table with correct schema
CREATE TABLE payment_requests (
    -- IDs
    id UUID PRIMARY KEY,
    
    -- Payment Details
    amount_lamports BIGINT NOT NULL,
    token_symbol TEXT DEFAULT 'SOL',
    memo TEXT UNIQUE NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'confirmed', 'expired'
    
    -- Wallet Addresses
    receiver_address TEXT NOT NULL,      -- YOUR merchant wallet
    sender_address TEXT,                 -- Customer's wallet (filled by indexer)
    
    -- Blockchain Info
    tx_sig TEXT,                         -- Transaction signature (filled by indexer)
    block_height BIGINT,                 -- Block number
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    paid_at TIMESTAMPTZ,                 -- Blockchain timestamp (filled by indexer)
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,              -- Payment deadline (15 minutes default)
    
    -- Metadata (optional)
    order_id TEXT,
    customer_email TEXT,
    metadata JSONB
);

-- Indexes for faster queries
CREATE INDEX idx_memo ON payment_requests(memo);
CREATE INDEX idx_status ON payment_requests(status);
CREATE INDEX idx_tx_sig ON payment_requests(tx_sig);
CREATE INDEX idx_sender ON payment_requests(sender_address);
CREATE INDEX idx_created_at ON payment_requests(created_at);
