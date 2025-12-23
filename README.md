# Economy & Household Simulator (Rust + Ratatui)

A **terminal-based economy and household simulation** written in **Rust**, featuring a **real-time, deterministic simulation engine**, an **interactive Ratatui (TUI) interface**, and an **XML-driven structural definition system**.

All **structural elements of the simulation**—including goods, needs, household archetypes, production rules, and economic relationships—are defined using **XML files**. This allows the simulation to be reconfigured, extended, or entirely redefined **without recompiling the binary**.

---

## Table of Contents

* [Overview](#overview)
* [Design Goals](#design-goals)
* [Core Concepts](#core-concepts)
* [XML-Driven Architecture](#xml-driven-architecture)
* [Simulation Model](#simulation-model)

    * [Households](#households)
    * [Economy](#economy)
    * [Market Mechanics](#market-mechanics)
* [User Interface (Ratatui)](#user-interface-ratatui)
* [Controls](#controls)
* [Project Structure](#project-structure)
* [Simulation Loop](#simulation-loop)
* [Configuration vs Structure](#configuration-vs-structure)
* [Saving & Loading](#saving--loading)
* [Determinism & Reproducibility](#determinism--reproducibility)
* [Performance Considerations](#performance-considerations)
* [Roadmap](#roadmap)
* [Building & Running](#building--running)
* [Contributing](#contributing)
* [License](#license)

---

## Overview

This project simulates a **closed economy** composed of households and markets. Each simulation tick advances time, updates production and consumption, recalculates prices, and modifies household wealth and utility.

The simulation logic is **fully decoupled** from presentation and data definition:

* **Rust code** defines *systems and rules*
* **XML files** define *what exists*
* **Ratatui** renders *current state*

This separation enables rapid iteration, modding, and experimentation.

---

## Design Goals

* Deterministic and reproducible simulation
* Data-driven economy definitions
* Zero hidden state
* Inspectable internal values
* High performance at large agent counts
* Terminal-first UX

---

## Core Concepts

| Concept   | Description                               |
| --------- | ----------------------------------------- |
| Tick      | Smallest unit of simulated time           |
| Household | Autonomous economic agent                 |
| Market    | Price discovery mechanism                 |
| Good      | Consumable or storable resource           |
| Need      | A recurring requirement for a household   |
| Utility   | Satisfaction derived from fulfilled needs |

---

## XML-Driven Architecture

All **structural content** of the simulation is defined using **XML**.

The Rust code does **not hardcode**:

* Goods
* Needs
* Household types
* Production chains
* Consumption rules
* Economic relationships

Instead, these are **loaded at startup** and compiled into efficient runtime representations.

### What Is Defined in XML

* **Goods**

    * Names, categories
    * Base prices
    * Stackability
    * Decay rules
* **Needs**

    * Frequency
    * Priority
    * Fulfillment conditions
* **Household Archetypes**

    * Starting inventory
    * Income sources
    * Behavioral parameters
* **Production Rules**

    * Inputs
    * Outputs
    * Time costs
* **Economic Constants**

    * Elasticities
    * Thresholds
    * Scaling factors

### Example XML Snippet

```xml
<good id="food">
    <display_name>Food</display_name>
    <base_price>10.0</base_price>
    <decay_rate>0.01</decay_rate>
</good>
```

```xml
<household_type id="worker">
    <starting_cash>1000</starting_cash>
    <needs>
        <need ref="food" amount="1.0" interval="1d"/>
    </needs>
</household_type>
```

### Runtime Representation

At load time:

1. XML is parsed and validated
2. Cross-references are resolved
3. IDs are interned
4. Data is converted into cache-friendly Rust structures

After loading, **no XML parsing occurs during simulation ticks**.

---

## Simulation Model

### Households

Each household instance is created from an XML-defined archetype and maintains:

* Cash balance
* Inventory
* Active needs
* Utility score
* Historical metrics

Decision-making is rule-based and deterministic.

```rust
struct Household {
    id: HouseholdId,
    cash: f64,
    inventory: Inventory,
    needs: NeedsState,
    utility: f64,
}
```

---

### Economy

The economy aggregates all agent activity and tracks:

* Total money supply
* Production output
* Consumption volume
* Average prices
* Wealth distribution

Macro metrics are derived, not stored.

---

### Market Mechanics

Markets operate via **continuous price adjustment** rather than instant clearing.

* Supply and demand are measured per tick
* Prices adjust gradually
* Shocks propagate over time
* No perfect information assumptions

---

## User Interface (Ratatui)

The UI is built using **Ratatui** and reflects **live simulation state**.

### Panels

* Simulation overview
* Household inspection
* Market data
* Debug / internal state view

### Rendering Guarantees

* No mutation during render
* Read-only access to simulation state
* Predictable frame times

---

## Controls

| Key   | Action                    |
| ----- | ------------------------- |
| `q`   | Quit                      |
| `p`   | Pause / Resume            |
| `.`   | Step one tick             |
| `+`   | Increase simulation speed |
| `-`   | Decrease simulation speed |
| `↑/↓` | Navigate lists            |
| `←/→` | Change view               |
| `r`   | Reset simulation          |
| `d`   | Toggle debug panel        |

---

## Project Structure

```text
src/
├── main.rs
├── app.rs
├── simulation/
│   ├── engine.rs
│   ├── household.rs
│   ├── market.rs
│   └── economy.rs
├── data/
│   ├── goods.xml
│   ├── needs.xml
│   ├── households.xml
│   └── production.xml
├── ui/
│   ├── layout.rs
│   └── render.rs
├── config.rs
└── util.rs
```

---

## Simulation Loop

The simulation loop is **time-agnostic** and **render-independent**:

1. Input handling
2. Zero or more simulation ticks
3. Metric recomputation
4. UI render
5. Frame pacing

Fast-forwarding runs ticks without rendering intermediate frames.

---

## Configuration vs Structure

**Configuration (TOML / CLI):**

* Tick rate
* UI refresh rate
* Debug flags
* Random seed

**Structure (XML):**

* What entities exist
* How they behave
* How the economy is shaped

This distinction is intentional and enforced.

---

## Saving & Loading

Planned support for:

* Full state serialization
* Deterministic replays
* XML + binary hybrid saves

---

## Determinism & Reproducibility

The simulation is deterministic given:

* XML definitions
* Config values
* Random seed
* Input sequence

This enables regression testing and scientific experimentation.

---

## Performance Considerations

* No allocation in hot loops
* Pre-resolved IDs
* Minimal indirection
* Scales to thousands of households

---

## Roadmap

* [ ] Multiple goods and industries
* [ ] Labor markets and wages
* [ ] Housing, rent, ownership
* [ ] Government and taxation
* [ ] External trade
* [ ] Save/load system
* [ ] Historical graphs
* [ ] Behavior scripting

---

## Building & Running

```bash
cargo run --release
```

---

## Contributing

* Keep logic deterministic
* Keep structure data-driven
* Avoid embedding content in code
* Prefer clarity over cleverness

---

## License
