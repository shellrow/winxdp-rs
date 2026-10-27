#include "bridge.h"
XDP_STATUS wx_XskCreate(HANDLE* Socket) { return XskCreate(Socket); }
XDP_STATUS wx_XskBind(HANDLE Socket, UINT32 IfIndex, UINT32 QueueId, XSK_BIND_FLAGS Flags) { return XskBind(Socket, IfIndex, QueueId, Flags); }
XDP_STATUS wx_XskActivate(HANDLE Socket, XSK_ACTIVATE_FLAGS Flags) { return XskActivate(Socket, Flags); }
XDP_STATUS wx_XskSetSockopt(HANDLE Socket, UINT32 OptionName, const VOID *OptionValue, UINT32 OptionLength) { return XskSetSockopt(Socket, OptionName, OptionValue, OptionLength); }
XDP_STATUS wx_XskGetSockopt(HANDLE Socket, UINT32 OptionName, VOID *OptionValue, UINT32 *OptionLength) { return XskGetSockopt(Socket, OptionName, OptionValue, OptionLength); }
XDP_STATUS wx_XskIoctl(HANDLE Socket, UINT32 OptionName, const VOID *InputValue, UINT32 InputLength, VOID *OutputValue, UINT32 *OutputLength) { return XskIoctl(Socket, OptionName, InputValue, InputLength, OutputValue, OutputLength); }
XDP_STATUS wx_XskNotifySocket(HANDLE Socket, XSK_NOTIFY_FLAGS Flags, UINT32 WaitTimeoutMilliseconds, XSK_NOTIFY_RESULT_FLAGS *Result) { return XskNotifySocket(Socket, Flags, WaitTimeoutMilliseconds, Result); }
XDP_STATUS wx_XskNotifyAsync(HANDLE Socket, XSK_NOTIFY_FLAGS Flags, XDP_OVERLAPPED *Overlapped) { return XskNotifyAsync(Socket, Flags, Overlapped); }
XDP_STATUS wx_XskGetNotifyAsyncResult(XDP_OVERLAPPED *Overlapped, XSK_NOTIFY_RESULT_FLAGS *Result) { return XskGetNotifyAsyncResult(Overlapped, Result); }
VOID wx_XskRingInitialize(XSK_RING *Ring, const XSK_RING_INFO *RingInfo) { XskRingInitialize(Ring, RingInfo); }
VOID * wx_XskRingGetElement(const XSK_RING *Ring, UINT32 Index) { return XskRingGetElement(Ring, Index); }
UINT32 wx_XskRingGetFlags(const XSK_RING *Ring) { return XskRingGetFlags(Ring); }
UINT32 wx_XskRingConsumerReserve(XSK_RING *Ring, UINT32 MaxCount, UINT32 *Index) { return XskRingConsumerReserve(Ring, MaxCount, Index); }
VOID wx_XskRingConsumerRelease(XSK_RING *Ring, UINT32 Count) { XskRingConsumerRelease(Ring, Count); }
UINT32 wx_XskRingProducerReserve(XSK_RING *Ring, UINT32 MaxCount, UINT32 *Index) { return XskRingProducerReserve(Ring, MaxCount, Index); }
VOID wx_XskRingProducerSubmit(XSK_RING *Ring, UINT32 Count) { XskRingProducerSubmit(Ring, Count); }
BOOLEAN wx_XskRingError(const XSK_RING *Ring) { return XskRingError(Ring); }
BOOLEAN wx_XskRingProducerNeedPoke(const XSK_RING *Ring) { return XskRingProducerNeedPoke(Ring); }
BOOLEAN wx_XskRingAffinityChanged(const XSK_RING *Ring) { return XskRingAffinityChanged(Ring); }
BOOLEAN wx_XskRingOffloadChanged(const XSK_RING *Ring) { return XskRingOffloadChanged(Ring); }
XDP_STATUS wx_XdpCreateProgram(UINT32 InterfaceIndex, const XDP_HOOK_ID *HookId, UINT32 QueueId, XDP_CREATE_PROGRAM_FLAGS Flags, const XDP_RULE *Rules, UINT32 RuleCount, HANDLE *Program) { return XdpCreateProgram(InterfaceIndex, HookId, QueueId, Flags, Rules, RuleCount, Program); }
XDP_STATUS wx_XdpInterfaceOpen(UINT32 InterfaceIndex, HANDLE *InterfaceHandle) { return XdpInterfaceOpen(InterfaceIndex, InterfaceHandle); }
XDP_STATUS wx_XdpMapCreate(HANDLE *Map, XDP_MAP_TYPE Type) { return XdpMapCreate(Map, Type); }
XDP_STATUS wx_XdpMapInsert(HANDLE Map, const VOID *Key, const VOID *Value) { return XdpMapInsert(Map, Key, Value); }
XDP_STATUS wx_XdpMapDelete(HANDLE Map, const VOID *Key) { return XdpMapDelete(Map, Key); }
VOID wx_XdpInitializeRssCapabilities(XDP_RSS_CAPABILITIES *RssCapabilities) { XdpInitializeRssCapabilities(RssCapabilities); }
VOID wx_XdpInitializeRssConfiguration(XDP_RSS_CONFIGURATION *RssConfiguration, UINT32 RssConfigurationSize) { XdpInitializeRssConfiguration(RssConfiguration, RssConfigurationSize); }
VOID wx_XdpInitializeQuicConnection(XDP_QUIC_CONNECTION *XdpQuicConnection, UINT32 XdpQuicConnectionSize) { XdpInitializeQuicConnection(XdpQuicConnection, XdpQuicConnectionSize); }
XDP_STATUS wx_XdpRssGetCapabilities(HANDLE InterfaceHandle, XDP_RSS_CAPABILITIES *RssCapabilities, UINT32 *RssCapabilitiesSize) { return XdpRssGetCapabilities(InterfaceHandle, RssCapabilities, RssCapabilitiesSize); }
XDP_STATUS wx_XdpRssSet(HANDLE InterfaceHandle, const XDP_RSS_CONFIGURATION *RssConfiguration, UINT32 RssConfigurationSize) { return XdpRssSet(InterfaceHandle, RssConfiguration, RssConfigurationSize); }
XDP_STATUS wx_XdpRssGet(HANDLE InterfaceHandle, XDP_RSS_CONFIGURATION *RssConfiguration, UINT32 *RssConfigurationSize) { return XdpRssGet(InterfaceHandle, RssConfiguration, RssConfigurationSize); }
XDP_STATUS wx_XdpQeoSet(HANDLE InterfaceHandle, XDP_QUIC_CONNECTION *QuicConnections, UINT32 QuicConnectionsSize) { return XdpQeoSet(InterfaceHandle, QuicConnections, QuicConnectionsSize); }
