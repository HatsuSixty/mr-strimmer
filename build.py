#!/bin/env python3

from os import mkdir
from os.path import isfile, isdir, basename
from shlex import join
from shutil import copy2
from subprocess import run
from sys import argv, stderr, stdout

def mkdir_ine(dirr):
    print(f"[INFO] Creating directory `./{dirr}`...")
    if not isdir(f"./{dirr}"):
        mkdir(f"./{dirr}")

def run_cmd(command):
    shell_formatted_command = join(command)
    print(f"[CMD] {shell_formatted_command}")
    returnc = run(command).returncode
    if returnc != 0:
        print(f"ERROR: Command `{shell_formatted_command}` exited with code `{returnc}`")
        exit(1)

def build_binary(binary, release):
    cmd = ["cargo", "build", f"--manifest-path=./{binary}/Cargo.toml"]
    if release:
        cmd += ["--release"]
    run_cmd(cmd)

    bin_path = ""
    if release:
        bin_path = f"./{binary}/target/release/{binary}"
    else:
        bin_path = f"./{binary}/target/debug/{binary}"

    mkdir_ine("bin")

    try:
        copy2(bin_path, "./bin/" + basename(bin_path))
    except FileNotFoundError:
        print(f"ERROR: Could not find `{bin_path}` executable (release = `{release}`)", file=stderr)
        exit(1)

def build_project(run, release):
    release_args = []
    if release:
        release_args = ["--release"]

    run_cmd(["cargo", "build"] + release_args)
    if run:
        run_cmd(["cargo", "run"] + release_args)

def prepare_assets():
    print("[INFO] Preparing assets...")

    mkdir_ine("assets")

    if isfile("./text-rs/Cantarell.ttf"):
        copy2("./text-rs/Cantarell.ttf", "./assets/Cantarell.ttf")
    else:
        print("ERROR: Could not find file `Cantarell.ttf`", file=stderr)
        exit(1)

def usage(stream, myself):
    print(f"USAGE: {myself} <SUBCOMMAND>", file=stream)
    print("  SUBCOMMANDs:", file=stream)
    print("    clean       Clean up project, but dont remove `target/` folder", file=stream)
    print("    clean_all   Clean up everything, even the `target/` folder", file=stream)
    print("    build       [SUBCOMMAND]", file=stream)
    print("      SUBCOMMANDs:")
    print("        release            Build the project with `--release` flag")
    print("        run [SUBCOMMAND]   Runs the bot after successful compilation")
    print("        SUBCOMMANDs:")
    print("          release   Runs the bot executable that was compiled with `--release` flag")
    print("    help        Prints this help", file=stream)

args = argv
if len(args) > 1:
    if args[1] == "clean":
        run_cmd(["find", ".", "-iname", "*~", "-exec", "rm", "{}", "+"])
        run_cmd(["rm", "-rf", "assets/"])
        run_cmd(["rm", "-rf", "bin/"])
    elif args[1] == "clean_all":
        run_cmd(["find", ".", "-iname", "*~", "-exec", "rm", "{}", "+"])
        run_cmd(["rm", "-rf", "assets/"])
        run_cmd(["rm", "-rf", "bin/"])
        run_cmd(["git", "clean", "-fdx"])
    elif args[1] == "build":
        runn = False
        release = False
        if len(args) > 2:
            if args[2] == "run":
                runn = True
                if len(args) > 3:
                    if args[3] == "release":
                        release = True
                    else:
                        print(f"ERROR: Unknown subcommand: {args[3]}", file=stderr)
                        usage(stderr, args[0])
                        exit(1)
            elif args[2] == "release":
                release = True
            else:
                print(f"ERROR: Unknown subcommand: {args[2]}", file=stderr)
                usage(stderr, args[0])
                exit(1)
        build_binary("image-rs", release)
        build_binary("text-rs", release)
        build_binary("webcam-rs", release)
        prepare_assets()
        build_project(runn, release)
    elif args[1] == "help":
        usage(stdout, args[0])
        exit(0)
    else:
        print(f"ERROR: Unknown subcommand: {args[1]}", file=stderr)
        usage(stderr, args[0])
        exit(1)
else:
    print("ERROR: No subcommand was provided", file=stderr)
    usage(stderr, args[0])
    exit(1)
