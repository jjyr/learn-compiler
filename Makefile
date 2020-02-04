CC := cc
CFLAGS := -I . -g

FILES := main.c parser.c error.c ast.c

build: ${FILES}
	${CC} ${CFLAGS} $^

run: build
	./a.out

clean: 
	rm *.out;
	rm *.o;
