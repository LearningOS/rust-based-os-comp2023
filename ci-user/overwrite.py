import argparse
import os

parser = argparse.ArgumentParser()
parser.add_argument("chapter", type=int)
chapter = parser.parse_args().chapter

if 5 >= chapter >= 4:
    os.system("cp overwrite/build-elf.rs ../os/build.rs")
elif chapter < 4:
    os.system("cp overwrite/build-bin.rs ../os/build.rs")

if chapter <= 5:
    os.system("cp overwrite/Makefile-ch3 ../os/Makefile")
elif chapter <= 6:
    os.system("cp overwrite/Makefile-ch6 ../os/Makefile")
    os.system("cp overwrite/easy-fs-fuse.rs ../easy-fs-fuse/src/main.rs")
elif chapter <= 8:
    os.system("cp overwrite/Makefile-ch6 ../os/Makefile")
    os.system("cp overwrite/easy-fs-fuse-ch7.rs ../easy-fs-fuse/src/main.rs")
