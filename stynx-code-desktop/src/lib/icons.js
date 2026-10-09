const doc = '<path d="M14 3H7a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V8z"/><path d="M14 3v5h5"/>';
const bubble = '<path d="M21 12a8 8 0 0 1-11.6 7.1L4 20l1-4.6A8 8 0 1 1 21 12z"/>';
const person = '<circle cx="9" cy="8" r="4"/><path d="M2 21v-1a6 6 0 0 1 6-6h2a6 6 0 0 1 6 6v1"/>';
const folder = '<path d="M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>';
const square = '<rect x="4" y="4" width="16" height="16" rx="3"/>';

export const ICONS = {
  folder,
  "folder.fill": "fill:" + folder,
  "plus.rectangle.on.folder": folder + '<path d="M12 10v6M9 13h6"/>',
  "plus.bubble": bubble + '<path d="M13 9v6M10 12h6"/>',
  "bubble.left.and.text.bubble.right":
    '<path d="M15 10a6 6 0 0 1-8.7 5.3L3 16l.7-3.3A6 6 0 1 1 15 10z"/><path d="M9.5 18.5A6 6 0 0 0 17.7 20l3.3.7-.7-3.3A6 6 0 0 0 17 9.6"/>',
  "bubble.left.and.bubble.right":
    '<path d="M15 10a6 6 0 0 1-8.7 5.3L3 16l.7-3.3A6 6 0 1 1 15 10z"/><path d="M9.5 18.5A6 6 0 0 0 17.7 20l3.3.7-.7-3.3A6 6 0 0 0 17 9.6"/>',
  "questionmark.bubble": bubble + '<path d="M10.5 10a2 2 0 1 1 2.5 2c-.6.2-1 .7-1 1.3V14"/><circle cx="12" cy="16.8" r=".6" fill="currentColor"/>',
  "quote.bubble": bubble + '<path d="M9 11h2v2.5c0 .8-.5 1.5-1.5 1.5M13 11h2v2.5c0 .8-.5 1.5-1.5 1.5"/>',
  doc,
  "doc.text": doc + '<path d="M9 13h6M9 17h6"/>',
  "doc.plaintext": doc + '<path d="M9 12h6M9 15h6M9 18h4"/>',
  "doc.richtext": doc + '<path d="M8.5 17l2.5-3 2 2 1.5-2 1.5 3z"/>',
  "doc.badge.plus": doc + '<path d="M12 11v6M9 14h6"/>',
  "doc.text.magnifyingglass": doc + '<circle cx="11.5" cy="14" r="2.5"/><path d="M13.3 15.8l2 2"/>',
  "xmark.circle.fill":
    'fill:<path d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20zm3.5 12.1-1.4 1.4L12 13.4l-2.1 2.1-1.4-1.4 2.1-2.1-2.1-2.1 1.4-1.4 2.1 2.1 2.1-2.1 1.4 1.4-2.1 2.1z"/>',
  xmark: '<path d="M6 6l12 12M18 6L6 18"/>',
  "person.fill.checkmark": person + '<path d="M16 11l2 2 4-4"/>',
  "person.slash": person + '<path d="M3 3l18 18"/>',
  "person.2": person + '<path d="M16 4a4 4 0 0 1 0 8M18 14a6 6 0 0 1 4 5.5V21"/>',
  key: '<circle cx="8" cy="15" r="4"/><path d="M10.8 12.2L20 3M16 7l3 3M14 9l2 2"/>',
  "key.fill": 'fill:<circle cx="8" cy="15" r="5"/><path d="M11 11.5l9-9 2 2-2 2 1.5 1.5-2 2L18 8.5l-1.5 1.5 1.5 1.5-2 2-1.5-1.5-1.3 1.3z"/>',
  "sidebar.left": '<rect x="3" y="4" width="18" height="16" rx="3"/><path d="M9 4v16M5.5 8h1.5M5.5 11h1.5"/>',
  "sidebar.right": '<rect x="3" y="4" width="18" height="16" rx="3"/><path d="M15 4v16M17 8h1.5M17 11h1.5"/>',
  "square.and.pencil": '<path d="M12 4H6a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-6"/><path d="M17.5 3.5a2.1 2.1 0 0 1 3 3L12 15l-4 1 1-4z"/>',
  "clock.arrow.circlepath": '<path d="M3 12a9 9 0 1 0 3-6.7L3 8"/><path d="M3 3v5h5M12 7v5l3 2"/>',
  "lock.shield": '<path d="M12 3l8 3v6c0 4.5-3.4 8.3-8 9-4.6-.7-8-4.5-8-9V6z"/><rect x="9" y="11" width="6" height="5" rx="1"/><path d="M10 11V9.5a2 2 0 0 1 4 0V11"/>',
  "checkmark.shield": '<path d="M12 3l8 3v6c0 4.5-3.4 8.3-8 9-4.6-.7-8-4.5-8-9V6z"/><path d="M8.5 12l2.5 2.5 4.5-5"/>',
  bolt: '<path d="M13 2L4 14h7l-1 8 9-12h-7z"/>',
  "list.bullet.clipboard": '<rect x="5" y="4" width="14" height="17" rx="2"/><path d="M9 2.5h6v3H9zM9 11h6M9 15h6"/>',
  "lock.open": '<rect x="5" y="11" width="14" height="10" rx="2"/><path d="M8 11V7a4 4 0 0 1 7.7-1.5"/>',
  brain: '<path d="M12 5a3 3 0 0 0-5.8-1A3 3 0 0 0 4 8.5a3.5 3.5 0 0 0 .5 6.3A3 3 0 0 0 8 19.5a3 3 0 0 0 4 .5zM12 5a3 3 0 0 1 5.8-1A3 3 0 0 1 20 8.5a3.5 3.5 0 0 1-.5 6.3 3 3 0 0 1-3.5 4.7 3 3 0 0 1-4 .5zM12 5v15"/>',
  "chevron.down": '<path d="M6 9l6 6 6-6"/>',
  "chevron.up": '<path d="M6 15l6-6 6 6"/>',
  "chevron.right": '<path d="M9 6l6 6-6 6"/>',
  paperclip: '<path d="M20 11.5l-8.3 8.3a5 5 0 0 1-7.1-7.1l8.6-8.6a3.4 3.4 0 0 1 4.8 4.8l-8.6 8.6a1.7 1.7 0 0 1-2.4-2.4l7.9-7.9"/>',
  "arrow.up.circle.fill":
    'fill:<path d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20zm.9 5.3 4.4 4.4-1.4 1.4-2.9-2.9V17h-2v-6.8l-2.9 2.9-1.4-1.4 4.4-4.4a.9.9 0 0 1 1.3 0z"/>',
  "stop.circle.fill": 'fill:<path d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20zM9 8h6a1 1 0 0 1 1 1v6a1 1 0 0 1-1 1H9a1 1 0 0 1-1-1V9a1 1 0 0 1 1-1z"/>',
  "arrow.up.circle.hierarchical":
    'fill:<circle cx="12" cy="12" r="10.5" opacity=".28"/><path d="M12 17V7.8M7.8 11.8 12 7.6l4.2 4.2" fill="none" stroke="currentColor" stroke-width="2.3" stroke-linecap="round" stroke-linejoin="round"/>',
  "stop.circle.hierarchical":
    'fill:<circle cx="12" cy="12" r="10.5" opacity=".28"/><rect x="8.3" y="8.3" width="7.4" height="7.4" rx="1.4"/>',
  pencil:'<path d="M16.5 3.5a2.1 2.1 0 0 1 3 3L7 19l-4 1 1-4z"/>',
  terminal: '<rect x="3" y="4" width="18" height="16" rx="2"/><path d="M7 9l3 3-3 3M12 15h5"/>',
  magnifyingglass: '<circle cx="11" cy="11" r="7"/><path d="M16 16l5 5"/>',
  "text.magnifyingglass": '<path d="M3 5h12M3 10h6M3 15h5"/><circle cx="15" cy="15" r="4"/><path d="M18 18l3 3"/>',
  globe: '<circle cx="12" cy="12" r="9"/><path d="M3 12h18M12 3a14 14 0 0 1 0 18M12 3a14 14 0 0 0 0 18"/>',
  checklist: '<path d="M3 6l1.5 1.5L7 5M3 12l1.5 1.5L7 11M3 18l1.5 1.5L7 17M11 6h10M11 12h10M11 18h10"/>',
  gearshape: '<circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.7 1.7 0 0 0 .3 1.8l.1.1a2 2 0 1 1-2.8 2.8l-.1-.1a1.7 1.7 0 0 0-1.8-.3 1.7 1.7 0 0 0-1 1.5V21a2 2 0 1 1-4 0v-.1a1.7 1.7 0 0 0-1.1-1.5 1.7 1.7 0 0 0-1.8.3l-.1.1a2 2 0 1 1-2.8-2.8l.1-.1a1.7 1.7 0 0 0 .3-1.8 1.7 1.7 0 0 0-1.5-1H3a2 2 0 1 1 0-4h.1a1.7 1.7 0 0 0 1.5-1.1 1.7 1.7 0 0 0-.3-1.8l-.1-.1a2 2 0 1 1 2.8-2.8l.1.1a1.7 1.7 0 0 0 1.8.3H9a1.7 1.7 0 0 0 1-1.5V3a2 2 0 1 1 4 0v.1a1.7 1.7 0 0 0 1 1.5 1.7 1.7 0 0 0 1.8-.3l.1-.1a2 2 0 1 1 2.8 2.8l-.1.1a1.7 1.7 0 0 0-.3 1.8V9a1.7 1.7 0 0 0 1.5 1H21a2 2 0 1 1 0 4h-.1a1.7 1.7 0 0 0-1.5 1z"/>',
  checkmark: '<path d="M5 12.5l4.5 4.5L19 7"/>',
  "checkmark.circle.fill": 'fill:<path d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20zm-1.3 14.4-4.4-4.4 1.4-1.4 3 3 6-6 1.4 1.4z"/>',
  "exclamationmark.triangle.fill":
    'fill:<path d="M10.3 3.9a2 2 0 0 1 3.4 0l8 13.9A2 2 0 0 1 20 21H4a2 2 0 0 1-1.7-3.2zM11 9v5h2V9zm0 6.5V17h2v-1.5z"/>',
  "tray.and.arrow.down.fill": 'fill:<path d="M11 2h2v7.6l2.3-2.3 1.4 1.4L12 13.4 7.3 8.7l1.4-1.4L11 9.6zM3 14l2-6h1.5l1 2H6.4l-1.2 4H9l1 2h4l1-2h3.8l-1.2-4h-1.1l1-2H19l2 6v5a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>',
  "paperplane.fill": 'fill:<path d="M21.7 2.3 2.8 9.6c-.9.4-.8 1.6.1 1.9l7.3 2.3 2.3 7.3c.3.9 1.5 1 1.9.1zM10.6 12.4l5.9-5.9-4.5 7.3z"/>',
  "arrow.right": '<path d="M4 12h16M14 6l6 6-6 6"/>',
  "arrow.counterclockwise": '<path d="M4 12a8 8 0 1 0 2.5-5.8L4 8.5"/><path d="M4 4v4.5h4.5"/>',
  "plus.forwardslash.minus": '<path d="M6 4v6M3 7h6M15 17h6M19 4L5 20"/>',
  square,
  "checkmark.square.fill": 'fill:<path d="M7 4h10a3 3 0 0 1 3 3v10a3 3 0 0 1-3 3H7a3 3 0 0 1-3-3V7a3 3 0 0 1 3-3zm3.3 12.4 7.1-7.1L16 7.9l-5.7 5.7-2.6-2.6-1.4 1.4z"/>',
};
