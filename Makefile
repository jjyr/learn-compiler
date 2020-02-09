CC := cc
CFLAGS := -I . -g

FILES := main.c parser.c error.c ast.c table.c flattern.c select_inst.c ast_printer.c assign_homes.c

build: ${FILES}
	${CC} ${CFLAGS} $^

run: build
	./a.out

clean: 
	rm *.out;
	rm *.o;
