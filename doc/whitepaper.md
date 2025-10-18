<meta charset="utf-8">
<meta name="viewport" content="width=device-width">
<meta name="description" content="">
<meta name="keywords" content="whitepaper, censorship">
<meta name="author" content="Unsudo">
<link rel="icon" href="">
<title>Whitepaper</title>

# Table Of Content
---
1. [Abstract](#abstract)
2. [Vision](#vision)
3. [Problem Statement](#problem-statement)
4. Value Proposition
5. Market Opportunity  
6. Governance  
7. Token Economics  
8. System Architecture  
   - Polkadot JAM  
   - Migration  
   - WASM
# Abstract
---
# Vision
---
## Cross Border
- A world where anyone in any country can work together.
- Mathematical certainty of the the preservation of freedom.
- A better world for future generations, and something to take ownership of.
- The decoupling of traditional finance from rich and few companies.
- The reduction of costs across the board so younger people can afford to start big projects with less.
- Trust-less digital environments.
# Problem Statement
---
## The Cost of Censorship
Censorship has evolved from a matter of platform policy into a deeply embedded systemic challenge, one with wide-reaching economic, legal, cultural, and human implications. While moderation is often necessary to manage harmful or illegal content, the mechanisms behind censorship today have created a costly and fragile system. The burden of enforcing moderation at scale, navigating increasingly complex legal frameworks, and preserving public trust has become a central obstacle to sustainable growth in the digital ecosystem.
Major platforms now spend billions of dollars annually on moderation infrastructure, including content filters, human reviewers, and appeals systems. These costs continue to rise, driven by pressure to comply with laws such as the EU Digital Services Act and similar regulatory frameworks worldwide. Compliance costs for these laws are not only significant—often reaching into the hundreds of millions—but are also jurisdictionally bounded. This creates a regulatory asymmetry: decentralized systems are largely outside of national legal reach, while centralized companies are left carrying the legal and operational liabilities.
Beyond direct expenses, censorship creates cascading effects on innovation and economic participation. Demonetization policies, advertiser boycotts, and "brand safety" measures lead to lost revenue for both creators and platforms. Startups are particularly disadvantaged, as the cost and complexity of building legally compliant moderation systems become a barrier to entry, stifling innovation before it starts. Even established companies face growing legal risks, with moderation-related lawsuits and settlements now a regular part of operating costs.
The cultural consequences are equally serious. The over-application of censorship leads to a chilling effect, where creators, academics, and ordinary users begin to self-censor to avoid backlash, bans, or reputational harm. Controversial research, minority viewpoints, and dissenting opinions are disproportionately silenced, often under vague or inconsistently enforced policies. As more users lose trust in platforms, many migrate to fringe or unregulated alternatives, reinforcing ideological echo chambers and driving social polarization. The result is a fractured and fragile information ecosystem that discourages open dialogue, weakens public discourse, and homogenizes cultural expression.

At an institutional level, censorship introduces reputational and financial volatility. Companies can suffer stock drops, shareholder lawsuits, and investor divestment following high-profile censorship scandals. Executives are forced to walk a narrow line between appeasing regulators, investors, and vocal user groups — often finding themselves unable to satisfy any side.

Users, meanwhile, are increasingly disempowered. Deplatforming can sever income streams, erase digital identities, and isolate individuals from online communities without due process. Automated moderation systems often collect massive amounts of behavioral data, exposing users to surveillance and algorithmic bias. The long-term erosion of trust in digital services is palpable — users feel monitored, misrepresented, and powerless.

**Real-world examples include:**
- **Facebook–Cambridge Analytica**: Showcased how censorship and data misuse can undermine democracy.
- **Apple in China**: Hosting iCloud data within authoritarian legal systems risks user safety.
- **Covid-19 platform bans**: Users were penalized for views later validated by evidence, undermining platform credibility.
- **TikTok**: Ongoing scrutiny over state surveillance risks leading to national bans and public distrust.
Censorship is no longer just about moderation — it's a **structural liability**. It distorts markets, suppresses creativity, and exacerbates political fragmentation. Its costs fall most heavily on users and creators, while institutions bear growing legal and reputational risks.

To build resilient, open systems for the future, we must recognize censorship for what it has become: **a systemic bottleneck to progress, prosperity, and trust**.
## The Hidden Cost of Corruption
Corruption is more than a moral failing — it is a systemic threat to economic efficiency, institutional legitimacy, and societal well-being. At every level of governance and enterprise, corrupt practices introduce distortions that erode trust, inflate costs, and undermine progress.

Whether through bribery, cronyism, nepotism, or opaque contracting, corruption drains public resources and favors entrenched interests over innovation and fairness.
### Key Impacts:
- **Public Funds**: Infrastructure, healthcare, and essential services suffer from inflated contracts and misallocation.
- **Investor Confidence**: Markets plagued by corruption deter foreign investment and transparency.
- **Competition**: Honest businesses are crowded out by those with political connections, damaging fair market dynamics.
- **Institutional Erosion**: Judiciary, media, and regulatory bodies become compromised, further undermining democratic processes.
- **Social Consequences**: Inequality rises as the poor suffer more from weak public services and lack of access.

As corruption becomes normalized, a culture of **impunity and disengagement** spreads. Young talent migrates to fairer systems. Innovation stalls. Institutions collapse inward.
### Corporate Risks:
- Legal fines, reputational damage, and compliance burdens mount.
- Shareholders see reduced value from scandals and litigation.
- ESG investors pull out.
- Employees and customers lose trust and disengage.

Executives must navigate informal systems and dual governance (official vs. shadow). Time and resources are diverted away from innovation and towards damage control.

In a global economy defined by **transparency and trustless systems**, opacity becomes a liability. Companies enmeshed in corruption lose market access, partnerships, and legitimacy.

> Corruption is a **hidden tax on society** — replacing value creation with value extraction, and fairness with favoritism.
## Barriers to Blockchain Adoption
Despite the promise of decentralization and financial empowerment, blockchain adoption faces **critical barriers** across technical, cultural, legal, and usability domains.
### Myth of Decentralization
Many so-called "decentralized" apps still rely on:
- Centralized servers  
- VC-controlled governance  
- Custodial wallets
This undermines the fundamental ethos and creates points of failure vulnerable to both external and internal pressure.
### Community Tribalism
Blockchain communities often resist critique, creating echo chambers. This:
- Discourages healthy debate  
- Slows progress on standards and interoperability  
- Hinders public accountability  
### Regulatory Uncertainty
- Laws are vague or in flux (especially around securities, privacy coins, DeFi).
- Compliance risk is high for both users and builders.
- Legal ambiguity deters innovation and adoption.
### ↡ Censorship at the Infrastructure Layer
Even decentralized protocols rely on centralized services:
- Web hosting (e.g., IPFS gateways, DNS)
- Cloud platforms
- App store distribution
Front-ends can be taken offline, wallets blocked, or content delisted — revealing **fragility beneath the surface**.
### ↡ Poor User Experience
- Private key management is error-prone.
- Wallet UX is complex and fragmented.
- Gas fees and smart contracts are intimidating.
- Mistakes are often irreversible.
### ↡ Architecture Limitations
- On-chain systems are poor at handling real-time interactions.
- Streaming, gaming, or live data often rely on centralized backends.
- Layer 2 and micropayment systems exist, but adoption remains limited due to UX complexity.
### ↡ Developer Burden
Developing for blockchain requires:
- Learning new tools (Solidity, Rust, Move, etc.)
- Managing wallets, gas, security
- Navigating fragmentation between chains
Bugs can be permanent and costly. The mental load is high.
### ↡ Multi-chain Fragmentation
- Bridges are vulnerable and inefficient.
- Cross-chain tools lack maturity.
- Users must juggle assets, risks, and inconsistent interfaces across ecosystems.
### ↡ Security Vulnerabilities
Common threats include:
- Reentrancy attacks  
- Flash loan exploits  
- Oracle manipulation  
- Rug pulls
Insurance exists but is fragmented and underused.
### 10. **Key Management and Identity Recovery**

- Lost private keys = lost assets  
- Social recovery and account abstraction show promise but aren't mainstream  
- No unified identity or recovery across chains

### 11. **Lack of Real-World Utility**

- Most apps still revolve around speculation or DeFi.
- Real-world integrations are rare, clunky, or experimental.
- Enterprises are cautious due to unclear ROI and regulatory friction.
### In Summary:
Blockchain’s full potential remains **locked behind real barriers**:
- Structural: centralized dependencies  
- Cultural: tribalism, elitism  
- Regulatory: hostile or ambiguous legal systems  
- Usability: steep learning curves, poor UX  
- Security: code is law, but few understand the law
Until these are addressed, the promise of Web3 will remain just that — **a promise**.
## ∗ Value Proposition
---
## ↘ Onchain Infrastructure
- Why not just server?
- Long term?
- Resillience?
- Tradeoff?
- Why fundamental to the project.
- Migration concerns?
- Immutability.
- Truely self sovreing code and autonomous machines.
- Transparency.
- Cross border collaboration.
## ↘ Rust as Strategic Foundation
Our choice to build in Rust provides a significant competitive advantage over Solidity and other non-Rust environments. Rust's performance, safety, and versatility make it the ideal foundation for building resilient, high-performance, and scalable decentralized infrastructure.
### ↓ Key Advantages of Building in Rust
#### ↡ Democratizing technologies
- Rust is more available across platforms making it more decentralized, can be run on poor hardware.
- Rust makes it more available for many programmers of various levels to write more robust software, as it is strict and ensures that contributions will reach a certain level of value.
#### ↡ Built-in Safety and Reliability
- Memory-safe by design - no garbage collector, no manual memory management.
- Eliminates entire classes of runtime bugs like buffer overflows and null exceptions.
- Strong static typing and compile-time checks reduce logic errors before deployment.
- Reduces attack surface common in Solidity-based systems.
#### ↡ System-Level Performance and Flexibility
- Compiles to high-performance WebAssembly (WASM) with deterministic execution.
- Supports asynchronous logic and multithreading. Ideal for real-time applications, collaborative tools and streaming data.
- Enables custom runtimes, freeing developers from the constraints of the EVM.
#### ↡ Cross-Platform Efficiency
- Write once, run anywhere. Rust code can power onchain logic, backend services, and even mobile or desktop apps. And remains accessible to anyone even on low powered devices in remote regions.
- It also reduces the energy require where environmental concerns are of importance, and is capable of doing more in environments unconstrained by energy requirements.
- Unified tooling and shared libraries across domains streamline development and reduce operational complexity.
#### ↡ Superior Developer Experience
- Backed by best-in-class tooling: Cargo, Clippy, Rust Analyzer.
- Supported by growing ecosystem of robust, production-ready libraries.
## ↘ Algorithmic Governance and Mathematical Precision
- We dont just want to build a DAO.
- Humans are flawed in their decisions, and Unsudo is an experiment and research to spear head decentralized collaboration across borders. To do so we need a system that is capable of deterministically acting in a rational matter.
- We envision Unsudo to be an autonomous system, as decentralization alsso means to not hold biases.
- **Unsudo** is a research-driven experiment in enabling **truly decentralized collaboration across borders**—without the inconsistencies of human decision-making.
- Human governance is inherently flawed—prone to bias, emotion, and inconsistency. While decentralized systems aim to distribute power, true decentralization also requires **minimizing subjective influence**.
- Unsudo envisions a future where governance is not only **transparent and decentralized**, but also **deterministic and rational**—guided by **algorithmic processes and mathematical principles**.
- We see a DAO that belongs to the community, but is smart enough to govern itself.
- By embedding **mathematical objectivity** into our governance architecture, Unsudo aims to push the boundaries of what DAOs and decentralized systems can achieve—creating not just **tools for coordination**, but **autonomous institutions** that act logically, consistently, and in the collective interest.
- We can fix the problems with current DAO models which are either always and increasingly reliant on a few members to manage and maintain the DAO, or a overly fragmented community which leaves the DAO vulnerable to attacks during low quorum or activity states. The solution is to delagate managment of day-to-day operations to the DAO itself and focus on building next-generation systems that dont act on static instructions, but are capable of performing decisions based on algorithms and policies as mathematical formulas.
- Our mission at Unsudo will be a success if we build a protocol capable of surviving critical inactivity and recover from volatile environments.
## ↘ Abstract Execution Layer
# Governance
---
Next gen DAO models, embrace algorithms for unbiased decision making vs traditional orgs.
## Dual Chamber System
- Much like a government Unsudo will operate as a dual chamber system.
- The community will UDO which grants them votes to pass proposals which can directly alter the protocol's code, or require transfers.
- The council will not be able pass proposals alone, they will need support of the wider community to be able to access funds, and pass updates.
- Proposals will need the approval of both chambers, the council only has the power to veto maliscious proposals
### Veto
A temporary mechanism and ability of the council to be able cancel malicious proposals, naturally this comes with concerns. The council itself cannot pass proposals, and act more as representatives (In its maturity, the DAO should be able to handle itself, but early on, council members must take responsibility for maintaining the project and elevating it to that status).
### Electoral Mechanism
### Quorum Algorithm
Base currency are always in ₿ and not $.
## Proposal
### Timelock
#### Quorum
#### Stalemate
## Tokenomics
- The maximum supply is 1,000,000 and it is a hard cap.
- Voting and governance.

| ◦ Category       | ◦ Balance | $ Price | ↗ Inflow    | ↘ Outflow   | 🔒 Vesting |
| ---------------- | --------- | ------- | ----------- | ----------- | ---------- |
| Council          | $200,000$ | $25$    | -           | $5,000,000$ | 20         |
| Core Contributor | $75,000$  | $25$    | -           | $1,875,000$ | 5          |
| Bounty           | $50,000$  | $25$    | -           | $1,250,000$ | 2          |
| Pre-Seed         | $20,000$  | $2.5$   | $50,000$    | -           |            |
| Seed             | $30,000$  | $5$     | $150,000$   | -           |            |
| Series A         | $40,000$  | $10$    | $400,000$   | -           |            |
| Series B         | $50,000$  | $17.5$  | $875,000$   | -           |            |
| Series C         | $60,000$  | $22$    | $1,320,000$ | -           |            |
| Public           | $100,000$ | $25$    | $2,500,000$ | -           | 3 months   |
| Liquidity        | $50,000$  | $25$    | -           | $1,250,000$ |            |
| Reserve          | $325,000$ | $25$    | -           | $8,750,000$ |            |

### ↘ Vesting
The vesting schedule will be linear meaning that tokens unlock on a per second basis from the TGE. The vesting schedules are soft targets for the protocol to honor, and this affects participants both positively or negatively but always in the best interest of the protocol. Unsudo DAO aims to be 99% algorithmic-ally governed, this meanings that the protocol itself has inflationary unlock targets to weight as well. In others words, based on algorithms that can be voted in by people, it can act as a sort of central bank making decisions autonomously. So if it is doing well, it may increase unlocks, but become more reserved if market conditions become volatile, and reduce situations where teams dump on the market. This mechanism is completely third party and self sovereign, meaning it is impartial to all participants once implemented. 
### ↘ Token Value Drivers
### ↘ Liquidity Planning
- Unsudo aims to be decentralization first so initially will be listed on DEXs, however this is still to be confirmed.
- 50,000 will be reserved as liquidity.
### ↘ Compliance & Legal Framework
- The token aims to be a governance token, as well as a commodity  because it is not controlled by any market participant or administration rather indirectly by algorithms voted in by the collective.
- The token or DAO does not take any country or national border as its home, it remains self sovreign.
- There may be a legal wrapper, but this is still to be determined.
- KYC/AML only apply to exchanges where it might be listed on, but Unsudo is largely autonomous, and serves everyone equally.
### ↘ Conversion
#### ↡ Off-Ramp
#### ↡ Volatility
#### ↡ Trust
### ↘ Unlock Rate Mechanism
- Algorithmic, fair, transparent.
- Aligns with protocol performance.
- Helps throttle unlocks in bear markets.
- Motivates stakeholders to maintain health.
$U(t)=(Stotal​−Scirc​(t))⋅[rbase​+(rmax​−rbase​)⋅γ(t)]⋅Δt​$

| Year   | Target Annual Unlock Rate | Circ Cap | Note |
| ------ | ------------------------- | -------- | ---- |
| 2026   | ~15%                      |          |      |
| 2027   | ~10%                      |          |      |
| 3 - 5  | ~5%                       |          |      |
| 6 - 20 | ~3%                       |          |      |

The rules apply to everyone equally.

```rust




type Outflow = Vec<(u32, u32)>;

fn permitted_inflation(
	inflation_target: u32,
	total_supply: u32,
	total_supply_locked: u32,
) -> u32 {

}


inflation(2_00, 1000000_00, 900000_00);
```


```rust
type Vested = (
	total_locked: f32,
	timestamp_begin: u32,
	timestamp_end: u32
);
type LookbackSeconds = usize;

fn compute<const T: LookbackSeconds>(
	timestamp: u32,
	udo_price_history: [f32; T],
	udo_price: f32,
	inflation_rate_history: [f32; T],
	inflation_rate_target: f32,
	inflation_rate: f32,
	growth: f32
) {

}




// chain of responsability - plug in architecture

pub struct Kernel {
	plugins
}

```

### Early Unlock




```rust
fn unlock(
	duration: u32,
	price: u32,
	difficulty: u32,
	lower_range_price: u32,
	upper_range_price: u32
) -> u32 {
	
}
```
## Treasury
### Purpose & Philosophy
- Define the treasury's role: is it simply to fund operations, or is it a long-term sovereign wealth fund? Is it algorithmically managed? This sets the tone for the rest.
- "The Unsudo Treasury is designed as a self-sovereign, algorithmically-influenced financial backbone of the protocol — funding development, incentivizing contributors, and sustaining public goods in a decentralized, trustless manner."
### Revenue Stream
We expect to earn from 
- List and explain where funds come from:
- Protocol fees (if any)
- Block rewards / inflation mechanisms
- Grants
- Token sales
- Donations
- Service fees (e.g., DAO-as-a-Service revenue)
#### Donation
- Who can donate?
	- Individuals.
	- DAO.
	- Philanthropists.
	- Foundation.
- Is KYC required for large donations?
- What can be donated?
- How are donations accepted?
	- Smart contract?
	- Multisig treasury wallet?
	- Platform integration (Gitcoin, Juicebox, OpenCollective)?
- Donation incentived or purely voluntary?
	- Do donors receive recognition, NFTs, governance rights, or nothing.
- Are there limits or conditions?
- How are donations used?
[For more information](#contact)
#### Grant
- Who can provide grants?
	- Web3 foundations? Polkadot? Ethereum Foundation?
	- NGOs, universities, public research orgs?
	- Corporate or ecosystem funds?
- What types of grants are eligible?
	- R&D.
	- Protocol development.
	- Governance tooling.
	- Education/Outreach.
- Does Unsudo accept grants?
- Are grants one-time or recurring?
- What terms or strings are attached?
- How will grant funds be managed?
	- General treasury? Specialized wallet?
	- Tracked via public dashboard?
#### Sponsorship
- What does a sponsorship mean in Unsudo's context?
	- Branding/visibility support?
	- Funding specific features or community initiatives?
- Who can become a sponsor?
	- Crypto projects? Universities? NGOs? Ethical corporations?
	- Any restrictions based on values or alignment?
- What do sponsors get in return?
	- Branding rights?
	- Governance participation (within limits)?
	- Custom integrations?
- Is there a sponsor framework or tiers?
	- Minimum contributions per tier?
- How are conflicts of interest avoided?
	- Can sponsors influence governance?
	- Are disclosures required?
- How is sponsorship revenue used?
	- Pooled into treasury?
	- Dedicated to outreach?
	- R&D?
	- Partner-specific goals?
### Spending Strategy
- Describe how funds are deployed:
- Bounties and contributor rewards
- Public goods funding
- Grants to ecosystem projects
- Strategic investments
- Liquidity provisioning
- Emergency reserves

- You could include policies like:
- Spending caps per quarter
- Community-approved budget cycles
- Emergency drawdown mechanisms
### ↘ Reserve Strategy & Buffer Management
- How much of the treasury is liquid vs. locked or reserved for long-term?
- Stablecoin buffer (to protect against volatility)
- BTC/DOT/ETH reserve ratios
- Algorithmic triggers to rebalance based on token price or volatility
### ↘ **Transparency and Reporting**
How does the community track treasury inflows/outflows?
- Onchain dashboards (e.g., Dune, Subscan)
- Monthly or quarterly reports
- Public access to multisig or autonomous contract logs
### ↘ Governance
How are treasury decisions made?
- Voting mechanism (DAO-wide? Council? Algorithm?)
- Emergency vetoes
- Proposal and voting thresholds
- Algorithmic treasury management — explain if portions are controlled via rules or formulas (e.g., inflation-aware spending, price-peg mechanisms)
### ↘ Security
- Is it a multisig wallet? Controlled by Council?
- Hardware wallet standards?
- Future plans for autonomous smart contract-controlled treasury
- Contingency in case of key loss or Council dispute
### ↘ Growth Policy
- Reinvestment strategies (e.g., yield farming, staking)
- Token buybacks or burns
- Building a runway for X years
- Treasury as a backstop for economic shock
## ↓ Community Input & Market Validation
# Market Opportunity
---
## Competitors
# Roadmap
---
## Now
## Long Term
The long term vision for Unsudo is to be self sufficient, needing no human input externally to keep operations running, rather to be completely governed by algorithms. A community of-course has to take ownership of the protocol (ideally a group of people located across borders who share the same mission). These people will have power to update and control the protocol through democratic systems. In the absence of people, the protocol will be able to perform electoral processes, asset re balancing, and autonomously maintain itself. The role of council and governance will still be required to further grow the protocol and ecosystem, but it would be safer, and less likely to suffer from centralized causes of failure.
# Team
---
## Role
### Council
- Act a mediator/judge in cases of dispute or governance deadlock.
- Veto proposals that are malicious.
- Oversee day-to-day DAO operation, funding allocation, and management.
- Coordinate task forces and working groups.
- Manage Strategic partnerships.
- Manage external relationships.
- Manage legal entities associated with the DAO.
- Guide the DAO towards progressive decentralization.
- Multisig holders.
#### Accountability and Penalties
To ensure trust, transparency, and integrity in DAO governance, council members are held to a high standard. Abuse of power, negligence, or failure to uphold responsibilities will result in clearly defined penalties.
#### Grounds for Penalty
Council members may be penalized for the following:
- *Conflict of interest*: Failing to disclose relationships or personal gain related to proposals or partnerships.
- *Abuse of veto power*: Repeatedly or arbitrarily vetoing proposals without valid jurisdiction.
###### Negligence
Failure to perform duties such as attending votes, managing multisig, or responding to governance needs.
###### Misuse of funds or access
Unauthorized use of DAO funds or privileged access.
##### Penalty Framework
Penalties are enforced progressively, with severity depending on the infraction and impact:

| Infraction Level | Description | Penalty |
|-|-|-|
| 1 | Minor negligence first-time procedural violation | Formal warning; public notice in DAO forum.
| 2 | Repeated negligence, unjustified veto use, moderate conflict of interest | Temporary suspension; multisig access revoked during period
| 3 | Proven abuse of power, financial misconduct, major governance disruption | Immediate dismissal; permanent removal from multisig and council; possible legal referral if applicable

##### Enforcement Process
- Any community member or contributor may submit an abuse report.
- Reports are reviewed by an external auditor.
- Accused members are given the right to respond or appeal.
- Final decisions are subject to a DAO-wide vote for high-severity cases (level 2 or 3).
### Core Contributors
- Entities contributing on a regular basis of at least 15 hours a week. These are the backbone of thee protocol. 
- Likely involved in implementation, research, dev, ops, or comms depending on the DAO's focus.
- Collaborate with Council, and other working groups.
#### ↡ Grounds for Penalty
| Infraction Level | Description                                                                                         | Penalty                                                                    |
| ---------------- | --------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| 1                | Minor negligence (missed deadlines, low-quality work)                                               | Warning and request for improvement                                        |
| 2                | Repeated negligence, failure to meet hour commitments without valid reasons, or disruptive behavior | Temporart suspension of role; loss of access to resources or DAO tooling   |
| 3                | Severe misconduct such as intentional sabotage, breach of confidentiality, or fraud                 | Immediate removal from role; possible rreferral to Council or legal action |
### Core
- 15 hours expected continuos contribution
- rewards in UDO, budget up to 3 years, and then receive reward in currency once protocol is profitable.
### Bounty Hunters
- Participate by completing bounty tasks.
- No minimum hours required.
- Ad hoc contributors who engage with specific DAO needs.
# Compliance
---
## Foundation







# System Architecture
---
Unlike many projects Unsudo aims to move slow, and priorities quality over raw development speed. This means we take time to plan out and map out our protocol.
## Rust
- We aimt to support Rust as a first class language within the protocol. Why?
### Security
- Memory safety.
- Compile-time guarantees.
- Avoidance of typical EVM/Solidity bugs such as reentrancy, overflows.
### Cross-Border, Cross-Platform
- Rust code can run everywhere: onchain, offchain, CLI, mobile, or edge devices.
- This allows Unsudo agents and contributors in resource-constrained regions to still operate nodes, run simulations, or participate meaningfully — reinforcing our goal of true global collaboration.
## Polkadot
Polkadot is
### Join Accumulate Machine
Polkadot JAM is a paradigm shift in how compute is allocated, scheduled, and consumed within a decentralized protocol. Rather than locking up entire parachain slots, JAM introduces a **more flexible, modular, and cost-efficient execution model**. It supports **actor-based services** with asynchronous capabilities, executed on-demand via **Coretime**, a new abstraction of decentralized computation.
#### Coretime: Decentralized Compute-as-a-Service
Coretime represents **purchasable slices of execution bandwidth**, rented directly from Polkadot validators. It decouples execution from consensus and enables dynamic workloads. Unlike traditional models that bill per transaction or state write, Coretime enables **scheduled and burst compute**, which is crucial for services like Unsudo.
Coretime comes in two modes:
- *Bulk Coretime*: Reserved in advance — ideal for predictable workloads, similar to leasing virtual machines or containers.
- *Instant Coretime*: Pay-per-use — dynamically acquired to handle unpredictable spikes or user-driven actions.
This abstraction allows protocols like Unsudo to **scale horizontally** — acquiring compute only when necessary, and **releasing it when idle**, dramatically improving capital efficiency.
#### Service
- **Metadata**: Describes the purpose, structure, and permissioning model of the service.
- **Bytecode (PVM)**: Executable logic written in a WASM-compatible language (e.g. Rust).
- **Entrypoints**: Functions like `refine()` or `accumulate()` that govern how state evolves over time or across events.
JAM Services are **first-class citizens** of the chain, capable of **suspending and resuming execution**, orchestrating multi-agent collaboration, or automating governance logic.
#### Modular Execution & Scheduling
JAM supports **deferred or event-driven execution**, rather than requiring computation to happen synchronously in a block. This allows for:
- Real-time simulations
- Periodic policy enforcement
- Off-chain data refinement (e.g. through verifiable compute)
- Coordination between multiple services

### CorePlay: Actor-Based Computation
CorePlay is an **experimental actor-model runtime** built atop JAM’s execution framework. Inspired by Erlang and Akka models, it introduces a **message-passing architecture** for decentralized agents.
#### Agents
CorePlay Agents are **autonomous compute actors** that:
- Run on JAM cores
- Hold local state and logic
- Communicate asynchronously (across cores) or synchronously (on the same core)
- Can spawn other agents or coordinate with services
This model aligns with Unsudo’s vision of **autonomous, self-governing infrastructure**. Each policy, governance logic, or treasury strategy can be encapsulated as a CorePlay agent — independent, auditable, and resilient.
#### Execution and Cost Model
- **Execution is metered via PVM weights** rather than gas.
- **Time slots (~6 seconds)** on cores are scheduled for each agent's execution.
- Cross-core messaging carries higher costs (due to queuing and latency).
- Coretime is either prepaid by the protocol or charged to users indirectly via services.
Unlike Ethereum’s per-transaction gas model, **JAM abstracts away cost from users**, enabling freemium models, background tasks, and coordinated governance computation — perfect for Unsudo’s **algorithmic treasury and policy agents**.
#### Use Cases for Unsudo
- **Patchlet Execution**: Run as agents that evaluate and apply protocol changes.
- **Policy Threads**: Long-running governance rules (e.g. inflation control, treasury policy).
- **Election Agents**: Oversee voting cycles and quorum algorithms.
- **DAO Watchdogs**: Monitor malicious activity and enforce automatic intervention.
- **Recovery Routines**: Trigger safe-state recovery in periods of DAO inactivity.
#### ↡ Service & CorePlay
A JAM service will include metadata and PVM bytecode with entry points like refine and accumulate, allowing modular execution on the JAM chain.

The concept of CorePlay (experimental) posits an actor-based smart contract model, where actors deployed onto JAM cores can call each other asynchronously if on different cores, or synchronously if co-located. All leveraring PVM's ability to suspend and resume execution.

Gas-like Behavior in JAM Service
Allocation of coretime (prepaid)
Service may charge users in DOT, stablecoins, or any token to cover costs.

JAM is more like a "hosting" compute raather than charging per transaction, so more flexible payment options, and abstracting payment from users who may not need to pay for each transaction.

Gas-like Behavior in CorePlay Agent
CorePlay (actor model on JAM) allows asynchronous/synchronous message passing between agents.
Each agent's execution is also metered by PVM weights.

Agents schedule on different cores conssume Coretime for each work package (~6 seconds slots).

The fee model is still evolving, but most likely:
Agent execution consume Coretime directly.

Cross-core calls are more expensive (like einter-contract calls on Ethereum).
Service/agents themselves may expose APIs to change users "gas-like" fees.
## Simulator
Focus on algorithmic precision and accuracy in governance.
## Patchlet

```rust
trait Patchlet {
	fn apply(s: State) -> State; 
}

trait Policy {
	fn enforce(s: State) -> State;
}
```


## Thread

```rust
struct Thread {
	policies: Vec<Policy>
}
```

## Bolt

# What We've Learnt
---
## Case Study: Bitcoin
### Overview
**Bitcoin (BTC)**, created by an anonymous person or group under the pseudonym _Satoshi Nakamoto_ in 2009, is the first decentralized digital currency. It allows peer-to-peer transactions without intermediaries such as banks or governments.

Bitcoin’s success can be attributed to a combination of **technological innovation, economic principles, market timing, and community support**. Below is an analysis of the key reasons behind its success.
### First-Mover Advantage
- **Innovation**: Bitcoin was the first successful implementation of a peer-to-peer, decentralized currency based on blockchain technology.
- **Brand Recognition**: Being first helped Bitcoin establish itself as the “face” of cryptocurrency.
- **Network Effect**: Early adoption led to more miners, developers, users, and merchants, reinforcing its dominance.
### Decentralization & Security
- **Trustless System**: Bitcoin eliminated the need for trust in centralized entities, using cryptographic proof instead.
- **Immutable Ledger**: Transactions are recorded on a public blockchain, resistant to censorship and fraud.
- **Proof of Work (PoW)**: Its consensus mechanism secures the network and incentivizes participation.
### Scarcity & Monetary Policy
- **Fixed Supply**: Only 21 million BTC will ever exist. This creates scarcity, similar to gold.
- **Deflationary Model**: Regular "halvings" reduce the block reward approximately every four years, decreasing the rate of new issuance and potentially increasing value over time.
- **Inflation Hedge**: During times of fiat currency devaluation, Bitcoin has been seen as a store of value.
### Ideological Appeal
- **Freedom & Sovereignty**: Bitcoin appeals to those who value privacy, autonomy, and financial sovereignty.
- **Anti-Censorship**: No single party can control or shut down the Bitcoin network, making it attractive in regions with authoritarian regimes or unstable banking systems.
### Global Accessibility
- **Borderless**: Anyone with internet access can use Bitcoin, regardless of geography or banking status.
- **Low Barrier to Entry**: Users can own a fraction of a bitcoin, making it accessible to people at all income levels.
### Developer and Community Support
- **Open Source**: Bitcoin’s code is publicly available, encouraging transparency and ongoing development.
- **Large Developer Base**: Continuous updates, security enhancements, and scalability solutions like the Lightning Network.
- **Active Community**: A robust global community of developers, miners, investors, and educators contributes to its resilience.
### Institutional Adoption & Recognition
- **Legitimization**: Growing acceptance by institutions, hedge funds, and public companies (e.g., Tesla, MicroStrategy).
- **Financial Products**: Introduction of Bitcoin ETFs, futures, and custody solutions has increased trust and accessibility.
- **Media Attention**: Constant coverage keeps Bitcoin in the public eye, driving curiosity and demand.
### Resilience & Longevity
- **Survived Multiple Crashes**: Bitcoin has faced numerous price crashes, regulatory attacks, and market competitors, yet remains dominant.
- **12+ Years of Uptime**: The Bitcoin network has never been hacked or gone offline since its launch in 2009.
### Simplicity of Protocol Design
- **Minimal, Purpose-Driven Design**:  
    Bitcoin was intentionally kept simple: it does one thing (store and transfer value) extremely well.
- **No Turing-Complete Smart Contracts**:  
    This makes the protocol safer and easier to audit compared to more complex blockchains like Ethereum.
- **Op_Return & Script Language**:  
    It has limited scripting capability to enable basic conditions (e.g., multisig, timelocks) without the risks of full programmability.
- This "do one thing well" approach improves reliability and reduces attack surfaces.
### Predictable Monetary Policy (Code as Law)
- **Hard-Coded Rules**:
    - Block time ≈ 10 minutes
    - Block reward halved every 210,000 blocks (~4 years)
    - Total supply capped at 21 million
- **Enforced by Consensus**:  
    These rules are not arbitrary — they’re embedded in the code and enforced by the network.
- Predictability builds user trust and prevents human manipulation.
### Backward Compatibility and Upgrades (Soft Forks)
- **Conservative Upgrade Path**:  
    Bitcoin avoids frequent changes. When upgrades happen, they’re typically **soft forks** to maintain compatibility (e.g., SegWit, Taproot).
- **Segregated Witness (SegWit)**:  
    Increased block capacity and fixed malleability issues.
- **Taproot (2021)**:  
    Improved privacy and smart contract flexibility while maintaining Bitcoin’s minimalist philosophy.
- Careful, peer-reviewed development avoids bugs and forks — helping Bitcoin retain community consensus.
### Security Through Incentives
- **Economic Incentives**:  
    Miners are financially motivated to act honestly, as cheating would harm the value of their own earnings.
- **Game Theory**:  
    Honest participation is the most profitable long-term strategy, aligning miner incentives with network health.
- Bitcoin combines cryptography with economics (crypto-economics) for robust security.
### Open Source & Transparent Development
- **Bitcoin Core**:  
    Maintained by a decentralized group of developers with open processes.
- **Peer Review**:  
    All changes undergo rigorous review and testing.
- **Global Participation**:  
    Anyone can audit the code, propose changes, or run a full node.
- Transparency builds trust and hardens security by exposing the code to public scrutiny.
### Scalability via Layers (e.g., Lightning Network)
- **Bitcoin Layer 1 = Secure, Base Layer**
- **Layer 2 (Lightning Network)**:
    - Enables fast, cheap microtransactions
    - Built on top of Bitcoin
    - Keeps main chain lightweight
- Bitcoin can scale without compromising the base layer’s decentralization or security.
## Case Study: The DAO
### The Hack
- **Date:** Attack began **June 17–18, 2016**.
- **Amount stolen:** about **3.6 million ETH** (worth ~$50M at the time; the USD figure varied as price fluctuated).
- **How it worked (root cause):**
    - The DAO contract allowed users to **split** their DAO shares into a newly created “child DAO”, then withdraw funds.
    - The withdrawal logic performed **an external call** (sending ETH to a recipient) **before** updating the attacker’s balance in the DAO contract state.
    - The attacker exploited this by creating a fallback function in the child DAO that **reentered** the parent DAO’s withdraw routine repeatedly before the balance was reduced — i.e., a **reentrancy** attack.
    - Reentrancy exploited the interleaving of external calls and state updates; because the state wasn’t changed before making the transfer, repeated recursive calls drained funds.
- **Key technical vector:** missing the **Checks‑Effects‑Interactions** pattern (or absence of a reentrancy guard), combined with use of external calls that forwarded sufficient gas to allow reentry.
- **Immediate aftermath:** A portion of the drained ETH was locked by the DAO contract’s rules into a “refund window”, giving the community time to react.
### Community & protocol responses
- There were two main proposals to recover funds:
    1. **Soft fork / blacklist** to freeze or prevent movement of the stolen funds — technically and politically controversial.
    2. **Hard fork** to change state so stolen funds were moved to a recovery contract allowing refunds to original token holders.
- The community **voted** and most of the active community, miners, and developers supported the **hard fork** (implemented July 20, 2016 at block **1,920,000**). The hard fork returned funds to a recovery contract.
- A portion of the community rejected the fork on principled “code is law” grounds and continued running the original chain: this became **Ethereum Classic (ETC)**. The forked chain is what we now commonly call **Ethereum (ETH)**.
### Insufficient Security Process & Audit Discipline
What went wrong:
- The DAO was **never formally audited** before going live, despite holding **~14% of all ETH** in existence at the time.
- Known issues had been raised on GitHub and forums but were not addressed before launch.
- The structure was **too complex to reason about**, and lacked formal specification or threat modeling.
Lesson:
> **Security is not just code correctness — it’s process, governance, communication, and accountability.**
How to apply:
- Audits must be **budgeted, scheduled, and required** before deployment — with at least two independent teams for critical contracts.
- **No-go-to-market** policies without testing, fuzzing, and threat modeling.
- **Delay launch** if the protocol isn’t safe — security debt only compounds under capital inflow.
### DAO Governance Was Premature and Ill-Defined
What went wrong:
- The DAO let token holders vote on proposals, but the system:
    - Had **no formal process** for vetting or ranking proposals
    - Was vulnerable to **low voter turnout**, **whale influence**, and **attack through malicious proposals**
- Governance was bolted on, not **iteratively tested or modeled**.
Lesson:
> Don’t over-decentralize governance before the community and tooling are ready. Authority and accountability must be well-scoped.
How to apply:
- Start with **progressive decentralization** — launch with a core team, transition to DAO over time.
- Design governance with **checks and balances**, quorum thresholds, review periods, and vetting mechanisms.
- Simulate governance attacks (e.g. vote buying, bribery, rushed proposals) in testing.
### Too Much Capital, Too Quickly
- The DAO raised **$150M in ETH in under a month**, without a product, live governance track record, or security guarantees.
- **Hype and lack of regulation** led many to invest under FOMO.
- The contract instantly became a high-value honeypot.
Lesson:
> Capital is not a proxy for readiness. The more value you hold, the more you are a target — economically and socially.
- Cap early inflows or **use tranches**, where more funds are unlocked after milestones or audits.
- Use **guarded launches**, testnets, betas, or sandbox phases before accepting public capital.
- Adopt **risk-aware language in marketing** — no "trustless" claims without proof.
### Misalignment of Stakeholders
- The DAO assumed token holders, developers, and miners would all share aligned incentives — but the fork proved otherwise.
- **Ethereum Classic split off** partly from ideological commitment to immutability, while others prioritized user restitution.
> Systems with many stakeholders need clearly aligned and formalized incentive structures — not just assumptions of good will.
- Build **explicit social contracts**: what happens in a crisis? Who decides? What are the bounds of governance?
- **Model stakeholder incentives** using game theory and simulation before deploying major funds or protocols.
### Technical lessons learned (concrete)
These are the practices, patterns, and changes that became widely accepted because of The DAO:
#### Limit external control and minimize trust boundaries
The DAO trusted too much:
- It allowed any contract to request funds and control withdrawal behavior.
Lesson:
> **Restrict what external contracts can do, and sandbox them where possible.**
### In Rust:
- Clearly define **entry points**, expose minimal APIs to external actors.
- In Substrate:
    - Validate origins carefully (`ensure_signed()`, `ensure_root()`).
    - Avoid exposing critical APIs to arbitrary pallets or extrinsics.
- In CosmWasm:
    - Avoid executing `WasmMsg::Execute` based on arbitrary user input without filtering or validation.
#### Formal verification and fuzzing are essential
The DAO was unaudited, untested under adversarial conditions.
#### Write contracts with strict, explicit state machines
> Make illegal states unrepresentable. Use the type system.

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
# Terminology 
---
# Contact
---
unsudo@atomicmail.io