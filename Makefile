test:
	 clear && rustc --test ./Tests/ENSI.rs -o ./Bin/ENSI
all:
	clear && rustc ./Source/main.rs -o ./Bin/main
hello:
	make all && ./Bin/main ./Source/BF/test.b
