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
rt: librt.dummy

librt.dummy: rt.rs
	$(RUSTC) rt.rs && touch librt.dummy


# example usage
$(HELLO): rtmain.rs rt $(AHEUI) README.o
	$(RUSTC) -o $@ $< -L . -C link-args="README.o"

README.o: README.ll
	$(LLC) -filetype=obj -relocation-model=pic -o $@ $<

README.ll: README.txt $(AHEUI)
	./$(AHEUI) -o $@ $<

.PHONY: clean
clean:
	rm -f $(AHEUI) $(HELLO) *.o *.ll *.so *.dummy
