#!/usr/bin/env python3

import shlex
import sys
import argparse
parser = argparse.ArgumentParser()
parser.add_argument('year', type=int)
parser.add_argument('problem', type=int)
args = parser.parse_args()

import pathlib
HERE = pathlib.Path(__file__).parent
template = (HERE / 'src' / 'template').relative_to(HERE)
assert template.is_dir()
dst = (HERE / 'src' / format(args.year, '04d') / format(args.problem, '02d')).relative_to(HERE)
assert not dst.exists()

import shutil
shutil.copytree(template, dst)
(dst/'main.rs').write_text((dst/'main.rs').read_text().replace('__TEMPLATE_HERE__', str(dst)))

import subprocess

input('Copy your problem input, then hit Enter: ')
subprocess.check_call(['pbpaste'], stdout=(dst/'in.txt').open('w'))

bin_name = f'{args.year-2000}p{args.problem}'

open('Cargo.toml', 'a').write(f'''
[[bin]]
name = "{bin_name}"
path = "{dst}/main.rs"
''')

watch_cmd = ['cargo', 'watch', '-B1', '-x', f'run --bin {bin_name}']
p = subprocess.Popen(watch_cmd, stdin=sys.stdin, stdout=sys.stdout, stderr=sys.stderr)
import sys
try:
  p.wait()
except KeyboardInterrupt:
  p.kill()
finally:
  print('To keep watching, run:')
  print('  ' + ' '.join([shlex.quote(w) for w in watch_cmd]))
