#!/usr/bin/env python3

from pathlib import Path
from argparse import ArgumentParser
import subprocess


def main():
    parser = ArgumentParser()
    parser.add_argument("day", type=int)
    args = parser.parse_args()

    folder_name = f"day{args.day:02d}"

    if Path(folder_name).exists():
        print(f"Day {args.day} already exists!")
        return

    cargo_sub = subprocess.Popen(
        f"cargo init ./{folder_name} --name {folder_name} --vcs none", shell=True
    )

    if cargo_sub.wait() != 0:
        print("Error occurred creating project")
        return

    subprocess.Popen(f"code ./{folder_name}", shell=True)


if __name__ == "__main__":
    main()
