// use bitvec::prelude as bv;
// use bitvec::prelude::*;

// pub enum AddressType {
//     AddressUnassigned,
//     AddressUnicast,
//     AddressVirtual = 0x8000,
//     AddressGroup = 0xc000,
// }

// pub fn new_pdu() -> BitVec<Msb0, u8>{

// }

// bitflags! {
//     struct PDUBITS: bv::BitVec<bv::Msb0, u8> {
//         const A = 0b00000001;
//         const B = 0b00000010;
//     }
// }

// PDU: Could be a struct, bitvector, byte array?
// Maybe a type
#[derive(Debug)]
struct NetworkPdu<'a> {
    ivi: bool,
    nid: u8,
    ctl: bool,
    ttl: u8,
    seq: u32,
    src: u16, // maybe an address type
    dst: u16,
    transport_pdu: &'a [u8],
    net_mic: &'a [u8],
    // transport_pdu: TransportPdu, // maybe a separate type
    // net_mic: NetMic, // maybe a separate type
}
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

    use crate::net::NetworkPdu;

    #[test]
    fn network_pdu_test() {
        let pdu = NetworkPdu {
            ivi: true,
            nid: 1,
            ctl: false,
            ttl: 1,
            seq: 4,
            src: 2,
            dst: 2,
            transport_pdu: &[0],
            net_mic: &[0],
        };
        assert_eq!(pdu.ivi, true);
    }
}