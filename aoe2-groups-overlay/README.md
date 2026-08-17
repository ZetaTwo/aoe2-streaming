# AoE2 Tournament Groups Overlay

When casting Age of Empires 2 tournament games, it is usually nice to be able to show the current standings of the group stages.
This repository provides a web-based overlay that you can easily add as a Browser Source in OBS or XSplit. It fetches real-time standings directly from the tournament's Google Sheets and formats them into a clean, easy-to-read, stream-friendly grid.

- [AoE2 Tournament Groups Overlay](#aoe2-tournament-groups-overlay)
  - [Preview](#preview)
  - [OBS Studio Instructions](#obs-studio-instructions)
  - [XSplit Broadcaster Setup](#xsplit-broadcaster-setup)
  - [Supported Tournaments](#supported-tournaments)
  - [Changing Tournaments \& Brackets](#changing-tournaments--brackets)
  - [Customization](#customization)

## Preview

This is what it can look like once you have set it up:

![OBS AoE2 Groups preview](aoe2groups-overlay.png)

## OBS Studio Instructions

1. Create a new source of type "Browser Source"
2. Go to the source settings
3. Enter the following URL to load a specific tournament and bracket:
   `https://aoe2streaming.zeta-two.com/?tournament=ttlc2&bracket=Tin`
4. Set the width and height to match your canvas or the desired area (e.g. Width `1920` and Height `1080`)
5. Clear the default text in the "Custom CSS" box (unless you plan to add your own styling). The overlay has a transparent background by default.
6. Click Ok
7. Move and resize the browser source on your canvas as needed
8. Click the lock icon on the source to avoid accidental edits

## XSplit Broadcaster Setup

1. Click "Add Source" -> "Webpage"
2. Enter the URL: `https://aoe2streaming.zeta-two.com/?tournament=ttlc2&bracket=Tin` and click "Ok"
3. Right-click the newly created source in the list and click "Settings"
4. Under "Display", select custom resolution
5. Set the width and height to match your canvas or desired area (e.g. `1920x1080`)
6. Right-click the source in the list and click "Lock Position" to avoid accidental edits

## Supported Tournaments

| `?tournament=` | Brackets (use as `?bracket=…`) |
|---|---|
| `ttlc2` | Obsidian, Titanium, Chromium, Nickel, Cobalt, Copper, Zinc, Aluminium, Tin, Lead, Steel, Iron, Brick, Stone, Granite, Slate, Concrete, Gravel, Sand, Ash, Dirt |
| `tcc2` | Champions, Monks, Mangonels, Knights, Pikemen, Longswords, Crossbows, Archers, Militia |
| `tsdc` | Champions, Monks, Mangonels, Knights, Crossbows, Militia |
| `spec2-continental` | Warlord, Conqueror, General, Chieftain, Commander, Warrior, Mercenary, Officer, Watcher, Recruit, Militia |
| `spec2-stronghold` | Palace, Fortress, Castle, Krepost, Donjon, Keep, Guard, Tower, Outpost |
| `spec2-thalassocracy` | Ocean, Coast, Archipelago, Reef, Bay, Channel, Lake, Pond, River, Beach |
| `spec2-nomad` | Trailblazer, Explorer, Wayfarer, Pioneer, Discoverer, Traveller, Pathfinder, Vagabond, Wanderer |

## Changing Tournaments & Brackets

You can change which tournament and bracket is displayed by modifying the URL parameters at the end of the link:
- `tournament`: The ID of the tournament — see the table above.
- `bracket`: The name of the specific bracket/division within that tournament — also from the table above.

For example, to show the Monks bracket for the TCC2 tournament, your URL would be:
`https://aoe2streaming.zeta-two.com/?tournament=tcc2&bracket=Monks`

*Note: If you omit the `bracket` parameter, the overlay will attempt to display all brackets for the tournament at once, which may heavily clutter your screen.*

## Customization

The overlay is designed to look great out-of-the-box on stream, with soft dark colors, drop shadows, and a transparent background to blend over your gameplay. 
However, if you know a bit of CSS, the app is built with highly descriptive class names so you can easily customize it via the "Custom CSS" box in OBS.

Here are some of the key classes you can target:
- `.app-main`: The main container for the whole page.
- `.bracket-title`: The header for the entire bracket.
- `.group-table`: The table element for a specific group.
- `.group-title`: The header for a specific group.
- `.group-table-header-row`, `.group-table-header-rank`, etc.: The header rows and specific column headers.
- `.player-standing-row`: The row containing a specific player's stats.
- `.player-standing-name`, `.player-standing-sets`, etc.: Individual stat cells for a player.

For example, to make the player names yellow, you could add this to the Custom CSS box in OBS:
```css
.player-standing-name {
  color: #ffd700 !important;
}
```
