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

Gonna need to rework this.