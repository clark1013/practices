	.file	"local_exchange.c"
	.option pic
	.text
	.align	1
	.globl	local_exchange
	.type	local_exchange, @function
local_exchange:
	mv	a5,a0
	ld	a0,0(a0)
	sd	a1,0(a5)
	ret
	.size	local_exchange, .-local_exchange
	.ident	"GCC: (Ubuntu 9.3.0-17ubuntu1~20.04) 9.3.0"
	.section	.note.GNU-stack,"",@progbits
