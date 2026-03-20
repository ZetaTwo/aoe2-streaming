# Streaming Overlay for AoE2 Insights Profiles

When casting Age of Empires 2 tournament games, it is usually nice to be able to show the player profiles and statistics from [aoe2insights.com](https://www.aoe2insights.com).
This repository contains custom CSS to allow you to easily fit two player profiles side-by-side in one scene with all the clutter removed.
The way it works is that you have two browser source elements, one for each player's profile.
Then you apply some custom CSS to both of them to remove certain web elements, make the background transparent, and tighten the margins.

Note that this is just a base. You probably want to add a more interesting background, any channel graphics you use, and maybe the logo of the tournament.

- [OBS Studio Instructions](#obs-studio-instructions)
- [XSplit Broadcaster Setup](#xsplit-broadcaster-setup)
- [Notes on Customization](#customization)

## OBS Studio Instructions

### Manual Setup

1. Create a new source of type "Browser Source"
2. Go to the source settings
3. Enter the URL of the player's profile as the URL: `https://www.aoe2insights.com/user/USERID/` (replace `USERID` with the actual player ID)
4. Set the width to half of the width of your canvas (e.g. `960`)
5. Set the height to the full height of your canvas (e.g. `1080`)
6. Copy the contents of the custom `.css` file into the "Custom CSS" box.
7. Click Ok
8. Repeat steps 1-7 for the second player
9. Align the first player's profile with the left side of your canvas
10. Align the second player's profile with the right side of your canvas
11. Click the lock icon on both sources to avoid accidental edits

### Changing Players

To change the players displayed for a new match:

1. Double click the first player's browser source
2. Change the `USERID` in the URL to the new player's profile ID
3. Repeat steps 1-2 for the second player

## XSplit Broadcaster Setup

### Manual Setup

1. Click "Add Source" -> "Webpage"
2. Enter the player's profile URL (`https://www.aoe2insights.com/user/USERID/`) and click "Ok"
3. Right click the newly created source in the list and click "Settings"
4. Under "Display", select custom resolution
5. Set the width to half of the width of your canvas (e.g. `960`)
6. Set the height to the full height of your canvas (e.g. `1080`)
7. Under "Custom Code", check the box for "Use Custom CSS"
8. Click "Edit CSS", paste the contents of the custom CSS file into the box and click "Apply"
9. Repeat steps 1-8 for the second player
10. Align the first player's profile with the left side of your canvas
11. Align the second player's profile with the right side of your canvas
12. Right click each source in the list and click "Lock Position" to avoid accidental edits

## Customization

If you know a bit of CSS, you can tweak the code to suit your specific needs. You can choose to show or hide certain statistics blocks by modifying or adding `display: none !important;` properties in the CSS file. You might also want to adjust the margins or text colors to match your stream's branding.