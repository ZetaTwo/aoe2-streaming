# Capture Age Event Hooking

This proof-of-concept script will hook Capture Age and monitor for sound events such as the chat ping and taunts.
The idea is that this could be used to trigger on-stream overlays etc. in reaction to those events.
The events are handled in the `on_message` function in the Python script. This is where you would need to add functionality to react to the events and do something interesting.

Usage:
```
python -m pip install -r requirements.txt
python hook-ca.py
```
