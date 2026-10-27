"""Check pinned bytes and public ABI coverage without Clang or a network connection."""
from pathlib import Path
import hashlib,json,re
root=Path(__file__).resolve().parent.parent
crate=root/'crates/winxdp-sys'
pin=json.loads((root/'docs/upstream-pin.json').read_text(encoding='utf-8'))
for entry in pin['files']:
 data=(crate/'vendor'/entry['path']).read_bytes().replace(b'\r\n',b'\n')
 assert hashlib.sha256(data).hexdigest()==entry['sha256'],entry['path']
rows=json.loads((root/'docs/api-coverage.json').read_text(encoding='utf-8'))
covered={r['symbol'] for r in rows}
bindings=(crate/'src/bindings.rs').read_text(encoding='utf-8')+(crate/'src/lib.rs').read_text(encoding='utf-8')
for row in rows:
 if row['status'] in ['implemented','deprecated']:
  assert re.search(r'pub (?:const(?: fn)?|fn|type|struct|union) '+row['symbol']+r'\b',bindings) or re.search(r'pub use [^;]+ as '+row['symbol']+r';',bindings),row['symbol']
for entry in pin['files']:
 name=entry['path']
 if not name.endswith('.h') or '/details/' in '/'+name or name in ['xdp/wincommon.h','xdp/rtl.h']:continue
 source=(crate/'vendor'/name).read_text(encoding='utf-8')
 # Names physically declared in the public headers, not implementation internals.
 declarations=set(re.findall(r'^}\s*((?:XDP_|XSK_|xdp_)\w+)\s*;',source,re.M))
 declarations.update(re.findall(r'^((?:Xsk|Xdp)\w+)\(\s*$',source,re.M))
 for macro,value in re.findall(r'^#define\s+((?:XDP_|XSK_)\w+)[ \t]+([^\r\n]+)',source,re.M):
  if macro!='XDP_STATUS':declarations.add(macro)
 for enum in re.findall(r'typedef enum[^\{]*\{(.*?)\}',source,re.S):
  enum=re.sub(r'//[^\n]*','',enum)
  declarations.update(re.findall(r'\b((?:XDP_|XSK_|XdpFrame)\w+)\b\s*(?:=|,|$)',enum))
 for symbol in declarations:assert symbol in covered,f'Missing public declaration: {name}: {symbol}'
for ret,name,args,params,header in json.loads((root/'tools/functions.json').read_text(encoding='utf-8')):
 assert f'wx_{name}(' in (crate/'native/bridge.c').read_text(),name
print(f'Pinned {len(pin["files"])} files and {len(rows)} coverage records verified.')
