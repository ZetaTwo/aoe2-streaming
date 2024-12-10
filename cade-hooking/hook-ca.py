#!/usr/bin/env python3

import sys
from typing import Optional

import frida
import psutil

with open("ca-hooks.js", "r") as fin:
    script_js = fin.read()


def get_captureage_process() -> Optional[psutil.Process]:
    for process in psutil.process_iter():
        if process.name() == "CaptureAge.exe":
            if len(process.cmdline()) == 1:
                return process
    return None


def on_message(message, data):
    if message["type"] == "send" and (send :=
                                      message["payload"])["type"] == "sound":
        print(f'[+] Sound event: {send["event"]}')
    else:
        print("[%s] => %s" % (message, data))


def main():
    print("[+] Finding CaptureAge process")
    captureage_process = get_captureage_process()
    if not captureage_process:
        print("[-] CaptureAge not found, make sure it is running")
        sys.exit(1)

    print("[+] Attaching to CaptureAge process")
    session = frida.attach(captureage_process.pid)
    script = session.create_script(script_js)
    script.on("message", on_message)
    script.load()
    print("[!] Ctrl+Z to exit.\n\n")
    sys.stdin.read()
    session.detach()


if __name__ == "__main__":
    main()
