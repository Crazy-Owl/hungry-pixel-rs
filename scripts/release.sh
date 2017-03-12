cargo build --release
mkdir release
cp target/release/hungry-pixel-rs ./release
cp scripts/run.sh ./release
chmod +x ./release/run.sh
cp /usr/lib/x86_64-linux-gnu/libSDL2_ttf-2.0.so.0 ./release
cp /usr/lib/x86_64-linux-gnu/libSDL2_image-2.0.so.0 ./release
cp /usr/local/lib/libSDL2-2.0.so.0 ./release
cp -R resources ./release
tar -zcvf hungry-pixel-rs-1.0.0-linux-64bit.tar.gz ./release
rm -rf ./release
