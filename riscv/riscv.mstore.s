	.file	"mstore.c"
	.option pic
	.text
	.align	1
	.globl	multstore
	.type	multstore, @function
multstore:
	addi	sp,sp,-16
	sd	ra,8(sp)
	sd	s0,0(sp)
	mv	s0,a2
	call	mult2@plt
	sd	a0,0(s0)
	ld	ra,8(sp)
	ld	s0,0(sp)
	addi	sp,sp,16
	jr	ra
	.size	multstore, .-multstore
	.ident	"GCC: (Ubuntu 9.3.0-17ubuntu1~20.04) 9.3.0"
	.section	.note.GNU-stack,"",@progbits
