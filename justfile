NAME := "GhostOS"
ARCH := "x86_64"

MODE := "debug"

SYSTEM_RAM := "1G"

IMG_PATH := "./sysroot/boot"
SYSROOT := "./sysroot"

BUILD_FLAGS := ""

build crate target:
    cargo build -p {{crate}} --target {{ARCH}}-{{target}} {{BUILD_FLAGS}} 

install:
    cp ./target/{{ARCH}}-unknown-uefi/{{MODE}}/bootloader.efi \
        {{SYSROOT}}/boot/efi/boot/bootx64.efi
    # cp ./target/{{ARCH}}-unknown-none/{{MODE}}/kernel \
    #     {{SYSROOT}}/boot/ghost-krnl

run: 
    qemu-system-{{ARCH}} -enable-kvm -m {{SYSTEM_RAM}} \
        -drive if=pflash,format=raw,readonly=on,file=./temp/OVMF.4m.fd \
        -drive format=raw,file=fat:rw:{{IMG_PATH}} \
        -serial stdio -display sdl,full-screen=on 

clean:
    cargo clean
