# What We've Learnt
---
## Case Study: **Bitcoin**
### Overview
Bitcoin (BTC), created by an anonymous person or group under the pseudonym _Satoshi Nakamoto_ in 2009, is the first decentralized digital currency. It allows peer-to-peer transactions without intermediaries such as banks or governments. Bitcoin’s success stems from a combination of technological innovation, sound economic principles, market timing, and strong community support.
### First-Mover Advantage
Bitcoin’s early introduction as a decentralized, peer-to-peer currency gave it a distinct advantage. Being the first successful implementation of blockchain technology allowed Bitcoin to establish itself as the “face” of cryptocurrency. Early adoption drove network effects, attracting more miners, developers, users, and merchants, reinforcing its dominance.
### Decentralization and Security
Bitcoin operates as a trustless system, eliminating reliance on centralized authorities through cryptographic proof. Its immutable blockchain records transactions in a manner resistant to censorship and fraud. The Proof of Work (PoW) consensus mechanism secures the network while incentivizing honest participation.
### Scarcity and Monetary Policy
Bitcoin’s fixed supply of 21 million coins creates digital scarcity similar to gold. Its deflationary model, with regular block reward halvings approximately every four years, reduces new issuance and may enhance long-term value. As a result, Bitcoin has often been perceived as a hedge against fiat currency inflation.
### Ideological Appeal
Bitcoin appeals to those valuing privacy, autonomy, and financial sovereignty. No single entity can control or shut down the network, making it especially attractive in regions with authoritarian regimes or unstable banking systems.
### Global Accessibility
Bitcoin is borderless, enabling anyone with internet access to participate, regardless of geography or banking status. Its divisibility allows users to own fractions of a bitcoin, lowering the barrier to entry.
### Developer and Community Support
The open-source nature of Bitcoin encourages transparency and continuous development. A robust global community of developers, miners, investors, and educators ensures security enhancements, scalability improvements, and ecosystem growth, exemplified by solutions like the Lightning Network.
### Institutional Adoption and Recognition
Growing acceptance by institutions, hedge funds, and corporations legitimizes Bitcoin. The introduction of Bitcoin ETFs, futures, and custody solutions has increased accessibility and trust. Media coverage keeps Bitcoin in the public consciousness, driving demand.
### Resilience and Longevity
Bitcoin has endured multiple market crashes and regulatory challenges while maintaining network integrity. The protocol has operated continuously since 2009 without compromise, demonstrating remarkable resilience.
### Simplicity of Protocol Design
Bitcoin’s minimalist design focuses on a single goal: storing and transferring value securely. Its limited scripting language allows basic conditions like multisignature wallets and timelocks without exposing the network to the risks of Turing-complete smart contracts. This "do one thing well" philosophy improves reliability and reduces attack surfaces.
### Predictable Monetary Policy
Bitcoin’s rules are hard-coded, with a ten-minute block interval, block reward halvings every 210,000 blocks, and a maximum supply of 21 million coins. These rules are enforced by consensus, creating predictability that builds user trust and prevents arbitrary manipulation.
### Backward Compatibility and Upgrades
Bitcoin favors conservative upgrades, typically implemented as soft forks to maintain backward compatibility. Examples include Segregated Witness (SegWit) to increase block capacity and Taproot (2021) to enhance privacy and smart contract flexibility. This cautious approach preserves community consensus while improving functionality.
### Security Through Incentives
Bitcoin combines cryptography with economic incentives to maintain network integrity. Miners are motivated to act honestly because cheating undermines their own financial interests. Game theory aligns miner behavior with long-term network health.
### Open Source and Transparent Development
Bitcoin Core is maintained by a decentralized group of developers following rigorous peer-review processes. Anyone can audit the code, propose changes, or run a full node. Transparency fosters trust and strengthens security.
### Scalability via Layers
Bitcoin maintains a secure base layer while supporting faster, cheaper transactions through Layer 2 solutions like the Lightning Network. This layered approach ensures scalability without compromising decentralization or security.



## Case Study: Tera (LUNA)
- Terra was a blockchain ecosystem built by Terraform Labs (co‑founded by Do Kwon).
- Its flagship stablecoin was UST: an **algorithmic stablecoin** pegged to the U.S. dollar via on‑chain mechanics rather than being fully fiat‑backed. [UCL Discovery+2LSE Research Online+2](https://discovery.ucl.ac.uk/id/eprint/10161357/1/2207.13914v3.pdf?utm_source=chatgpt.com)
- Companion token: LUNA, which played a key role in the mechanism to absorb instability of UST (mint/burn arbitrage between UST & LUNA). [UCL Discovery+1](https://discovery.ucl.ac.uk/id/eprint/10161357/1/2207.13914v3.pdf?utm_source=chatgpt.com)
- Another major component: the Anchor Protocol, which offered very high yields for UST deposits (at one point ~20 % APY) thereby driving huge UST demand. [Department of Finance+1](https://finance.unibocconi.eu/sites/default/files/files/media/attachments/terralunacrash_april202320230516135124.pdf?utm_source=chatgpt.com)
- At its peak, Terra/Luna was one of the largest crypto ecosystems. Among the top by market cap.
### Mechanics of the model (why it appeared to work)
- The model: If UST is above $1, users can exchange $1 UST → $1 worth of LUNA (or vice‑versa) — creating arbitrage pressure to maintain peg. [UCL Discovery+1](https://discovery.ucl.ac.uk/id/eprint/10161357/1/2207.13914v3.pdf?utm_source=chatgpt.com)
- The high‑yield Anchor deposit mechanism encouraged UST issuance: people deposited UST into Anchor, got high yield, increasing demand for UST → more minting of UST → more ecosystem growth.
- LUNA’s value benefitted (in theory) because UST creation implied burning LUNA or vice‑versa (depending on the mechanics) — so growing issuance of UST should support LUNA price (in optimistic view).
- The system was built on **confidence and looped incentives**: high yield → demand → issuance → ecosystem growth → positive sentiment → more demand.
### What went wrong — collapse timeline & trigger
#### Timeline
- April 2022: LUNA reached an all‑time high (~US$119) per reference. [Harvard Law Forum on Governance+1](https://corpgov.law.harvard.edu/2023/05/22/anatomy-of-a-run-the-terra-luna-crash/?utm_source=chatgpt.com)
- May 7, 2022: The first signs of trouble: large withdrawals of UST from Anchor, and UST lost its tight peg (began de‑pegging). [Sveriges Riksbank](https://www.riksbank.se/globalassets/media/konferenser/2023/session-1-liu_makarov_schoar-anatomy_of_a_run-_the_terra_luna_crash.pdf?utm_source=chatgpt.com)
- May 9, 2022 (approx): UST lost the $1 peg significantly; LUNA’s price dropped catastrophically (from tens/hundreds USD to near zero for all practical purposes) in just days. [Programming Insider+1](https://programminginsider.com/case-study-terra-lunas-collapse/?utm_source=chatgpt.com)
- The mechanism collapsed: as UST holders redeemed en masse → huge minting of LUNA → hyper‑dilution of LUNA → further loss of confidence → vicious circle. [Programming Insider+1](https://programminginsider.com/case-study-terra-lunas-collapse/?utm_source=chatgpt.com)
#### Key Trigger / Structural Faults
- The yield offered by Anchor was **extremely high** relative to what underlying borrowers or economic activity supported — making the system fragile. [UCL Discovery](https://discovery.ucl.ac.uk/id/eprint/10161357/1/2207.13914v3.pdf?utm_source=chatgpt.com)
- UST was **not truly fiat‑backed**, its stability relied on the LUNA/UST mechanism and market confidence — which is inherently riskier. [IDEAS/RePEc](https://ideas.repec.org/p/arx/papers/2207.13914.html?utm_source=chatgpt.com)
- As LUNA’s market cap lagged UST issuance, the backing or effective collateral became weaker: the ratio of UST to LUNA value became precarious. (If LUNA market cap < UST issued → systemic risk.) Reddit users flagged similar risks. [Reddit](https://www.reddit.com/r/terraluna/comments/s2bnbw?utm_source=chatgpt.com)
- The blockchain transparency allowed large players to observe flow and act early; but retail users were slower, meaning the “run” dynamic accelerated. [Department of Finance](https://finance.unibocconi.eu/sites/default/files/files/media/attachments/terralunacrash_april202320230516135124.pdf?utm_source=chatgpt.com)
- The model assumed continued demand, rational arbitrage, and liquidity; these assumptions broke when sentiment turned.
- Once a large withdrawal/run started, it triggered the death‑spiral: redemption pressure → more LUNA minted → dilution → LUNA price crash → less ability to stabilize UST → further redemptions.
### Technical Organisational Mistakes
#### Technical / Economic Design Flaws
- The algorithmic stablecoin design lacked robust **collateralization** — essentially depending on LUNA’s value dynamics and demand rather than real assets.
- High dependence on one protocol (Anchor) created **concentration risk**. The failure of UST peg had outsized effect because Anchor held a major portion of UST deposits. [LSE Research Online](https://eprints.lse.ac.uk/117147/?utm_source=chatgpt.com)
- Over‑optimistic incentive structure: Very high yields required either constant inflows or unsustainable subsidy.
- Lack of scenario planning for severe redemptions or lack of arbitrage liquidity.
- Insufficient stability reserves: While Terra set up the Luna Foundation Guard (LFG) to hold reserves (including Bitcoin), the reserves were not enough once the run started.
- The model had a contagion risk: losses in one part (UST) propagate to the other (LUNA) due to the coupling.
#### Organisational/governance mistakes
- Overpromotion of the model without adequately educating investors about risks (many retail investors did not understand the algorithmic backing). [Harvard Law Forum on Governance](https://corpgov.law.harvard.edu/2023/05/22/anatomy-of-a-run-the-terra-luna-crash/?utm_source=chatgpt.com)
- Rapid scaling of capital inflows (large amounts of deposits) without matured risk management.
- The project’s governance, oversight and risk controls were weak — e.g., no adequate circuit breakers, no clear stress‑testing publicly.
- The ecosystem’s reliance on hype and promise rather than proven durability. For example, aggressive yield marketing.
- Regulatory and legal risk were high: because the model pushed high yields and large promises, legal/regulator scrutiny followed.
- When the crisis hit, response mechanisms were insufficient or too late; communication broke down, trust evaporated quickly.
#### Effects & Impact
- The collapse wiped out **tens of billions of USD** of value in a few days. [NBER+1](https://www.nber.org/papers/w31160?utm_source=chatgpt.com)
- It triggered a loss of confidence in algorithmic stablecoins more broadly.
- It affected many other players and protocols in the DeFi ecosystem (since inter‑connectedness amplified spill‑over).
- Regulatory attention increased markedly across stablecoins, crypto lending, and algorithmic models.
- For many retail investors, the collapse led to large losses and wealth destruction.
#### Positive Lessons / What can be learned
Even though the collapse was severe, there are valuable lessons:
##### On technical/economic design
- Algorithmic stablecoins must include **robust collateralization** or hybrid models (not pure dual‑token mint/burn if the backing token can collapse).
- Stress‑test models: consider worst‑case redemption scenarios, market panic, liquidity evaporation.
- Avoid over‑dependence on **single protocols** or single high‑yield events that attract large deposits. Diversify risk.
- Incentive structures (e.g., high yields) should be sustainable or gradually reduce as the system matures; do not overly rely on perpetual subsidies.
- Transparent risk disclosures: allow all participants (especially retail) to understand mechanisms, exposures, and failure modes.
##### On organisational/governance side
- Governance frameworks for DeFi should include **emergency protocols**, **pause mechanisms**, **circuit breakers** when model assumptions break.
- Projects should design for **decentralization with accountability**: but still include operational risk control and transparency.
- Marketing should not oversell and must align with realistic risk. Over‑hype leads to fragility.
- Legal and regulatory strategy is critical: stablecoins especially may attract regulation; know the jurisdictional, consumer‑protection and oversight aspects ahead of time.
- The collapse reinforces that **trust and credibility matter** in crypto: once trust breaks, the system unravels quickly.
##### On ecosystem‐level
- The Terra crash is a cautionary tale for institutions and investors: watch for **hidden coupling**, **fragile incentives**, and models that rely on continuous growth in deposits or yield.
- For protocol designers working in languages such as Rust: design your economic‑mechanism models with fail‑safe paths, resistance to “bank run” logic, and clear audits of systemic risk.
- It catalyzed more research into crypto financial stability, network‑science models for runs, algorithmic stablecoin design improvements. (For example: "Making Algorithmic Stablecoins More Stable" paper).
## FTX
- FTX was a major cryptocurrency exchange, once valued at ~$32 billion (in early 2022). [MIT Sloan+2Wikipedia+2](https://mitsloan.mit.edu/teaching-resources-library/sam-bankman-frieds-ftx?utm_source=chatgpt.com)
- It was co‑founded by Sam Bankman‑Fried, and closely connected to a trading firm, Alameda Research, which was also controlled by SBF. [MIT Sloan+1](https://mitsloan.mit.edu/teaching-resources-library/sam-bankman-frieds-ftx?utm_source=chatgpt.com)
- The business model: provide trading, derivatives, custody of crypto assets for users, while investing capital and operating other crypto‑businesses.
- FTX also issued its own token, FTT, which was used within the ecosystem (discounts, loyalty, collateral) and held on the balance sheet of Alameda/FTX.
### Timeline and trigger events
- On **2 November 2022** a report (e.g., by CoinDesk) revealed that Alameda’s balance sheet was heavily exposed to FTT tokens, many of which were issued by FTX itself. [Congress.gov+2TechForing+2](https://www.congress.gov/crs_external_products/IN/PDF/IN12047/IN12047.1.pdf?utm_source=chatgpt.com)
- Soon after, depositors and users began withdrawing en‐masse from FTX, triggering a liquidity crisis. [BDO Canada+1](https://www.bdo.ca/insights/fraud-deconstructed-ftx-cryptocurrency?utm_source=chatgpt.com)
- On **11 November 2022**, FTX, Alameda Research, and many affiliated entities filed for bankruptcy protection. [Wikipedia+1](https://en.wikipedia.org/wiki/Bankruptcy_of_FTX?utm_source=chatgpt.com)
- Afterwards: forensic examination revealed missing customer funds, lack of proper record‑keeping, commingling of funds, and huge related‑party exposures. [Learnsignal+1](https://www.learnsignal.com/blog/ftx-bankruptcy-analysis/?utm_source=chatgpt.com)
### What went wrong — root causes
#### Technical / financial / economic failures
- **Misuse of customer funds**: Customers’ deposits into FTX were not properly segregated from FTX corporate assets or those of Alameda. According to analyses, billions of dollars of FTX customer funds were used by Alameda for trading and other investments. [BDO Canada+1](https://www.bdo.ca/insights/fraud-deconstructed-ftx-cryptocurrency?utm_source=chatgpt.com)
- **Excessive leverage and poor collateralization**: Alameda had a very large credit line from FTX, backed by FTT token holdings (which were illiquid and self‐issued) rather than robust external collateral. [Seven Pillars Institute](https://sevenpillarsinstitute.org/case-study-ftx-and-sam-bankman-fried/?utm_source=chatgpt.com)
- **Liquidity mismatch**: While FTX had obligations to users (withdrawals) that were immediate, many assets were tied up, illiquid, or overvalued. The firm couldn’t meet demands when they came. [Mitosis University+1](https://university.mitosis.org/the-ftx-collapse-lessons-in-custodial-risk/?utm_source=chatgpt.com)
- **Token risk (FTT) and circular dependencies**: FTT’s value underpinned many exposures; once its value dropped, the whole structure became unstable. [arXiv](https://arxiv.org/abs/2212.09436?utm_source=chatgpt.com)
- **Poor risk management / internal controls**: The new bankruptcy overseer stated he had “never in my career seen such a complete failure of corporate controls and such a complete absence of trustworthy financial information.” [Wikipedia+1](https://en.wikipedia.org/wiki/FTX?utm_source=chatgpt.com)
#### Organisational / governance issues
- **Concentration of power and lack of independent oversight**: The management (SBF and very close associates) controlled many decisions; there were virtually no checks & balances. [The Corporate Governance Institute](https://www.thecorporategovernanceinstitute.com/insights/news-analysis/governance-causes-ftx-collapse/?utm_source=chatgpt.com)
- **Related‑party transactions and opacity**: FTX and Alameda had intertwined operations, loans, and exposures that were not publicly transparent. [Mondaq+1](https://www.mondaq.com/india/fin-tech/1268678/the-ftx-collapse-key-takeaways?utm_source=chatgpt.com)
- **Aggressive marketing + growth without appropriate risk framework**: FTX expanded quickly, made high‑profile sponsorships, raised capital, issued tokens, while underlying risk controls lagged. [Seven Pillars Institute](https://sevenpillarsinstitute.org/case-study-ftx-and-sam-bankman-fried/?utm_source=chatgpt.com)
- **Regulatory and custodial misrepresentations**: Customers believed funds were safe, maybe FDIC insured (in some messaging); in fact the custodial arrangements were weak. [Reddit](https://www.reddit.com/r/FTXOfficial/comments/ytv0qd?utm_source=chatgpt.com)
- **Trigger dynamics and run risk underestimated**: The firm did not prepare for a large withdrawal event triggered by loss of confidence.
#### Effects & impact
- Massive losses: Customer deposits not fully returned; significant shortfall in assets vs liabilities when true audit started. [Learnsignal+1](https://www.learnsignal.com/blog/ftx-bankruptcy-analysis/?utm_source=chatgpt.com)
- The collapse shook confidence in centralized exchanges and the broader crypto ecosystem.
- Regulatory attention increased globally: more scrutiny of crypto exchanges, custodians, audit firms, stablecoins etc.
- Legal consequences: SBF was later convicted of fraud (see news).
#### Lessons learned (technical, organisational & for crypto ecosystem)
##### Technical & financial lessons
- **Segregate customer funds**: Custodial platforms must never commingle user deposits with proprietary trading or sister entities.
- **Strong collateralization and liquidity risk management**: Use high‑quality liquid assets as backing where obligations to users exist; avoid over‑reliance on internal tokens or assets that can vaporize.
- **Transparent risk disclosures**: Markets and customers need clarity on exposures (e.g., related parties, token holdings) to assess risk.
- **Design for withdrawal stress / run scenarios**: As a custodial/exchange business you must assume ‘everyone withdraws’ and you must have capacity to meet it.
- **Audit and recordkeeping integrity**: Proper books, records, independent audit, clear accounting of inter‐company dealings.
##### Organisational/governance lessons
- **Independent oversight and governance**: Boards, audit committees, external directors, clear separation of functions.    
- **Avoid conflicts of interest**: Entities with overlapping ownership (e.g., sister trading firm and exchange) must have strong firewalls and transparency.
- **Cautious growth and marketing with matching risk systems**: Growth is fine but must be matched with robust internal controls.
- **Regulatory compliance and clear messaging**: Especially in emerging sectors, misrepresentation (even implicit) about safety or backing is dangerous.
##### Ecosystem / strategic lessons
- Centralized platforms carry systemic risk: One major failure can trigger broader contagion. The FTX case shows the fragility of centralized crypto finance. [arXiv](https://arxiv.org/abs/2302.11371?utm_source=chatgpt.com)
- Token issuance by a platform which then backs its own liabilities is risky: If the platform’s token falls, the whole system is at risk.
- Importance of on‑chain transparency & analytics: The collapse spurred more on‑chain analysis, network studies, etc. [arXiv](https://arxiv.org/abs/2407.12683?utm_source=chatgpt.com)
##### Why this matters (context)
- Many crypto users trusted FTX with funds, believing in the brand, size, credibility. The failure impacted not just sophisticated investors but also retail.
- It highlights that even “big name” companies can fail if core risk controls are absent.
- For anyone building in crypto (exchanges, protocols, custodians) the FTX collapse is a cautionary lesson: **technical innovation is not enough without proper governance, risk discipline, and custodial integrity**.