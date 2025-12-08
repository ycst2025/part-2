
.PHONY: run build clean

build:
	mkdir -p out/objs
	make -C out

clean:
	make -C out clean

run: build
	@echo "Hit Ctrl-A → X to exit"
	qemu-system-arm -M realview-eb-mpcore -semihosting -m 128M \
		-nographic -kernel out/asp -cpu cortex-a9 -s
