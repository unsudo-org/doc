---
title: "Unsudo Protocol Specification"
version: "0.1.0"
date: "2025-10-11"
language: "en"
encoding: "UTF-8"
hash: "sha256:b59c67bf196a4758191e42f76670ceba"
license: "CC-BY-4.0"
---


This is a mock up

# Unsudo 0.1.0  

### ⌘ A story about resilience

---

## Contents

1. Abstract  
2. Vision  
3. Value Proposition  
4. Problem Statement  
5. Market Opportunity  
6. Governance  
7. Token Economics  
8. System Architecture  
   - Polkadot JAM  
   - Migration  
   - WASM  

---

# Problem Statement

---

## ⌘ The Cost of Censorship

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

---

## ⌘ The Hidden Cost of Corruption

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

---

# Barriers to Blockchain Adoption

Despite the promise of decentralization and financial empowerment, blockchain adoption faces **critical barriers** across technical, cultural, legal, and usability domains.

### 1. **Myth of Decentralization**

Many so-called "decentralized" apps still rely on:

- Centralized servers  
- VC-controlled governance  
- Custodial wallets

This undermines the fundamental ethos and creates points of failure vulnerable to both external and internal pressure.

### 2. **Community Tribalism**

Blockchain communities often resist critique, creating echo chambers. This:

- Discourages healthy debate  
- Slows progress on standards and interoperability  
- Hinders public accountability  

### 3. **Regulatory Uncertainty**

- Laws are vague or in flux (especially around securities, privacy coins, DeFi).
- Compliance risk is high for both users and builders.
- Legal ambiguity deters innovation and adoption.

### 4. **Censorship at the Infrastructure Layer**

Even decentralized protocols rely on centralized services:

- Web hosting (e.g., IPFS gateways, DNS)
- Cloud platforms
- App store distribution

Front-ends can be taken offline, wallets blocked, or content delisted — revealing **fragility beneath the surface**.

### 5. **Poor User Experience**

- Private key management is error-prone.
- Wallet UX is complex and fragmented.
- Gas fees and smart contracts are intimidating.
- Mistakes are often irreversible.

### 6. **Architecture Limitations**

- On-chain systems are poor at handling real-time interactions.
- Streaming, gaming, or live data often rely on centralized backends.
- Layer 2 and micropayment systems exist, but adoption remains limited due to UX complexity.

### 7. **Developer Burden**

Developing for blockchain requires:

- Learning new tools (Solidity, Rust, Move, etc.)
- Managing wallets, gas, security
- Navigating fragmentation between chains

Bugs can be permanent and costly. The mental load is high.

### 8. **Multichain Fragmentation**

- Bridges are vulnerable and inefficient.
- Cross-chain tools lack maturity.
- Users must juggle assets, risks, and inconsistent interfaces across ecosystems.

### 9. **Security Vulnerabilities**

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

---

### In Summary:

Blockchain’s full potential remains **locked behind real barriers**:

- Structural: centralized dependencies  
- Cultural: tribalism, elitism  
- Regulatory: hostile or ambiguous legal systems  
- Usability: steep learning curves, poor UX  
- Security: code is law, but few understand the law

Until these are addressed, the promise of Web3 will remain just that — **a promise**.



# Value Proposition

---

Our decision to build in *Rust* offers a clear competitive advantage over *Solidity* and other non-Rust platforms. Rust provides a modern, performance-oriented, and secure foundation that aligns with the demands of next-generation Web3 infrastructure.

Below are the key areas where Rust stands out:

1. Safely and Reliability by Design







- Competitive

>> Tech Stack

>> Competitive advantages of building in Rust (vs Solidity or non-Rust platforms).
    
    >> Superior safety and reliability - Rust offers memory safety without garbage collection or manual memory management.
        >> Reduces attack surface (buffer overflows, null pointer errors).
        >> Eliminates whole classes of runtime bugs common in Solidity and JavaScript-based chains.
    
        >> Strong static typing and compile time checks.
            >> Prevents logic errors before deployment.
            >> More secure by design compared to Solidity's dynamic typing.

    >> System-level flexibility - Custom runtimes possible in Rust (vs fixed VM in Solidity).
        >> No need to conform to EVM opcodes or limitations.
        >> Developers can build tailored execution environments for specific use cases.
        
        >> Supports async logic and multithreading.
            >> Enabled use cases like real-time collaboration, gaming, or data streaming.
            >> Solidity and EVM are fundamentally synchronous, blocking execution models limit innovation.

    >> Performance and scalability.
        >> Rust compiles to high-performance WASM (used in Polkadot JAM). With lower latency, faster execution, and deterministic behavior.
        >> Faster runtimes mean cheaper operations. JAM-style chains can offer much lower transaction fees vs EVM chains.
        >> Parallelism-ready architecture. Unlike Solidity's single-threaded model, Rust-based chains can support parallel execution and async processing.

    >> Developer experience and ecosystem quality.
        >> Modern developer tooling and IDE support. Rust has best-in-class tooling (cargo, clippy, rust analyzer).
        >> Growing open-source ecosystem. Shared libraries (cryptography, networking, WASM) are actively maintained and production-grade.
        
        >> Stronger engineering talent pool.
            >> Rust developers tend to be experienced systems engineers.
            >> Harder to hire solidity developers, and many Solidity developers lack relevant experience.

    >> Governance and upgradability.
        >> Rust-based chains like Polkadot support native onchain governance.
            >> More robust upgrade mechanisms than most Solidity chains.
            >> Avoids the risk of proxy patterns and delegate calls common in Solidity upgrades.
        
    >> Interoperability and composability.
        >> Polkadot JAM offers async cross-chain messaging XCM. Rust native parachains can interoperate without relying on fragile bridges.
        >> Most EVM chains are isolated unless bridged. Bridges introduce security vulnerabilities (Harmony, Ronin, Wormhole hacks).
        >> Rust ecosystem can interconnect or influence shared standards.

    >> First-mover advantage in Rust-native web3.
        >> Most web3 projects are still Solidity/EVM-based. Rust-native services are underrepresented despite better architecture.
        >> Being early in the JAM ecosystem or Polkadot 2.0 positions us positively.
            >> As an infrastructure leader rather than another dApp.
            >> To capture developer migration from Web2 systems engineering to Web3.






>> Rust vs. Solidity.
    >> Solidity is still the dominant language for smart contract develoment (EVM, Ethereum).
    
    >> Solidity's core limitations - Solidity is ideal for DeFi 1.0 and simple contracts, not full-scale decentralized systems.
        >> Poor scalability - Monolithic execution layers can't handle complex dApps.
        >> Gas cost unpredictability - Friction for both developers and users.
        >> Security vulnerabilities - Weak typing leads to runtime errors and exploits.
        >> Limited async support - Streaming, real-time logic is nearly impossible natively.
        >> Tooling fragmentation - Debugging, testing, and optimization are still immature.
       
    >> Rust.
        >> Custom runtimes - Full control over execution logic.
        >> Asynchronous composability - Support for real-time coordination and data flow.
        >> Shared security - No need to bootstrap own validator set.
        
   >> Infrastructure-layer innovation.





# Governance

---




# Compliance

## GDPR Compliance

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

##### Grounds for Penalty

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



# Core Contributor

# Bounty











# System Architecture

## Polkadot JAM

### Service & CorePlay

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




