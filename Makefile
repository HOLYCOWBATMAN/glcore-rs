all:
	@mkdir -p lib
	rustc src/glcorearb.rc --out-dir lib --lib

clean:
	rm -R -f lib