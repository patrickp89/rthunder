SHELL = /bin/sh
GCC = gcc -Wall

build_dir = ../target/native
target_dir = ../target/debug

all: clean build

build: .o
	$(GCC) $(build_dir)/libcdio-wrapper.o -shared -o $(target_dir)/libcdio-wrapper.so -L/usr/lib/x86_64-linux-gnu/ -lcdio
#	$(GCC) $(build_dir)/libcddb-wrapper.o -shared -o $(target_dir)/llibcddb-wrapper.so -L/usr/lib/x86_64-linux-gnu/ -lcdio -lcddb

.o:
	mkdir -p $(build_dir)
	mkdir -p $(target_dir)
	$(GCC) -c libcdio_wrapper.c -o $(build_dir)/libcdio-wrapper.o -fPIC
#	$(GCC) -c libcddb_wrapper.c -o $(build_dir)/libcddb-wrapper.o -fPIC

clean:
	-@$(RM) $(build_dir)/*.o
	-@$(RM) $(target_dir)/*.so
