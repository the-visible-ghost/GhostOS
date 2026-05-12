NAME := "GhostOS"
ARCH := "x86_64"

SYSTEM_RAM := "1G"

IMG_PATH := "./sysroot/boot"
SYSROOT := "./sysroot"

BUILD_FLAGS := ""

bootloader:
    cargo build -r -p bootloader \
        --target {{ARCH}}-unknown-uefi \
        --config ./bootloader/.cargo/config.toml
    @cp ./target/{{ARCH}}-unknown-uefi/release/bootloader.efi \
        {{SYSROOT}}/boot/efi/boot/bootx64.efi

kernel:
    cargo build -r -p kernel \
        --target {{ARCH}}-unknown-none \
        --config ./kernel/.cargo/config.toml
    @cp ./target/{{ARCH}}-unknown-none/release/kernel \
        {{SYSROOT}}/boot/ghost-krnl

build: bootloader kernel

run:
    qemu-system-{{ARCH}} -enable-kvm -m {{SYSTEM_RAM}} \
        -drive if=pflash,format=raw,readonly=on,file=./temp/OVMF.4m.fd \
        -drive format=raw,file=fat:rw:{{IMG_PATH}} \
        -serial stdio -display sdl,full-screen=on  \
        -qmp tcp:localhost:4444,server,nowait -d guest_errors,int

clean:
    cargo clean
