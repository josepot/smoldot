// Smoldot
// Copyright (C) 2019-2022  Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! JSON-RPC servers. Trusted access to the blockchain.
//!
//! # Context
//!
//! The traditional model used in the blockchain ecosystem is this one:
//!
//! ```notrust
//!
//! +---------------+              +--------+            +----------------------+
//! |               |   JSON-RPC   |        |   libp2p   |                      |
//! |  Application  |  <-------->  |  Node  |  <------>  |  Blockchain network  |
//! |  (e.g. a UI)  |              |        |            |                      |
//! +---------------+              +--------+            +----------------------+
//!
//! ```
//!
//! The node is connected to the blockchain network using the libp2p protocol, and one or more
//! applications are connected to the node using the JSON-RPC protocol.
//!
//! > **Note**: An example application that can be put on top of a node is
//! >           [PolkadotJS Apps](https://cloudflare-ipfs.com/ipns/dotapps.io/).
//!
//! Contrary to the traffic with the blockchain network, the communication between the JSON-RPC
//! client (i.e. the application) and the JSON-RPC server (i.e. the node) is not trustless. In
//! other words, the client has no way to check the accuracy of what the server sends, and
//! therefore trusts that the server isn't malicious. The role of the node is precisely to turn
//! untrusted data coming from the blockchain network into trusted information.
//!
//! The trust goes the other way around: some of the JSON-RPC requests that the client can perform
//! can modify the configuration of the node, while some others are very CPU or I/O-intensive. As
//! such, the server also trusts the client to not be malicious.
//!
//! # JSON-RPC protocol
//!
//! The protocol used is the JSON-RPC v2.0 protocol described [here](https://www.jsonrpc.org/).
//!
//! As specified in the JSON-RPC v2.0 protocol, both sides of the TCP/IP connection may send
//! requests and/or notifications to the other side.
//!
//! In practice, the listening side of the connection should be capable of serving the various
//! JSON-RPC functions described in the [`methods`] submodule. As part of the logic of these
//! functions, the listening side might send notifications to the initiator of the connection.
//!

// TODO: write docs about usage ^

pub mod methods;
pub mod parse;
pub mod payment_info;
pub mod service;
pub mod websocket_server;
