# bluetooth-mesh

![Rust](https://github.com/mfiumara/ble-mesh/workflows/Rust/badge.svg)

A rust bluetooth mesh implementation. The goals for this repo are to achieve a BLE mesh implementation, initially using only the Advertiser Bearer.

## BLE Core Implementation

There are some BLE implementations floating around on crates.io but these do not provide the GAP roles needed to implement a mesh stack. Mesh spec 3.3.1:

"All devices shall support both the GAP Observer role and GAP Broadcaster role."

Additionally:

"Any advertisement using the Mesh Message AD Type shall be non-connectable and non-scannable undirected advertising events. If a node receives a Mesh Message AD Type in a connectable advertisement or scannable advertising event, the message shall be ignored."

and:

"A device supporting only the advertising bearer should perform passive scanning with a duty cycle as close to 100 percent as possible in order to avoid missing any incoming mesh messages or Provisioning PDUs."

lead to the conclusion that the BLE core spec implementation only needs to perform passive scanning continuously, and some advertisement scheduling.
