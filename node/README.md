# Delmonicos

## Project Overview

### Overview

The goals of the project is to develop and deploy an infrastructure dedicated to the supervision of charging station,
strongly coupled with a micro-payment infrastructure. The technical requirements of the solution are the following :

* It must be very **secure**. Especially, only allowed people and computers should be able to interact with charging
  stations. Payments must fulfill latests security constraints especially regarding DSP2.
* It must be as **cheap** as possible to operate. The challenge is to meet the security requirements without having
  additonal costs or having to rely on external third-party.
* It must be **easy to deploy**.
* It must be **easily extensible**. Other parties should be able to join the networks without compromising the security
  of the infrastructure.

Taking into account the above-mentioned constraints, we have chosen to base our solution on blockchain technology, for
the following reasons :

* It is secure by design and **quasi impossible to hack** if carefully used and implemented.
* It allows **strong authentication** through the use of wallets
* It allows **secured delivery versus payment**. Each payment can be related to a dedicated delivery of service and the
  proofs of the delivery can be stored in case of dispute.
* The **trustless** nature of the technology allows adding new network participants without compromising the security of
  other participants.
* The deployment can be easy and secured if **each charging station is also a network node**.

The reasons why we chose Substrate/Polkadot are :

* By having nodes with a dedicated purpose, we will be able to optimise the footprint of the node in order to deploy it
  to charging stations, that have limited available resources
* We can choose the most suitable consensus mechanism and especially combine POA on the parachain with PoS on the relay
  chain.
* We have the flexibility to chose between multiple target topologies. E.g. one parachain for all energy providers on
  the platform, or one parachain by energy provider.
* The composability of the FRAME architecture allows us to reuse existing pallets (e.g. identity, membership, contracts,
  ...) while being able to add our own specific pallets.
* Since we have connections to multiple interfaces (payments, charging power, mobile apps) the offchain features of
  Substrate will alllow us to handle the connection with external sources in both directions in a secure manner.

### Project Details

The following schema describes the overall architecture of the project.
![Delmonicos architecture](https://github.com/lumena-tech/delmonicos/raw/main/w3f-grant-submission/delmonicos.jpg)


### Ecosystem Fit

We don't know any similar projects. The fact that charging stations themselves will be blockchain nodes is a real
innovation. We are currently speaking with [DBT](https://www.dbt.fr/en/)
and they are interested in working with us. If first phases are successful, we will very probably create a spinoff with
them.

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

## Future Plans

If this phase is successful, we will create a company dedicated to the development, marketing and selling of the
solution. First step will be to find investors (coming from industry or VCs). Future developments will include :
* The ability to automatically detect who is trying to charge his EV, based on the location of the station and on
  the location of users, choosing the closest user.
* A fully working production ready mobile app with an embedded wallet to identify and authenticate users, being
  compliant with RDPG and ideally EIDAS for signature. Being EIDAS compliant will probably not be possible at short
  term because the current regulation doesn't take into account blockchain specificities. We will nevertheless
  integrate concepts described in the following document : [SSI eIDAS Legal Report. How eIDAS can legally support digital identity and trustworthy DLT-based transactions in the Digital Single Market](https://joinup.ec.europa.eu/sites/default/files/document/2020-04/SSI_eIDAS_legal_report_final_0.pdf)
* A payment module that will be compliant with DSP2. Our goal is to develop a reusable module that interact with
  banks to prove that the acount owner has given his consent to initiate payment from his account. Here we can
  leverage the security of the blockchain to comply with PISP constraints. Opportunities are well described [here](https://worldline.com/en/home/knowledgehub/blog/2018/january/PSD2-and-instant-payment-what-are-the-opportunities-for-e-merchants.html).
* Capability to add innovative possibilities via Ink! Smart Contracts. e.g. variable pricing depending on the load
  of the energy network, pricing could be determined by consensus, charging session reservations, etc...

## Additional Information :heavy_plus_sign:

We are convinced that this project has a huge potential to solve the majority of problems or paint points that EV
users are facing regarding charging their vehicule. This is typically a domain where blockchain technology is the ideal
technology by allowing the secure digitalisation of assets (charging power, money and identity) and the secure
conversion of value between these assets. Our potential partnership with DBT will give us a direct access to the
market.
