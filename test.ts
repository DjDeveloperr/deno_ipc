import { IPC } from "./mod.ts";

const ipc = new IPC("discord-ipc-0");

console.log("Connected!");

console.log("Writing...");
ipc.write(`W{"v":1}`);

console.log("Reading...");
console.log(ipc.read());
