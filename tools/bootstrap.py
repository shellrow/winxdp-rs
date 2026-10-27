from pathlib import Path
import re, json, hashlib
root=Path('crates/winxdp-sys')
headers=['afxdp.h','afxdp_helper.h','xdpapi.h','xdpapi_experimental.h']
functions=[]
for h in headers:
 s=(root/'vendor'/h).read_text()
 for m in re.finditer(r'\n(XDP_STATUS|VOID|VOID \*|UINT32|BOOLEAN)\s*\n((?:Xsk|Xdp)\w+)\(\s*([\s\S]*?)\n    \)',s):
  ret,name,args=m.groups()
  if name in ['XdpRegisterDriverStartCallback','XdpDeregisterDriverStartCallback']: continue
  args=re.sub(r'(?<!\w)_\w+_(?:\([^\n]*?\))?\s*','',args)
  args=' '.join(args.split())
  names=[re.search(r'(\w+)\s*$',a).group(1) for a in args.split(',')]
  functions.append((ret,name,args,names,h))
base='#include "base.h"\n'
protos=[]; impl=[]
for ret,name,args,names,h in functions:
 protos.append(f'{ret} wx_{name}({args});')
 impl.append(f'{ret} wx_{name}({args}) {{ '+('' if ret=='VOID' else 'return ')+f'{name}('+', '.join(names)+'); }')
(root/'native/bridge.h').write_text(base+'\n'.join(protos)+'\n')
(root/'native/bridge.c').write_text('#include "bridge.h"\n'+'\n'.join(impl)+'\n')
Path('tools/functions.json').write_text(json.dumps(functions,indent=2))
files=[]
for p in sorted((root/'vendor').rglob('*')):
 if p.is_file():files.append({'path':p.relative_to(root/'vendor').as_posix(),'sha256':hashlib.sha256(p.read_bytes().replace(b"\r\n",b"\n")).hexdigest()})
Path('docs/upstream-pin.json').write_text(json.dumps({'tag':'v1.4.0','revision':'d372b52577a724e04fa4c06acb90bbfa4719fc25','api_version':3,'files':files},indent=2)+'\n')
print('Functions:',len(functions))
