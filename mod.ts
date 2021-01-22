Deno.openPlugin("./target/debug/deno_ipc.dll");

const core = (Deno as any).core;
const ops = core.ops();
const ERROR_PREFIX = "ipc_err::";

const dec = new TextDecoder("utf-8");
const enc = new TextEncoder();

function dispatch(id: string, ...args: any[]) {
  const res = dec.decode(
    core.dispatch(
      ops[id],
      ...args.map((e) =>
        typeof e === "object"
          ? e instanceof Uint8Array
            ? e
            : enc.encode(JSON.stringify(e))
          : enc.encode(String(e))
      )
    )
  );

  if (res.startsWith(ERROR_PREFIX)) {
    const err = res.slice(ERROR_PREFIX.length).trim();
    throw new Error(err);
  }

  return res;
}

function op_ipc_new(path: string): number {
  const res = dispatch("op_ipc_new", path);
  return Number(res);
}

function op_ipc_close(id: number): void {
  dispatch("op_ipc_close", id);
}

function op_ipc_read_string(id: number): string {
  return dispatch("op_ipc_read_string", id);
}

function op_ipc_write_all(id: number, data: string | Uint8Array): void {
  dispatch("op_ipc_write_all", id, data);
}

/** IPC class to interact with local sockets */
export class IPC {
  /** Internal resource ID (from Rust) */
  #id: number;

  constructor(path: string) {
    // Create socket in plugin and store ID.
    this.#id = op_ipc_new(path);
  }

  /** Read from the socket to String. */
  read(): string {
    return op_ipc_read_string(this.#id);
  }

  /** Write to the Socket. */
  write(data: string | Uint8Array): void {
    return op_ipc_write_all(this.#id, data);
  }

  /** Close the Socket connection. */
  close(): void {
    return op_ipc_close(this.#id);
  }
}
