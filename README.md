**🦀 SOLANA PAYMENT GATEWAY**


A fully on-chain Solana payment gateway built with Rust backend, PostgreSQL, and Solana Memos.
Supports:

💳 Payments via Phantom approval (with memo auto-attached)

📱 Payments via QR Scan (Solana Pay format)

⚡ Real-time transaction indexing + confirmation

🔁 Status polling & auto-update

⛓ Memo-based matching → no smart contract required

This project demonstrates how to accept SOL payments on-chain, verify them through memos, and expose them through clean APIs & a UI.


**📌 Demo Architecture:**

docs/architecture.mmd + docs/sequence_flow.mmd + docs/components.mmd


> ASCII diagram available inside docs/ folder.






| Feature                                                   | Status               |
| --------------------------------------------------------- | -------------------- |
| Create payment request                                    | ✅                    |
| Auto memo generation                                      | ✅                    |
| Store payment details in PostgreSQL                       | ✅                    |
| Redis queue + Indexer for background transaction scanning | ✅                    |
| Phantom wallet payment approval                           | ✅                    |
| QR scan payment (Solana Pay URL)                          | ✅                    |
| Auto confirmation detection                               | ✅                    |
| List & filter recent payments                             | ✅                    |
| Expiry handling for unpaid transactions                   | 🔜 (can be extended) |

Works on Solana Devnet by default — completely free to test.

**⚙️ Setup
1️⃣ Clone:**

git clone https://github.com/OGGY418/Payment-gateway-rust.git

cd Payment-gateway-rust

**2️⃣ Create .env(refer .env.example)**

**3️⃣ Start everything:**


> cargo build 

> ./start.sh


This launches automatically:

>PostgreSQL connection

>Redis

>Backend API (localhost:3000)

>Indexer


🎯 Testing Options

You can test this project in 3 different ways depending on your skill level.

**Option A: Test via API (for backend devs)**


Create payment:


curl -X POST http://localhost:3000/payments/create \
-H "Content-Type: application/json" \
-d '{"amount_lamports": 10000000, "order_id": "order_123"}'

Get payment status:

curl http://localhost:3000/payments/<PAYMENT_ID>

**💳 Option B: Phantom Wallet (no manual memo typing):**


open browser:


> public/phantom-test.html

**Workflow:**


- Connect Phantom

- Enter amount

- Click Send Payment

- Phantom auto-signs transaction + memo → Indexer confirms

**📱 Option C: QR Scan (Solana Pay)**

Open:

> public/payment.html

**Workflow:**

- Enter amount

- Generate payment QR

- Scan from Phantom / Solflare / Glow / TokenPocket

- Status updates automatically

  **📊 Payment Status Lifecycle:**

| Status      | Meaning                                    |
| ----------- | ------------------------------------------ |
| `pending`   | Payment request created, waiting for funds |
| `confirmed` | Payment received on-chain, memo matched    |
| `expired`   | Payment not received within 5 minutes      |
| `failed`    | (Reserved) for future error handling       |

**🧰 Developer Commands:**

| Task               | Command                                    |
| ------------------ | ------------------------------------------ |
| Start full stack   | `./start.sh`                               |
| Run backend only   | `cargo run`                                |
| Run indexer only   | `cargo run --bin indexer`                  |
| Run Redis manually | `redis-server`                             |
| Reset database     | `psql -d <DB_NAME> -f database/schema.sql` |


**📜 Notes for Developers**

- System works without smart contracts

- Memo + amount verification ensures one-time payments

- No private keys stored anywhere — fully non-custodial

- Works with mobile wallets + desktop wallets


> This project is not limited to one website or one product — it can be integrated into any platform that wants to accept SOL payments.


**Some example use cases:**

- ⚡️ Instant crypto payments

- 🔓 Paywall unlocks


- 🤖 Telegram/Discord bot integration


- 🧾 Subscription + membership access


- 🛒 Digital goods checkout


- 🎟 Ticketing & course access


- 💬 Tipping + donations


- 📦 SaaS micro-transactions via QR

  **🤝 Contributing**

- Contributions are welcome!

- PRs, issues & feedback are always appreciated.

**⭐ Support**

>If you like this project, star the repo — it really helps ✨

>Originally built for fun + learning Solana payments — now open sourced for the community ❤️


