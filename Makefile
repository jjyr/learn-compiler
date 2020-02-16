CC := cc
CFLAGS := -I . -g
SRC := src
BUILD := build

FILES := main.c parser.c error.c ast.c table.c flattern.c select_inst.c ast_printer.c assign_homes.c patch_inst.c asm_printer.c uniquify.c partial_eval.c
FILES := $(addprefix ${SRC}/, ${FILES})

build: ${FILES}
	${CC} ${CFLAGS} -o ${BUILD}/compiler $^

runtime: ${SRC}/runtime.c
	${CC} ${CFLAGS} -c -o ${BUILD}/runtime.o $^

run: build
	${BUILD}/compiler

clean: 
	rm ${BUILD}/*
