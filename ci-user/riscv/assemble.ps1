New-Item -Force -Name bin -Type Directory

# remove existing blobs because otherwise this will append object files to the old blobs
Remove-Item -Force bin/*.a

$crate = "riscv"

riscv64-unknown-elf-gcc -c -mabi=ilp32 -march=rv32i asm.S -o bin/$crate.o
riscv64-unknown-elf-ar crs bin/riscv32i-unknown-none-elf.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=ilp32 -march=rv32ic asm.S -o bin/$crate.o
riscv64-unknown-elf-ar crs bin/riscv32ic-unknown-none-elf.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=lp64 -march=rv64i asm.S -o bin/$crate.o
riscv64-unknown-elf-ar crs bin/riscv64i-unknown-none-elf.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=lp64 -march=rv64ic asm.S -o bin/$crate.o
riscv64-unknown-elf-ar crs bin/riscv64ic-unknown-none-elf.a bin/$crate.o

Remove-Item bin/$crate.o
