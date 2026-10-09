import { invoke } from "@tauri-apps/api/core";
import { Menu, MenuItem, PredefinedMenuItem } from "@tauri-apps/api/menu";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { LogicalPosition } from "@tauri-apps/api/dpi";

export const isMac = navigator.userAgent.includes("Mac");

export const pickFolder = () => invoke("pick_folder");

export const pickFiles = ({ title = "Open", images = false } = {}) =>
  invoke("pick_files", { title, images });

export const confirmDestructive = (title, message, okLabel) =>
  invoke("confirm_destructive", { title, message, okLabel });

export async function popupMenu(items, event) {
  const built = await Promise.all(
    items.map((item) =>
      item === "-"
        ? PredefinedMenuItem.new({ item: "Separator" })
        : MenuItem.new({
            text: item.text,
            enabled: item.enabled ?? Boolean(item.action),
            action: item.action,
          }),
    ),
  );
  const menu = await Menu.new({ items: built });
  if (event?.currentTarget && !event.clientX) {
    const rect = event.currentTarget.getBoundingClientRect();
    await menu.popup(new LogicalPosition(rect.left, rect.bottom + 4), getCurrentWindow());
  } else {
    await menu.popup();
  }
}

export function startWindowDrag(event) {
  if (event.button !== 0 || event.target.closest("button, input, select, textarea")) return;
  if (event.detail === 2) {
    getCurrentWindow().toggleMaximize();
  } else {
    getCurrentWindow().startDragging();
  }
}
