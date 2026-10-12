import { invoke } from "@tauri-apps/api/core";

export const ideAttach = () => invoke("ide_attach");
export const ideListDir = (path) => invoke("ide_list_dir", { path });
export const ideRead = (path) => invoke("ide_read", { path });
export const ideSave = (path, content, fingerprint) => invoke("ide_save", { path, content, fingerprint });
export const ideCreate = (path, isDir) => invoke("ide_create", { path, isDir });
export const ideRename = (from, to) => invoke("ide_rename", { from, to });
export const ideDelete = (path) => invoke("ide_delete", { path });
export const ideIndex = () => invoke("ide_index");
