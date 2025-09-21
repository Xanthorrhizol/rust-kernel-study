build:
ifneq ($(wildcard libhello.so), libhello.so)
	gcc -c c_src/hello.c
	gcc -shared -o libhello.so hello.o
endif
	LD_LIBRARY_PATH=. cargo build
