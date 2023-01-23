files_to_process=$(find ./assets/gfx_bmpall/ -type f -name *.bmp)
mkdir -p ./out
for f in ${files_to_process}
  do ./target/release/pngify ${f} ./out/
done
