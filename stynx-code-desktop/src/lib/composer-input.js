import { imagesFromPaste, addImagePaths } from "./images.js";
import { addReferenceFromUrl } from "./references.js";
import { pickFiles } from "./native.js";

export function handlePaste(event) {
  if (imagesFromPaste(event)) {
    event.preventDefault();
    return;
  }
  const text = event.clipboardData?.getData("text") ?? "";
  if (/^https?:\/\/\S+$/.test(text.trim())) {
    event.preventDefault();
    addReferenceFromUrl(text);
  }
}

export async function attachImages() {
  addImagePaths(await pickFiles({ title: "Attach", images: true }));
}
