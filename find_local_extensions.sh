echo 'GL_VERSION_1_0' > local_extensions
glewinfo | sed -ne '/^GL_.*OK\s*$/s/:.*$//p' >> local_extensions

for ext in $(cat local_extensions) ; do
	grep -i "^${ext}\$" extensions | sed -ne '/./s/^/ --cfg /p'
done