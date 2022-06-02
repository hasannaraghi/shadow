# Debugging

## Debugging the Shadow process

Shadow is currently built with debugging symbols in both debug and release
builds, though it may be easier to debug a debug build (generated by passing
the `--debug` flag to `setup build`).

Shadow can be run under GDB by prepending `gdb --args` to its command-line.
e.g.:

```
gdb --args shadow shadow.yaml
```

An alternative is to run Shadow with the `--gdb` flag, which will pause shadow
after startup and print its PID. You can then simply attach GDB to Shadow in a
new terminal and continue the experiment.

Example:

```
# terminal 1
# shadow will print its PID and pause
$ shadow --gdb shadow.yaml > shadow.log
** Starting Shadow
** Pausing with SIGTSTP to enable debugger attachment (pid 1234)

# terminal 2
$ gdb --pid=1234
> continue
```

## Debugging managed processes

A simulation's managed processes are implemented as native OS processes, with
their syscalls interposed by Shadow. Since they are native processes, many
normal tools for inspecting native processes can be used on those as well. e.g.
`top` will show how much CPU and memory they are using.

### Generating a core file

If a managed process is crashing, it is sometimes easiest to let the native
process to generate a core file, and then use GDB to inspect it afterwards.

```
# Enable core dumps.
ulimit -c unlimited

# Run the simulation in which a process is crashing.
shadow shadow.yaml

# Tell gdb to inspect the core file. From within gdb you'll be able to
# inspect the state of the process when it was killed.
gdb <path-to-process-executable> <path-to-core-file>
```

### Attaching with GDB

You can attach GDB directly to the managed process. To make this easier you can
use the `--debug-hosts` option to pause Shadow after it launches each managed
process on the given hosts. Shadow will print the native process' PID before
stopping. For example, `--debug-hosts client,server` will pause Shadow after
launching any managed processees on hosts "client" and "server". This allows
you to attach GDB directly to those managed processes before resuming Shadow.

```
# terminal 1
$ shadow --debug-hosts client,server shadow.yaml > shadow.log
** Starting Shadow
** Pausing with SIGTSTP to enable debugger attachment to managed process 'server.nginx.1000' (pid 1234)
** If running Shadow under Bash, resume Shadow by pressing Ctrl-Z to background this task and then typing "fg".
** (If you wish to kill Shadow, type "kill %%" instead.)
** If running Shadow under GDB, resume Shadow by typing "signal SIGCONT".

# terminal 2
$ gdb --pid=1234
```

### Debugging with GDB

```
# It's often useful to look at the stack backtrace:
> bt

# Or for a multi-threaded process, to look at all of the threads' backtraces:
> thread apply all bt
```