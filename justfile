NAME := "GhostOS"
ARCH := "x86_64"

MODE := "debug"

SYSTEM_RAM := "1G"

IMG_PATH := "./sysroot/boot"
SYSROOT := "./sysroot"

BUILD_FLAGS := ""

bootloader:
    cargo build -p bootloader --target {{ARCH}}-unknown-uefi
    @cp ./target/{{ARCH}}-unknown-uefi/{{MODE}}/bootloader.efi \
        {{SYSROOT}}/boot/efi/boot/bootx64.efi

kernel:
    cargo build -p kernel --target {{ARCH}}-unknown-none
    @cp ./target/{{ARCH}}-unknown-none/{{MODE}}/kernel \
        {{SYSROOT}}/boot/ghost-krnl

build: bootloader kernel

run:
    qemu-system-{{ARCH}} -enable-kvm -m {{SYSTEM_RAM}} \
        -drive if=pflash,format=raw,readonly=on,file=./temp/OVMF.4m.fd \
        -drive format=raw,file=fat:rw:{{IMG_PATH}} \
        -serial stdio -display sdl,full-screen=on 

clean:
    cargo clean
