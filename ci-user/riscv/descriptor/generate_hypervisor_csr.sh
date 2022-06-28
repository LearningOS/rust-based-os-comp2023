#!/bin/bash
rustc generator.rs
rm -f ../src/register/hypervisorx64/mod.rs;
for i in *.txt; do 
    ./generator <$i > ../src/register/hypervisorx64/`basename -s .txt $i`.rs; 
    echo "pub mod $(basename -s .txt $i);" >> ../src/register/hypervisorx64/mod.rs; 
done
rm -f generator