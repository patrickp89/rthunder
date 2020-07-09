SHELL = /bin/sh
GCC = gcc -Wall

build_dir = ../target/native
target_dir = ../target/debug

all: clean build

build: .o
	$(GCC) $(build_dir)/libcddb-wrapper.o -shared -o $(target_dir)/libcddb-wrapper.so -L/usr/lib/x86_64-linux-gnu/ -lcdio -lcddb

.o:
	mkdir -p $(build_dir)
	mkdir -p $(target_dir)
	$(GCC) -c libcddb_wrapper.c -o $(build_dir)/libcddb-wrapper.o -fPIC

clean:
	-@$(RM) $(build_dir)/*.o
	-@$(RM) $(target_dir)/*.so
