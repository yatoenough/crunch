_default:
    @just --list

# Runs crunch TUI app with provided flags
crunch flags="":
    cargo run -p crunch {{ flags }}

# Runs crunchburn CLI app with provided input WAV file and provided flags
crunchburn input flags="":
    cargo run -p crunchburn -- {{ input }} -o testdata/output.wav {{ flags }}

# Clean target files and testdata
[unix]
clean:
    rm -rf testdata
    cargo clean

# Clean target files and testdata
[windows]
clean:
    Remove-Item -Recurse -Force testdata
    cargo clean
