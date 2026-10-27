// SPDX-License-Identifier: MIT
// Source-only example. Compile, verify and sign through the eBPF native workflow.
// Matching traffic is diverted from the host stack; use a dedicated lab NIC.
#include <stdint.h>
#include "bpf_helpers.h"
#include "xdp/ebpfhook.h"

struct {
    __uint(type, BPF_MAP_TYPE_XSKMAP);
    __type(key, uint64_t);
    __type(value, void *);
    __uint(max_entries, 64);
} xsk_map SEC(".maps");

SEC("xdp/winxdp_selected")
int winxdp_selected(xdp_md_t *ctx)
{
    uint8_t *p = ctx->data;
    uint8_t *end = ctx->data_end;
    if (p + 34 > end || p[12] != 0x08 || p[13] != 0x00)
        return XDP_PASS;
    if ((p[14] >> 4) != 4 || p[23] != 17)
        return XDP_PASS;
    uint32_t ihl = (p[14] & 15) * 4;
    uint32_t total = ((uint32_t)p[16] << 8) | p[17];
    // Exclude fragments (MF or nonzero offset), malformed lengths and truncation.
    // Compare lengths instead of constructing a pointer beyond the verifier's
    // maximum packet offset (an IPv4 length can be 65535 before L2 is added).
    if (ihl < 20 || (p[20] & 0x3f) || p[21] || total < ihl + 8 || total > (uint64_t)(end - p) - 14)
        return XDP_PASS;
    uint8_t *udp = p + 14 + ihl;
    if (udp + 8 > end || udp[2] != 0xbf || udp[3] != 0x68)
        return XDP_PASS;
    uint32_t length = ((uint32_t)udp[4] << 8) | udp[5];
    if (length < 8 || length > total - ihl)
        return XDP_PASS;
    return (int)bpf_redirect_map(&xsk_map, ctx->rx_queue_index, XDP_PASS);
}
