# Project Esperanto

Esperanto is a centralized attestation and policy management platform, designed to provide a single source of truth for hardware-level trust across heterogeneous systems. It functions as an "Okta for Hardware Root of Trust," allowing you to define and verify the integrity of your compute environments, from cloud to edge.

## About The Project

In modern computing, ensuring that your software is running on trusted hardware is critical, but also incredibly complex. Each platform—AWS Nitro Enclaves, TPM-enabled servers, ARM TrustZone devices—has its own unique method for cryptographic attestation.

Project Esperanto solves this by providing a unified API and a central policy engine to abstract away this complexity. Instead of building bespoke verification logic for each platform, you can simply ask Esperanto a single question: **"Is this environment trustworthy according to my organization's policy?"**

This allows you to securely provision secrets, bootstrap applications, and build zero-trust architectures with confidence.

### Core Architecture

The system is designed around a central `Esperanto Secure Core` that handles all verification logic, and lightweight agents that run on client machines.

![System Architecture](docs/architecture.jpg)

### Built With

- **Backend:** [Rust](https://golang.org/)
- **API:** RESTful API
- **Core Technologies:** AWS Nitro Enclaves, Intel SGX, AMD SEV, TPM 2.0, ARM TrustZone

# Codebase Architecture

This document outlines the codebase structure for the Project Esperanto workspace. The architecture is designed to enforce a clean separation of concerns, making the project testable, maintainable, and scalable.

The core principle is the **binary-library crate separation**. We treat our core business logic as an independent "engine" (`esperanto-core`) and the web server as a separate "driver" (`esperanto-server`) that simply exposes the engine's functionality.

## File Structure Overview

The project is a Cargo Workspace containing two primary crates: `esperanto-core` and `esperanto-server`.

/esperanto/ <-- The Git Repo & Cargo Workspace Root
├── .git/
├── Cargo.toml # The workspace manifest file
├── README.md
└── src/ # Contains BOTH crates
├── esperanto-core/ # The Engine / Library Crate
│ ├── Cargo.toml
│ └── src/
│ ├── lib.rs
│ ├── policy.rs
│ ├── verifier.rs
│ ├── error.rs
│ └── attestation/
│ ├── mod.rs
│ ├── types.rs
│ ├── nitro.rs
│ └── keylime.rs
└── esperanto-server/ # The Driver / Binary Crate
├── Cargo.toml
└── src/
├── main.rs
└── api/
├── mod.rs
├── router.rs
└── handlers.rs

## Component Breakdown

### Workspace Root (`/esperanto/`)

- **`Cargo.toml`**
  - **Purpose:** The Workspace Manifest. This is the top-level file that tells Rust that this directory manages a collection of crates. Its primary role is to list the `members` of the workspace (e.g., `esperanto-server`, `esperanto-core`).

### The Core Logic: `esperanto-core` (The Engine)

This crate is the heart of the product. It contains all valuable business logic and is completely independent of how that logic is presented (e.g., it has no web-related code).

- **`esperanto-core/Cargo.toml`**

  - **Purpose:** Defines the dependencies for the core library. This will include crates for parsing (`serde_yaml`), cryptography, and error handling (`thiserror`).

- **`esperanto-core/src/lib.rs`**

  - **Purpose:** The entrypoint of the library crate. It declares the public modules (like `policy`, `verifier`, etc.) to make their functions available to other crates that use `esperanto-core`.

- **`esperanto-core/src/policy.rs`**

  - **Purpose:** To handle all logic related to loading, parsing, and retrieving verification policies from the `policies.yml` file.

- **`esperanto-core/src/verifier.rs`**

  - **Purpose:** To act as the main dispatcher for verification logic. It contains functions that orchestrate the verification process by calling the appropriate platform-specific modules.

- **`esperanto-core/src/error.rs`**

  - **Purpose:** To define all possible custom errors that the core logic can produce (e.g., `PolicyNotFound`, `SignatureInvalid`, `PcrMismatch`). This ensures robust error handling.

- **`esperanto-core/src/attestation/`** (directory)
  - **Purpose:** A module dedicated to platform-specific attestation logic and data structures.
  - **`types.rs`**: Contains all the shared Rust `structs` for the project, such as `Policy`, `GoldenValues`, and the structures for API request and response bodies.
  - **`nitro.rs`**: Holds the expert cryptographic logic for verifying AWS Nitro Enclave attestations.
  - **`keylime.rs`**: Holds the logic for verifying attestations from a Keylime agent (TPM-based).

### The Web Server: `esperanto-server` (The Driver)

This crate's sole responsibility is to expose the functionality of `esperanto-core` over a web API. It contains no core business logic itself.

- **`esperanto-server/Cargo.toml`**

  - **Purpose:** Defines the dependencies for the web server executable.
  - **Content:** This is where `esperanto-core` is linked as a local dependency (`esperanto-core = { path = "../esperanto-core" }`). It also includes web-related crates like `tokio` and `axum`.

- **`esperanto-server/src/main.rs`**

  - **Purpose:** The main entrypoint for the runnable application.
  - **Content:** Responsible for setting up the runtime, initializing logging, creating the API router, binding to a port, and starting the server.

- **`esperanto-server/src/api/`** (directory)
  - **Purpose:** A module that contains all web-related logic.
  - **`router.rs`**: Defines all API routes (e.g., `POST /verify/nitro_enclave`) and maps them to the appropriate handler functions.
  - **`handlers.rs`**: The "bridge" between the web and the core logic. Each handler function parses an incoming HTTP request, calls a function from `esperanto-core` to do the actual work, and translates the result (`Ok` or `Err`) into a proper HTTP response for the client.
