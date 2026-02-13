use std::cmp::min;

use bincode::serialize;
use solana_perf::packet::{Packet, PACKET_DATA_SIZE};
use solana_sdk::transaction::VersionedTransaction;

use crate::packet::{Meta as ProtoMeta, Packet as ProtoPacket};

/// Converts a protobuf packet to a VersionedTransaction
pub fn versioned_tx_from_packet(p: &ProtoPacket) -> Option<VersionedTransaction> {
    let mut data = [0; PACKET_DATA_SIZE];
    let copy_len = min(data.len(), p.data.len());
    data[..copy_len].copy_from_slice(&p.data[..copy_len]);
    let mut packet = Packet::new(data, Default::default());
    if let Some(meta) = &p.meta {
        packet.meta_mut().size = meta.size as usize;
    }
    packet.deserialize_slice(..).ok()
}

/// Coverts a VersionedTransaction to packet
pub fn packet_from_versioned_tx(tx: VersionedTransaction) -> Packet {
    let tx_data = serialize(&tx).expect("serializes");
    let mut data = [0; PACKET_DATA_SIZE];
    let copy_len = min(tx_data.len(), data.len());
    data[..copy_len].copy_from_slice(&tx_data[..copy_len]);
    let mut packet = Packet::new(data, Default::default());
    packet.meta_mut().size = copy_len;
    packet
}

/// Converts a VersionedTransaction to a protobuf packet
pub fn proto_packet_from_versioned_tx(tx: &VersionedTransaction) -> ProtoPacket {
    let data = serialize(tx).expect("serializes");
    let size = data.len() as u64;
    ProtoPacket {
        data,
        meta: Some(ProtoMeta {
            size,
            addr: "".to_string(),
            port: 0,
            flags: None,
            sender_stake: 0,
        }),
    }
}
