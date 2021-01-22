# deno_ipc

This is a Deno plugin made in Rust, to do cross-platform Interprocess communication. Only Client implementation at the moment.

## Usage

```ts
const ipc = new IPC("named-pipe-name-here OR unix-socket-path");

ipc.write("something");

const reply = ipc.read();
console.log("reply:", reply);

ipc.close();
```

## Contributing

You're always welcome to contribute!

## License

Check [LICENSE](LICENSE) for more info.

Copyright 2021 @ DjDeveloperr