format ELF64 executable 

segment readable executable

main:
	mov	rdx,msg_size	; CPU zero extends 32-bit operation to 64-bit
				; we can use less bytes than in case mov rdx,...
	lea	rsi,[msg]
	mov	rdi,1		; STDOUT
	mov	rax,1		; sys_write
	syscall

	xor	rdi,rdi 	; exit code 0
	mov	rax,60		; sys_exit
	syscall

segment readable writeable

msg db 'Hello 64-bit world!',0xA
msg_size = $-msg
