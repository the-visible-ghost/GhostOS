default rel
global _start
bits 64

extern bootstrap
extern kernel_main


section .text
_start:
    lea rsp, [rel __stack_bottom]
    ; push rdi
    push [rdi]

    sub rsp, 16
    lea rax, [__pt_arena]
    mov [rsp], rax
    mov [rsp + 8], __pt_arena_size
    mov rsi, rsp
    lea rax, [rel bootstrap]
    call rax ; calls bootstrap
    add rsp, 16

    ; lea rsp, __stack_bottom
    ; mov cr3, rax ; takeover paging
    jmp $

    ; pop rdi
    ; mov rsp, __stack_bottom
    ; mov rax, kernel_main
    ; jmp kernel_main

    pop rdi                 ; framebuffer ptr
    lea rax, [rel fill_white]
    call rax
    jmp $

fill_white:
    mov rax, rdi
    mov rcx, 0x1FA400       ; 1920 x 0180
    shl rcx, 2

    .loop:
    mov qword [rax], 0xFFFFFFFFFFFFFFFF
    add rax, 8
    sub rcx, 8
    jnz .loop
    ret

section .bss

; Page Table Arena
align 0x1000
__pt_arena: resb 0xF00000
__pt_arena_size equ $ - __pt_arena

; Kernel Stack
align 16
__stack_top: resb 0x100000
__stack_bottom:
