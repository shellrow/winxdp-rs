#include "base.h"
UINT64 wx_test_address(UINT64 base, UINT16 offset) {
    XSK_BUFFER_ADDRESS a = {0}; a.BaseAddress = base; a.Offset = offset;
    return a.AddressAndOffset;
}
UINT64 wx_test_layout(UINT16 l2, UINT16 l3, UINT8 l4, UINT8 t2, UINT8 t3, UINT8 t4) {
    XDP_FRAME_LAYOUT l = {0}; UINT64 bytes = 0;
    l.Layer2HeaderLength=l2; l.Layer3HeaderLength=l3; l.Layer4HeaderLength=l4;
    l.Layer2Type=t2; l.Layer3Type=t3; l.Layer4Type=t4;
    memcpy(&bytes,&l,sizeof(l)); return bytes;
}
