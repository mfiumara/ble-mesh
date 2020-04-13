// use bitvec::prelude as bv;
use bitvec::prelude::*;

// enum NetAddr {
//     UnassignedAddr,
//     UnicastAddr,
//     VirtualAddr,
//     GroupAddr,
// }

// bitflags! {
//     struct PDUBITS: bv::BitVec<bv::Msb0, u8> {
//         const A = 0b00000001;
//         const B = 0b00000010;
//     }
// }

// #[derive(Debug)]
struct Pdu {
    // Mesh spec 3.1.1: "For the network layer, lower transport layer, upper
    // transport layer, mesh beacons, and Provisioning, all multiple-octect
    // numeric values shall be sent in big endian"
    bits: BitVec<Msb0, u8>
}

// Mesh Spec 3.4.4
impl Pdu {
    // The IVI field contains the least significant bit of the IV Index used in
    // the nonce to authenticate and encrypt this Network PDU.
    fn ivi(&self) -> &BitSlice<Msb0, u8> {
        &self.bits[0..1]
    }
    // The NID field contains a 7-bit network identifier that allows for an
    // easier lookup of the Encryption Key and Privacy Key used to 
    // authenticate and encrypt this Network PDU.
    fn nid(&self) -> &BitSlice<Msb0, u8> {
        &self.bits[1..8]
    }
    // The CTL field is a 1-bit value that is used to determine if the message
    // is part of a Control Message or an Access Message.
    fn ctl(&self) -> &BitSlice<Msb0, u8> {
        &self.bits[8..9]
    }
    fn ttl(&self) -> &BitSlice<Msb0, u8> {
        &self.bits[9..16]
    }
    fn seq(&self) -> &BitSlice<Msb0, u8> {
        &self.bits[16..40]
    }
    fn src(&self) -> &BitSlice<Msb0, u8> {
        &self.bits[40..56]
    }
    fn dst(&self) -> &BitSlice<Msb0, u8> {
        &self.bits[56..72]
    }
    fn transport_pdu(&self) -> &BitSlice<Msb0, u8> {
        match self.ctl()[0] {
            false => &self.bits[72..self.bits.len() - 32],
            true  => &self.bits[72..self.bits.len() - 64]
        }
    }
    fn net_mic(&self) -> &BitSlice<Msb0, u8> {
        match self.ctl()[0] {
            false => &self.bits[self.bits.len() - 32..],
            true  => &self.bits[self.bits.len() - 64..]
        }
    }
}

struct Interface {}

impl Interface {
    fn discard(&self) {
        // Take ownership of a PDU and discard it.
    }
    fn relay(&self) {
        // Relay a PDU.
    }
}

// pub fn new_pdu() -> BitVec<Msb0, u8>{

// }

//     IVI // 1 bit
//     NID // 7 bits
//     CTL // 1 bit
//     TTL // 7 bits
//     SEQ // 3 bytes
//     SRC // 2 bytes
//     DST // 2 bytes
//     TransportPdu // 8 - 128 bits. if CTL == 0 max 128, if CTL == 1 max 96
//     NetMIC // 32 bits if CTL == 0, 64 bits if CTL == 1

// label UUID = 128bit

#[cfg(test)]
mod tests {

    use crate::net::Pdu;

    #[test]
    fn network_pdu_test() {
        // let pdu = Pdu {
        //     ivi: true,
        //     nid: 1,
        //     ctl: false,
        //     ttl: 1,
        //     seq: 4,
        //     src: 2,
        //     dst: 2,
        //     transport_pdu: &[0],
        //     net_mic: &[0],
        // };
        assert_eq!(true, true);
    }
}
