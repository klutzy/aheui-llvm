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
$(HELLO): rtmain.rs rt $(AHEUI) libREADME.so
	$(RUSTC) -o $@ $< -L . --link-args "-L. -lREADME"

libREADME.so: README.o
	ld -shared -o $@ $<

README.o: README.ll
	$(LLC) -filetype=obj -relocation-model=pic -o $@ $<

README.ll: README $(AHEUI)
	./$(AHEUI) $<


.PHONY: clean
clean:
	rm -f $(AHEUI) $(HELLO) *.o *.ll *.so
