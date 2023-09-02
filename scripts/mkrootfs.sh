#! /bin/bash
# pass dockerfile as $1
lang="$1"

# check if command line argument is there
if [ -z "$1" ]; then
    echo "provide language for which to build rootfs"
    exit 0
fi

all=("rust" "python" "java" "bash" "sh", "haskell")

if [[ $lang == "all" ]] then
    echo "building rootfs for all languages"
    for l in "${all[@]}"; do
        echo "building rootfs for $l"
        ./$0 $l
    done
    exit 0
fi

# rootfs sizes for each language
case $lang in
"rust")
    size=1000 #large because cargo moment - TODO make smaller
    ;;
"python")
    size=100
    ;;
"java")
    size=400
    ;;
"bash")
    size=50
    ;;
"sh")
    size=20
    ;;
"haskell")
    size=1400 #large because we have to install gcc and lots of build/toolchain shite
    ;;
*)
    echo "unrecognised language $1"
    exit 0
    ;;
esac


# build pyrod
cargo build --release --bin=pyrod --target=x86_64-unknown-linux-musl

dd if=/dev/zero of=rootfs.ext4 bs=1M count=$size
/usr/sbin/mkfs.ext4 rootfs.ext4


sudo rm -rf /tmp/rootfs && mkdir /tmp/rootfs
sudo mount rootfs.ext4 /tmp/rootfs

sudo docker build --no-cache . -f "scripts/images/Dockerfile.$lang" -t "pyro-$lang"
sudo docker run -it --rm -v /tmp/rootfs:/rootfs "pyro-$lang"

sudo umount /tmp/rootfs
mv rootfs.ext4 "resources/rootfs-$lang.ext4"