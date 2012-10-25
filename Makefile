all:
	@mkdir -p lib
	rustc src/glcore.rc --out-dir lib --lib

clean:
	rm -R -f lib