RUSTC?=rustc
AHEUI?=aheui
HELLO?=hello
LLC?=llc

.SUFFIXES: .aheui .o .ll .rs

.PHONY: all
all: $(HELLO)

$(HELLO): rtmain.rs rt.rs $(AHEUI) libhello.aheui.so
	$(RUSTC) rt.rs && $(RUSTC) -o $@ rtmain.rs -L .

libhello.aheui.so: hello.aheui.o
	ld -shared -o $@ $<

hello.aheui.o: hello.aheui.ll
	$(LLC) -filetype=obj -relocation-model=pic -o $@ $<

hello.aheui.ll: hello.aheui $(AHEUI)
	./$(AHEUI) hello.aheui

$(AHEUI): aheui.rs
	$(RUSTC) -o $@ $<

.PHONY: clean
clean:
	rm -f $(AHEUI) $(HELLO) *.o *.ll *.so
