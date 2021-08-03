# (Deprecated) deno_ipc

This is a Deno plugin made in Rust, to do cross-platform Interprocess communication. Only Client implementation at the moment.

## Usage

```ts
import { IPC } from "https://deno.land/x/ipc/mod.ts"

const ipc = new IPC("named-pipe-name-here OR unix-socket-path");

ipc.write("something");

const reply = ipc.read();
console.log("reply:", reply);

ipc.close();
```

API is very simple, there is IPC class, with read, write and close methods!

## Permissions

This needs all permissions to work because -
- --allow-read/write - Caching plugin binary (quite large to be downloaded every time)
- --allow-net - Downloading plugin binary.
- --allow-plugin - Well this makes sense.

and must be run with `--unstable` flag!

In short, `deno run -A --unstable <file>` is best option.

## Contributing

You're always welcome to contribute!

## License

Check [LICENSE](LICENSE) for more info.

Copyright 2021 @ DjDeveloperr
