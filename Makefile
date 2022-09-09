kfs-objs := fs.o
obj-m := kfs.o

PWD := $(shell pwd)
OUTPUT_DIR = ./build
modules:
		make -C /opt/cproject/linux M=$(PWD) LLVM=1 modules $< -o $(OUTPUT_DIR)/$@

clean:
		make -C /opt/cproject/linux M=$(PWD) clean 

