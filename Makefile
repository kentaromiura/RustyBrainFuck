test:
	 clear && rustc --test ./Tests/ENSI.rs
all:
	clear && rustc ./Source/main.rs
hello:
	make all && ./main ./Source/BF/test.b
