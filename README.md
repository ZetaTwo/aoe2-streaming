# Age of Empires 2 - Stream Draft Settings

When casting Age of Empires 2 tournament games, it is usually nice to be able to show the map and civ drafts from [aoe2cm.net](https://aoe2cm.net).
This repository contains some settings and preset to allow you to easily fit both the map and civ drafts in one scene with all the clutter removed.
The way it works is that you have two browser source elements, one for the map draft and one for the civ draft.
Then you apply some custom CSS to both of them to remove some elements, make the background transparent and tighten the margins.
There are two ways to set this up: importing the pre-made scene collection or following the step-by-step guide to set it up yourself.

Note that this is just a base. You probably want to add a more interesting background, any channel graphics you use and maybe the logo of the tournament.

- [OBS Studio Instructions](#obs-studio-instructions)
- [XSplit Broadcaster Setup](#xsplit-broadcaster-setup)

## Preview

This is what it can look like once you have set it up:

![OBS AoE2 Draft preview](aoe2cm-obs.png)

## OBS Studio Instructions

### Import Setup

1. Download [aoe2cm-obs.json](aoe2cm-obs.json)
2. Go to OBS
3. Click "Scene collection" -> "Import"
4. Click the three dots
5. Select the "aoe2cm-obs.json" file and click "Open"
6. Check that the name column lists "AoE2" and the "Discovered Applications" column lists "OBS Studio"
7. Click "Import"

### Manual Setup

1. Create a new source of type "Browser Source"
2. Go to the source settings
3. Enter the URL of the map draft as the URL
4. Set the width to the full width of your canvas (e.g. 1920)
5. Set the height to half of the height of your canvas (e.g. 1080/2 = 540)
6. Copy the contents of [aoe2cm-obs.css](aoe2cm-obs.css) into the "Custom CSS" box.
7. Click Ok
8. Repeat step 1-7 for the civ draft
9. Align the map draft with the top of your canvas
10. Align the civ draft with the bottom of your canvas
11. Click the lock icon on both sources to avoid accidental edits

### Changing Drafts

To change the draft:

1. Double click map draft source
2. Change the URL to your new draft
3. Repeat step 1-2 for the civ draft

## XSplit Broadcaster Setup

### Manual Setup

1. Click "Add Source" -> "Webpage"
2. Enter the map draft URL and click "Ok"
3. Right click the newly created source in the list and click "Settings"
4. Under "Display", select custom resolution
5. Set the width to the full width of your canvas (e.g. 1920)
6. Set the height to half of the height of your canvas (e.g. 1080/2 = 540)
7. Under "Custom Code", check the box for "Use Custom CSS"
8. Click "Edit CSS", paste the contents of [aoe2cm-obs.css](aoe2cm-obs.css) into the box and click "Apply"
9. Repeat step 1-8 for the civ draft
10. Align the map draft with the top of your canvas
11. Align the civ draft with the bottom of your canvas
12. Right click each source in the list and click "Lock Position" to avoid accidental edits
