RUSTC?=rustc
AHEUI?=aheui
HELLO?=hello
LLC?=llc

.SUFFIXES: .aheui .o .ll .rs


.PHONY: all
all: $(AHEUI)

$(AHEUI): aheui.rs
	$(RUSTC) -o $@ $<

.PHONY: rt
rt: rt.rs
	$(RUSTC) rt.rs


# example usage
$(HELLO): rtmain.rs rt $(AHEUI) libhello.aheui.so
	$(RUSTC) -o $@ $< -L .

libhello.aheui.so: hello.aheui.o
	ld -shared -o $@ $<

hello.aheui.o: hello.aheui.ll
	$(LLC) -filetype=obj -relocation-model=pic -o $@ $<

hello.aheui.ll: hello.aheui $(AHEUI)
	./$(AHEUI) hello.aheui


.PHONY: clean
clean:
	rm -f $(AHEUI) $(HELLO) *.o *.ll *.so
