crunchburn input flags="":
    cargo run -p crunchburn -- {{ input }} -o testdata/output.wav {{ flags }}

[unix]
clean:
    rm -rf testdata
    cargo clean
