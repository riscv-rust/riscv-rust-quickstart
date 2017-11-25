set remotetimeout 240
target extended-remote localhost:3333

define upload
  monitor reset halt
  monitor flash protect 0 64 last off
  load
  monitor flash protect 0 64 last on
  continue
end
document upload
Upload program to hifive board
end

# Load Rust's GDB pretty printers
python
import os;
import sys;
path = os.environ['TOOLCHAIN'] + '/lib/rustlib/etc'
sys.path.append(path)

gdb.execute('directory %s' % path)
gdb.execute('add-auto-load-safe-path %s' % path)
end
