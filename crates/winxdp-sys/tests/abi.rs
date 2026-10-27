#![cfg(windows)]
#![allow(clippy::unnecessary_cast)]
use winxdp_sys::*;
unsafe extern "C" {
    fn wx_abi(id: u32) -> usize;
}
#[test]
fn native_layouts_and_constants() {
    assert_eq!(
        unsafe { wx_abi(0) },
        std::mem::size_of::<XDP_CHECKSUM_CONFIGURATION>(),
        "XDP_CHECKSUM_CONFIGURATION"
    );
    assert_eq!(
        unsafe { wx_abi(1) },
        std::mem::align_of::<XDP_CHECKSUM_CONFIGURATION>(),
        "XDP_CHECKSUM_CONFIGURATION"
    );
    assert_eq!(
        unsafe { wx_abi(2) },
        std::mem::offset_of!(XDP_CHECKSUM_CONFIGURATION, Header),
        "XDP_CHECKSUM_CONFIGURATION.Header"
    );
    assert_eq!(
        unsafe { wx_abi(3) },
        std::mem::offset_of!(XDP_CHECKSUM_CONFIGURATION, Enabled),
        "XDP_CHECKSUM_CONFIGURATION.Enabled"
    );
    assert_eq!(
        unsafe { wx_abi(4) },
        std::mem::offset_of!(XDP_CHECKSUM_CONFIGURATION, TcpOptions),
        "XDP_CHECKSUM_CONFIGURATION.TcpOptions"
    );
    assert_eq!(
        unsafe { wx_abi(5) },
        std::mem::size_of::<XDP_CREATE_PROGRAM_FLAGS>(),
        "XDP_CREATE_PROGRAM_FLAGS"
    );
    assert_eq!(
        unsafe { wx_abi(6) },
        std::mem::align_of::<XDP_CREATE_PROGRAM_FLAGS>(),
        "XDP_CREATE_PROGRAM_FLAGS"
    );
    assert_eq!(
        unsafe { wx_abi(7) },
        std::mem::size_of::<XDP_EBPF_PARAMS>(),
        "XDP_EBPF_PARAMS"
    );
    assert_eq!(
        unsafe { wx_abi(8) },
        std::mem::align_of::<XDP_EBPF_PARAMS>(),
        "XDP_EBPF_PARAMS"
    );
    assert_eq!(
        unsafe { wx_abi(9) },
        std::mem::offset_of!(XDP_EBPF_PARAMS, Target),
        "XDP_EBPF_PARAMS.Target"
    );
    assert_eq!(
        unsafe { wx_abi(10) },
        std::mem::size_of::<XDP_FRAME_CHECKSUM>(),
        "XDP_FRAME_CHECKSUM"
    );
    assert_eq!(
        unsafe { wx_abi(11) },
        std::mem::align_of::<XDP_FRAME_CHECKSUM>(),
        "XDP_FRAME_CHECKSUM"
    );
    assert_eq!(
        unsafe { wx_abi(12) },
        std::mem::size_of::<XDP_FRAME_LAYER2_TYPE>(),
        "XDP_FRAME_LAYER2_TYPE"
    );
    assert_eq!(
        unsafe { wx_abi(13) },
        std::mem::align_of::<XDP_FRAME_LAYER2_TYPE>(),
        "XDP_FRAME_LAYER2_TYPE"
    );
    assert_eq!(
        unsafe { wx_abi(14) },
        std::mem::size_of::<XDP_FRAME_LAYER3_TYPE>(),
        "XDP_FRAME_LAYER3_TYPE"
    );
    assert_eq!(
        unsafe { wx_abi(15) },
        std::mem::align_of::<XDP_FRAME_LAYER3_TYPE>(),
        "XDP_FRAME_LAYER3_TYPE"
    );
    assert_eq!(
        unsafe { wx_abi(16) },
        std::mem::size_of::<XDP_FRAME_LAYER4_TYPE>(),
        "XDP_FRAME_LAYER4_TYPE"
    );
    assert_eq!(
        unsafe { wx_abi(17) },
        std::mem::align_of::<XDP_FRAME_LAYER4_TYPE>(),
        "XDP_FRAME_LAYER4_TYPE"
    );
    assert_eq!(
        unsafe { wx_abi(18) },
        std::mem::size_of::<XDP_FRAME_LAYOUT>(),
        "XDP_FRAME_LAYOUT"
    );
    assert_eq!(
        unsafe { wx_abi(19) },
        std::mem::align_of::<XDP_FRAME_LAYOUT>(),
        "XDP_FRAME_LAYOUT"
    );
    assert_eq!(
        unsafe { wx_abi(20) },
        std::mem::size_of::<XDP_FRAME_RX_CHECKSUM_EVALUATION>(),
        "XDP_FRAME_RX_CHECKSUM_EVALUATION"
    );
    assert_eq!(
        unsafe { wx_abi(21) },
        std::mem::align_of::<XDP_FRAME_RX_CHECKSUM_EVALUATION>(),
        "XDP_FRAME_RX_CHECKSUM_EVALUATION"
    );
    assert_eq!(
        unsafe { wx_abi(22) },
        std::mem::size_of::<XDP_FRAME_TIMESTAMP>(),
        "XDP_FRAME_TIMESTAMP"
    );
    assert_eq!(
        unsafe { wx_abi(23) },
        std::mem::align_of::<XDP_FRAME_TIMESTAMP>(),
        "XDP_FRAME_TIMESTAMP"
    );
    assert_eq!(
        unsafe { wx_abi(24) },
        std::mem::offset_of!(XDP_FRAME_TIMESTAMP, Timestamp),
        "XDP_FRAME_TIMESTAMP.Timestamp"
    );
    assert_eq!(
        unsafe { wx_abi(25) },
        std::mem::size_of::<XDP_FRAME_TX_CHECKSUM_ACTION>(),
        "XDP_FRAME_TX_CHECKSUM_ACTION"
    );
    assert_eq!(
        unsafe { wx_abi(26) },
        std::mem::align_of::<XDP_FRAME_TX_CHECKSUM_ACTION>(),
        "XDP_FRAME_TX_CHECKSUM_ACTION"
    );
    assert_eq!(
        unsafe { wx_abi(27) },
        std::mem::size_of::<XDP_HOOK_DATAPATH_DIRECTION>(),
        "XDP_HOOK_DATAPATH_DIRECTION"
    );
    assert_eq!(
        unsafe { wx_abi(28) },
        std::mem::align_of::<XDP_HOOK_DATAPATH_DIRECTION>(),
        "XDP_HOOK_DATAPATH_DIRECTION"
    );
    assert_eq!(
        unsafe { wx_abi(29) },
        std::mem::size_of::<XDP_HOOK_ID>(),
        "XDP_HOOK_ID"
    );
    assert_eq!(
        unsafe { wx_abi(30) },
        std::mem::align_of::<XDP_HOOK_ID>(),
        "XDP_HOOK_ID"
    );
    assert_eq!(
        unsafe { wx_abi(31) },
        std::mem::offset_of!(XDP_HOOK_ID, Layer),
        "XDP_HOOK_ID.Layer"
    );
    assert_eq!(
        unsafe { wx_abi(32) },
        std::mem::offset_of!(XDP_HOOK_ID, Direction),
        "XDP_HOOK_ID.Direction"
    );
    assert_eq!(
        unsafe { wx_abi(33) },
        std::mem::offset_of!(XDP_HOOK_ID, SubLayer),
        "XDP_HOOK_ID.SubLayer"
    );
    assert_eq!(
        unsafe { wx_abi(34) },
        std::mem::size_of::<XDP_HOOK_LAYER>(),
        "XDP_HOOK_LAYER"
    );
    assert_eq!(
        unsafe { wx_abi(35) },
        std::mem::align_of::<XDP_HOOK_LAYER>(),
        "XDP_HOOK_LAYER"
    );
    assert_eq!(
        unsafe { wx_abi(36) },
        std::mem::size_of::<XDP_HOOK_SUBLAYER>(),
        "XDP_HOOK_SUBLAYER"
    );
    assert_eq!(
        unsafe { wx_abi(37) },
        std::mem::align_of::<XDP_HOOK_SUBLAYER>(),
        "XDP_HOOK_SUBLAYER"
    );
    assert_eq!(
        unsafe { wx_abi(38) },
        std::mem::size_of::<XDP_INET_ADDR>(),
        "XDP_INET_ADDR"
    );
    assert_eq!(
        unsafe { wx_abi(39) },
        std::mem::align_of::<XDP_INET_ADDR>(),
        "XDP_INET_ADDR"
    );
    assert_eq!(
        unsafe { wx_abi(40) },
        std::mem::offset_of!(XDP_INET_ADDR, Ipv4),
        "XDP_INET_ADDR.Ipv4"
    );
    assert_eq!(
        unsafe { wx_abi(41) },
        std::mem::offset_of!(XDP_INET_ADDR, Ipv6),
        "XDP_INET_ADDR.Ipv6"
    );
    assert_eq!(
        unsafe { wx_abi(42) },
        std::mem::size_of::<XDP_IP_ADDRESS_MASK>(),
        "XDP_IP_ADDRESS_MASK"
    );
    assert_eq!(
        unsafe { wx_abi(43) },
        std::mem::align_of::<XDP_IP_ADDRESS_MASK>(),
        "XDP_IP_ADDRESS_MASK"
    );
    assert_eq!(
        unsafe { wx_abi(44) },
        std::mem::offset_of!(XDP_IP_ADDRESS_MASK, Mask),
        "XDP_IP_ADDRESS_MASK.Mask"
    );
    assert_eq!(
        unsafe { wx_abi(45) },
        std::mem::offset_of!(XDP_IP_ADDRESS_MASK, Address),
        "XDP_IP_ADDRESS_MASK.Address"
    );
    assert_eq!(
        unsafe { wx_abi(46) },
        std::mem::size_of::<XDP_IP_PORT_SET>(),
        "XDP_IP_PORT_SET"
    );
    assert_eq!(
        unsafe { wx_abi(47) },
        std::mem::align_of::<XDP_IP_PORT_SET>(),
        "XDP_IP_PORT_SET"
    );
    assert_eq!(
        unsafe { wx_abi(48) },
        std::mem::offset_of!(XDP_IP_PORT_SET, Address),
        "XDP_IP_PORT_SET.Address"
    );
    assert_eq!(
        unsafe { wx_abi(49) },
        std::mem::offset_of!(XDP_IP_PORT_SET, PortSet),
        "XDP_IP_PORT_SET.PortSet"
    );
    assert_eq!(
        unsafe { wx_abi(50) },
        std::mem::size_of::<XDP_MAP_TYPE>(),
        "XDP_MAP_TYPE"
    );
    assert_eq!(
        unsafe { wx_abi(51) },
        std::mem::align_of::<XDP_MAP_TYPE>(),
        "XDP_MAP_TYPE"
    );
    assert_eq!(
        unsafe { wx_abi(52) },
        std::mem::size_of::<XDP_MATCH_PATTERN>(),
        "XDP_MATCH_PATTERN"
    );
    assert_eq!(
        unsafe { wx_abi(53) },
        std::mem::align_of::<XDP_MATCH_PATTERN>(),
        "XDP_MATCH_PATTERN"
    );
    assert_eq!(
        unsafe { wx_abi(54) },
        std::mem::offset_of!(XDP_MATCH_PATTERN, Port),
        "XDP_MATCH_PATTERN.Port"
    );
    assert_eq!(
        unsafe { wx_abi(55) },
        std::mem::offset_of!(XDP_MATCH_PATTERN, IpMask),
        "XDP_MATCH_PATTERN.IpMask"
    );
    assert_eq!(
        unsafe { wx_abi(56) },
        std::mem::offset_of!(XDP_MATCH_PATTERN, Tuple),
        "XDP_MATCH_PATTERN.Tuple"
    );
    assert_eq!(
        unsafe { wx_abi(57) },
        std::mem::offset_of!(XDP_MATCH_PATTERN, QuicFlow),
        "XDP_MATCH_PATTERN.QuicFlow"
    );
    assert_eq!(
        unsafe { wx_abi(58) },
        std::mem::offset_of!(XDP_MATCH_PATTERN, PortSet),
        "XDP_MATCH_PATTERN.PortSet"
    );
    assert_eq!(
        unsafe { wx_abi(59) },
        std::mem::offset_of!(XDP_MATCH_PATTERN, IpPortSet),
        "XDP_MATCH_PATTERN.IpPortSet"
    );
    assert_eq!(
        unsafe { wx_abi(60) },
        std::mem::offset_of!(XDP_MATCH_PATTERN, NextHeader),
        "XDP_MATCH_PATTERN.NextHeader"
    );
    assert_eq!(
        unsafe { wx_abi(61) },
        std::mem::size_of::<XDP_MATCH_TYPE>(),
        "XDP_MATCH_TYPE"
    );
    assert_eq!(
        unsafe { wx_abi(62) },
        std::mem::align_of::<XDP_MATCH_TYPE>(),
        "XDP_MATCH_TYPE"
    );
    assert_eq!(
        unsafe { wx_abi(63) },
        std::mem::size_of::<XDP_OBJECT_HEADER>(),
        "XDP_OBJECT_HEADER"
    );
    assert_eq!(
        unsafe { wx_abi(64) },
        std::mem::align_of::<XDP_OBJECT_HEADER>(),
        "XDP_OBJECT_HEADER"
    );
    assert_eq!(
        unsafe { wx_abi(65) },
        std::mem::offset_of!(XDP_OBJECT_HEADER, Revision),
        "XDP_OBJECT_HEADER.Revision"
    );
    assert_eq!(
        unsafe { wx_abi(66) },
        std::mem::offset_of!(XDP_OBJECT_HEADER, Size),
        "XDP_OBJECT_HEADER.Size"
    );
    assert_eq!(
        unsafe { wx_abi(67) },
        std::mem::size_of::<XDP_OVERLAPPED>(),
        "XDP_OVERLAPPED"
    );
    assert_eq!(
        unsafe { wx_abi(68) },
        std::mem::align_of::<XDP_OVERLAPPED>(),
        "XDP_OVERLAPPED"
    );
    assert_eq!(
        unsafe { wx_abi(69) },
        std::mem::size_of::<XDP_PORT_SET>(),
        "XDP_PORT_SET"
    );
    assert_eq!(
        unsafe { wx_abi(70) },
        std::mem::align_of::<XDP_PORT_SET>(),
        "XDP_PORT_SET"
    );
    assert_eq!(
        unsafe { wx_abi(71) },
        std::mem::offset_of!(XDP_PORT_SET, PortSet),
        "XDP_PORT_SET.PortSet"
    );
    assert_eq!(
        unsafe { wx_abi(72) },
        std::mem::offset_of!(XDP_PORT_SET, Reserved),
        "XDP_PORT_SET.Reserved"
    );
    assert_eq!(
        unsafe { wx_abi(73) },
        std::mem::size_of::<XDP_QUIC_ADDRESS_FAMILY>(),
        "XDP_QUIC_ADDRESS_FAMILY"
    );
    assert_eq!(
        unsafe { wx_abi(74) },
        std::mem::align_of::<XDP_QUIC_ADDRESS_FAMILY>(),
        "XDP_QUIC_ADDRESS_FAMILY"
    );
    assert_eq!(
        unsafe { wx_abi(75) },
        std::mem::size_of::<XDP_QUIC_CIPHER_TYPE>(),
        "XDP_QUIC_CIPHER_TYPE"
    );
    assert_eq!(
        unsafe { wx_abi(76) },
        std::mem::align_of::<XDP_QUIC_CIPHER_TYPE>(),
        "XDP_QUIC_CIPHER_TYPE"
    );
    assert_eq!(
        unsafe { wx_abi(77) },
        std::mem::size_of::<XDP_QUIC_CONNECTION>(),
        "XDP_QUIC_CONNECTION"
    );
    assert_eq!(
        unsafe { wx_abi(78) },
        std::mem::align_of::<XDP_QUIC_CONNECTION>(),
        "XDP_QUIC_CONNECTION"
    );
    assert_eq!(
        unsafe { wx_abi(79) },
        std::mem::offset_of!(XDP_QUIC_CONNECTION, Header),
        "XDP_QUIC_CONNECTION.Header"
    );
    assert_eq!(
        unsafe { wx_abi(80) },
        std::mem::offset_of!(XDP_QUIC_CONNECTION, AddressFamily),
        "XDP_QUIC_CONNECTION.AddressFamily"
    );
    assert_eq!(
        unsafe { wx_abi(81) },
        std::mem::offset_of!(XDP_QUIC_CONNECTION, UdpPort),
        "XDP_QUIC_CONNECTION.UdpPort"
    );
    assert_eq!(
        unsafe { wx_abi(82) },
        std::mem::offset_of!(XDP_QUIC_CONNECTION, NextPacketNumber),
        "XDP_QUIC_CONNECTION.NextPacketNumber"
    );
    assert_eq!(
        unsafe { wx_abi(83) },
        std::mem::offset_of!(XDP_QUIC_CONNECTION, ConnectionIdLength),
        "XDP_QUIC_CONNECTION.ConnectionIdLength"
    );
    assert_eq!(
        unsafe { wx_abi(84) },
        std::mem::offset_of!(XDP_QUIC_CONNECTION, Address),
        "XDP_QUIC_CONNECTION.Address"
    );
    assert_eq!(
        unsafe { wx_abi(85) },
        std::mem::offset_of!(XDP_QUIC_CONNECTION, ConnectionId),
        "XDP_QUIC_CONNECTION.ConnectionId"
    );
    assert_eq!(
        unsafe { wx_abi(86) },
        std::mem::offset_of!(XDP_QUIC_CONNECTION, PayloadKey),
        "XDP_QUIC_CONNECTION.PayloadKey"
    );
    assert_eq!(
        unsafe { wx_abi(87) },
        std::mem::offset_of!(XDP_QUIC_CONNECTION, HeaderKey),
        "XDP_QUIC_CONNECTION.HeaderKey"
    );
    assert_eq!(
        unsafe { wx_abi(88) },
        std::mem::offset_of!(XDP_QUIC_CONNECTION, PayloadIv),
        "XDP_QUIC_CONNECTION.PayloadIv"
    );
    assert_eq!(
        unsafe { wx_abi(89) },
        std::mem::offset_of!(XDP_QUIC_CONNECTION, Status),
        "XDP_QUIC_CONNECTION.Status"
    );
    assert_eq!(
        unsafe { wx_abi(90) },
        std::mem::size_of::<XDP_QUIC_DECRYPT_FAILURE_ACTION>(),
        "XDP_QUIC_DECRYPT_FAILURE_ACTION"
    );
    assert_eq!(
        unsafe { wx_abi(91) },
        std::mem::align_of::<XDP_QUIC_DECRYPT_FAILURE_ACTION>(),
        "XDP_QUIC_DECRYPT_FAILURE_ACTION"
    );
    assert_eq!(
        unsafe { wx_abi(92) },
        std::mem::size_of::<XDP_QUIC_DIRECTION>(),
        "XDP_QUIC_DIRECTION"
    );
    assert_eq!(
        unsafe { wx_abi(93) },
        std::mem::align_of::<XDP_QUIC_DIRECTION>(),
        "XDP_QUIC_DIRECTION"
    );
    assert_eq!(
        unsafe { wx_abi(94) },
        std::mem::size_of::<XDP_QUIC_FLOW>(),
        "XDP_QUIC_FLOW"
    );
    assert_eq!(
        unsafe { wx_abi(95) },
        std::mem::align_of::<XDP_QUIC_FLOW>(),
        "XDP_QUIC_FLOW"
    );
    assert_eq!(
        unsafe { wx_abi(96) },
        std::mem::offset_of!(XDP_QUIC_FLOW, UdpPort),
        "XDP_QUIC_FLOW.UdpPort"
    );
    assert_eq!(
        unsafe { wx_abi(97) },
        std::mem::offset_of!(XDP_QUIC_FLOW, CidLength),
        "XDP_QUIC_FLOW.CidLength"
    );
    assert_eq!(
        unsafe { wx_abi(98) },
        std::mem::offset_of!(XDP_QUIC_FLOW, CidOffset),
        "XDP_QUIC_FLOW.CidOffset"
    );
    assert_eq!(
        unsafe { wx_abi(99) },
        std::mem::offset_of!(XDP_QUIC_FLOW, CidData),
        "XDP_QUIC_FLOW.CidData"
    );
    assert_eq!(
        unsafe { wx_abi(100) },
        std::mem::size_of::<XDP_QUIC_OPERATION>(),
        "XDP_QUIC_OPERATION"
    );
    assert_eq!(
        unsafe { wx_abi(101) },
        std::mem::align_of::<XDP_QUIC_OPERATION>(),
        "XDP_QUIC_OPERATION"
    );
    assert_eq!(
        unsafe { wx_abi(102) },
        std::mem::size_of::<XDP_REDIRECT_PARAMS>(),
        "XDP_REDIRECT_PARAMS"
    );
    assert_eq!(
        unsafe { wx_abi(103) },
        std::mem::align_of::<XDP_REDIRECT_PARAMS>(),
        "XDP_REDIRECT_PARAMS"
    );
    assert_eq!(
        unsafe { wx_abi(104) },
        std::mem::offset_of!(XDP_REDIRECT_PARAMS, TargetType),
        "XDP_REDIRECT_PARAMS.TargetType"
    );
    assert_eq!(
        unsafe { wx_abi(105) },
        std::mem::offset_of!(XDP_REDIRECT_PARAMS, Target),
        "XDP_REDIRECT_PARAMS.Target"
    );
    assert_eq!(
        unsafe { wx_abi(106) },
        std::mem::size_of::<XDP_REDIRECT_TARGET_TYPE>(),
        "XDP_REDIRECT_TARGET_TYPE"
    );
    assert_eq!(
        unsafe { wx_abi(107) },
        std::mem::align_of::<XDP_REDIRECT_TARGET_TYPE>(),
        "XDP_REDIRECT_TARGET_TYPE"
    );
    assert_eq!(
        unsafe { wx_abi(108) },
        std::mem::size_of::<XDP_RSS_CAPABILITIES>(),
        "XDP_RSS_CAPABILITIES"
    );
    assert_eq!(
        unsafe { wx_abi(109) },
        std::mem::align_of::<XDP_RSS_CAPABILITIES>(),
        "XDP_RSS_CAPABILITIES"
    );
    assert_eq!(
        unsafe { wx_abi(110) },
        std::mem::offset_of!(XDP_RSS_CAPABILITIES, Header),
        "XDP_RSS_CAPABILITIES.Header"
    );
    assert_eq!(
        unsafe { wx_abi(111) },
        std::mem::offset_of!(XDP_RSS_CAPABILITIES, Flags),
        "XDP_RSS_CAPABILITIES.Flags"
    );
    assert_eq!(
        unsafe { wx_abi(112) },
        std::mem::offset_of!(XDP_RSS_CAPABILITIES, HashTypes),
        "XDP_RSS_CAPABILITIES.HashTypes"
    );
    assert_eq!(
        unsafe { wx_abi(113) },
        std::mem::offset_of!(XDP_RSS_CAPABILITIES, HashSecretKeySize),
        "XDP_RSS_CAPABILITIES.HashSecretKeySize"
    );
    assert_eq!(
        unsafe { wx_abi(114) },
        std::mem::offset_of!(XDP_RSS_CAPABILITIES, NumberOfReceiveQueues),
        "XDP_RSS_CAPABILITIES.NumberOfReceiveQueues"
    );
    assert_eq!(
        unsafe { wx_abi(115) },
        std::mem::offset_of!(XDP_RSS_CAPABILITIES, NumberOfIndirectionTableEntries),
        "XDP_RSS_CAPABILITIES.NumberOfIndirectionTableEntries"
    );
    assert_eq!(
        unsafe { wx_abi(116) },
        std::mem::size_of::<XDP_RSS_CONFIGURATION>(),
        "XDP_RSS_CONFIGURATION"
    );
    assert_eq!(
        unsafe { wx_abi(117) },
        std::mem::align_of::<XDP_RSS_CONFIGURATION>(),
        "XDP_RSS_CONFIGURATION"
    );
    assert_eq!(
        unsafe { wx_abi(118) },
        std::mem::offset_of!(XDP_RSS_CONFIGURATION, Header),
        "XDP_RSS_CONFIGURATION.Header"
    );
    assert_eq!(
        unsafe { wx_abi(119) },
        std::mem::offset_of!(XDP_RSS_CONFIGURATION, Flags),
        "XDP_RSS_CONFIGURATION.Flags"
    );
    assert_eq!(
        unsafe { wx_abi(120) },
        std::mem::offset_of!(XDP_RSS_CONFIGURATION, HashType),
        "XDP_RSS_CONFIGURATION.HashType"
    );
    assert_eq!(
        unsafe { wx_abi(121) },
        std::mem::offset_of!(XDP_RSS_CONFIGURATION, HashSecretKeyOffset),
        "XDP_RSS_CONFIGURATION.HashSecretKeyOffset"
    );
    assert_eq!(
        unsafe { wx_abi(122) },
        std::mem::offset_of!(XDP_RSS_CONFIGURATION, HashSecretKeySize),
        "XDP_RSS_CONFIGURATION.HashSecretKeySize"
    );
    assert_eq!(
        unsafe { wx_abi(123) },
        std::mem::offset_of!(XDP_RSS_CONFIGURATION, IndirectionTableOffset),
        "XDP_RSS_CONFIGURATION.IndirectionTableOffset"
    );
    assert_eq!(
        unsafe { wx_abi(124) },
        std::mem::offset_of!(XDP_RSS_CONFIGURATION, IndirectionTableSize),
        "XDP_RSS_CONFIGURATION.IndirectionTableSize"
    );
    assert_eq!(
        unsafe { wx_abi(125) },
        std::mem::size_of::<XDP_RULE>(),
        "XDP_RULE"
    );
    assert_eq!(
        unsafe { wx_abi(126) },
        std::mem::align_of::<XDP_RULE>(),
        "XDP_RULE"
    );
    assert_eq!(
        unsafe { wx_abi(127) },
        std::mem::offset_of!(XDP_RULE, Match),
        "XDP_RULE.Match"
    );
    assert_eq!(
        unsafe { wx_abi(128) },
        std::mem::offset_of!(XDP_RULE, Pattern),
        "XDP_RULE.Pattern"
    );
    assert_eq!(
        unsafe { wx_abi(129) },
        std::mem::offset_of!(XDP_RULE, Action),
        "XDP_RULE.Action"
    );
    assert_eq!(
        unsafe { wx_abi(130) },
        std::mem::size_of::<XDP_RULE_ACTION>(),
        "XDP_RULE_ACTION"
    );
    assert_eq!(
        unsafe { wx_abi(131) },
        std::mem::align_of::<XDP_RULE_ACTION>(),
        "XDP_RULE_ACTION"
    );
    assert_eq!(
        unsafe { wx_abi(132) },
        std::mem::size_of::<XDP_TUPLE>(),
        "XDP_TUPLE"
    );
    assert_eq!(
        unsafe { wx_abi(133) },
        std::mem::align_of::<XDP_TUPLE>(),
        "XDP_TUPLE"
    );
    assert_eq!(
        unsafe { wx_abi(134) },
        std::mem::offset_of!(XDP_TUPLE, SourceAddress),
        "XDP_TUPLE.SourceAddress"
    );
    assert_eq!(
        unsafe { wx_abi(135) },
        std::mem::offset_of!(XDP_TUPLE, DestinationAddress),
        "XDP_TUPLE.DestinationAddress"
    );
    assert_eq!(
        unsafe { wx_abi(136) },
        std::mem::offset_of!(XDP_TUPLE, SourcePort),
        "XDP_TUPLE.SourcePort"
    );
    assert_eq!(
        unsafe { wx_abi(137) },
        std::mem::offset_of!(XDP_TUPLE, DestinationPort),
        "XDP_TUPLE.DestinationPort"
    );
    assert_eq!(
        unsafe { wx_abi(138) },
        std::mem::size_of::<XDP_VERSION>(),
        "XDP_VERSION"
    );
    assert_eq!(
        unsafe { wx_abi(139) },
        std::mem::align_of::<XDP_VERSION>(),
        "XDP_VERSION"
    );
    assert_eq!(
        unsafe { wx_abi(140) },
        std::mem::offset_of!(XDP_VERSION, Major),
        "XDP_VERSION.Major"
    );
    assert_eq!(
        unsafe { wx_abi(141) },
        std::mem::offset_of!(XDP_VERSION, Minor),
        "XDP_VERSION.Minor"
    );
    assert_eq!(
        unsafe { wx_abi(142) },
        std::mem::offset_of!(XDP_VERSION, Patch),
        "XDP_VERSION.Patch"
    );
    assert_eq!(
        unsafe { wx_abi(143) },
        std::mem::size_of::<XSK_ACTIVATE_FLAGS>(),
        "XSK_ACTIVATE_FLAGS"
    );
    assert_eq!(
        unsafe { wx_abi(144) },
        std::mem::align_of::<XSK_ACTIVATE_FLAGS>(),
        "XSK_ACTIVATE_FLAGS"
    );
    assert_eq!(
        unsafe { wx_abi(145) },
        std::mem::size_of::<XSK_BIND_FLAGS>(),
        "XSK_BIND_FLAGS"
    );
    assert_eq!(
        unsafe { wx_abi(146) },
        std::mem::align_of::<XSK_BIND_FLAGS>(),
        "XSK_BIND_FLAGS"
    );
    assert_eq!(
        unsafe { wx_abi(147) },
        std::mem::size_of::<XSK_BUFFER_ADDRESS>(),
        "XSK_BUFFER_ADDRESS"
    );
    assert_eq!(
        unsafe { wx_abi(148) },
        std::mem::align_of::<XSK_BUFFER_ADDRESS>(),
        "XSK_BUFFER_ADDRESS"
    );
    assert_eq!(
        unsafe { wx_abi(149) },
        std::mem::offset_of!(XSK_BUFFER_ADDRESS, AddressAndOffset),
        "XSK_BUFFER_ADDRESS.AddressAndOffset"
    );
    assert_eq!(
        unsafe { wx_abi(150) },
        std::mem::size_of::<XSK_BUFFER_DESCRIPTOR>(),
        "XSK_BUFFER_DESCRIPTOR"
    );
    assert_eq!(
        unsafe { wx_abi(151) },
        std::mem::align_of::<XSK_BUFFER_DESCRIPTOR>(),
        "XSK_BUFFER_DESCRIPTOR"
    );
    assert_eq!(
        unsafe { wx_abi(152) },
        std::mem::offset_of!(XSK_BUFFER_DESCRIPTOR, Address),
        "XSK_BUFFER_DESCRIPTOR.Address"
    );
    assert_eq!(
        unsafe { wx_abi(153) },
        std::mem::offset_of!(XSK_BUFFER_DESCRIPTOR, Length),
        "XSK_BUFFER_DESCRIPTOR.Length"
    );
    assert_eq!(
        unsafe { wx_abi(154) },
        std::mem::offset_of!(XSK_BUFFER_DESCRIPTOR, Reserved),
        "XSK_BUFFER_DESCRIPTOR.Reserved"
    );
    assert_eq!(
        unsafe { wx_abi(155) },
        std::mem::size_of::<XSK_ERROR>(),
        "XSK_ERROR"
    );
    assert_eq!(
        unsafe { wx_abi(156) },
        std::mem::align_of::<XSK_ERROR>(),
        "XSK_ERROR"
    );
    assert_eq!(
        unsafe { wx_abi(157) },
        std::mem::size_of::<XSK_FRAME_DESCRIPTOR>(),
        "XSK_FRAME_DESCRIPTOR"
    );
    assert_eq!(
        unsafe { wx_abi(158) },
        std::mem::align_of::<XSK_FRAME_DESCRIPTOR>(),
        "XSK_FRAME_DESCRIPTOR"
    );
    assert_eq!(
        unsafe { wx_abi(159) },
        std::mem::offset_of!(XSK_FRAME_DESCRIPTOR, Buffer),
        "XSK_FRAME_DESCRIPTOR.Buffer"
    );
    assert_eq!(
        unsafe { wx_abi(160) },
        std::mem::size_of::<XSK_FRAME_ORIGINAL_LENGTH>(),
        "XSK_FRAME_ORIGINAL_LENGTH"
    );
    assert_eq!(
        unsafe { wx_abi(161) },
        std::mem::align_of::<XSK_FRAME_ORIGINAL_LENGTH>(),
        "XSK_FRAME_ORIGINAL_LENGTH"
    );
    assert_eq!(
        unsafe { wx_abi(162) },
        std::mem::offset_of!(XSK_FRAME_ORIGINAL_LENGTH, OriginalLength),
        "XSK_FRAME_ORIGINAL_LENGTH.OriginalLength"
    );
    assert_eq!(
        unsafe { wx_abi(163) },
        std::mem::size_of::<XSK_NOTIFY_FLAGS>(),
        "XSK_NOTIFY_FLAGS"
    );
    assert_eq!(
        unsafe { wx_abi(164) },
        std::mem::align_of::<XSK_NOTIFY_FLAGS>(),
        "XSK_NOTIFY_FLAGS"
    );
    assert_eq!(
        unsafe { wx_abi(165) },
        std::mem::size_of::<XSK_NOTIFY_RESULT_FLAGS>(),
        "XSK_NOTIFY_RESULT_FLAGS"
    );
    assert_eq!(
        unsafe { wx_abi(166) },
        std::mem::align_of::<XSK_NOTIFY_RESULT_FLAGS>(),
        "XSK_NOTIFY_RESULT_FLAGS"
    );
    assert_eq!(
        unsafe { wx_abi(167) },
        std::mem::size_of::<XSK_POLL_MODE>(),
        "XSK_POLL_MODE"
    );
    assert_eq!(
        unsafe { wx_abi(168) },
        std::mem::align_of::<XSK_POLL_MODE>(),
        "XSK_POLL_MODE"
    );
    assert_eq!(
        unsafe { wx_abi(169) },
        std::mem::size_of::<XSK_RING>(),
        "XSK_RING"
    );
    assert_eq!(
        unsafe { wx_abi(170) },
        std::mem::align_of::<XSK_RING>(),
        "XSK_RING"
    );
    assert_eq!(
        unsafe { wx_abi(171) },
        std::mem::offset_of!(XSK_RING, SharedProducer),
        "XSK_RING.SharedProducer"
    );
    assert_eq!(
        unsafe { wx_abi(172) },
        std::mem::offset_of!(XSK_RING, SharedConsumer),
        "XSK_RING.SharedConsumer"
    );
    assert_eq!(
        unsafe { wx_abi(173) },
        std::mem::offset_of!(XSK_RING, SharedFlags),
        "XSK_RING.SharedFlags"
    );
    assert_eq!(
        unsafe { wx_abi(174) },
        std::mem::offset_of!(XSK_RING, SharedElements),
        "XSK_RING.SharedElements"
    );
    assert_eq!(
        unsafe { wx_abi(175) },
        std::mem::offset_of!(XSK_RING, Mask),
        "XSK_RING.Mask"
    );
    assert_eq!(
        unsafe { wx_abi(176) },
        std::mem::offset_of!(XSK_RING, Size),
        "XSK_RING.Size"
    );
    assert_eq!(
        unsafe { wx_abi(177) },
        std::mem::offset_of!(XSK_RING, ElementStride),
        "XSK_RING.ElementStride"
    );
    assert_eq!(
        unsafe { wx_abi(178) },
        std::mem::offset_of!(XSK_RING, CachedProducer),
        "XSK_RING.CachedProducer"
    );
    assert_eq!(
        unsafe { wx_abi(179) },
        std::mem::offset_of!(XSK_RING, CachedConsumer),
        "XSK_RING.CachedConsumer"
    );
    assert_eq!(
        unsafe { wx_abi(180) },
        std::mem::size_of::<XSK_RING_FLAGS>(),
        "XSK_RING_FLAGS"
    );
    assert_eq!(
        unsafe { wx_abi(181) },
        std::mem::align_of::<XSK_RING_FLAGS>(),
        "XSK_RING_FLAGS"
    );
    assert_eq!(
        unsafe { wx_abi(182) },
        std::mem::size_of::<XSK_RING_INFO>(),
        "XSK_RING_INFO"
    );
    assert_eq!(
        unsafe { wx_abi(183) },
        std::mem::align_of::<XSK_RING_INFO>(),
        "XSK_RING_INFO"
    );
    assert_eq!(
        unsafe { wx_abi(184) },
        std::mem::offset_of!(XSK_RING_INFO, Ring),
        "XSK_RING_INFO.Ring"
    );
    assert_eq!(
        unsafe { wx_abi(185) },
        std::mem::offset_of!(XSK_RING_INFO, DescriptorsOffset),
        "XSK_RING_INFO.DescriptorsOffset"
    );
    assert_eq!(
        unsafe { wx_abi(186) },
        std::mem::offset_of!(XSK_RING_INFO, ProducerIndexOffset),
        "XSK_RING_INFO.ProducerIndexOffset"
    );
    assert_eq!(
        unsafe { wx_abi(187) },
        std::mem::offset_of!(XSK_RING_INFO, ConsumerIndexOffset),
        "XSK_RING_INFO.ConsumerIndexOffset"
    );
    assert_eq!(
        unsafe { wx_abi(188) },
        std::mem::offset_of!(XSK_RING_INFO, FlagsOffset),
        "XSK_RING_INFO.FlagsOffset"
    );
    assert_eq!(
        unsafe { wx_abi(189) },
        std::mem::offset_of!(XSK_RING_INFO, Size),
        "XSK_RING_INFO.Size"
    );
    assert_eq!(
        unsafe { wx_abi(190) },
        std::mem::offset_of!(XSK_RING_INFO, ElementStride),
        "XSK_RING_INFO.ElementStride"
    );
    assert_eq!(
        unsafe { wx_abi(191) },
        std::mem::offset_of!(XSK_RING_INFO, Reserved),
        "XSK_RING_INFO.Reserved"
    );
    assert_eq!(
        unsafe { wx_abi(192) },
        std::mem::size_of::<XSK_RING_INFO_SET>(),
        "XSK_RING_INFO_SET"
    );
    assert_eq!(
        unsafe { wx_abi(193) },
        std::mem::align_of::<XSK_RING_INFO_SET>(),
        "XSK_RING_INFO_SET"
    );
    assert_eq!(
        unsafe { wx_abi(194) },
        std::mem::offset_of!(XSK_RING_INFO_SET, Fill),
        "XSK_RING_INFO_SET.Fill"
    );
    assert_eq!(
        unsafe { wx_abi(195) },
        std::mem::offset_of!(XSK_RING_INFO_SET, Completion),
        "XSK_RING_INFO_SET.Completion"
    );
    assert_eq!(
        unsafe { wx_abi(196) },
        std::mem::offset_of!(XSK_RING_INFO_SET, Rx),
        "XSK_RING_INFO_SET.Rx"
    );
    assert_eq!(
        unsafe { wx_abi(197) },
        std::mem::offset_of!(XSK_RING_INFO_SET, Tx),
        "XSK_RING_INFO_SET.Tx"
    );
    assert_eq!(
        unsafe { wx_abi(198) },
        std::mem::size_of::<XSK_STATISTICS>(),
        "XSK_STATISTICS"
    );
    assert_eq!(
        unsafe { wx_abi(199) },
        std::mem::align_of::<XSK_STATISTICS>(),
        "XSK_STATISTICS"
    );
    assert_eq!(
        unsafe { wx_abi(200) },
        std::mem::offset_of!(XSK_STATISTICS, RxDropped),
        "XSK_STATISTICS.RxDropped"
    );
    assert_eq!(
        unsafe { wx_abi(201) },
        std::mem::offset_of!(XSK_STATISTICS, RxTruncated),
        "XSK_STATISTICS.RxTruncated"
    );
    assert_eq!(
        unsafe { wx_abi(202) },
        std::mem::offset_of!(XSK_STATISTICS, RxInvalidDescriptors),
        "XSK_STATISTICS.RxInvalidDescriptors"
    );
    assert_eq!(
        unsafe { wx_abi(203) },
        std::mem::offset_of!(XSK_STATISTICS, TxInvalidDescriptors),
        "XSK_STATISTICS.TxInvalidDescriptors"
    );
    assert_eq!(
        unsafe { wx_abi(204) },
        std::mem::size_of::<XSK_UMEM_REG>(),
        "XSK_UMEM_REG"
    );
    assert_eq!(
        unsafe { wx_abi(205) },
        std::mem::align_of::<XSK_UMEM_REG>(),
        "XSK_UMEM_REG"
    );
    assert_eq!(
        unsafe { wx_abi(206) },
        std::mem::offset_of!(XSK_UMEM_REG, TotalSize),
        "XSK_UMEM_REG.TotalSize"
    );
    assert_eq!(
        unsafe { wx_abi(207) },
        std::mem::offset_of!(XSK_UMEM_REG, ChunkSize),
        "XSK_UMEM_REG.ChunkSize"
    );
    assert_eq!(
        unsafe { wx_abi(208) },
        std::mem::offset_of!(XSK_UMEM_REG, Headroom),
        "XSK_UMEM_REG.Headroom"
    );
    assert_eq!(
        unsafe { wx_abi(209) },
        std::mem::offset_of!(XSK_UMEM_REG, Address),
        "XSK_UMEM_REG.Address"
    );
    assert_eq!(
        unsafe { wx_abi(210) },
        std::mem::size_of::<xdp_md_t>(),
        "xdp_md_t"
    );
    assert_eq!(
        unsafe { wx_abi(211) },
        std::mem::align_of::<xdp_md_t>(),
        "xdp_md_t"
    );
    assert_eq!(
        unsafe { wx_abi(212) },
        std::mem::offset_of!(xdp_md_t, data),
        "xdp_md_t.data"
    );
    assert_eq!(
        unsafe { wx_abi(213) },
        std::mem::offset_of!(xdp_md_t, data_end),
        "xdp_md_t.data_end"
    );
    assert_eq!(
        unsafe { wx_abi(214) },
        std::mem::offset_of!(xdp_md_t, data_meta),
        "xdp_md_t.data_meta"
    );
    assert_eq!(
        unsafe { wx_abi(215) },
        std::mem::offset_of!(xdp_md_t, ingress_ifindex),
        "xdp_md_t.ingress_ifindex"
    );
    assert_eq!(
        unsafe { wx_abi(216) },
        std::mem::offset_of!(xdp_md_t, rx_queue_index),
        "xdp_md_t.rx_queue_index"
    );
    assert_eq!(
        unsafe { wx_abi(217) },
        std::mem::size_of::<xdp_action_t>(),
        "xdp_action_t"
    );
    assert_eq!(
        unsafe { wx_abi(218) },
        std::mem::align_of::<xdp_action_t>(),
        "xdp_action_t"
    );
    assert_eq!(
        unsafe { wx_abi(219) },
        XDP_API_VERSION_1 as u32 as usize,
        "XDP_API_VERSION_1"
    );
    assert_eq!(
        unsafe { wx_abi(220) },
        XDP_API_VERSION_2 as u32 as usize,
        "XDP_API_VERSION_2"
    );
    assert_eq!(
        unsafe { wx_abi(221) },
        XDP_API_VERSION_3 as u32 as usize,
        "XDP_API_VERSION_3"
    );
    assert_eq!(
        unsafe { wx_abi(222) },
        XDP_API_VERSION_LATEST as u32 as usize,
        "XDP_API_VERSION_LATEST"
    );
    assert_eq!(
        unsafe { wx_abi(223) },
        XDP_CHECKSUM_CONFIGURATION_REVISION_1 as u32 as usize,
        "XDP_CHECKSUM_CONFIGURATION_REVISION_1"
    );
    assert_eq!(
        unsafe { wx_abi(224) },
        XDP_CREATE_PROGRAM_FLAG_ALL_QUEUES as u32 as usize,
        "XDP_CREATE_PROGRAM_FLAG_ALL_QUEUES"
    );
    assert_eq!(
        unsafe { wx_abi(225) },
        XDP_CREATE_PROGRAM_FLAG_GENERIC as u32 as usize,
        "XDP_CREATE_PROGRAM_FLAG_GENERIC"
    );
    assert_eq!(
        unsafe { wx_abi(226) },
        XDP_CREATE_PROGRAM_FLAG_NATIVE as u32 as usize,
        "XDP_CREATE_PROGRAM_FLAG_NATIVE"
    );
    assert_eq!(
        unsafe { wx_abi(227) },
        XDP_CREATE_PROGRAM_FLAG_NONE as u32 as usize,
        "XDP_CREATE_PROGRAM_FLAG_NONE"
    );
    assert_eq!(
        unsafe { wx_abi(228) },
        XDP_HOOK_INJECT as u32 as usize,
        "XDP_HOOK_INJECT"
    );
    assert_eq!(
        unsafe { wx_abi(229) },
        XDP_HOOK_INSPECT as u32 as usize,
        "XDP_HOOK_INSPECT"
    );
    assert_eq!(
        unsafe { wx_abi(230) },
        XDP_HOOK_L2 as u32 as usize,
        "XDP_HOOK_L2"
    );
    assert_eq!(
        unsafe { wx_abi(231) },
        XDP_HOOK_RX as u32 as usize,
        "XDP_HOOK_RX"
    );
    assert_eq!(
        unsafe { wx_abi(232) },
        XDP_HOOK_TX as u32 as usize,
        "XDP_HOOK_TX"
    );
    assert_eq!(
        unsafe { wx_abi(233) },
        XDP_MAP_TYPE_XSKMAP as u32 as usize,
        "XDP_MAP_TYPE_XSKMAP"
    );
    assert_eq!(
        unsafe { wx_abi(234) },
        XDP_MATCH_ALL as u32 as usize,
        "XDP_MATCH_ALL"
    );
    assert_eq!(
        unsafe { wx_abi(235) },
        XDP_MATCH_ICMPV4_ECHO_REPLY_IP_DST as u32 as usize,
        "XDP_MATCH_ICMPV4_ECHO_REPLY_IP_DST"
    );
    assert_eq!(
        unsafe { wx_abi(236) },
        XDP_MATCH_ICMPV6_ECHO_REPLY_IP_DST as u32 as usize,
        "XDP_MATCH_ICMPV6_ECHO_REPLY_IP_DST"
    );
    assert_eq!(
        unsafe { wx_abi(237) },
        XDP_MATCH_INNER_IPV4_DST_MASK_UDP as u32 as usize,
        "XDP_MATCH_INNER_IPV4_DST_MASK_UDP"
    );
    assert_eq!(
        unsafe { wx_abi(238) },
        XDP_MATCH_INNER_IPV6_DST_MASK_UDP as u32 as usize,
        "XDP_MATCH_INNER_IPV6_DST_MASK_UDP"
    );
    assert_eq!(
        unsafe { wx_abi(239) },
        XDP_MATCH_IPV4_DST_MASK as u32 as usize,
        "XDP_MATCH_IPV4_DST_MASK"
    );
    assert_eq!(
        unsafe { wx_abi(240) },
        XDP_MATCH_IPV4_TCP_PORT_SET as u32 as usize,
        "XDP_MATCH_IPV4_TCP_PORT_SET"
    );
    assert_eq!(
        unsafe { wx_abi(241) },
        XDP_MATCH_IPV4_UDP_PORT_SET as u32 as usize,
        "XDP_MATCH_IPV4_UDP_PORT_SET"
    );
    assert_eq!(
        unsafe { wx_abi(242) },
        XDP_MATCH_IPV4_UDP_TUPLE as u32 as usize,
        "XDP_MATCH_IPV4_UDP_TUPLE"
    );
    assert_eq!(
        unsafe { wx_abi(243) },
        XDP_MATCH_IPV6_DST_MASK as u32 as usize,
        "XDP_MATCH_IPV6_DST_MASK"
    );
    assert_eq!(
        unsafe { wx_abi(244) },
        XDP_MATCH_IPV6_TCP_PORT_SET as u32 as usize,
        "XDP_MATCH_IPV6_TCP_PORT_SET"
    );
    assert_eq!(
        unsafe { wx_abi(245) },
        XDP_MATCH_IPV6_UDP_PORT_SET as u32 as usize,
        "XDP_MATCH_IPV6_UDP_PORT_SET"
    );
    assert_eq!(
        unsafe { wx_abi(246) },
        XDP_MATCH_IPV6_UDP_TUPLE as u32 as usize,
        "XDP_MATCH_IPV6_UDP_TUPLE"
    );
    assert_eq!(
        unsafe { wx_abi(247) },
        XDP_MATCH_IP_NEXT_HEADER as u32 as usize,
        "XDP_MATCH_IP_NEXT_HEADER"
    );
    assert_eq!(
        unsafe { wx_abi(248) },
        XDP_MATCH_QUIC_FLOW_DST_CID as u32 as usize,
        "XDP_MATCH_QUIC_FLOW_DST_CID"
    );
    assert_eq!(
        unsafe { wx_abi(249) },
        XDP_MATCH_QUIC_FLOW_SRC_CID as u32 as usize,
        "XDP_MATCH_QUIC_FLOW_SRC_CID"
    );
    assert_eq!(
        unsafe { wx_abi(250) },
        XDP_MATCH_TCP_CONTROL_DST as u32 as usize,
        "XDP_MATCH_TCP_CONTROL_DST"
    );
    assert_eq!(
        unsafe { wx_abi(251) },
        XDP_MATCH_TCP_DST as u32 as usize,
        "XDP_MATCH_TCP_DST"
    );
    assert_eq!(
        unsafe { wx_abi(252) },
        XDP_MATCH_TCP_QUIC_FLOW_DST_CID as u32 as usize,
        "XDP_MATCH_TCP_QUIC_FLOW_DST_CID"
    );
    assert_eq!(
        unsafe { wx_abi(253) },
        XDP_MATCH_TCP_QUIC_FLOW_SRC_CID as u32 as usize,
        "XDP_MATCH_TCP_QUIC_FLOW_SRC_CID"
    );
    assert_eq!(
        unsafe { wx_abi(254) },
        XDP_MATCH_UDP as u32 as usize,
        "XDP_MATCH_UDP"
    );
    assert_eq!(
        unsafe { wx_abi(255) },
        XDP_MATCH_UDP_DST as u32 as usize,
        "XDP_MATCH_UDP_DST"
    );
    assert_eq!(
        unsafe { wx_abi(256) },
        XDP_MATCH_UDP_PORT_SET as u32 as usize,
        "XDP_MATCH_UDP_PORT_SET"
    );
    assert_eq!(
        unsafe { wx_abi(257) },
        XDP_PORT_SET_BUFFER_SIZE as u32 as usize,
        "XDP_PORT_SET_BUFFER_SIZE"
    );
    assert_eq!(
        unsafe { wx_abi(258) },
        XDP_PROGRAM_ACTION_DROP as u32 as usize,
        "XDP_PROGRAM_ACTION_DROP"
    );
    assert_eq!(
        unsafe { wx_abi(259) },
        XDP_PROGRAM_ACTION_EBPF as u32 as usize,
        "XDP_PROGRAM_ACTION_EBPF"
    );
    assert_eq!(
        unsafe { wx_abi(260) },
        XDP_PROGRAM_ACTION_L2FWD as u32 as usize,
        "XDP_PROGRAM_ACTION_L2FWD"
    );
    assert_eq!(
        unsafe { wx_abi(261) },
        XDP_PROGRAM_ACTION_PASS as u32 as usize,
        "XDP_PROGRAM_ACTION_PASS"
    );
    assert_eq!(
        unsafe { wx_abi(262) },
        XDP_PROGRAM_ACTION_REDIRECT as u32 as usize,
        "XDP_PROGRAM_ACTION_REDIRECT"
    );
    assert_eq!(
        unsafe { wx_abi(263) },
        XDP_QUIC_ADDRESS_FAMILY_INET4 as u32 as usize,
        "XDP_QUIC_ADDRESS_FAMILY_INET4"
    );
    assert_eq!(
        unsafe { wx_abi(264) },
        XDP_QUIC_ADDRESS_FAMILY_INET6 as u32 as usize,
        "XDP_QUIC_ADDRESS_FAMILY_INET6"
    );
    assert_eq!(
        unsafe { wx_abi(265) },
        XDP_QUIC_CIPHER_TYPE_AEAD_AES_128_CCM as u32 as usize,
        "XDP_QUIC_CIPHER_TYPE_AEAD_AES_128_CCM"
    );
    assert_eq!(
        unsafe { wx_abi(266) },
        XDP_QUIC_CIPHER_TYPE_AEAD_AES_128_GCM as u32 as usize,
        "XDP_QUIC_CIPHER_TYPE_AEAD_AES_128_GCM"
    );
    assert_eq!(
        unsafe { wx_abi(267) },
        XDP_QUIC_CIPHER_TYPE_AEAD_AES_256_GCM as u32 as usize,
        "XDP_QUIC_CIPHER_TYPE_AEAD_AES_256_GCM"
    );
    assert_eq!(
        unsafe { wx_abi(268) },
        XDP_QUIC_CIPHER_TYPE_AEAD_CHACHA20_POLY1305 as u32 as usize,
        "XDP_QUIC_CIPHER_TYPE_AEAD_CHACHA20_POLY1305"
    );
    assert_eq!(
        unsafe { wx_abi(269) },
        XDP_QUIC_CONNECTION_REVISION_1 as u32 as usize,
        "XDP_QUIC_CONNECTION_REVISION_1"
    );
    assert_eq!(
        unsafe { wx_abi(270) },
        XDP_QUIC_DECRYPT_FAILURE_ACTION_CONTINUE as u32 as usize,
        "XDP_QUIC_DECRYPT_FAILURE_ACTION_CONTINUE"
    );
    assert_eq!(
        unsafe { wx_abi(271) },
        XDP_QUIC_DECRYPT_FAILURE_ACTION_DROP as u32 as usize,
        "XDP_QUIC_DECRYPT_FAILURE_ACTION_DROP"
    );
    assert_eq!(
        unsafe { wx_abi(272) },
        XDP_QUIC_DIRECTION_RECEIVE as u32 as usize,
        "XDP_QUIC_DIRECTION_RECEIVE"
    );
    assert_eq!(
        unsafe { wx_abi(273) },
        XDP_QUIC_DIRECTION_TRANSMIT as u32 as usize,
        "XDP_QUIC_DIRECTION_TRANSMIT"
    );
    assert_eq!(
        unsafe { wx_abi(274) },
        XDP_QUIC_MAX_CID_LENGTH as u32 as usize,
        "XDP_QUIC_MAX_CID_LENGTH"
    );
    assert_eq!(
        unsafe { wx_abi(275) },
        XDP_QUIC_OPERATION_ADD as u32 as usize,
        "XDP_QUIC_OPERATION_ADD"
    );
    assert_eq!(
        unsafe { wx_abi(276) },
        XDP_QUIC_OPERATION_REMOVE as u32 as usize,
        "XDP_QUIC_OPERATION_REMOVE"
    );
    assert_eq!(
        unsafe { wx_abi(277) },
        XDP_REDIRECT_TARGET_TYPE_XSK as u32 as usize,
        "XDP_REDIRECT_TARGET_TYPE_XSK"
    );
    assert_eq!(
        unsafe { wx_abi(278) },
        XDP_REDIRECT_TARGET_TYPE_XSKMAP_BY_QUEUEID as u32 as usize,
        "XDP_REDIRECT_TARGET_TYPE_XSKMAP_BY_QUEUEID"
    );
    assert_eq!(
        unsafe { wx_abi(279) },
        XDP_RSS_CAPABILITIES_REVISION_1 as u32 as usize,
        "XDP_RSS_CAPABILITIES_REVISION_1"
    );
    assert_eq!(
        unsafe { wx_abi(280) },
        XDP_RSS_CAPABILITIES_REVISION_2 as u32 as usize,
        "XDP_RSS_CAPABILITIES_REVISION_2"
    );
    assert_eq!(
        unsafe { wx_abi(281) },
        XDP_RSS_CONFIGURATION_REVISION_1 as u32 as usize,
        "XDP_RSS_CONFIGURATION_REVISION_1"
    );
    assert_eq!(
        unsafe { wx_abi(282) },
        XDP_RSS_FLAG_DISABLED as u32 as usize,
        "XDP_RSS_FLAG_DISABLED"
    );
    assert_eq!(
        unsafe { wx_abi(283) },
        XDP_RSS_FLAG_SET_HASH_SECRET_KEY as u32 as usize,
        "XDP_RSS_FLAG_SET_HASH_SECRET_KEY"
    );
    assert_eq!(
        unsafe { wx_abi(284) },
        XDP_RSS_FLAG_SET_HASH_TYPE as u32 as usize,
        "XDP_RSS_FLAG_SET_HASH_TYPE"
    );
    assert_eq!(
        unsafe { wx_abi(285) },
        XDP_RSS_FLAG_SET_INDIRECTION_TABLE as u32 as usize,
        "XDP_RSS_FLAG_SET_INDIRECTION_TABLE"
    );
    assert_eq!(
        unsafe { wx_abi(286) },
        XDP_RSS_HASH_TYPE_IPV4 as u32 as usize,
        "XDP_RSS_HASH_TYPE_IPV4"
    );
    assert_eq!(
        unsafe { wx_abi(287) },
        XDP_RSS_HASH_TYPE_IPV6 as u32 as usize,
        "XDP_RSS_HASH_TYPE_IPV6"
    );
    assert_eq!(
        unsafe { wx_abi(288) },
        XDP_RSS_HASH_TYPE_IPV6_EX as u32 as usize,
        "XDP_RSS_HASH_TYPE_IPV6_EX"
    );
    assert_eq!(
        unsafe { wx_abi(289) },
        XDP_RSS_HASH_TYPE_TCP_IPV4 as u32 as usize,
        "XDP_RSS_HASH_TYPE_TCP_IPV4"
    );
    assert_eq!(
        unsafe { wx_abi(290) },
        XDP_RSS_HASH_TYPE_TCP_IPV6 as u32 as usize,
        "XDP_RSS_HASH_TYPE_TCP_IPV6"
    );
    assert_eq!(
        unsafe { wx_abi(291) },
        XDP_RSS_HASH_TYPE_TCP_IPV6_EX as u32 as usize,
        "XDP_RSS_HASH_TYPE_TCP_IPV6_EX"
    );
    assert_eq!(
        unsafe { wx_abi(292) },
        XDP_RSS_HASH_TYPE_UDP_IPV4 as u32 as usize,
        "XDP_RSS_HASH_TYPE_UDP_IPV4"
    );
    assert_eq!(
        unsafe { wx_abi(293) },
        XDP_RSS_HASH_TYPE_UDP_IPV6 as u32 as usize,
        "XDP_RSS_HASH_TYPE_UDP_IPV6"
    );
    assert_eq!(
        unsafe { wx_abi(294) },
        XDP_RSS_HASH_TYPE_UDP_IPV6_EX as u32 as usize,
        "XDP_RSS_HASH_TYPE_UDP_IPV6_EX"
    );
    assert_eq!(
        unsafe { wx_abi(295) },
        XDP_RSS_VALID_FLAGS as u32 as usize,
        "XDP_RSS_VALID_FLAGS"
    );
    assert_eq!(
        unsafe { wx_abi(296) },
        XDP_RSS_VALID_HASH_TYPES as u32 as usize,
        "XDP_RSS_VALID_HASH_TYPES"
    );
    assert_eq!(
        unsafe { wx_abi(297) },
        XDP_SIZEOF_CHECKSUM_CONFIGURATION_REVISION_1 as u32 as usize,
        "XDP_SIZEOF_CHECKSUM_CONFIGURATION_REVISION_1"
    );
    assert_eq!(
        unsafe { wx_abi(298) },
        XDP_SIZEOF_QUIC_CONNECTION_REVISION_1 as u32 as usize,
        "XDP_SIZEOF_QUIC_CONNECTION_REVISION_1"
    );
    assert_eq!(
        unsafe { wx_abi(299) },
        XDP_SIZEOF_RSS_CAPABILITIES_REVISION_1 as u32 as usize,
        "XDP_SIZEOF_RSS_CAPABILITIES_REVISION_1"
    );
    assert_eq!(
        unsafe { wx_abi(300) },
        XDP_SIZEOF_RSS_CAPABILITIES_REVISION_2 as u32 as usize,
        "XDP_SIZEOF_RSS_CAPABILITIES_REVISION_2"
    );
    assert_eq!(
        unsafe { wx_abi(301) },
        XDP_SIZEOF_RSS_CONFIGURATION_REVISION_1 as u32 as usize,
        "XDP_SIZEOF_RSS_CONFIGURATION_REVISION_1"
    );
    assert_eq!(
        unsafe { wx_abi(302) },
        XDP_STATUS_INTERNAL_ERROR as u32 as usize,
        "XDP_STATUS_INTERNAL_ERROR"
    );
    assert_eq!(
        unsafe { wx_abi(303) },
        XDP_STATUS_INVALID_PARAMETER as u32 as usize,
        "XDP_STATUS_INVALID_PARAMETER"
    );
    assert_eq!(
        unsafe { wx_abi(304) },
        XDP_STATUS_NOT_SUPPORTED as u32 as usize,
        "XDP_STATUS_NOT_SUPPORTED"
    );
    assert_eq!(
        unsafe { wx_abi(305) },
        XDP_STATUS_OUT_OF_MEMORY as u32 as usize,
        "XDP_STATUS_OUT_OF_MEMORY"
    );
    assert_eq!(
        unsafe { wx_abi(306) },
        XDP_STATUS_PENDING as u32 as usize,
        "XDP_STATUS_PENDING"
    );
    assert_eq!(
        unsafe { wx_abi(307) },
        XDP_STATUS_SUCCESS as u32 as usize,
        "XDP_STATUS_SUCCESS"
    );
    assert_eq!(
        unsafe { wx_abi(308) },
        XDP_STATUS_TIMEOUT as u32 as usize,
        "XDP_STATUS_TIMEOUT"
    );
    assert_eq!(
        unsafe { wx_abi(309) },
        XSK_ACTIVATE_FLAG_NONE as u32 as usize,
        "XSK_ACTIVATE_FLAG_NONE"
    );
    assert_eq!(
        unsafe { wx_abi(310) },
        XSK_BIND_FLAG_GENERIC as u32 as usize,
        "XSK_BIND_FLAG_GENERIC"
    );
    assert_eq!(
        unsafe { wx_abi(311) },
        XSK_BIND_FLAG_NATIVE as u32 as usize,
        "XSK_BIND_FLAG_NATIVE"
    );
    assert_eq!(
        unsafe { wx_abi(312) },
        XSK_BIND_FLAG_NONE as u32 as usize,
        "XSK_BIND_FLAG_NONE"
    );
    assert_eq!(
        unsafe { wx_abi(313) },
        XSK_BIND_FLAG_RX as u32 as usize,
        "XSK_BIND_FLAG_RX"
    );
    assert_eq!(
        unsafe { wx_abi(314) },
        XSK_BIND_FLAG_TX as u32 as usize,
        "XSK_BIND_FLAG_TX"
    );
    assert_eq!(
        unsafe { wx_abi(315) },
        XSK_ERROR_INTERFACE_DETACH as u32 as usize,
        "XSK_ERROR_INTERFACE_DETACH"
    );
    assert_eq!(
        unsafe { wx_abi(316) },
        XSK_ERROR_INVALID_RING as u32 as usize,
        "XSK_ERROR_INVALID_RING"
    );
    assert_eq!(
        unsafe { wx_abi(317) },
        XSK_FRAME_EXTENSION_ORIGINAL_LENGTH_VERSION_1 as u32 as usize,
        "XSK_FRAME_EXTENSION_ORIGINAL_LENGTH_VERSION_1"
    );
    assert_eq!(
        unsafe { wx_abi(318) },
        XSK_NOTIFY_FLAG_NONE as u32 as usize,
        "XSK_NOTIFY_FLAG_NONE"
    );
    assert_eq!(
        unsafe { wx_abi(319) },
        XSK_NOTIFY_FLAG_POKE_RX as u32 as usize,
        "XSK_NOTIFY_FLAG_POKE_RX"
    );
    assert_eq!(
        unsafe { wx_abi(320) },
        XSK_NOTIFY_FLAG_POKE_TX as u32 as usize,
        "XSK_NOTIFY_FLAG_POKE_TX"
    );
    assert_eq!(
        unsafe { wx_abi(321) },
        XSK_NOTIFY_FLAG_WAIT_RX as u32 as usize,
        "XSK_NOTIFY_FLAG_WAIT_RX"
    );
    assert_eq!(
        unsafe { wx_abi(322) },
        XSK_NOTIFY_FLAG_WAIT_TX as u32 as usize,
        "XSK_NOTIFY_FLAG_WAIT_TX"
    );
    assert_eq!(
        unsafe { wx_abi(323) },
        XSK_NOTIFY_RESULT_FLAG_NONE as u32 as usize,
        "XSK_NOTIFY_RESULT_FLAG_NONE"
    );
    assert_eq!(
        unsafe { wx_abi(324) },
        XSK_NOTIFY_RESULT_FLAG_RX_AVAILABLE as u32 as usize,
        "XSK_NOTIFY_RESULT_FLAG_RX_AVAILABLE"
    );
    assert_eq!(
        unsafe { wx_abi(325) },
        XSK_NOTIFY_RESULT_FLAG_TX_COMP_AVAILABLE as u32 as usize,
        "XSK_NOTIFY_RESULT_FLAG_TX_COMP_AVAILABLE"
    );
    assert_eq!(
        unsafe { wx_abi(326) },
        XSK_NO_ERROR as u32 as usize,
        "XSK_NO_ERROR"
    );
    assert_eq!(
        unsafe { wx_abi(327) },
        XSK_POLL_MODE_BUSY as u32 as usize,
        "XSK_POLL_MODE_BUSY"
    );
    assert_eq!(
        unsafe { wx_abi(328) },
        XSK_POLL_MODE_DEFAULT as u32 as usize,
        "XSK_POLL_MODE_DEFAULT"
    );
    assert_eq!(
        unsafe { wx_abi(329) },
        XSK_POLL_MODE_SOCKET as u32 as usize,
        "XSK_POLL_MODE_SOCKET"
    );
    assert_eq!(
        unsafe { wx_abi(330) },
        XSK_RING_FLAG_AFFINITY_CHANGED as u32 as usize,
        "XSK_RING_FLAG_AFFINITY_CHANGED"
    );
    assert_eq!(
        unsafe { wx_abi(331) },
        XSK_RING_FLAG_ERROR as u32 as usize,
        "XSK_RING_FLAG_ERROR"
    );
    assert_eq!(
        unsafe { wx_abi(332) },
        XSK_RING_FLAG_NEED_POKE as u32 as usize,
        "XSK_RING_FLAG_NEED_POKE"
    );
    assert_eq!(
        unsafe { wx_abi(333) },
        XSK_RING_FLAG_NONE as u32 as usize,
        "XSK_RING_FLAG_NONE"
    );
    assert_eq!(
        unsafe { wx_abi(334) },
        XSK_RING_FLAG_OFFLOAD_CHANGED as u32 as usize,
        "XSK_RING_FLAG_OFFLOAD_CHANGED"
    );
    assert_eq!(
        unsafe { wx_abi(335) },
        XSK_SOCKOPT_POLL_MODE as u32 as usize,
        "XSK_SOCKOPT_POLL_MODE"
    );
    assert_eq!(
        unsafe { wx_abi(336) },
        XSK_SOCKOPT_RING_INFO as u32 as usize,
        "XSK_SOCKOPT_RING_INFO"
    );
    assert_eq!(
        unsafe { wx_abi(337) },
        XSK_SOCKOPT_RX_ERROR as u32 as usize,
        "XSK_SOCKOPT_RX_ERROR"
    );
    assert_eq!(
        unsafe { wx_abi(338) },
        XSK_SOCKOPT_RX_FILL_ERROR as u32 as usize,
        "XSK_SOCKOPT_RX_FILL_ERROR"
    );
    assert_eq!(
        unsafe { wx_abi(339) },
        XSK_SOCKOPT_RX_FILL_RING_SIZE as u32 as usize,
        "XSK_SOCKOPT_RX_FILL_RING_SIZE"
    );
    assert_eq!(
        unsafe { wx_abi(340) },
        XSK_SOCKOPT_RX_FRAME_CHECKSUM_EXTENSION as u32 as usize,
        "XSK_SOCKOPT_RX_FRAME_CHECKSUM_EXTENSION"
    );
    assert_eq!(
        unsafe { wx_abi(341) },
        XSK_SOCKOPT_RX_FRAME_LAYOUT_EXTENSION as u32 as usize,
        "XSK_SOCKOPT_RX_FRAME_LAYOUT_EXTENSION"
    );
    assert_eq!(
        unsafe { wx_abi(342) },
        XSK_SOCKOPT_RX_FRAME_ORIGINAL_LENGTH_EXTENSION as u32 as usize,
        "XSK_SOCKOPT_RX_FRAME_ORIGINAL_LENGTH_EXTENSION"
    );
    assert_eq!(
        unsafe { wx_abi(343) },
        XSK_SOCKOPT_RX_FRAME_TIMESTAMP_EXTENSION as u32 as usize,
        "XSK_SOCKOPT_RX_FRAME_TIMESTAMP_EXTENSION"
    );
    assert_eq!(
        unsafe { wx_abi(344) },
        XSK_SOCKOPT_RX_HOOK_ID as u32 as usize,
        "XSK_SOCKOPT_RX_HOOK_ID"
    );
    assert_eq!(
        unsafe { wx_abi(345) },
        XSK_SOCKOPT_RX_OFFLOAD_CHECKSUM as u32 as usize,
        "XSK_SOCKOPT_RX_OFFLOAD_CHECKSUM"
    );
    assert_eq!(
        unsafe { wx_abi(346) },
        XSK_SOCKOPT_RX_OFFLOAD_CURRENT_CONFIG_CHECKSUM as u32 as usize,
        "XSK_SOCKOPT_RX_OFFLOAD_CURRENT_CONFIG_CHECKSUM"
    );
    assert_eq!(
        unsafe { wx_abi(347) },
        XSK_SOCKOPT_RX_OFFLOAD_TIMESTAMP as u32 as usize,
        "XSK_SOCKOPT_RX_OFFLOAD_TIMESTAMP"
    );
    assert_eq!(
        unsafe { wx_abi(348) },
        XSK_SOCKOPT_RX_ORIGINAL_LENGTH as u32 as usize,
        "XSK_SOCKOPT_RX_ORIGINAL_LENGTH"
    );
    assert_eq!(
        unsafe { wx_abi(349) },
        XSK_SOCKOPT_RX_PROCESSOR_AFFINITY as u32 as usize,
        "XSK_SOCKOPT_RX_PROCESSOR_AFFINITY"
    );
    assert_eq!(
        unsafe { wx_abi(350) },
        XSK_SOCKOPT_RX_RING_SIZE as u32 as usize,
        "XSK_SOCKOPT_RX_RING_SIZE"
    );
    assert_eq!(
        unsafe { wx_abi(351) },
        XSK_SOCKOPT_STATISTICS as u32 as usize,
        "XSK_SOCKOPT_STATISTICS"
    );
    assert_eq!(
        unsafe { wx_abi(352) },
        XSK_SOCKOPT_TX_COMPLETION_ERROR as u32 as usize,
        "XSK_SOCKOPT_TX_COMPLETION_ERROR"
    );
    assert_eq!(
        unsafe { wx_abi(353) },
        XSK_SOCKOPT_TX_COMPLETION_RING_SIZE as u32 as usize,
        "XSK_SOCKOPT_TX_COMPLETION_RING_SIZE"
    );
    assert_eq!(
        unsafe { wx_abi(354) },
        XSK_SOCKOPT_TX_ERROR as u32 as usize,
        "XSK_SOCKOPT_TX_ERROR"
    );
    assert_eq!(
        unsafe { wx_abi(355) },
        XSK_SOCKOPT_TX_FRAME_CHECKSUM_EXTENSION as u32 as usize,
        "XSK_SOCKOPT_TX_FRAME_CHECKSUM_EXTENSION"
    );
    assert_eq!(
        unsafe { wx_abi(356) },
        XSK_SOCKOPT_TX_FRAME_LAYOUT_EXTENSION as u32 as usize,
        "XSK_SOCKOPT_TX_FRAME_LAYOUT_EXTENSION"
    );
    assert_eq!(
        unsafe { wx_abi(357) },
        XSK_SOCKOPT_TX_FRAME_TIMESTAMP_EXTENSION as u32 as usize,
        "XSK_SOCKOPT_TX_FRAME_TIMESTAMP_EXTENSION"
    );
    assert_eq!(
        unsafe { wx_abi(358) },
        XSK_SOCKOPT_TX_HOOK_ID as u32 as usize,
        "XSK_SOCKOPT_TX_HOOK_ID"
    );
    assert_eq!(
        unsafe { wx_abi(359) },
        XSK_SOCKOPT_TX_OFFLOAD_CHECKSUM as u32 as usize,
        "XSK_SOCKOPT_TX_OFFLOAD_CHECKSUM"
    );
    assert_eq!(
        unsafe { wx_abi(360) },
        XSK_SOCKOPT_TX_OFFLOAD_CURRENT_CONFIG_CHECKSUM as u32 as usize,
        "XSK_SOCKOPT_TX_OFFLOAD_CURRENT_CONFIG_CHECKSUM"
    );
    assert_eq!(
        unsafe { wx_abi(361) },
        XSK_SOCKOPT_TX_OFFLOAD_TIMESTAMP as u32 as usize,
        "XSK_SOCKOPT_TX_OFFLOAD_TIMESTAMP"
    );
    assert_eq!(
        unsafe { wx_abi(362) },
        XSK_SOCKOPT_TX_PROCESSOR_AFFINITY as u32 as usize,
        "XSK_SOCKOPT_TX_PROCESSOR_AFFINITY"
    );
    assert_eq!(
        unsafe { wx_abi(363) },
        XSK_SOCKOPT_TX_RING_SIZE as u32 as usize,
        "XSK_SOCKOPT_TX_RING_SIZE"
    );
    assert_eq!(
        unsafe { wx_abi(364) },
        XSK_SOCKOPT_UMEM_REG as u32 as usize,
        "XSK_SOCKOPT_UMEM_REG"
    );
    assert_eq!(
        unsafe { wx_abi(365) },
        XdpFrameLayer2TypeEthernet as u32 as usize,
        "XdpFrameLayer2TypeEthernet"
    );
    assert_eq!(
        unsafe { wx_abi(366) },
        XdpFrameLayer2TypeNull as u32 as usize,
        "XdpFrameLayer2TypeNull"
    );
    assert_eq!(
        unsafe { wx_abi(367) },
        XdpFrameLayer2TypeUnspecified as u32 as usize,
        "XdpFrameLayer2TypeUnspecified"
    );
    assert_eq!(
        unsafe { wx_abi(368) },
        XdpFrameLayer3TypeIPv4NoOptions as u32 as usize,
        "XdpFrameLayer3TypeIPv4NoOptions"
    );
    assert_eq!(
        unsafe { wx_abi(369) },
        XdpFrameLayer3TypeIPv4UnspecifiedOptions as u32 as usize,
        "XdpFrameLayer3TypeIPv4UnspecifiedOptions"
    );
    assert_eq!(
        unsafe { wx_abi(370) },
        XdpFrameLayer3TypeIPv4WithOptions as u32 as usize,
        "XdpFrameLayer3TypeIPv4WithOptions"
    );
    assert_eq!(
        unsafe { wx_abi(371) },
        XdpFrameLayer3TypeIPv6NoExtensions as u32 as usize,
        "XdpFrameLayer3TypeIPv6NoExtensions"
    );
    assert_eq!(
        unsafe { wx_abi(372) },
        XdpFrameLayer3TypeIPv6UnspecifiedExtensions as u32 as usize,
        "XdpFrameLayer3TypeIPv6UnspecifiedExtensions"
    );
    assert_eq!(
        unsafe { wx_abi(373) },
        XdpFrameLayer3TypeIPv6WithExtensions as u32 as usize,
        "XdpFrameLayer3TypeIPv6WithExtensions"
    );
    assert_eq!(
        unsafe { wx_abi(374) },
        XdpFrameLayer3TypeUnspecified as u32 as usize,
        "XdpFrameLayer3TypeUnspecified"
    );
    assert_eq!(
        unsafe { wx_abi(375) },
        XdpFrameLayer4TypeIPFragment as u32 as usize,
        "XdpFrameLayer4TypeIPFragment"
    );
    assert_eq!(
        unsafe { wx_abi(376) },
        XdpFrameLayer4TypeIPNotFragment as u32 as usize,
        "XdpFrameLayer4TypeIPNotFragment"
    );
    assert_eq!(
        unsafe { wx_abi(377) },
        XdpFrameLayer4TypeTcp as u32 as usize,
        "XdpFrameLayer4TypeTcp"
    );
    assert_eq!(
        unsafe { wx_abi(378) },
        XdpFrameLayer4TypeUdp as u32 as usize,
        "XdpFrameLayer4TypeUdp"
    );
    assert_eq!(
        unsafe { wx_abi(379) },
        XdpFrameLayer4TypeUnspecified as u32 as usize,
        "XdpFrameLayer4TypeUnspecified"
    );
    assert_eq!(
        unsafe { wx_abi(380) },
        XdpFrameRxChecksumEvaluationFailed as u32 as usize,
        "XdpFrameRxChecksumEvaluationFailed"
    );
    assert_eq!(
        unsafe { wx_abi(381) },
        XdpFrameRxChecksumEvaluationInvalid as u32 as usize,
        "XdpFrameRxChecksumEvaluationInvalid"
    );
    assert_eq!(
        unsafe { wx_abi(382) },
        XdpFrameRxChecksumEvaluationNotChecked as u32 as usize,
        "XdpFrameRxChecksumEvaluationNotChecked"
    );
    assert_eq!(
        unsafe { wx_abi(383) },
        XdpFrameRxChecksumEvaluationSucceeded as u32 as usize,
        "XdpFrameRxChecksumEvaluationSucceeded"
    );
    assert_eq!(
        unsafe { wx_abi(384) },
        XdpFrameTxChecksumActionPassthrough as u32 as usize,
        "XdpFrameTxChecksumActionPassthrough"
    );
    assert_eq!(
        unsafe { wx_abi(385) },
        XdpFrameTxChecksumActionRequired as u32 as usize,
        "XdpFrameTxChecksumActionRequired"
    );
    assert_eq!(unsafe { wx_abi(386) }, XDP_PASS as u32 as usize, "XDP_PASS");
    assert_eq!(unsafe { wx_abi(387) }, XDP_DROP as u32 as usize, "XDP_DROP");
    assert_eq!(unsafe { wx_abi(388) }, XDP_TX as u32 as usize, "XDP_TX");
    assert_eq!(
        unsafe { wx_abi(389) },
        XDP_REDIRECT as u32 as usize,
        "XDP_REDIRECT"
    );
}
