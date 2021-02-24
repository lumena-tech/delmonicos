# Open Grant Proposal

> This document is referenced in the terms and conditions and therefore needs to contain all the required information. Don't remove any of the mandatory parts presented in bold letters or as headlines! See the [Open Grants Program Process](https://github.com/w3f/Open-Grants-Program/blob/master/README_2.md) on how to submit a proposal.

> This page is also available in [Chinese (中文)](./application-template-cn.md).

* **Project Name:** Delmonicos
* **Team Name:** Lumena
* **Payment Address:** BTC 

*The above combination of your GitHub account submitting the application and payment address will be your unique identifier during the program. Please keep them safe.*

## Project Overview :page_facing_up:
This application is not in response to an RFP.

### Overview

The goals of the project is to develop and deploy an infrastructure dedicated to the supervision of charging station, strongly coupled
with a micro-payment infrastructure. The technical requirements of the solution are the following :
* It must be very **secure**. Especially, only allowed people and computers should be able to interact with charging stations.
  Payments must fulfill latests security constraints especially regarding DSP2.
* It must be as **cheap** as possible to operate. The challenge is to meet the security requirements without having additonal costs or
  having to rely on external third-party.
* It must be **easy to deploy**.
* It must be **easily extensible**. Other parties should be able to join the networks without compromising the security of the infrastructure.

Taking into account the above-mentioned constraints, we have chosen to base our solution on blockchain technology, for the following reasons :
* It is secure by design and **quasi impossible to hack** if carefully used and implemented.
* It allows **strong authentication** through the use of wallets
* It allows **secured delivery versus payment**. Each payment can be related to a dedicated delivery of service and the proofs of the delivery can
  be stored in case of dispute.
* The **trustless** nature of the technology allows adding new network participants without compromising the security of other participants.
* The deployment can be easy and secured if **each charging station is also a network node**.

The reasons why we chose Substrate/Polkadot are :
* By having nodes with a dedicated purpose, we will be able to optimise the footprint of the node in order to deploy it to charging stations,
  that have limited available resources
* We can choose the most suitable consensus mechanism and especially combine POA on the parachain with PoS on the relay chain.
* We have the flexibility to chose between multiple target topologies. E.g. one parachain for all energy providers on the platform, or one parachain by
  energy provider.
* The composability of the FRAME architecture allows us to reuse existing pallets (e.g. identity, membership, contracts, ...) while being able to add
  our own specific pallets.
* Since we have connections to multiple interfaces (payments, charging power, mobile apps) the offchain features of Substrate will alllow us to handle
  the connection with external sources in both directions in a secure manner.

Lumena is creating this project because it is a startup studio focused on innovative technologies and especially blockchain. Our goal is to create new generation of services
in collaboration with industry partners. We have a well structured process in order to decide if we create a company or not. The following schema details this process :

![Lumena Process Description](https://github.com/lumena-tech/delmonicos/blob/main/w3f-grant-submission/lumena_process.png?raw=true)

For Delmonicos, we are currently in phase two where we have to develop a POC of the platform, in order to address risks and to show the feasibility of the project.
If we can fund this phase and if the results are positive, we'll create a company dedicated to develop and sell the platform.
We are applying for the 30k$ Open Grant Program from w3f that will definitely allow us to do these two phases.

### Project Details

* Mockups/designs of any UI components : We have not yet realized mockups nor UI design. We will have connect existing supervision platform to a back-end API.
  We will also have a mobile app with an embedded wallet to validate charging sessions. 
* API's specifications of the core functionality : not yet defined.
* An overview of the technology stack to be used. Rust for the blockchain development with substrate and veru probably Ink! to be able to easily inject new rules.
  React on the front-end. Mobile app will be developed natively, starting with ios. The non blockchain back-end will be developed in Rust with actix-web for the http API. We will start by
  developing a blockchain front-end similar to substrate-node-ui-template.
* Documentation of core components, protocols, architecture etc. to be deployed
* PoC/MVP or other relevant prior work or research on the topic. Current work is located at <https://github.com/lumena-tech/delmonicos>

### Ecosystem Fit
We don't know any similar projects. The fact that charging stations themselves will be blockchain nodes is a real innovation. We are currently speaking with [DBT](https://www.dbt.fr/en/)
and they are interested in working with us. If first phases are successful, we will very probably create a spinoff with them. 

## Team :busts_in_silhouette:

### Team members
* Fabrice CROISEAUX
* Michel ONFRAY
* Antoine DETANTE
* Franck LEGARDEUR

### Contact
* **Contact Name:** Fabrice CROISEAUX
* **Contact Email:** fabrice.croiseaux@lumena.tech
* Website: <https://www.lumena.tech>

### Legal Structure
* **Registered Address:** 2 rue Maurice Barrès, 57000 METZ FR
* **Registered Legal Entity:** Lumena

### Team's experience
Please describe the team's relevant experience.  If the project involves development work, then we'd appreciated if you can single out a few interesting code commits made by team members on their past projects. For research-related grants, references to past publications and projects in a related domain are helpful.

### Team Code Repos
* https://github.com/lumena-tech/delmonico

### Team LinkedIn Profiles
* https://www.linkedin.com/in/fcroiseaux
* https://www.linkedin.com/in/michel-onfray-464918b7/
* https://www.linkedin.com/in/adetante/
* https://www.linkedin.com/in/franck-legardeur-a05577/

## Development Roadmap :nut_and_bolt:

This section should break out the development roadmap into a number of milestones. Since the milestones will appear in the grant contract, it helps to describe the functionality we should expect, plus how we can check that such functionality exists in the product. Whenever milestones are delivered, we refer to the contract to ensure that everything has been delivered as expected.

Below we provide an **example roadmap**. In the descriptions it should be clear how the project is related to Substrate and/or Polkadot. We recommend that the scope of the work can fit within a 3 month period and that teams structure their roadmap as 1 month = 1 milestone.

For each milestone:
* Please be sure to include a specification of your software. Treat it as a contract - the level of detail must be enough to later verify that the software meets the specification.
  To assist you in defining it, we created a document with examples for some grant categories [here](../src/grant_guidelines_per_category.md).
* Please include total amount of funding requested per milestone.
* Please note that we require documentation (e.g. tutorials, API specifications, architecture details) in each milestone. This ensures that the code can be widely used by the community.
* Please provide a test suite, comprising unit and integration tests, along with a guide on how to run these.
* Please commit to providing a dockerfiles for the delivery of your project.
* Please indicate the milestone duration, as well as number of Full-Time Employees working on each milestone, and include the number of days along with their cost per day.
* Deliverables 0a-0d are mandatory and should not be removed, unless you explicitly specify a reason within the PR's `Additional Notes` section (e.g. Milestone X is research oriented and as such there is no code to test)

### Overview
* **Total Estimated Duration:** Duration of the whole project (e.g. 2 months)
* **Full-time equivalent (FTE):**  Workload of an employed person ([see](https://en.wikipedia.org/wiki/Full-time_equivalent)) (e.g. 2 FTE)
* **Total Costs:** Amount of Payment in USD for the whole project. The total amount of funding needs to be below $30k for initial grants and $100k for follow-up grants at the time of submission. (e.g. 1.000 USD)

### Milestone 1 Example — Implement Substrate Modules
* **Estimated Duration:** 1 month
* **FTE:**  1
* **Costs:** 1.000 USD

| Number | Deliverable | Specification |
| ------------- | ------------- | ------------- |
| 0a. | License | Apache 2.0 / MIT / Unlicense |
| 0b. | Documentation | We will provide both inline documentation of the code and a basic tutorial that explains how a user can (for example) spin up one of our Substrate nodes. Once the node is up, it will be possible to send test transactions that will show how the new functionality works. |
| 0c. | Testing Guide | The code will have unit-test coverage (min. 70%) to ensure functionality and robustness. In the guide we will describe how to run these tests | 
| 0d. | Article/Tutorial | We will write an article or tutorial that explains the work done as part of the grant.
| 1. | Substrate module: X | We will create a Substrate module that will... (Please list the functionality that will be coded for the first milestone) |  
| 2. | Substrate module: Y | We will create a Substrate module that will... |  
| 3. | Substrate module: Z | We will create a Substrate module that will... |  
| 4. | Substrate chain | Modules X, Y & Z of our custom chain will interact in such a way... (Please describe the deliverable here as detailed as possible) |  
| 5. | Docker | We will provide a dockerfile to demonstrate the full functionality of our chain |

### Milestone 2 Example — Additional features
...

## Future Plans
Please include the team's long-term plans and intentions.

## Additional Information :heavy_plus_sign:
Any additional information that you think is relevant to this application that hasn't already been included.

Possible additional information to include:
* What work has been done so far?
* Are there are any teams who have already contributed (financially) to the project?
* Have you applied for other grants so far?
