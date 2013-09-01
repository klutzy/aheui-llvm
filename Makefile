RUSTC?=rustc
AHEUI?=aheui
HELLO?=hello
LLC?=llc

.SUFFIXES: .aheui .o .ll .rs

.PHONY: all
all: $(HELLO)

$(HELLO): rt.rs $(AHEUI) hello.aheui.o
	$(RUSTC) -o $@ $< --link-args "hello.aheui.o"

hello.aheui.o: hello.aheui.ll
	$(LLC) -filetype=obj -o $@ $<

hello.aheui.ll: hello.aheui $(AHEUI)
	./$(AHEUI) hello.aheui

$(AHEUI): aheui.rs
	$(RUSTC) -o $@ $<

.PHONY: clean
clean:
	rm -f $(AHEUI) $(HELLO) *.o *.ll
