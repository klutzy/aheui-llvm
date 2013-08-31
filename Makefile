RUSTC?=rustc
AHEUI?=aheui
HELLO?=hello
LLC?=llc

.PHONY: all
all: $(HELLO)

$(HELLO): rt.rs $(AHEUI) hello.aheui.o
	$(RUSTC) -o $(HELLO) rt.rs --link-args "hello.aheui.o"

hello.aheui.o:
	./$(AHEUI) && $(LLC) -filetype=obj -o $@ hello.ll

$(AHEUI): aheui.rs
	$(RUSTC) -o $@ $<

.PHONY: clean
clean:
	rm -f $(AHEUI) hello.o $(HELLO)
