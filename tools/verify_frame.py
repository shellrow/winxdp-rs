"""Independently validate the Ethernet/IPv4/UDP frame emitted by tx_nex_packet."""
from pathlib import Path
import sys,struct
frame=Path(sys.argv[1]).read_bytes()
assert len(frame)>=60 and frame[12:14]==b'\x08\x00'
ip=frame[14:]
ihl=(ip[0]&15)*4
length=int.from_bytes(ip[2:4],'big')
assert ip[0]>>4==4 and ihl>=20 and ihl<=length<=len(ip) and ip[9]==17
udp=ip[ihl:length]
assert len(udp)>=8 and int.from_bytes(udp[4:6],'big')==len(udp)
def checksum(data):
 if len(data)%2:data+=b'\x00'
 value=sum(struct.unpack('!'+str(len(data)//2)+'H',data))
 while value>>16:value=(value&65535)+(value>>16)
 return value
assert checksum(ip[:ihl])==65535
assert checksum(ip[12:20]+bytes([0,17])+len(udp).to_bytes(2,'big')+udp)==65535
print(f'Validated {len(frame)} frame bytes, IPv4 header checksum and UDP pseudo-header checksum.')
