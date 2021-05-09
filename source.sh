#export PATH=$PATH:/home/meteor/Playground/aarch/gcc-arm-9.2-2019.12-x86_64-aarch64-none-elf/bin/
export RUSTFLAGS="-C llvm-args=-global-isel=false"
PREFIX=aarch64-none-elf
alias objdump="${PREFIX}-objdump"
alias gdb="${PREFIX}-gdb"
