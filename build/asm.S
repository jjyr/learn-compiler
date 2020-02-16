.global main
main:
PUSHQ %rbp
MOVQ %rsp, %rbp
SUBQ $40, %rsp
CALLQ read_int
MOVQ %rax, -8(%rbp)
MOVQ $32, -16(%rbp)
MOVQ $10, -24(%rbp)
MOVQ -24(%rbp), %rax
MOVQ %rax, -32(%rbp)
MOVQ -16(%rbp), %rax
ADDQ %rax, -32(%rbp)
MOVQ -8(%rbp), %rax
MOVQ %rax, -40(%rbp)
MOVQ -32(%rbp), %rax
ADDQ %rax, -40(%rbp)
MOVQ -40(%rbp), %rax
MOVQ %rax, %rdi
CALLQ print_int
ADDQ $40, %rsp
MOVQ $0, %rax
POPQ %rbp
retq
